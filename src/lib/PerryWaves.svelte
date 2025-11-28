<!-- Powered by https://github.com/ZTMYO/NanoFlow | MIT License -->
<script module>
  import nanoflowCfg from "../assets/nanoflow.json";
  type ParticleData = (typeof nanoflowCfg.particles)[number];
  class Particle {
    cx: number;
    cy: number;
    baseX: number;
    baseY: number;
    x: number;
    y: number;
    vx: number;
    vy: number;
    size: number;
    color: string;
    offsetX: number;
    offsetY: number;
    destX: number;
    destY: number;
    elasticityFactor: number;
    maxPushForce: number;

    constructor(data: ParticleData) {
      this.cx = data.x;
      this.cy = data.y;
      this.baseX = data.x;
      this.baseY = data.y;
      this.x = data.x;
      this.y = data.y;
      this.vx = 0;
      this.vy = 0;
      this.size = data.size;
      this.color = data.color;
      this.offsetX = 0;
      this.offsetY = 0;
      this.destX = data.x;
      this.destY = data.y;
      this.elasticityFactor = nanoflowCfg.elasticityFactor;
      this.maxPushForce = nanoflowCfg.maxPushForce;
    }

    update(
      mouse: {
        x: number;
        y: number;
        vx: number;
        vy: number;
        speed: number;
      } | null,
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
        let mx = mouse!.x;
        let my = mouse!.y;
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
  let canvas: HTMLCanvasElement;

  onMount(() => {
    const ctx = canvas.getContext("2d")!;
    canvas.width = nanoflowCfg.cwidth;
    canvas.height = nanoflowCfg.cheight;

    let particles = nanoflowCfg.particles.map((p) => new Particle(p));
    let mouse = { x: -1000, y: -1000, vx: 0, vy: 0, speed: 0 };

    canvas.addEventListener("mousemove", function (e: MouseEvent) {
      const rect = canvas.getBoundingClientRect();
      const cx = e.clientX - rect.left;
      const cy = e.clientY - rect.top;
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
    });

    canvas.addEventListener("mouseleave", function () {
      mouse.x = -1000;
      mouse.y = -1000;
      mouse.vx = 0;
      mouse.vy = 0;
      mouse.speed = 0;
    });

    function animate() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      let disperseFactor = 1;
      let scatterStrength = 0;
      if (mouse) {
        if (mouse.speed > 220) {
          scatterStrength = Math.min((mouse.speed - 220) / 8, 16);
        }
      }
      for (let p of particles) {
        p.update(mouse, disperseFactor, scatterStrength);
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fillStyle = p.color;
        ctx.fill();
      }
      requestAnimationFrame(animate);
    }

    animate();
  });
</script>

<canvas bind:this={canvas}></canvas>

<style>
  canvas {
    transform: translateX(200px);
  }
</style>
