<script lang="ts">
  import { onMount, tick } from "svelte";
  import GeometryPlayground from "@/components/GeometryPlayground.svelte";
  import PerryWaves from "@/components/PerryWaves.svelte";
  import Essay from "@/pages/Essay.svelte";
  import coverMusic from "@/assets/pics/cover_netease_gpui.webp";
  import coverLauncher from "@/assets/pics/cover_pcl_proto.webp";
  import coverPichamber from "@/assets/pics/cover_pichamber.webp";
  import avatar from "@/assets/pics/perry-avatar.webp";
  import coverPyroGenesis from "@/assets/pics/cover_PyroGenesis.webp";
  import coverEchoesphere from "@/assets/pics/cover_Echoesphere.jpeg";

  let menuOpen = $state(false);
  let isEssay = $state(false);

  const featuredWorks = [
    {
      name: "网易云音乐 · 原生重奏",
      english: "netease-music-gpui",
      kind: "Rust / GPUI / 音乐播放器",
      text: "熟悉的音乐界面，从 Rust 开始重新写起。原生 UI、真实歌单、音频播放，让对音乐的喜欢多一种打开方式。",
      href: "https://github.com/AMagicPear/netease-music-gpui",
      image: coverMusic,
      className: "music-feature",
      link: "看看这次重奏",
    },
    {
      name: "pichamber",
      english: "给 Agent 一张工作台",
      kind: "Vue / Bun / Pi Coding Agent",
      text: "把对话、文件和真正的终端放进同一个浏览器工作区，让 Pi Coding Agent 的工作过程看得见、接得上。",
      href: "https://github.com/AMagicPear/pichamber",
      image: coverPichamber,
      className: "agent-feature",
      link: "走进工作区",
    },
    {
      name: "PCL.Proto",
      english: "从原型到启动器",
      kind: "Vue / Tauri / Minecraft",
      text: "沿着 PCL 的设计语言，把界面原型继续做成桌面启动器，也把组件演示与主页制作工具留给更多创作者。",
      href: "https://github.com/AMagicPear/PCL.Proto",
      image: coverLauncher,
      className: "launcher-feature",
      link: "打开项目",
    },
  ];

  const experiments = [
    {
      title: "回声之境",
      label: "Echoesphere / Unity / 交互装置",
      text: "让声音塑造世界，让 AI、传感器与真实按钮一起参与叙事。",
      href: "https://github.com/AMagicPear/Echoesphere",
      image: coverEchoesphere,
    },
    {
      title: "燧火启明录",
      label: "PyroGenesis / 教育游戏",
      text: "从燧人取火出发，把古代物理变成可以亲手探索的故事。",
      href: "https://www.bilibili.com/video/BV16pgBz7EMW",
      image: coverPyroGenesis,
    },
    {
      title: "Perry Hermes",
      label: "Rust / Agent runtime / Ratatui",
      text: "从终端出发，探索工具调用、技能与会话如何组成一个 Agent。",
      href: "https://github.com/AMagicPear/perry_hermes",
      image: "",
    },
  ];

  function followPointer(node: HTMLElement) {
    const motion = window.matchMedia("(prefers-reduced-motion: reduce)");
    function move(event: PointerEvent) {
      if (motion.matches || event.pointerType !== "mouse") return;
      const rect = node.getBoundingClientRect();
      node.style.setProperty("--mx", `${((event.clientX - rect.left) / rect.width - .5) * 16}px`);
      node.style.setProperty("--my", `${((event.clientY - rect.top) / rect.height - .5) * 12}px`);
    }
    function reset() {
      node.style.setProperty("--mx", "0px");
      node.style.setProperty("--my", "0px");
    }
    node.addEventListener("pointermove", move);
    node.addEventListener("pointerleave", reset);
    motion.addEventListener("change", reset);
    return { destroy() {
      node.removeEventListener("pointermove", move);
      node.removeEventListener("pointerleave", reset);
      motion.removeEventListener("change", reset);
    } };
  }

  function setRoute() {
    isEssay = window.location.pathname.replace(/\/+$/, "") === "/essay";
    menuOpen = false;
  }

  function goHome() {
    menuOpen = false;
    if (isEssay || window.location.hash) {
      history.pushState({}, "", "/");
      setRoute();
    }
    window.scrollTo({ top: 0 });
  }

  function goEssay() {
    history.pushState({}, "", "/essay");
    setRoute();
    window.scrollTo({ top: 0 });
  }

  function isModifiedClick(event: MouseEvent) {
    return (
      event.button !== 0 ||
      event.metaKey ||
      event.ctrlKey ||
      event.shiftKey ||
      event.altKey
    );
  }

  function goSectionLink(event: MouseEvent, id: string) {
    if (isModifiedClick(event)) return;
    event.preventDefault();
    goTo(id);
  }

  function goEssayLink(event: MouseEvent) {
    if (isModifiedClick(event)) return;
    event.preventDefault();
    goEssay();
  }

  async function goTo(id: string) {
    history.pushState({}, "", `/#${id}`);
    setRoute();
    await tick();
    document.getElementById(id)?.scrollIntoView({ block: "start" });
  }

  async function scrollToHash() {
    await tick();
    const id = window.location.hash.slice(1);
    if (id) document.getElementById(id)?.scrollIntoView({ block: "start" });
  }

  onMount(() => {
    setRoute();
    const handleKeydown = (event: KeyboardEvent) => {
      if (event.key === "Escape") menuOpen = false;
    };
    scrollToHash();
    const handlePopstate = () => {
      setRoute();
      scrollToHash();
    };
    window.addEventListener("popstate", handlePopstate);
    window.addEventListener("hashchange", scrollToHash);
    window.addEventListener("keydown", handleKeydown);
    return () => {
      window.removeEventListener("popstate", handlePopstate);
      window.removeEventListener("hashchange", scrollToHash);
      window.removeEventListener("keydown", handleKeydown);
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

<a class="skip-link" href="#main-content">跳到主要内容</a>
<header class="site-header" class:open={menuOpen}>
  <a class="wordmark" href="/" onclick={(event) => { if (isModifiedClick(event)) return; event.preventDefault(); goHome(); }}>perry’s playground<span aria-hidden="true">.</span></a>
  <button
    class="menu-button"
    aria-label={menuOpen ? "关闭导航菜单" : "打开导航菜单"}
    aria-expanded={menuOpen}
    aria-controls="main-navigation"
    onclick={() => (menuOpen = !menuOpen)}><i></i><i></i></button
  >
  <nav id="main-navigation" aria-label="主导航">
    <a href="/#works" onclick={(event) => goSectionLink(event, "works")}>作品</a>
    <a href="/#about" onclick={(event) => goSectionLink(event, "about")}>关于</a>
    <a href="/essay" aria-current={isEssay ? "page" : undefined} onclick={goEssayLink}>随笔</a>
    <a class="nav-github" href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">GitHub <span aria-hidden="true">↗</span></a>
  </nav>
</header>

{#if isEssay}
  <main id="main-content" class="essay-page"><Essay onBack={goHome} /></main>
{:else}
  <main id="main-content">
    <section class="hero" aria-labelledby="hero-title">
      <div class="hero-copy">
        <p class="hero-greeting">你好，我是 Perry</p>
        <h1 id="hero-title"><span class="title-first">好奇心，<i aria-hidden="true">✳</i></span><span class="title-second">不必<span class="title-outline">循规<svg class="title-compass" viewBox="0 0 180 120" fill="none" aria-hidden="true"><path class="compass-arc" d="M 17 93 Q 86 126 166 84" pathLength="100" /><g class="compass-tool"><path d="M 126 16 L 111 61 M 126 16 L 151 57 M 117 43 Q 130 50 143 42 M 111 61 L 108 70 M 151 57 L 156 65" /><circle cx="126" cy="16" r="4" /></g></svg></span>蹈矩。</span></h1>
        <p class="hero-intro">
          写代码，做音乐，也创造一些不一样的互动体验。<br class="desktop-break" />
          在技术与表达之间，把想法一点点变成现实。
        </p>
        <div class="hero-actions">
          <a class="button button-primary" href="/#works" onclick={(event) => goSectionLink(event, "works")}>探索作品 <span aria-hidden="true">↓</span></a>
          <a class="button button-secondary" href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">访问 GitHub <span aria-hidden="true">↗</span></a>
        </div>
      </div>
      <div class="hero-art" aria-hidden="true">
        <div class="hero-particle-stage"><PerryWaves /></div>
      </div>
    </section>

    <div class="play-strip" aria-hidden="true"><span>写一点代码</span><i>●</i><span>制造一点声音</span><i>▲</i><span>保持好奇</span><i>✳</i><span>保持天真</span><i>◇</i><span>追寻艺术</span><i>■</i></div>

    <section id="works" class="works section-shell" aria-labelledby="works-title">
      <div class="section-heading works-heading">
        <h2 id="works-title">想法<span class="ink-outline">落地<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 16 40 L 16 6 L 52 6" /><circle cx="172" cy="26" r="22" /><path d="M 78 44 L 92 16 L 106 44 Z" fill="#e44030" stroke="none" /></svg></span>，<br />各有形状。</h2>
        <p>最近在做的，是<span class="ink-mark">更顺手的工具</span>，也是<span class="ink-mark">不一样的表达</span>。</p>
      </div>
      <div class="feature-grid">
        {#each featuredWorks as work}
          <a use:followPointer class={`feature-card ${work.className}`} href={work.href} target="_blank" rel="noreferrer">
            <div class="card-copy">
              <p class="content-label">{work.kind}</p>
              <h3>{work.name}<span>{work.english}</span></h3>
              <p class="card-description">{work.text}</p>
              <span class="card-link">{work.link} <span aria-hidden="true">↗</span></span>
            </div>
            <div class="card-visual">
              {#if work.className === "music-feature"}
                <div class="vinyl" aria-hidden="true"><span>Perry’s<br />side projects</span></div>
              {/if}
              {#if work.image}
                <img src={work.image} alt={work.name + "项目画面"} loading="lazy" decoding="async" />
              {/if}
            </div>
          </a>
        {/each}
      </div>
    </section>

    <section class="geometry-section section-shell" aria-labelledby="geometry-title">
      <div class="geometry-copy"><h2 id="geometry-title">圆的，<span class="ink-outline">方的<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 40 30 H 160" /><path d="M 62 30 V 4 M 100 30 V 0" /><path d="M 138 30 V 4" stroke="#e44030" /></svg></span>，<br />都可以。</h2><p>代码之外，也给直觉一点空间。<br />把这些形状，摆成你喜欢的样子。</p><span class="geometry-flower" aria-hidden="true">✳</span></div>
      <GeometryPlayground />
    </section>

    <section class="systems section-shell" aria-labelledby="systems-title">
      <div class="section-heading">
        <h2 id="systems-title">也在别的世界，留下过<span class="ink-outline">脚印<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><ellipse cx="100" cy="26" rx="84" ry="22" transform="rotate(-10 100 26)" /><circle cx="172" cy="6" r="6" fill="#e44030" stroke="none" /></svg></span>。</h2>
        <p>游戏、声音、智能体。不同方向的尝试，最后总会重新相遇。</p>
      </div>
      <div class="experiment-grid">
        {#each experiments as item}
          <a href={item.href} target="_blank" rel="noreferrer" class="experiment" use:followPointer>
            {#if item.image}<img class="experiment-image" src={item.image} alt={item.title + "项目画面"} loading="lazy" decoding="async" />{:else}<div class="hermes-art" aria-hidden="true"><span>hello, hermes.</span><code>想法 → 工具 → 行动</code><span class="terminal-cursor">▍</span></div>{/if}
            <div class="experiment-copy"><p class="content-label">{item.label}</p>
            <h3>{item.title}<span aria-hidden="true">↗</span></h3>
            <p class="experiment-description">{item.text}</p></div>
          </a>
        {/each}
      </div>
    </section>

    <section id="about" class="about section-shell" aria-labelledby="about-title">
      <div class="about-profile">
        <div class="portrait-frame"><img src={avatar} alt="Perry 的手绘头像" loading="lazy" decoding="async" width="240" height="240" /></div>
        <h3>Perry<span>AMagicPear</span></h3>
        <p>苏州大学 / 计算机科学</p>
        <span class="location">南京 <span aria-hidden="true">⟶</span> 苏州</span>
        <div class="profile-shapes" aria-hidden="true"><i></i><i></i><i></i></div>
      </div>
      <div class="about-copy">
        <h2 id="about-title">创作的<span class="ink-outline">边界<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path class="mark-dash" d="M 30 8 H 170" /><path d="M 22 8 V 24" /><path d="M 178 8 V 24" stroke="#e44030" /></svg></span>，<br />可以再<span class="ink-mark">宽一点。</span></h2>
        <p>最近在写 Rust、做原生桌面应用，也在为 AI Agent 搭工作台。从 Unity 游戏到 Tauri 和 GPUI，我喜欢亲手试试不同的技术能带来怎样的体验。</p>
        <p>业余时间做电子音乐，学习 Blender，也在 B 站记录创作。这些看似绕远的事情，最后常常会在一个新项目里重新相遇。</p>
        <div class="about-interests"><span>写代码</span><span>做音乐</span><span>造世界</span></div>
        <dl class="about-facts">
          <div><dt>正在探索</dt><dd>原生界面 / Agent 工具 / 音乐与多模态</dd></div>
          <div><dt>常用工具</dt><dd>TypeScript / C# / Python / Unity / Rust</dd></div>
        </dl>
      </div>
    </section>

    <section class="notes section-shell" aria-labelledby="notes-title">
      <div class="section-heading">
        <h2 id="notes-title">过程，也值得<span class="ink-outline">记录<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 20 26 Q 46 2 72 26 T 124 26 T 176 22" /><path d="M 170 26 L 180 8" stroke="#e44030" /></svg></span>。</h2>
        <p>把折腾过的问题、走过的弯路，留在这里。</p>
      </div>
      <a href="/essay" class="note-entry" onclick={goEssayLink}>
        <div class="note-meta"><span>家庭网络 / 树莓派</span><time datetime="2025-07-15">2025.07.15</time></div>
        <h3>有了树莓派以后，<br />我是如何改造家里的网络的？</h3>
        <p>从 mihomo、TUN 到全屋网络拓扑的一次完整实践。</p>
        <span class="card-link">阅读文章 <span aria-hidden="true">↗</span></span>
      </a>
    </section>

    <section class="closing section-shell" aria-labelledby="closing-title">
      <div class="closing-glow" aria-hidden="true"></div>
      <h2 id="closing-title">下一个有意思的<span class="ink-outline">想法<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 56 4 H 144 Q 164 4 164 22 V 36 Q 164 50 144 50 H 112 L 96 64 L 96 50 H 56 Q 36 50 36 36 V 22 Q 36 4 56 4" /><circle cx="76" cy="27" r="3.5" fill="#262622" stroke="none" /><circle cx="100" cy="27" r="4.5" fill="#e44030" stroke="none" /><circle cx="124" cy="27" r="3.5" fill="#262622" stroke="none" /></svg></span>，<br />或许从一次聊天开始。</h2>
      <p>关于技术、创作，或者一个还不太确定的灵感。</p>
      <a class="button button-primary" href="mailto:hello@amagicpear.top">和我聊聊 <span aria-hidden="true">↗</span></a>
      <a class="closing-email" href="mailto:hello@amagicpear.top">hello@amagicpear.top</a>
    </section>
  </main>
{/if}

<footer class="site-footer section-shell">
  <span>© 2026 AMagicPear</span>
  <span class="footer-note">保持好奇，继续创造。</span>
  <div><a href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">GitHub</a><a href="https://space.bilibili.com/52833994" target="_blank" rel="noreferrer">Bilibili</a><button onclick={() => window.scrollTo({ top: 0 })} aria-label="回到顶部">↑</button></div>
</footer>
