<script lang="ts">
  import { onMount, tick } from "svelte";
  import GeometryPlayground from "@/components/GeometryPlayground.svelte";
  import PerryWaves from "@/components/PerryWaves.svelte";
  import Essay from "@/pages/Essay.svelte";
  import EssayIndex from "@/pages/EssayIndex.svelte";
  import { findPost, posts, type Post } from "@/data/posts";
  import coverMusic from "@/assets/pics/cover_netease_gpui.webp";
  import coverLauncher from "@/assets/pics/cover_pcl_proto.webp";
  import coverPichamber from "@/assets/pics/cover_pichamber.webp";
  import avatar from "@/assets/pics/perry-avatar.webp";
  import coverPyroGenesis from "@/assets/pics/cover_PyroGenesis.webp";
  import coverEchoesphere from "@/assets/pics/cover_Echoesphere.jpeg";

  type Route = { kind: "home" } | { kind: "essay" } | { kind: "post"; slug: string };

  let menuOpen = $state(false);
  let route = $state<Route>({ kind: "home" });

  const isEssay = $derived(route.kind !== "home");
  const currentPost = $derived(route.kind === "post" ? findPost(route.slug) : undefined);
  const otherPosts = $derived(posts.filter((post) => post.slug !== currentPost?.slug));

  const homeDescription =
    "沈立（Perry / AMagicPear）的个人主页：AIGC 算法研发方向，做智能体、多模态交互与教育游戏。南京邮电大学 → 苏州大学。";

  const pageTitle = $derived(
    route.kind === "post" && currentPost
      ? `${currentPost.title} - AMagicPear`
      : route.kind === "home"
        ? "AMagicPear - Perry's playground"
        : "随笔 - AMagicPear",
  );

  const pageDescription = $derived(
    route.kind === "post" && currentPost
      ? currentPost.lead
      : route.kind === "home"
        ? homeDescription
        : "随笔：把折腾过的问题、走过的弯路留在这里。",
  );

  const featuredWorks = [
    {
      name: "网易云音乐 · 原生重奏",
      english: "netease-music-gpui",
      kind: "Rust / GPUI / 音乐播放器",
      text: "熟悉的音乐界面，从 Rust 开始重新写起。原生 UI、真实歌单、音频播放，让对音乐的喜欢多一种打开方式。",
      href: "https://github.com/AMagicPear/netease-music-gpui",
      image: coverMusic,
      width: 1600,
      height: 1136,
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
      width: 1600,
      height: 1033,
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
      width: 1628,
      height: 928,
      className: "launcher-feature",
      link: "打开项目",
    },
  ];

  const projects = [
    {
      title: "回声之境",
      english: "Echoesphere",
      period: "2025.10 — 2026.05",
      kind: "本科毕业设计 · Unity · 多模态交互 · 实体装置 · AI Agent",
      summary:
        "毕业设计《面向实体展览的沉浸式多模态交互系统设计与实现》里的交互作品。我想试着让智能体成为展览的认知核心，看看观众的动作、表情和声音能不能被一起看见。",
      detail:
        "系统是感知—理解—反馈三层的星型结构，由五个子项目拼起来：DeepAgents 智能体与 TCP 服务端、MediaPipe 手势与表情识别客户端、Unity 客户端，以及树莓派驱动的 WS2812B 可编程 LED 灯阵。模块之间走自定义 TCP 协议转发消息与指令，虚实两路输出保持同频。",
      tags: ["Unity", "多模态交互", "实体装置", "AI Agent"],
      href: "https://github.com/AMagicPear/Echoesphere",
      link: "看看这个装置",
      image: coverEchoesphere,
      width: 2000,
      height: 2000,
      layout: "square",
    },
    {
      title: "燧火启明录",
      english: "PyroGenesis",
      period: "2025.01 — 2025.07",
      kind: "中国大学生计算机设计大赛三等奖 · Unity · 教育游戏",
      summary:
        "和三位同学一起做的教育文字冒险，面向小学高年级。以一幅神秘画卷开场，玩家钻木取火、冶铸金属，在探险里把热学知识走一遍。",
      detail:
        "剧情、原创音效和中国传统文化元素由团队一起打磨。我负责客户端：接入扣子平台的问答智能体 API、多对象自动寻路调度、DOTween 与 Timeline 动画，还有一套自己设计的 ECS 小游戏模块抽象。",
      tags: ["Unity", "教育游戏", "团队协作"],
      href: "https://www.bilibili.com/video/BV16pgBz7EMW",
      link: "看看游戏录像",
      image: coverPyroGenesis,
      width: 3840,
      height: 2160,
      layout: "wide",
    },
  ];

  const education = [
    {
      school: "苏州大学",
      period: "2026.09 — 至今",
      datetime: "2026-09",
      degree: "人工智能 · 硕士研究生",
      note: "计算机科学与技术学院 · NLP 研究中心（导师 王晶晶）",
    },
    {
      school: "南京邮电大学",
      period: "2022.09 — 2026.06",
      datetime: "2022-09",
      degree: "数字媒体技术 · 本科",
      note: "数字媒体与设计艺术学院",
    },
  ];

  const skills = [
    {
      label: "熟练运用",
      items: ["Python", "Blender", "ComfyUI", "Unity", "GitHub 开源协作", "Vue3"],
    },
    {
      label: "接触过",
      items: ["三维重建", "电子音乐编曲", "平面设计", "摄影", "合唱", "Rust", "Coding Agent", "…"],
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

  function parseRoute(): Route {
    const path = window.location.pathname.replace(/\/+$/, "");
    if (path === "/essay") return { kind: "essay" };
    const match = path.match(/^\/essay\/([^/]+)$/);
    if (match) return { kind: "post", slug: decodeURIComponent(match[1]) };
    return { kind: "home" };
  }

  function setRoute() {
    const next = parseRoute();
    if (next.kind === "post" && !findPost(next.slug)) {
      history.replaceState({}, "", "/essay");
      route = { kind: "essay" };
    } else {
      route = next;
    }
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

  function goPost(slug: string) {
    history.pushState({}, "", `/essay/${slug}`);
    setRoute();
    window.scrollTo({ top: 0 });
  }

  function goPostLink(event: MouseEvent, slug: string) {
    if (isModifiedClick(event)) return;
    event.preventDefault();
    goPost(slug);
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
  <title>{pageTitle}</title>
  <meta name="description" content={pageDescription} />
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
    <a href="/#experience" onclick={(event) => goSectionLink(event, "experience")}>经历</a>
    <a href="/#about" onclick={(event) => goSectionLink(event, "about")}>关于</a>
    <a href="/essay" aria-current={isEssay ? "page" : undefined} onclick={goEssayLink}>随笔</a>
    <a class="nav-github" href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">GitHub <span aria-hidden="true">↗</span></a>
  </nav>
</header>

{#if route.kind !== "home"}
  <main id="main-content" class="essay-page">
    {#if route.kind === "post" && currentPost}
      <Essay post={currentPost} others={otherPosts} onHome={goHome} onIndex={goEssay} onOpen={goPost} />
    {:else}
      <EssayIndex onHome={goHome} onOpen={goPost} />
    {/if}
  </main>
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
                <img src={work.image} alt={work.name + "项目画面"} width={work.width} height={work.height} loading="lazy" decoding="async" />
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

    <section id="projects" class="projects section-shell" aria-labelledby="projects-title">
      <div class="section-heading">
        <h2 id="projects-title">也在别的世界，留下过<span class="ink-outline">脚印<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><ellipse cx="100" cy="26" rx="84" ry="22" transform="rotate(-10 100 26)" /><circle cx="172" cy="6" r="6" fill="#e44030" stroke="none" /></svg></span>。</h2>
        <p>游戏、声音、智能体。不同方向的尝试，最后总会重新相遇。</p>
      </div>

      <div class="project-list">
        {#each projects as project, index}
          <article class="project" class:reverse={index % 2 === 1}>
            <div class="project-visual">
              <img class={project.layout} src={project.image} alt={project.title + "项目画面"} width={project.width} height={project.height} loading="lazy" decoding="async" />
              <span class="project-period">{project.period}</span>
            </div>
            <div class="project-copy">
              <p class="content-label">{project.kind}</p>
              <h3>{project.title}<span>{project.english}</span></h3>
              <p class="project-summary">{project.summary}</p>
              <p class="project-detail">{project.detail}</p>
              <ul class="project-tags" aria-label="技术栈">{#each project.tags as tag}<li>{tag}</li>{/each}</ul>
              <a class="card-link" href={project.href} target="_blank" rel="noreferrer">{project.link} <span aria-hidden="true">↗</span></a>
            </div>
          </article>
        {/each}
      </div>

      <a class="project-note" href="https://github.com/AMagicPear/perry_hermes" target="_blank" rel="noreferrer">
        <div>
          <p class="content-label">Rust / Agent runtime / Ratatui</p>
          <h3>Perry Hermes<span aria-hidden="true">↗</span></h3>
          <p>从终端出发，摸索工具调用、技能与会话怎么拼成一个 Agent。</p>
        </div>
        <div class="hermes-art" aria-hidden="true"><span>hello, hermes.</span><code>想法 → 工具 → 行动</code><span class="terminal-cursor">▍</span></div>
      </a>
    </section>

    <section id="experience" class="experience section-shell" aria-labelledby="experience-title">
      <p class="section-kicker">RESUME · 2026<svg class="kicker-clock" viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="9" /><path d="M 12 6.5 V 12 L 16 14.5" /></svg></p>
      <div class="experience-head">
        <h2 id="experience-title">做过的事，<span class="ink-outline">按时间</span>排一排。</h2>
        <div class="experience-aside">
          <p>从数字媒体技术到人工智能，从课程作业到跑在服务器和展厅里的东西。<br class="desktop-break" />简历上的每一行，背后都是一段可以展开的时间。</p>
          <a class="button button-secondary" href="/resume.pdf?v=2026-11" target="_blank" rel="noreferrer">PDF 版简历 <span aria-hidden="true">↗</span></a>
        </div>
      </div>

      <div class="expectation-panel">
        <div class="expectation-label"><span>求职意向</span><small>EXPECTATION</small></div>
        <div class="expectation-body">
          <p class="expectation-title">AIGC 算法研发 <span>实习</span></p>
          <ul class="expectation-tags">
            <li>AIGC Agent</li><li>多模态</li><li>视觉</li><li>3D</li><li>动作</li><li>AI 工具</li>
          </ul>
        </div>
        <span class="expectation-star" aria-hidden="true">✳</span>
      </div>

      <div class="resume-grid">
        <section class="resume-block" aria-labelledby="education-title">
          <h3 id="education-title">教育经历 <small>EDUCATION</small></h3>
          <ol class="timeline">
            {#each education as item}
              <li>
                <div class="timeline-head"><strong>{item.school}</strong><time datetime={item.datetime}>{item.period}</time></div>
                <p class="timeline-role">{item.degree}</p>
                <p class="timeline-note">{item.note}</p>
              </li>
            {/each}
          </ol>
          <p class="edu-note">本科读数字媒体技术，课程和项目大多绕着图像、交互和游戏；研究生进 NLP 研究中心，开始和点云、视觉语言模型打交道。</p>
        </section>

        <section class="resume-block internship" aria-labelledby="internship-title">
          <h3 id="internship-title">实习经历 <small>INTERNSHIP</small></h3>
          <ol class="timeline">
            <li>
              <div class="timeline-head"><strong>苏州明新智算科技有限公司</strong><time datetime="2025-10">2025.10 — 2026.05</time></div>
              <p class="timeline-role">Python · 大模型部署 · 模型评测</p>
              <p class="timeline-text">一家面向建筑行业的初创公司，想做 CAD 图纸的智能解析和算量，再一键出 BIM。我去的时候产品还在预研，所以能坐进比较核心的技术讨论里。</p>
              <ul class="brief-list">
                <li>在 Linux 服务器上用 SGLang 和 vLLM 部署 Qwen3.5 的不同规格</li>
                <li>在多种公开 VLM 和公司自研的全景分割模型上，评测建筑图纸划分、图例识别匹配任务</li>
              </ul>
              <div class="patents">
                <p class="mini-label">专利 · 发明人</p>
                <ul>
                  <li>《一种 DXF 格式 CAD 图纸的墙体识别方法及系统》<em>已受理</em></li>
                  <li>《基于三段视觉智能体递归的工程图纸子图划分系统》</li>
                </ul>
              </div>
            </li>
          </ol>
        </section>
      </div>

      <section class="skills-block" aria-labelledby="skills-title">
        <h3 id="skills-title">技能图谱 <small>SKILLS</small></h3>
        {#each skills as group}
          <div class="skill-line">
            <span class="skill-k">{group.label}</span>
            <ul class="chips">
              {#each group.items as item}<li>{item}</li>{/each}
            </ul>
          </div>
        {/each}
        <div class="skill-line">
          <span class="skill-k">其他奖项</span>
          <ul class="award-list">
            <li><strong>中国国际大学生创新大赛高教主赛道金奖</strong><time datetime="2023">2023</time></li>
            <li><strong>江苏省第七届大学生艺术展演（合唱）甲组一等奖</strong><time datetime="2023">2023</time></li>
          </ul>
        </div>
        <p class="skills-more">更多个人开源项目见 <a href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">github.com/AMagicPear <span aria-hidden="true">↗</span></a></p>
      </section>

    </section>

    <section id="about" class="about section-shell" aria-labelledby="about-title">
      <div class="about-profile">
        <div class="portrait-frame"><img src={avatar} alt="Perry 的手绘头像" loading="lazy" decoding="async" width="240" height="240" /></div>
        <h3>沈立<span>Perry / AMagicPear</span></h3>
        <p>苏州大学 / 人工智能硕士</p>
        <span class="location">南京 <span aria-hidden="true">⟶</span> 苏州</span>
        <div class="profile-shapes" aria-hidden="true"><i></i><i></i><i></i></div>
      </div>
      <div class="about-copy">
        <h2 id="about-title">创作的<span class="ink-outline">边界<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path class="mark-dash" d="M 30 8 H 170" /><path d="M 22 8 V 24" /><path d="M 178 8 V 24" stroke="#e44030" /></svg></span>，<br />可以再<span class="ink-mark">宽一点。</span></h2>
        <p>本科学数字媒体技术，研究生转到人工智能。做过游戏、装置和智能体，也写 Rust、搭 Agent 的工作台——技术在变，想做出让人愿意多待一会儿的东西这件事没变。</p>
        <p>我自以为最大的优势是这种交叉：技术也好，艺术也好，都是真正感兴趣的事。研究生入学前后，我在做三维点云证据记忆与 VLM 风险理解的工作，目前有一篇 The Web Conference 一作论文在投；最近在读 VLA 和 WAM，想以游戏引擎为媒介，探索更三维原生的具身动作交互。</p>
        <p>业余时间做电子音乐、学 Blender，也在 B 站记录创作。这些看似绕远的事情，最后常常会在一个新项目里重新相遇。</p>
        <div class="about-interests"><span>写代码</span><span>做音乐</span><span>造世界</span></div>
        <div class="about-contact">
          <a href="mailto:job@amagicpear.top"><span>邮箱</span>job@amagicpear.top</a>
          <a href="tel:+8617714095162"><span>电话</span>17714095162</a>
          <a href="https://amagicpear.top" target="_blank" rel="noreferrer"><span>主页</span>amagicpear.top</a>
          <a href="https://github.com/AMagicPear" target="_blank" rel="noreferrer"><span>GitHub</span>github.com/AMagicPear</a>
        </div>
      </div>
    </section>

    <section id="notes" class="notes section-shell" aria-labelledby="notes-title">
      <div class="section-heading notes-heading">
        <div>
          <h2 id="notes-title">过程，也值得<span class="ink-outline">记录<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 20 26 Q 46 2 72 26 T 124 26 T 176 22" /><path d="M 170 26 L 180 8" stroke="#e44030" /></svg></span>。</h2>
          <p>把折腾过的问题、走过的弯路，留在这里。</p>
        </div>
        <a class="notes-all" href="/essay" onclick={goEssayLink}>全部随笔 <span aria-hidden="true">↗</span></a>
      </div>
      <ul class="note-list">
        {#each posts as post}
          <li>
            <a class="note-row" href={`/essay/${post.slug}`} onclick={(event) => goPostLink(event, post.slug)}>
              <span class="note-date"><time datetime={post.datetime}>{post.date}</time></span>
              <span class="note-copy">
                <span class="content-label">{post.kicker}</span>
                <strong>{post.title}</strong>
                <span class="note-lead">{post.lead}</span>
              </span>
              <span class="note-arrow" aria-hidden="true">↗</span>
            </a>
          </li>
        {/each}
      </ul>
    </section>

    <section class="closing section-shell" aria-labelledby="closing-title">
      <div class="closing-glow" aria-hidden="true"></div>
      <h2 id="closing-title">下一个有意思的<span class="ink-outline">想法<svg class="smark" viewBox="0 0 200 200" aria-hidden="true"><path d="M 56 4 H 144 Q 164 4 164 22 V 36 Q 164 50 144 50 H 112 L 96 64 L 96 50 H 56 Q 36 50 36 36 V 22 Q 36 4 56 4" /><circle cx="76" cy="27" r="3.5" fill="#262622" stroke="none" /><circle cx="100" cy="27" r="4.5" fill="#e44030" stroke="none" /><circle cx="124" cy="27" r="3.5" fill="#262622" stroke="none" /></svg></span>，<br />或许从一次聊天开始。</h2>
      <p>关于技术、创作，或者一个还不太确定的灵感。</p>
      <a class="button button-primary" href="mailto:job@amagicpear.top">和我聊聊 <span aria-hidden="true">↗</span></a>
      <a class="closing-email" href="mailto:job@amagicpear.top">job@amagicpear.top</a>
    </section>
  </main>
{/if}

<footer class="site-footer section-shell">
  <span>© 2026 AMagicPear</span>
  <span class="footer-note">保持好奇，保持天真，持续创造。</span>
  <div><a href="https://github.com/AMagicPear" target="_blank" rel="noreferrer">GitHub</a><a href="https://space.bilibili.com/52833994" target="_blank" rel="noreferrer">Bilibili</a><button onclick={() => window.scrollTo({ top: 0 })} aria-label="回到顶部">↑</button></div>
</footer>
