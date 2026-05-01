var e=`前段时间买了一个树莓派5以后，我发现它既有网卡，又有无线功能。在简单折腾了点亮二极管以及 yolo 物体识别这些树莓派常规玩法之后，我突然想起来以前在一些平台看见的软路由玩法，似乎是可以实现全局的代理。于是我就开始了这一次的折腾。
查阅了一些资料以后，我得知这种经由软路由的代理方式叫做透明代理。而通过网络中的某一个设备，而非主路由器的处理又叫做旁路由。

安装系统和 ssh 连接就不谈了，毕竟每个涉及树莓派的介绍当中都会把这个过程重复一遍。我是直接使用 macOS 自带的终端，用 ssh 来连接树莓派的。
**⚠️注意：本操作过程全部在原生树莓派系统内完成，没有安装第三方镜像，也未使用docker，因此部分配置可能比较繁琐。**
# mihomo配置
为了稳定性，我们使用网线连接树莓派。看到黄灯常亮就是连上了。
![[IMG_20250715_133205.jpg]]
不讲别的什么网络结构，首先肯定要先在这个树莓派上把代理给跑起来。我不想使用openWRT 之类的专用系统，因为我可能还要同时跑其他东西，我现在只有一张tf卡，所以我选择直接在树莓派本身的系统上进行配置。
## 下载和运行mihomo服务
如果用 GUI 应用的话，也许资源占用会比较大。所以我想直接使用 mihomo 内核来运行代理。
由于树莓派的芯片是 arm 架构的，而我安装的是 64 位的 RaspberryPiOS 官方系统，基于 Debian。因此，我下载了 Github Release 中的 deb 包来安装：
\`\`\`bash
wget https://github.com/MetaCubeX/mihomo/releases/download/v1.19.11/mihomo-linux-arm64-v1.19.11.deb
sudo apt install ./mihomo-linux-arm64-v1.19.11.deb
\`\`\`
安装完成后，敲一句\`which mihomo\`可以得知，mihomo 内核所在位置为\`/usr/bin/mihomo\`。然后，就可以参考其文档给出的[创建运行服务](https://wiki.metacubex.one/startup/service/)的方法创建运行服务。
首先我们需要先将 yaml 格式的配置文件准备好，一开始可以直接照搬已有的配置文件。把配置文件命名为\`config.yaml\`，放在\`/etc/mihomo/\`目录下即可。然后创建创建 systemd 配置文件 \`/etc/systemd/system/mihomo.service\`，注意 ExecStart 后面写的应该是前面得知的 mihomo 所在路径：
\`\`\`ini
[Unit]
Description=mihomo Daemon, Another Clash Kernel.
After=network.target NetworkManager.service systemd-networkd.service iwd.service

[Service]
Type=simple
LimitNPROC=500
LimitNOFILE=1000000
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_RAW CAP_NET_BIND_SERVICE CAP_SYS_TIME CAP_SYS_PTRACE CAP_DAC_READ_SEARCH CAP_DAC_OVERRIDE
AmbientCapabilities=CAP_NET_ADMIN CAP_NET_RAW CAP_NET_BIND_SERVICE CAP_SYS_TIME CAP_SYS_PTRACE CAP_DAC_READ_SEARCH CAP_DAC_OVERRIDE
Restart=always
ExecStartPre=/usr/bin/sleep 1s
ExecStart=/usr/bin/mihomo -d /etc/mihomo
ExecReload=/bin/kill -HUP $MAINPID

[Install]
WantedBy=multi-user.target
\`\`\`
使用\`systemctl daemon-reload\`重新加载 systemd，再依次使用\`systemctl enable mihomo\`和\`systemctl start mihomo\`启用 mihomo 服务并立即启动 mihomo。
随后，使用\`systemctl status mihomo\`检查 mihomo 的运行状况，应该会看到服务已经启动，但是有可能看到日志停留在下面这样一行：
\`\`\`log
INFO[2025-07-15T10:54:05.468932000+08:00] Can't find MMDB, start download
\`\`\`
这是因为它在尝试从 GitHub 下载 MMDB\xA0的步骤，而下载不下来，导致服务卡在这一步。我们可以先使用\`systemctl stop mihomo\`停止服务。这时候，其实只需要让我们的配置目录\`/ect/mihomo/\`内有一个 geoip.metadb 的文件，有两种解决办法。
第一种是预先在局域网内的其他设备上跑一个代理，可以在我 IP 为 192.168.1.101 的电脑上先跑一个代理，开启局域网共享，随后在树莓派终端输入下面的命令经过代理，其中的 IP 和端口需要根据已开服务的设备的设置来写：
\`\`\`bash
export https_proxy=http://192.168.1.101:7890 http_proxy=http://192.168.1.101:7890 all_proxy=http://192.168.1.101:7890
\`\`\`
然后，手动运行\`mihomo -d /etc/mihomo\`，等待它下载完成顺利启动，就可以按下 \`^C\`（control + C）中止了。最后，再重新使用\`systemctl start mihomo\`启动服务，就会发现服务顺利运行。
另外，也可以先手动从\`https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geoip.metadb\`下载一个，放进配置目录，再开启服务，具体怎么放，可以用 scp，也可以用其他局域网传输工具。
## 开启 TUN 转发
光运行起来还不够，我们需要让 mihomo 开启 TUN 转发，才能在后续作为网关使用。这里我参考的是一个 YouTube 视频 https://youtu.be/WXuuOm8dQtw?si=gx9f2pazgJ2_v2Yq 中给出的配置文件。最主要的是下面这些内容，其他的就省略了吧，有需要的可以去看原视频，原有的 proxies 和 rules 都不用动：
\`\`\`yaml
interface-name: eth0
# 监听地址，"*"表示监听所有地址，这样局域网内其他设备才能连接
bind-address: "*"
mode: rule
# 是否允许来自局域网的连接（开启后，其他设备可以通过你的电脑上网）
allow-lan: true
dns:
\xA0 enable: true
\xA0 listen: 0.0.0.0:53
\xA0 ipv6: false 
\xA0 default-nameserver:
\xA0 \xA0 - 223.5.5.5\xA0 # 阿里DNS
\xA0 \xA0 - 8.8.8.8
# TUN模式设置
tun:
\xA0 enable: true # 开启后所有流量都会通过
\xA0 stack: system # 使用系统网络栈
\xA0 auto-route: true
\xA0 auto-detect-interface: true
\xA0 dns-hijack:
\xA0 \xA0 - tcp://any:53\xA0 # 接管所有DNS请求，防止DNS泄露
\xA0 \xA0 - udp://any:53\xA0 # 接管所有DNS请求，防止DNS泄露
\`\`\`
为了方便管理，我们也可以添加一个网页后台。在配置文件中添加如下内容：
\`\`\`yaml
# 控制面板访问地址和端口，可以通过浏览器访问查看连接状态
external-controller: 0.0.0.0:9090
# MetaCubeXd网页界面
external-ui: ui
\`\`\`
并将 UI 网页内容克隆到 ui 目录：
\`\`\`bash
git clone https://github.com/metacubex/metacubexd.git -b gh-pages /etc/mihomo/ui
\`\`\`
完成上面的配置后，我们在局域网内访问\`http://树莓派的地址:9090/ui/\`即可进入后台。然后，后端地址处继续填写树莓派的地址。例如，我这里保留了树莓派的默认主机名，因此可通过\`http://raspberrypi.local:9090\`来访问。
![[截屏2025-07-15 13.26.08.png]]
在配置中如果看到 TUN 转发已开启，且出站接口名称为eth0（树莓派默认的网口），就应该配置好了。因为我安装的树莓派系统是带桌面的，所以我可以用VNC连接过去，在树莓派桌面上开一个浏览器，可以通过浏览器检查是否可以顺利访问网页。
# 树莓派作为其他局域网设备的网关
要让这个树莓派作为其他设备的网关，我们需要先运行下面的命令，开启树莓派的IPv4转发功能，并使得重启后依然保持开启。
\`\`\`bash
echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.conf && sudo sysctl -p
\`\`\`
另外，为了让其他设备方便访问，我们需要手动设置这个树莓派的IP，而非使用DHCP获取。
我一开始是在树莓派桌面的编辑连接里面设置的。将IPv4设置为手动，地址得根据家里的网段来写。我家里的网都在192.168.1.0/24之下，所以我就填了192.168.1.64，子网掩码255.255.255.0，网关192.168.1.1（即我家做路由模式的光猫的地址）。这样，树莓派重启以后就会是固定的 IP。
当时树莓派的网线的另一端连在我房间的路由器上，而我的手机也通过WiFi连在这个路由器上。于是，我便更改手机的网络设置——先切换为静态，IP地址手动设置一个同网段下的，路由器写树莓派的IP，前缀长度相当于子网掩码填24，DNS随便写一个。
![[Screenshot_2025-07-15-14-41-07-436_com.android.se.jpg]]
确定之后，我的手机就顺利通过树莓派作为网关了。测试发现，能够正常上网。
但是，如果每个设备都要手动设置一遍，那还不如设置代理，用网关的意义就不是很大。所以接下来，我对家里的网络拓扑结构做了重组。
# 网络拓扑结构优化
我家里的网络结构是这样的：光纤入户连接光猫，光猫开启了路由模式以及DHCP服务器。光猫接一个交换机，而这个交换机又接了通往家里各个房间墙面网口的埋线。楼上楼下两个路由器都是通过墙面网口连接自身WAN口。拓扑结构如下图所示：
\`\`\`
[光猫]（路由模式，DHCP开）
   │
[交换机]
 ├── [电视、台式电脑等]
 ├── [路由器A]（WAN口接线，DHCP开）
 │     └── 无线设备接入
 ├── [路由器B]（WAN口接线，DHCP开）
 │     ├── LAN口连接树莓派（静态IP）
 │     └── 无线设备接入

\`\`\`
在这种网络结构下，光猫自身是一层NAT，而两个路由器各自带一层NAT。连接在路由器下的设备就处于两层NAT之下。仅有我连接树莓派的路由器B下的设备才能访问到树莓派，而路由器A下设备以及电视和台式电脑就无法访问到它了。
所以，后来我改成了下面的配置。
\`\`\`
[光猫]（路由模式，DHCP关）
   │
[交换机]
 ├── [电视、台式电脑等]
 ├── [路由器A]（LAN口接线，DHCP开，为全屋唯一DHCP，网关设为树莓派）
 │     └── 无线设备接入
 ├── [路由器B]（LAN口接线，DHCP关）
 │     ├── 无线设备接入
 │     └── [树莓派]（运行mihomo的TUN转发）

\`\`\`
在这种结构下，路由器 A/B 均改为 **AP 模式** 或类交换机（LAN 接入 + DHCP 关闭），避免 NAT 层叠导致的设备互访受限、端口转发困难；所有设备无论通过哪个路由器连接，抑或直接连接交换机，都在同一个网段中（192.168.1.0/24）。统一由路由器 A 提供 DHCP 服务：所有设置为DHCP自动获取IP的接入设备都能获得一个统一的默认网关 —— 设为 **树莓派的 IP**。
具体操作方式是，使用路由器的LAN口连接墙面接口，WAN口留口，其余LAN口连接台式电脑等。这样一来，这两个路由器的NAT便不复存在，全屋的设备都处于同一网段，可相互通信。
![[IMG_20250715_155025.jpg]]
然后，选择其中一个路由器，开启DHCP服务，将其网关设置为树莓派的静态IP地址。由于DHCP是广播机制，如此一来，所有设备在连接到家中任何一处的网络后，都会通过这个DHCP获取IP。上网时，数据会先发给树莓派，经过其运行的 mihomo 服务（TUN 模式）再发往公网，实现透明代理。
![[截屏2025-07-15 16.01.36.png]]
# 树莓派AP热点
完成上面这番操作之后，我看着接在路由器下面的树莓派，心想树莓派也有无线模块，那理论上树莓派自己就可以当做一个WiFi发射器，不一定非得给它挂在某个路由器下面。
于是，我又进行了一番搜索，找到了 [RaspAP](https://github.com/RaspAP/raspap-webgui) 项目。使用下面的脚本来一键安装：
\`\`\`bash
curl -sL https://install.raspap.com | bash
\`\`\`
根据提示安装完成后，RaspAP 会在80端口开放控制网页。直接在浏览器访问树莓派的主机名即可进入控制网页。RaspAP 的默认用户名和密码分别为\`admin\`和\`secret\`。默认处于 NAT 模式，我来到 WLAN 热点设置-高级，开启桥接 AP 模式，然后设置SSID和密码与原有 WiFi 相同。
需要注意的是，RasAP 会新增一个 br0 网桥，并且让网络设置采用\`dhcpd\`管理。在这之后，我们的出口网络设备就不是一开始的\`eth0\`了。所以我们需要到 mihomo 的配置文件中更改\`interface-name\`为\`br0\`。
默认 br0 是使用 DHCP 获取 IP 的。要将它设置为静态 IP，需要使用\`/etc/dhcpcd.conf\` 手动指定。这里我保持与原来对 th0 的 IP 设置相同。
\`\`\`conf
interface br0
static ip_address=192.168.1.64/24
static routers=192.168.1.1
static domain_name_servers=223.5.5.5 8.8.8.8
\`\`\`
最后，网络拓扑结构变成了下面这样：
\`\`\`
[光猫]（路由模式，DHCP关）
   │
[交换机]
 ├── [电视等]
 ├── [路由器A]（LAN口接线，DHCP开，为全屋唯一DHCP，网关设为树莓派）
 │     └── 无线设备接入
 ├── [路由器B]（LAN口接线，DHCP关）
 │     ├── 无线设备接入
 │     └── 台式电脑
 ├── [树莓派]（eth0接入交换机，静态IP）
 │  （运行mihomo的TUN转发，同时wlan0启用RaspAP桥接模式，发射WiFi热点）
 │     └── 无线设备接入
\`\`\`
完美！`;export{e as default};