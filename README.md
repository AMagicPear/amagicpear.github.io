# Perry Home
本项目是我的新版个人主页。进入[Perry Home](https://amagicpear.top/)查看。

## 开发配置
本项目为 Svelte + Vite + Typescript + WebAssembly 的前端单页应用（SPA）。要在本地开发，首先你需要拥有一个 node.js 环境，并配置好 pnpm。

由于本项目中粒子流计算使用了 WebAssembly 计算。在运行前端开发服务器之前，你需要拥有 rust 工具链，并配置`wasm32-unknown-unknown`的 target。

然后安装`wasm-pack`：
```sh
cargo install wasm-pack
```

进入 `src/lib/wasm-perryhome` 目录，使用以下命令来编译 WebAssembly。编译后无需对文件位置进行更改，保留原样即可。

```sh
wasm-pack build --target web
```

最后，你可以使用以下命令来安装依赖并启动开发服务器：

```sh
pnpm install
pnpm run dev
```


## 致谢
- [Svelte](https://svelte.dev/)
- [NanoFlow](https://github.com/ZTMYO/NanoFlow)
- [阿里妈妈方圆体VF](https://www.alibabafonts.com/#/home)
- [WebAssembly](https://webassembly.org/)
