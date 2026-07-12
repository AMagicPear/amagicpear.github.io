<script lang="ts">
  import { onMount } from "svelte";
  import PerryWaves from "@/components/PerryWaves.svelte";
  import Essay from "@/pages/Essay.svelte";
  import avatar from "@/assets/pics/perry-avatar.webp";
  import coverPyroGenesis from "@/assets/pics/cover_PyroGenesis.webp";
  import coverEchoesphere from "@/assets/pics/cover_Echoesphere.jpeg";

  let menuOpen = $state(false);
  let scrollProgress = $state(0);
  let isEssay = $state(false);

  const featuredWorks = [
    {
      name: "回声之境",
      english: "Echoesphere",
      kind: "本科毕业设计 / 交互装置",
      text: "让声音、实体按钮和 AI 决策共同塑造展厅里的游戏体验。",
      href: "https://github.com/AMagicPear/Echoesphere",
      image: coverEchoesphere,
      className: "echo",
    },
    {
      name: "燧火启明录",
      english: "PyroGenesis",
      kind: "教育游戏 / 叙事设计",
      text: "从燧人取火出发，把中国古代物理做成一趟适合孩子亲手探索的旅程。",
      href: "https://www.bilibili.com/video/BV16pgBz7EMW",
      image: coverPyroGenesis,
      className: "fire",
    },
    {
      name: "创作记录",
      english: "Things I am making",
      kind: "VIDEO / MUSIC / SKETCHBOOK",
      text: "关于游戏、音乐和一些还没想好名字的尝试，都留在持续更新的创作记录里。",
      href: "https://space.bilibili.com/52833994",
      image: "",
      className: "music",
    },
  ];

  const experiments = [
    {
      title: "Perry Hermes",
      label: "RUST / AGENT RUNTIME",
      text: "有工具调用、技能系统、上下文压缩与多平台网关的自主智能体运行时。",
      href: "https://github.com/AMagicPear/perry_hermes",
    },
    {
      title: "Echoesphere · 通信",
      label: "RASPBERRY PI / PHYSICAL",
      text: "读取传感器事件、驱动灯带，并将真实装置接入主体验。",
      href: "https://github.com/AMagicPear/echoesphere-communication",
    },
    {
      title: "Echoesphere · 感知",
      label: "MEDIAPIPE / COMPUTER VISION",
      text: "通过手势和面部 landmark 检测，将视觉事件转成统一的交互输入。",
      href: "https://github.com/AMagicPear/echesphere-mediapipe",
    },
    {
      title: "PCL.Proto",
      label: "FULL STACK / OPEN SOURCE",
      text: "为 PCL 分支生态建立可复用、可扩展的标准化原型。",
      href: "https://github.com/PCL-Community/PCL.Proto",
    },
  ];

  function setRoute() {
    isEssay = window.location.pathname === "/essay";
    menuOpen = false;
  }

  function goHome() {
    if (isEssay) {
      history.pushState({}, "", "/");
      setRoute();
      window.scrollTo({ top: 0, behavior: "instant" });
    }
  }

  function goEssay() {
    history.pushState({}, "", "/essay");
    setRoute();
    window.scrollTo({ top: 0, behavior: "instant" });
  }

  function goTo(id: string) {
    goHome();
    requestAnimationFrame(() =>
      document
        .getElementById(id)
        ?.scrollIntoView({ behavior: "smooth", block: "start" }),
    );
  }

  onMount(() => {
    setRoute();
    const updateProgress = () => {
      const limit = document.documentElement.scrollHeight - window.innerHeight;
      scrollProgress = limit > 0 ? window.scrollY / limit : 0;
    };
    updateProgress();
    window.addEventListener("scroll", updateProgress, { passive: true });
    window.addEventListener("popstate", setRoute);
    return () => {
      window.removeEventListener("scroll", updateProgress);
      window.removeEventListener("popstate", setRoute);
    };
  });
</script>

<svelte:head>
  <title
    >{isEssay
      ? "折腾记录 - AMagicPear"
      : "AMagicPear - Perry's playground"}</title
  >
  <meta
    name="description"
    content="Perry 的个人主页：把技术、音乐和互动体验慢慢做成具体的东西。"
  />
</svelte:head>

<div class="progress" style={`transform: scaleX(${scrollProgress})`}></div>
<header class:essay-header={isEssay} class:open={menuOpen}>
  <button class="brand" aria-label="返回首页" onclick={goHome}>
    <span class="brand-stamp">P</span>
    <span class="brand-name">Perry's playground</span>
  </button>
  <button
    class="menu-button"
    aria-label="切换导航菜单"
    aria-expanded={menuOpen}
    onclick={() => (menuOpen = !menuOpen)}><i></i><i></i></button
  >
  <nav aria-label="主导航">
    <button onclick={() => goTo("works")}>作品</button>
    <button onclick={() => goTo("about")}>关于</button>
    <button onclick={goEssay}>随笔</button>
    <a href="https://github.com/AMagicPear" target="_blank" rel="noreferrer"
      >GitHub ↗</a
    >
  </nav>
</header>

{#if isEssay}
  <main class="essay-page"><Essay onBack={goHome} /></main>
{:else}
  <main>
    <section class="hero" aria-labelledby="hero-title">
      <div class="hero-waves" aria-hidden="true">
        <div class="hero-particle-stage"><PerryWaves /></div>
      </div>
      <div class="hero-wash" aria-hidden="true"></div>
      <div class="hero-copy">
        <p class="kicker">HELLO, I AM PERRY</p>
        <h1 id="hero-title">把想法慢慢<br />做成<span>好玩的东西。</span></h1>
        <p class="hero-intro">
          在技术、音乐和叙事之间游走。喜欢把抽象的好奇心，变成看得见、听得到、能亲手按下的体验。
        </p>
        <div class="hero-actions">
          <button class="round-link" onclick={() => goTo("works")}
            >看看我在做什么 <span>↓</span></button
          >
          <a href="mailto:hello@amagicpear.top" class="email-link"
            >hello@amagicpear.top</a
          >
        </div>
      </div>
      <p class="hero-side-note">
        南京 → 苏州<br />数字媒体 / AI × creative coding
      </p>
    </section>

    <section
      id="works"
      class="works section-shell"
      aria-labelledby="works-title"
    >
      <div class="section-top">
        <div>
          <p class="kicker">SELECTED WORK</p>
          <h2 id="works-title">做过的几个世界</h2>
        </div>
        <p>有的在屏幕里发生，有的需要真实的按钮、光线和声音才会醒来。</p>
      </div>
      <div class="feature-grid">
        {#each featuredWorks as work, index}
          <a
            class={`feature-card ${work.className}`}
            href={work.href}
            target="_blank"
            rel="noreferrer"
          >
            <span class="card-no">0{index + 1}</span>
            {#if work.image}<img src={work.image} alt="" />{:else}<div
                class="music-art"
                aria-hidden="true"
              >
                <i></i><i></i><i></i><b>♪</b>
              </div>{/if}
            <div class="card-copy">
              <p>{work.kind}</p>
              <h3>{work.name}</h3>
              <span>{work.english}</span><b>{work.text}</b>
            </div>
            <span class="card-arrow">↗</span>
          </a>
        {/each}
      </div>
    </section>

    <section class="systems section-shell" aria-labelledby="systems-title">
      <div class="systems-heading">
        <p class="kicker">UNDER THE HOOD</p>
        <h2 id="systems-title">也爱把事情<br />做得<span>更完整。</span></h2>
        <p>
          一个作品不止是一个界面。关于装置、模型、运行时和工具，我也享受把它们一点点接起来。
        </p>
      </div>
      <div class="experiment-list">
        {#each experiments as item, index}
          <a
            href={item.href}
            target="_blank"
            rel="noreferrer"
            class="experiment"
            ><span>0{index + 1}</span>
            <div>
              <p>{item.label}</p>
              <h3>{item.title}</h3>
            </div>
            <b>{item.text}</b><i>↗</i></a
          >
        {/each}
      </div>
    </section>

    <section
      id="about"
      class="about section-shell"
      aria-labelledby="about-title"
    >
      <div class="about-photo">
        <div class="tape">CURRENTLY</div>
        <img src={avatar} alt="Perry 的手绘头像" />
        <p>在苏州，准备开始新的学习阶段。</p>
      </div>
      <div class="about-copy">
        <p class="kicker">A LITTLE ABOUT ME</p>
        <h2 id="about-title">
          不止写代码，<br />也想留下<span>一点感受。</span>
        </h2>
        <p>
          从 Unity 游戏开发、Tauri
          桌面应用到交互装置和视觉设计，我总想试试不同的媒介会不会带来新的表达。
        </p>
        <p>
          业余时间做电子音乐，学习 Blender，也在 B
          站记录创作。这些看似绕远的事情，最后常常会在一个新项目里重新相遇。
        </p>
        <div class="fact-row">
          <span>正在关心</span><b>AI 驱动的交互 / 实时图形 / 音乐智能</b>
        </div>
        <div class="fact-row">
          <span>常用工具</span><b>TypeScript / C# / Python / Unity / Rust</b>
        </div>
      </div>
    </section>

    <section class="notes section-shell" aria-labelledby="notes-title">
      <div class="notes-title">
        <p class="kicker">FIELD NOTES</p>
        <h2 id="notes-title">写下的折腾</h2>
      </div>
      <a
        href="/essay"
        class="note-entry"
        onclick={(event) => {
          event.preventDefault();
          goEssay();
        }}
      >
        <span class="note-date"><b>15</b><small>JUL<br />2025</small></span>
        <div>
          <p>HOMELAB / NETWORK</p>
          <h3>有了树莓派以后，我是如何改造家里的网络的？</h3>
          <span>从 mihomo、TUN 到全屋网络拓扑的一次完整实践。</span>
        </div>
        <i>阅读文章 ↗</i>
      </a>
    </section>

    <section class="closing section-shell">
      <p class="kicker">SAY HELLO</p>
      <h2>有意思的想法，<a href="mailto:hello@amagicpear.top">来聊聊。</a></h2>
      <div>
        <a href="mailto:hello@amagicpear.top">hello@amagicpear.top</a><a
          href="https://github.com/AMagicPear"
          target="_blank"
          rel="noreferrer">GitHub ↗</a
        ><a
          href="https://space.bilibili.com/52833994"
          target="_blank"
          rel="noreferrer">Bilibili ↗</a
        >
      </div>
    </section>
  </main>
{/if}

<footer>
  <span>AMAGICPEAR / 2026</span><span>MADE WITH CURIOSITY</span><button
    onclick={goHome}
    aria-label="返回首页">↑</button
  >
</footer>
