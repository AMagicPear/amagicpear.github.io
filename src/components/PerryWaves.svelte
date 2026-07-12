<!--
@component
Modified from [NanoFlow](https://github.com/ZTMYO/NanoFlow)
| MIT License
-->

<script module lang="ts">
  import nanoflowCfg from "@/data/simplified_nanoflow.json";
  const COUNT = nanoflowCfg.particles.length;
  import { type Particles } from "@/lib/wasm-perryhome/pkg";

  const hasMouse = window.matchMedia("(pointer: fine)").matches;
  const prefersReducedMotion = window.matchMedia(
    "(prefers-reduced-motion: reduce)",
  ).matches;
</script>

<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  let scatterStrength = 0;
  let isVisible = false;
  let mouse = { x: -1000, y: -1000, vx: 0, vy: 0, speed: 0 };

  let svg: SVGSVGElement;
  let particleElements: SVGCircleElement[] = new Array(COUNT);
  // let isIntersecting = true;

  const handlePointerMove = (e: PointerEvent) => {
    // getScreenCTM accounts for every CSS transform, viewport scale, and SVG
    // viewBox mapping. The inverse maps a screen-space pointer precisely back
    // into the coordinate system consumed by the WASM simulation.
    const screenMatrix = svg.getScreenCTM();
    if (!screenMatrix) return;

    const pointer = svg.createSVGPoint();
    pointer.x = e.clientX;
    pointer.y = e.clientY;
    const { x: cx, y: cy } = pointer.matrixTransform(screenMatrix.inverse());
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

  // 仅在有鼠标且设置不为削弱动画时执行
  // 以免浪费设备性能以及造成移动端的渲染异常
  if (hasMouse && !prefersReducedMotion) {
    // 在加载时初始化，卸载时释放的两个东西
    let particles: Particles | undefined;
    let intersectionObserver: IntersectionObserver | undefined;
    onMount(async () => {
      try {
        // 初始化wasm模块，在这里异步引入的好处是只有需要用到的时候浏览器才会去获取
        // 可以让移动端省去加载这个模块的步骤
        const wasmModule = await import("@/lib/wasm-perryhome/pkg");
        const wasmInitOutput = await wasmModule.default();
        particles = new wasmModule.Particles(
          nanoflowCfg.particles.map((p) => new wasmModule.Position(p.x, p.y)),
          nanoflowCfg.elasticityFactor,
          nanoflowCfg.maxPushForce,
        );
        // 获取指向各个粒子位置的数组的指针，直接读取内存以免频繁复制传递造成性能开销
        const ptr = particles.positions_ptr();
        const len = particles.positions_len();
        const positions = new Float32Array(
          wasmInitOutput.memory.buffer,
          ptr,
          len * 2,
        );

        // 用于监听SVG画布是否可见，不可见时暂停动画以避免浪费性能
        intersectionObserver = new IntersectionObserver((entries) => {
          // 仅监听SVG元素，若不是则报错
          console.assert(
            entries.length === 1 && entries[0].target.isSameNode(svg),
          );
          if (entries[0].isIntersecting) {
            isVisible = true;
            requestAnimationFrame(animate);
          } else {
            handleMouseLeave();
            isVisible = false;
          }
        });

        const updateParticleCoordinates = () => {
          particles?.update(mouse.x, mouse.y, mouse.speed, scatterStrength);
          for (let i = 0; i < len; i++) {
            particleElements[i].cx.baseVal.value = positions[i * 2];
            particleElements[i].cy.baseVal.value = positions[i * 2 + 1];
          }
        };
        // updateParticleCoordinates();
        // 动画主循环
        function animate() {
          if (mouse.speed > 220) {
            scatterStrength = Math.min((mouse.speed - 220) / 8, 16);
          } else {
            // 添加scatterStrength衰减机制，防止粒子一直处于散射状态
            scatterStrength *= 0.9;
          }
          updateParticleCoordinates();
          // 仅当处于视口时才继续请求下一帧动画
          if (isVisible) {
            requestAnimationFrame(animate);
          }
        }
        intersectionObserver.observe(svg);
        requestAnimationFrame(animate);
      } catch (e) {
        console.error("引入WebAssembly模块失败，已禁用粒子交互：", e);
      }
    });

    onDestroy(() => {
      // 清理事件监听
      particles?.free();
      intersectionObserver?.disconnect();
    });
  }
</script>

<svg
  bind:this={svg}
  width={nanoflowCfg.cwidth}
  height={nanoflowCfg.cheight}
  viewBox={`0 0 ${nanoflowCfg.cwidth} ${nanoflowCfg.cheight}`}
  preserveAspectRatio="xMidYMid meet"
  onpointermove={handlePointerMove}
  onpointerleave={handleMouseLeave}
  role="presentation"
  xmlns="http://www.w3.org/2000/svg"
>
  {#each nanoflowCfg.particles as particle, index}
    <circle
      r={particle.size}
      fill={`rgb(${particle.color.join(",")})`}
      bind:this={particleElements[index]}
      cx={particle.x}
      cy={particle.y}
    />
  {/each}
</svg>

<style>
  svg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;

    @media (pointer: fine) {
      cursor: crosshair;
    }
  }
</style>
