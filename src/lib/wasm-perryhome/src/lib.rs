use fastrand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(C)] // 保证内存连续
#[derive(Clone, Copy)]
pub struct Position(f32, f32);

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
}

pub struct Particle {
    base_x: f32,
    base_y: f32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    offset_x: f32,
    offset_y: f32,
    elasticity_factor: f32,
    max_push_force: f32,
}

impl Particle {
    pub fn new(data: &Position, elasticity_factor: f32, max_push_force: f32) -> Self {
        Self {
            base_x: data.0,
            base_y: data.1,
            x: data.0,
            y: data.1,
            vx: 0.0,
            vy: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            elasticity_factor,
            max_push_force,
        }
    }

    pub fn update(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        mouse_speed: f32,
        scatter_strength: f32,
        rng: &mut Rng,
    ) {
        const MAX_SPEED: f32 = 10.0;
        const DRAG: f32 = 0.7;
        const OFFSET_DAMP: f32 = 0.8;
        let scatter_active = scatter_strength > 0.01;

        if scatter_active {
            let explosion_factor = f32::min(scatter_strength * 2.5, 6.0);

            self.offset_x += (rng.f32() - 0.5) * scatter_strength * explosion_factor;
            self.offset_y += (rng.f32() - 0.5) * scatter_strength * explosion_factor;

            let max_offset = scatter_strength.mul_add(18.0, 18.0);
            self.offset_x = self.offset_x.clamp(-max_offset, max_offset);
            self.offset_y = self.offset_y.clamp(-max_offset, max_offset);

            if scatter_strength > 6.0 {
                self.vx += (rng.f32() - 0.5) * scatter_strength * 2.0;
                self.vy += (rng.f32() - 0.5) * scatter_strength * 2.0;
            }
        } else {
            self.offset_x *= OFFSET_DAMP;
            self.offset_y *= OFFSET_DAMP;
        }

        let dx = self.base_x + self.offset_x - self.x;
        let dy = self.base_y + self.offset_y - self.y;

        self.vx = self.vx.mul_add(DRAG, dx * self.elasticity_factor);
        self.vy = self.vy.mul_add(DRAG, dy * self.elasticity_factor);

        if scatter_strength < 0.05 {
            let dx_mouse = self.x - mouse_x;
            let dy_mouse = self.y - mouse_y;
            let dist2 = dx_mouse.powi(2) + dy_mouse.powi(2);

            let min_dist = mouse_speed.mul_add(2.5, 18.0).min(138.0);
            let thickness_factor = 1.5 + self.max_push_force * 7.0;
            let thickness = (min_dist * thickness_factor).powi(2);

            if dist2 < thickness {
                let mut f = thickness / dist2;
                f = f.clamp(0.1, 20.0);
                if (0.5..=1.5).contains(&f) {
                    f = 0.5;
                }

                let angle = dy_mouse.atan2(dx_mouse);
                let cos_angle = angle.cos();
                let sin_angle = angle.sin();

                let vx = f * cos_angle;
                let vy = f * sin_angle;

                let push_strength = self.max_push_force * 1.5;
                let elasticity_factor = self.elasticity_factor / 250.0;

                self.vx -= vx.mul_add(push_strength, (self.base_x - self.x) * elasticity_factor);
                self.vy -= vy.mul_add(push_strength, (self.base_y - self.y) * elasticity_factor);
            }
        }

        let speed_sq = self.vx * self.vx + self.vy * self.vy;
        if speed_sq > MAX_SPEED * MAX_SPEED {
            let inv_speed = speed_sq.sqrt().recip();
            self.vx *= MAX_SPEED * inv_speed;
            self.vy *= MAX_SPEED * inv_speed;
        }

        self.x += self.vx;
        self.y += self.vy;
    }
}

#[wasm_bindgen]
pub struct Particles {
    data: Vec<Particle>,
    positions: Vec<Position>,
    rng: Rng,
}

#[wasm_bindgen]
impl Particles {
    #[wasm_bindgen(constructor)]
    pub fn new(particle_datas: Vec<Position>, elasticity_factor: f32, max_push_force: f32) -> Self {
        Particles {
            positions: particle_datas.clone(),
            data: particle_datas
                .into_iter()
                .map(|pos| Particle::new(&pos, elasticity_factor, max_push_force))
                .collect(),
            rng: Rng::new(),
        }
    }

    pub fn update(&mut self, mouse_x: f32, mouse_y: f32, mouse_speed: f32, scatter_strength: f32) {
        for (i, particle) in self.data.iter_mut().enumerate() {
            particle.update(
                mouse_x,
                mouse_y,
                mouse_speed,
                scatter_strength,
                &mut self.rng,
            );
            self.positions[i].0 = particle.x;
            self.positions[i].1 = particle.y;
        }
    }

    /// JS 直接读取内存用的指针
    pub fn positions_ptr(&self) -> *const Position {
        self.positions.as_ptr()
    }

    pub fn positions_len(&self) -> usize {
        self.positions.len()
    }
}
