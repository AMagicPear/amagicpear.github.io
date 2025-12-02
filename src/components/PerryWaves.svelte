<!-- Modified from https://github.com/ZTMYO/NanoFlow | MIT License -->
<script module lang="ts">
  import nanoflowCfg from "@/assets/simplified_nanoflow.json";

  interface ParticleData {
    x: number;
    y: number;
    size: number;
    color: [number, number, number];
  }

  class Particle implements ParticleData {
    cx: number;
    cy: number;
    baseX: number;
    baseY: number;
    x: number = 0;
    y: number = 0;
    vx: number = 0;
    vy: number = 0;
    size: number = 0;
    color: [number, number, number] = [0, 0, 0];
    offsetX: number = 0;
    offsetY: number = 0;
    destX: number = 0;
    destY: number = 0;
    elasticityFactor: number = nanoflowCfg.elasticityFactor;
    maxPushForce: number = nanoflowCfg.maxPushForce;
    id: string;

    constructor(data: ParticleData, index: number) {
      Object.assign(this, data);
      this.cx = data.x;
      this.cy = data.y;
      this.baseX = data.x;
      this.baseY = data.y;
      this.destX = data.x;
      this.destY = data.y;
      this.id = `particle-${index}`;
    }

    update(
      mouse: {
        x: number;
        y: number;
        vx: number;
        vy: number;
        speed: number;
      },
      disperseFactor: number,
      scatterStrength: number
    ) {
      if (scatterStrength > 0.01) {
        let explosionFactor = Math.min(scatterStrength * 2.5, 6);
        this.offsetX +=
          (Math.random() - 0.5) * scatterStrength * explosionFactor;
        this.offsetY +=
          (Math.random() - 0.5) * scatterStrength * explosionFactor;
        let maxOffset = scatterStrength * 18 + 18;
        this.offsetX = Math.max(-maxOffset, Math.min(maxOffset, this.offsetX));
        this.offsetY = Math.max(-maxOffset, Math.min(maxOffset, this.offsetY));
        if (scatterStrength > 6) {
          this.vx += (Math.random() - 0.5) * scatterStrength * 2;
          this.vy += (Math.random() - 0.5) * scatterStrength * 2;
        }
      } else {
        this.offsetX *= 0.8;
        this.offsetY *= 0.8;
      }
      this.destX =
        this.cx + (this.baseX - this.cx) * disperseFactor + this.offsetX;
      this.destY =
        this.cy + (this.baseY - this.cy) * disperseFactor + this.offsetY;
      let dx = this.destX - this.x;
      let dy = this.destY - this.y;
      this.vx += dx * this.elasticityFactor;
      this.vy += dy * this.elasticityFactor;
      let enableEffect =
        (disperseFactor <= 1.01 && scatterStrength < 0.05 && mouse) ||
        (scatterStrength > 0 && mouse);
      if (enableEffect) {
        let mx = mouse.x;
        let my = mouse.y;
        let dist2 =
          (this.x - mx) * (this.x - mx) + (this.y - my) * (this.y - my);
        let minDist = 18 + Math.min(mouse!.speed * 2.5, 120);
        let thickness = (minDist * (1.5 + this.maxPushForce * 7)) ** 2;
        let angle = Math.atan2(this.y - my, this.x - mx);
        if (dist2 < thickness) {
          let f = thickness / dist2;
          f = Math.max(0.1, Math.min(f, 20));
          if (f > 0.5 && f <= 1.5) f = 0.5;
          let vx = f * Math.cos(angle);
          let vy = f * Math.sin(angle);
          this.vx -=
            vx * this.maxPushForce * 1.5 +
            ((this.baseX - this.x) * this.elasticityFactor) / 250;
          this.vy -=
            vy * this.maxPushForce * 1.5 +
            ((this.baseY - this.y) * this.elasticityFactor) / 250;
        }
      }

      this.vx *= 0.7;
      this.vy *= 0.7;
      this.x += this.vx;
      this.y += this.vy;
    }
  }
</script>

<script lang="ts">
  import { onMount } from "svelte";

  // SVG缩放因子，与CSS中的scale值保持一致
  const scaleFactor = 1.4;
  const hasMouse = window.matchMedia("(pointer: fine)").matches;
  const disperseFactor = 1;
  let scatterStrength = 0;

  let svg: SVGSVGElement;
  let particles: Particle[] = nanoflowCfg.particles.map(
    (p, index) => new Particle(p as ParticleData, index)
  );
  let mouse = { x: -1000, y: -1000, vx: 0, vy: 0, speed: 0 };
  let particleElements: { [key: string]: SVGCircleElement } = {};

  const updateParticles = (particles: Particle[]) => {
    particles.forEach((particle) => {
      particle.update(mouse, disperseFactor, scatterStrength);
      const element = particleElements[particle.id];
      element?.setAttribute("cx", String(particle.x));
      element?.setAttribute("cy", String(particle.y));
    });
  };

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
      mouse.speed = Math.sqrt(mouse.vx * mouse.vx + mouse.vy * mouse.vy);
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
  };

  onMount(() => {
    // 初始化粒子元素引用
    particles.forEach((particle) => {
      const element = svg.querySelector(`#${particle.id}`);
      if (element) {
        particleElements[particle.id] = element as SVGCircleElement;
      }
    });
    updateParticles(particles);
    if (!hasMouse) return;

    function animate() {
      if (mouse.speed > 220) {
        scatterStrength = Math.min((mouse.speed - 220) / 8, 16);
      }
      updateParticles(particles);
      requestAnimationFrame(animate);
    }
    requestAnimationFrame(animate);
  });
</script>

<svg
  bind:this={svg}
  width={nanoflowCfg.cwidth}
  height={nanoflowCfg.cheight}
  on:mousemove={handleMouseMove}
  on:mouseleave={handleMouseLeave}
  role="presentation"
  xmlns="http://www.w3.org/2000/svg"
>
  {#each particles as particle}
    <circle
      id={particle.id}
      r={particle.size}
      fill={`rgb(${particle.color.join(",")})`}
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
