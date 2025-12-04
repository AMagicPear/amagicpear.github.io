use js_sys::Math;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ParticleData {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
pub struct Particle {
    pub cx: f32,
    pub cy: f32,
    pub base_x: f32,
    pub base_y: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub dest_x: f32,
    pub dest_y: f32,
    pub elasticity_factor: f32,
    pub max_push_force: f32,
}

#[wasm_bindgen]
pub struct ParticleMouse {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub speed: f32,
}

#[wasm_bindgen]
impl ParticleData {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
impl ParticleMouse {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            x: -1000.0,
            y: -1000.0,
            vx: 0.0,
            vy: 0.0,
            speed: 0.0,
        }
    }
}

#[wasm_bindgen]
impl Particle {
    #[wasm_bindgen(constructor)]
    pub fn new(data: ParticleData, elasticity_factor: f32, max_push_force: f32) -> Self {
        let ParticleData { x, y } = data;
        Self {
            cx: x,
            cy: y,
            base_x: x,
            base_y: y,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            dest_x: x,
            dest_y: y,
            elasticity_factor,
            max_push_force,
        }
    }

    pub fn update(&mut self, mouse: ParticleMouse, scatter_strength: f32) -> ParticleData {
        if scatter_strength > 0.01 {
            let explosion_factor = f32::min(scatter_strength * 2.5, 6.0);
            self.offset_x += (Math::random() as f32 - 0.5) * scatter_strength * explosion_factor;
            self.offset_y += (Math::random() as f32 - 0.5) * scatter_strength * explosion_factor;
            let max_offset = scatter_strength * 18.0 + 18.0;
            self.offset_x = f32::max(-max_offset, f32::min(max_offset, self.offset_x));
            self.offset_y = f32::max(-max_offset, f32::min(max_offset, self.offset_y));
            if scatter_strength > 6.0 {
                self.vx += (Math::random() as f32 - 0.5) * scatter_strength * 2.0;
                self.vy += (Math::random() as f32 - 0.5) * scatter_strength * 2.0;
            }
        } else {
            self.offset_x *= 0.8;
            self.offset_y *= 0.8;
        }
        self.dest_x = self.cx + (self.base_x - self.cx) + self.offset_x;
        self.dest_y = self.cy + (self.base_y - self.cy) + self.offset_y;
        let dx = self.dest_x - self.x;
        let dy = self.dest_y - self.y;
        self.vx += dx * self.elasticity_factor;
        self.vy += dy * self.elasticity_factor;
        if scatter_strength < 0.05 {
            let (mx, my) = (mouse.x, mouse.y);
            let dist2 = (self.x - mx).powi(2) + (self.y - my).powi(2);
            let min_dist = 18.0 + f32::min(mouse.speed * 2.5, 120.0);
            let thickness = (min_dist * (1.5 + self.max_push_force * 7.0)).powi(2);
            let angle = Math::atan2((self.y - my) as f64, (self.x - mx) as f64);
            if dist2 < thickness {
                let mut f = thickness / dist2;
                f = f32::max(0.1, f32::min(f, 20.0));
                if f > 0.5 && f <= 1.5 {
                    f = 0.5;
                }
                let vx = f * Math::cos(angle) as f32;
                let vy = f * Math::sin(angle) as f32;
                self.vx -= vx * self.max_push_force * 1.5
                    + ((self.base_x - self.x) * self.elasticity_factor) / 250.0;
                self.vy -= vy * self.max_push_force * 1.5
                    + ((self.base_y - self.y) * self.elasticity_factor) / 250.0;
            }
        }
        const MAX_SPEED: f32 = 10.0;
        let speed = Math::sqrt((self.vx.powi(2) + self.vy.powi(2)) as f64) as f32;
        if speed > MAX_SPEED {
            self.vx = self.vx / speed * MAX_SPEED;
            self.vy = self.vy / speed * MAX_SPEED;
        }
        self.vx *= 0.7;
        self.vy *= 0.7;
        self.x += self.vx;
        self.y += self.vy;
        ParticleData {
            x: self.x,
            y: self.y,
        }
    }
}
