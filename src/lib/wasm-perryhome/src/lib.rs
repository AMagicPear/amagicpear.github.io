use std::ops::AddAssign;
use fastrand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(C)] // 保证内存连续
#[derive(Clone, Copy)]
pub struct Position(f32, f32);

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
}

pub struct Particle {
    pos: Position,
    base_pos: Position,
    v: Position,
    offset: Position,
    elasticity_factor: f32,
    max_push_force: f32,
}

impl Particle {
    pub fn new(data: &Position, elasticity_factor: f32, max_push_force: f32) -> Self {
        Self {
            base_pos: *data,
            pos: *data,
            v: Position(0.0, 0.0),
            offset: Position(0.0, 0.0),
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
            self.offset += Position(
                (rng.f32() - 0.5) * scatter_strength * explosion_factor,
                (rng.f32() - 0.5) * scatter_strength * explosion_factor,
            );
            let max_offset = scatter_strength.mul_add(18.0, 18.0);
            self.offset.0 = self.offset.0.clamp(-max_offset, max_offset);
            self.offset.1 = self.offset.1.clamp(-max_offset, max_offset);

            if scatter_strength > 6.0 {
                self.offset += Position(
                    (rng.f32() - 0.5) * scatter_strength * 2.0,
                    (rng.f32() - 0.5) * scatter_strength * 2.0,
                );
            }
        } else {
            self.offset.0 *= OFFSET_DAMP;
            self.offset.1 *= OFFSET_DAMP;
        }

        let dx = self.base_pos.0 + self.offset.0 - self.pos.0;
        let dy = self.base_pos.1 + self.offset.1 - self.pos.1;

        self.v.0 = self.v.0.mul_add(DRAG, dx * self.elasticity_factor);
        self.v.1 = self.v.1.mul_add(DRAG, dy * self.elasticity_factor);

        if scatter_strength < 0.05 {
            let dx_mouse = self.pos.0 - mouse_x;
            let dy_mouse = self.pos.1 - mouse_y;
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

                self.v.0 -= vx.mul_add(
                    push_strength,
                    (self.base_pos.0 - self.pos.0) * elasticity_factor,
                );
                self.v.1 -= vy.mul_add(
                    push_strength,
                    (self.base_pos.1 - self.pos.1) * elasticity_factor,
                );
            }
        }

        let speed_sq = self.v.0 * self.v.0 + self.v.1 * self.v.1;
        if speed_sq > MAX_SPEED * MAX_SPEED {
            let inv_speed = speed_sq.sqrt().recip();
            self.v.0 *= MAX_SPEED * inv_speed;
            self.v.1 *= MAX_SPEED * inv_speed;
        }
        self.pos += Position(self.v.0, self.v.1);
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
            self.positions[i] = particle.pos;
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
