export interface ParticleData {
  x: number;
  y: number;
  size: number;
  color: [number, number, number];
}

export class Particle implements ParticleData {
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
  elasticityFactor: number;
  maxPushForce: number;
  id: string;

  constructor(data: ParticleData, index: number, elasticityFactor: number, maxPushForce: number) {
    Object.assign(this, data);
    this.cx = data.x;
    this.cy = data.y;
    this.baseX = data.x;
    this.baseY = data.y;
    this.destX = data.x;
    this.destY = data.y;
    this.elasticityFactor = elasticityFactor;
    this.maxPushForce = maxPushForce;
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
    scatterStrength: number,
  ) {
    if (scatterStrength > 0.01) {
      let explosionFactor = Math.min(scatterStrength * 2.5, 6);
      this.offsetX += (Math.random() - 0.5) * scatterStrength * explosionFactor;
      this.offsetY += (Math.random() - 0.5) * scatterStrength * explosionFactor;
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
      let dist2 = (this.x - mx) * (this.x - mx) + (this.y - my) * (this.y - my);
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

    // 限制粒子速度，防止异常大值导致粒子飞散
    const maxSpeed = 10;
    const speed = Math.sqrt(this.vx * this.vx + this.vy * this.vy);
    if (speed > maxSpeed) {
      this.vx = (this.vx / speed) * maxSpeed;
      this.vy = (this.vy / speed) * maxSpeed;
    }

    this.vx *= 0.7;
    this.vy *= 0.7;
    this.x += this.vx;
    this.y += this.vy;
  }
}
