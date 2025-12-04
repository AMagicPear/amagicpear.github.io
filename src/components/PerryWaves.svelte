<!--
@component
Modified from [NanoFlow](https://github.com/ZTMYO/NanoFlow)
| MIT License
-->

<script module lang="ts">
  import nanoflowCfg from "@/data/simplified_nanoflow.json";
  import { Particle, updateParticleCoordinates, type ParticleData, type ParticleMouse } from "@/lib/particle";

  // SVG缩放因子，与CSS中的scale值保持一致
  const scaleFactor = 1.4;
  const hasMouse = window.matchMedia("(pointer: fine)").matches;
  const prefersReducedMotion = window.matchMedia(
    "(prefers-reduced-motion: reduce)",
  ).matches;
</script>

<script lang="ts">
  import { onMount } from "svelte";
  let scatterStrength = 0;

  let svg: SVGSVGElement;
  let particles: Particle[] = nanoflowCfg.particles.map(
    (p) =>
      new Particle(
        p as ParticleData,
        nanoflowCfg.elasticityFactor,
        nanoflowCfg.maxPushForce,
      ),
  );
  let mouse: ParticleMouse = { x: -1000, y: -1000, vx: 0, vy: 0, speed: 0 };
  let particleElements: SVGCircleElement[] = new Array(particles.length);
  let isIntersecting = true;

  const handleMouseMove = (e: MouseEvent) => {
    const rect = svg.getBoundingClientRect();
    // 计算原始鼠标坐标，并除以缩放因子以匹配SVG内部坐标系统
    const cx = (e.clientX - rect.left) / scaleFactor;
    const cy = (e.clientY - rect.top) / scaleFactor;
    if (mouse.x < 0 || mouse.y < 0) {
      mouse.vx = 0;
      mouse.vy = 0;
      mouse.speed = 0;
    } else {
      mouse.vx = cx - mouse.x;
      mouse.vy = cy - mouse.y;
      // 限制鼠标速度，防止窗口切换等操作导致的异常大值
      mouse.speed = Math.min(
        Math.sqrt(mouse.vx * mouse.vx + mouse.vy * mouse.vy),
        500,
      );
    }
    mouse.x = cx;
    mouse.y = cy;
  };

  const handleMouseLeave = () => {
    mouse.x = -1000;
    mouse.y = -1000;
    mouse.vx = 0;
    mouse.vy = 0;
    mouse.speed = 0;
    scatterStrength = 0; // 鼠标离开时重置散射强度
  };

  // 根据传入的坐标更新SVG元素的位置
  const updateParticleElements = (coordinates: [number, number][]) => {
    particleElements.forEach((element, index) => {
      element.cx.baseVal.value = coordinates[index][0];
      element.cy.baseVal.value = coordinates[index][1];
    });
  }

  // 动画主循环
  function animate() {
    if (mouse.speed > 220) {
      scatterStrength = Math.min((mouse.speed - 220) / 8, 16);
    } else {
      // 添加scatterStrength衰减机制，防止粒子一直处于散射状态
      scatterStrength *= 0.9;
    }

    const coordinates = updateParticleCoordinates(particles, mouse, scatterStrength);
    updateParticleElements(coordinates);

    // 仅当处于视口时才继续请求下一帧动画
    if (isIntersecting) {
      requestAnimationFrame(animate);
    }
  }

  // 用于监听SVG画布是否可见，不可见时暂停动画以避免浪费性能
  const intersectionObserver = new IntersectionObserver((entries) => {
    // 仅监听SVG元素，若不是则报错
    console.assert(entries.length === 1 && entries[0].target.isSameNode(svg));
    if (entries[0].isIntersecting) {
      isIntersecting = true;
      requestAnimationFrame(animate);
    } else {
      handleMouseLeave();
      isIntersecting = false;
    }
  });

  onMount(() => {
    const coordinates = updateParticleCoordinates(particles, mouse, scatterStrength);
    updateParticleElements(coordinates);
    if (!hasMouse || prefersReducedMotion) return;
    // 下面的部分仅在有鼠标且设置不为削弱动画时执行
    // 以免浪费设备性能以及造成移动端的渲染异常
    intersectionObserver.observe(svg);
    requestAnimationFrame(animate);
    // 清理事件监听
    return () => {
      intersectionObserver.disconnect();
      svg.removeEventListener("mousemove", handleMouseMove);
      svg.removeEventListener("mouseleave", handleMouseLeave);
    };
  });
</script>

<svg
  bind:this={svg}
  width={nanoflowCfg.cwidth}
  height={nanoflowCfg.cheight}
  onmousemove={handleMouseMove}
  onmouseleave={handleMouseLeave}
  role="presentation"
  xmlns="http://www.w3.org/2000/svg"
>
  {#each particles as particle, index}
    <circle
      r={particle.size}
      fill={`rgb(${particle.color.join(",")})`}
      bind:this={particleElements[index]}
    />
  {/each}
</svg>

<style>
  svg {
    position: absolute;
    top: calc(36vh - 250px);
    right: calc(2.65vw - 395.1px);
    scale: 1.4;

    @media (pointer: fine) {
      cursor: cell;
    }

    @media screen and (min-width: 1260px) {
      right: calc(16vw - 450px);
    }

    @media screen and (max-width: 1068px) {
      right: calc(60vw - 1007.6px);
    }
  }
</style>
