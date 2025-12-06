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

    pub fn update_scalar(
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

    /// The SIMD-optimized update function.
    #[cfg(target_arch = "wasm32")]
    #[target_feature(enable = "simd128")]
    pub fn update(&mut self, mouse_x: f32, mouse_y: f32, mouse_speed: f32, scatter_strength: f32) {
        use std::arch::wasm32::*;
        const MAX_SPEED: f32 = 10.0;
        const DRAG: f32 = 0.7;
        const OFFSET_DAMP: f32 = 0.8;
        let scatter_active = scatter_strength > 0.01;

        let len = self.data.len();
        let simd_len = len - (len % 4);

        // Pre-calculate SIMD constants
        let max_speed_vec = f32x4_splat(MAX_SPEED);
        let drag_vec = f32x4_splat(DRAG);
        let offset_damp_vec = f32x4_splat(OFFSET_DAMP);
        let one_vec = f32x4_splat(1.0);
        let half_vec = f32x4_splat(0.5);
        let min_dist_val = mouse_speed.mul_add(2.5, 18.0).min(138.0);
        let min_dist_vec = f32x4_splat(min_dist_val);

        // Mouse position vector
        let mouse_x_vec = f32x4_splat(mouse_x);
        let mouse_y_vec = f32x4_splat(mouse_y);

        for i in (0..simd_len).step_by(4) {
            let particles = &mut self.data[i..i+4];

            // Load particle data into SIMD vectors
            let pos_x_vec = f32x4(particles[0].pos.0, particles[1].pos.0, particles[2].pos.0, particles[3].pos.0);
            let pos_y_vec = f32x4(particles[0].pos.1, particles[1].pos.1, particles[2].pos.1, particles[3].pos.1);
            let base_pos_x_vec = f32x4(particles[0].base_pos.0, particles[1].base_pos.0, particles[2].base_pos.0, particles[3].base_pos.0);
            let base_pos_y_vec = f32x4(particles[0].base_pos.1, particles[1].base_pos.1, particles[2].base_pos.1, particles[3].base_pos.1);
            let v_x_vec = f32x4(particles[0].v.0, particles[1].v.0, particles[2].v.0, particles[3].v.0);
            let v_y_vec = f32x4(particles[0].v.1, particles[1].v.1, particles[2].v.1, particles[3].v.1);
            let offset_x_vec = f32x4(particles[0].offset.0, particles[1].offset.0, particles[2].offset.0, particles[3].offset.0);
            let offset_y_vec = f32x4(particles[0].offset.1, particles[1].offset.1, particles[2].offset.1, particles[3].offset.1);
            let elasticity_vec = f32x4(particles[0].elasticity_factor, particles[1].elasticity_factor, particles[2].elasticity_factor, particles[3].elasticity_factor);
            let max_push_vec = f32x4(particles[0].max_push_force, particles[1].max_push_force, particles[2].max_push_force, particles[3].max_push_force);

            // --- Scatter effect ---
            let mut new_offset_x = offset_x_vec;
            let mut new_offset_y = offset_y_vec;

            if scatter_active {
                let explosion_factor_val = f32::min(scatter_strength * 2.5, 6.0);
                let explosion_factor_vec = f32x4_splat(explosion_factor_val);
                let scatter_vec = f32x4_splat(scatter_strength);

                let rand_x1 = f32x4(self.rng.f32(), self.rng.f32(), self.rng.f32(), self.rng.f32());
                let rand_y1 = f32x4(self.rng.f32(), self.rng.f32(), self.rng.f32(), self.rng.f32());
                let offset_add_x = f32x4_mul(f32x4_sub(rand_x1, half_vec), f32x4_mul(scatter_vec, explosion_factor_vec));
                let offset_add_y = f32x4_mul(f32x4_sub(rand_y1, half_vec), f32x4_mul(scatter_vec, explosion_factor_vec));

                new_offset_x = f32x4_add(new_offset_x, offset_add_x);
                new_offset_y = f32x4_add(new_offset_y, offset_add_y);

                let max_offset_val = scatter_strength.mul_add(18.0, 18.0);
                let max_offset_vec = f32x4_splat(max_offset_val);
                new_offset_x = f32x4_max(f32x4_min(new_offset_x, max_offset_vec), f32x4_neg(max_offset_vec));
                new_offset_y = f32x4_max(f32x4_min(new_offset_y, max_offset_vec), f32x4_neg(max_offset_vec));

                if scatter_strength > 6.0 {
                    let rand_x2 = f32x4(self.rng.f32(), self.rng.f32(), self.rng.f32(), self.rng.f32());
                    let rand_y2 = f32x4(self.rng.f32(), self.rng.f32(), self.rng.f32(), self.rng.f32());
                    let extra_scatter_vec = f32x4_splat(scatter_strength * 2.0);
                    new_offset_x = f32x4_add(new_offset_x, f32x4_mul(f32x4_sub(rand_x2, half_vec), extra_scatter_vec));
                    new_offset_y = f32x4_add(new_offset_y, f32x4_mul(f32x4_sub(rand_y2, half_vec), extra_scatter_vec));
                }
            } else {
                new_offset_x = f32x4_mul(new_offset_x, offset_damp_vec);
                new_offset_y = f32x4_mul(new_offset_y, offset_damp_vec);
            }

            // --- Elasticity ---
            let target_x = f32x4_add(base_pos_x_vec, new_offset_x);
            let target_y = f32x4_add(base_pos_y_vec, new_offset_y);
            let dx = f32x4_sub(target_x, pos_x_vec);
            let dy = f32x4_sub(target_y, pos_y_vec);
            let mut new_v_x = f32x4_add(f32x4_mul(v_x_vec, drag_vec), f32x4_mul(dx, elasticity_vec));
            let mut new_v_y = f32x4_add(f32x4_mul(v_y_vec, drag_vec), f32x4_mul(dy, elasticity_vec));

            // --- Mouse interaction (Hybrid SIMD/Scalar) ---
            if scatter_strength < 0.05 {
                let dx_mouse = f32x4_sub(pos_x_vec, mouse_x_vec);
                let dy_mouse = f32x4_sub(pos_y_vec, mouse_y_vec);
                let dist2 = f32x4_add(f32x4_mul(dx_mouse, dx_mouse), f32x4_mul(dy_mouse, dy_mouse));

                let thickness_factor_vec = f32x4_add(f32x4_splat(1.5), f32x4_mul(max_push_vec, f32x4_splat(7.0)));
                let thickness_sq = f32x4_mul(f32x4_mul(min_dist_vec, thickness_factor_vec), f32x4_mul(min_dist_vec, thickness_factor_vec));

                let mask = f32x4_lt(dist2, thickness_sq);

                // Check if any particle is within the mouse influence radius
                if v128_any_true(mask) {
                    // De-vectorize for trigonometry by extracting to arrays
                    let dx_mouse_arr = [
                        f32x4_extract_lane::<0>(dx_mouse), f32x4_extract_lane::<1>(dx_mouse),
                        f32x4_extract_lane::<2>(dx_mouse), f32x4_extract_lane::<3>(dx_mouse),
                    ];
                    let dy_mouse_arr = [
                        f32x4_extract_lane::<0>(dy_mouse), f32x4_extract_lane::<1>(dy_mouse),
                        f32x4_extract_lane::<2>(dy_mouse), f32x4_extract_lane::<3>(dy_mouse),
                    ];
                    let dist2_arr = [
                        f32x4_extract_lane::<0>(dist2), f32x4_extract_lane::<1>(dist2),
                        f32x4_extract_lane::<2>(dist2), f32x4_extract_lane::<3>(dist2),
                    ];
                    let thickness_sq_arr = [
                        f32x4_extract_lane::<0>(thickness_sq), f32x4_extract_lane::<1>(thickness_sq),
                        f32x4_extract_lane::<2>(thickness_sq), f32x4_extract_lane::<3>(thickness_sq),
                    ];

                    let mut vx_scalar = [0.0; 4];
                    let mut vy_scalar = [0.0; 4];

                    for j in 0..4 {
                        if dist2_arr[j] < thickness_sq_arr[j] {
                            let mut f = thickness_sq_arr[j] / dist2_arr[j];
                            f = f.clamp(0.1, 20.0);
                            if (0.5..=1.5).contains(&f) { f = 0.5; }

                            let angle = dy_mouse_arr[j].atan2(dx_mouse_arr[j]);
                            vx_scalar[j] = f * angle.cos();
                            vy_scalar[j] = f * angle.sin();
                        }
                    }

                    let vx_vec = f32x4(vx_scalar[0], vx_scalar[1], vx_scalar[2], vx_scalar[3]);
                    let vy_vec = f32x4(vy_scalar[0], vy_scalar[1], vy_scalar[2], vy_scalar[3]);

                    let push_strength_vec = f32x4_mul(max_push_vec, f32x4_splat(1.5));
                    let elasticity_div_vec = f32x4_div(elasticity_vec, f32x4_splat(250.0));
                    let elastic_x = f32x4_mul(f32x4_sub(base_pos_x_vec, pos_x_vec), elasticity_div_vec);
                    let elastic_y = f32x4_mul(f32x4_sub(base_pos_y_vec, pos_y_vec), elasticity_div_vec);

                    let push_x = f32x4_mul(vx_vec, push_strength_vec);
                    let push_y = f32x4_mul(vy_vec, push_strength_vec);

                    new_v_x = f32x4_sub(new_v_x, f32x4_add(push_x, elastic_x));
                    new_v_y = f32x4_sub(new_v_y, f32x4_add(push_y, elastic_y));
                }
            }

            // --- Speed limiting ---
            let speed_sq = f32x4_add(f32x4_mul(new_v_x, new_v_x), f32x4_mul(new_v_y, new_v_y));
            let max_speed_sq_vec = f32x4_splat(MAX_SPEED * MAX_SPEED);
            let over_speed_mask = f32x4_gt(speed_sq, max_speed_sq_vec);

            if v128_any_true(over_speed_mask) {
                let inv_speed = f32x4_div(one_vec, f32x4_sqrt(speed_sq));
                let speed_scale = f32x4_mul(max_speed_vec, inv_speed);
                // Use v128_bitselect for conditional operations
                new_v_x = v128_bitselect(over_speed_mask, f32x4_mul(new_v_x, speed_scale), new_v_x);
                new_v_y = v128_bitselect(over_speed_mask, f32x4_mul(new_v_y, speed_scale), new_v_y);
            }

            // --- Update position ---
            let new_pos_x = f32x4_add(pos_x_vec, new_v_x);
            let new_pos_y = f32x4_add(pos_y_vec, new_v_y);

            // --- Store results back to particles (verbose but necessary) ---
            particles[0].offset.0 = f32x4_extract_lane::<0>(new_offset_x);
            particles[0].offset.1 = f32x4_extract_lane::<0>(new_offset_y);
            particles[0].v.0 = f32x4_extract_lane::<0>(new_v_x);
            particles[0].v.1 = f32x4_extract_lane::<0>(new_v_y);
            particles[0].pos.0 = f32x4_extract_lane::<0>(new_pos_x);
            particles[0].pos.1 = f32x4_extract_lane::<0>(new_pos_y);

            particles[1].offset.0 = f32x4_extract_lane::<1>(new_offset_x);
            particles[1].offset.1 = f32x4_extract_lane::<1>(new_offset_y);
            particles[1].v.0 = f32x4_extract_lane::<1>(new_v_x);
            particles[1].v.1 = f32x4_extract_lane::<1>(new_v_y);
            particles[1].pos.0 = f32x4_extract_lane::<1>(new_pos_x);
            particles[1].pos.1 = f32x4_extract_lane::<1>(new_pos_y);

            particles[2].offset.0 = f32x4_extract_lane::<2>(new_offset_x);
            particles[2].offset.1 = f32x4_extract_lane::<2>(new_offset_y);
            particles[2].v.0 = f32x4_extract_lane::<2>(new_v_x);
            particles[2].v.1 = f32x4_extract_lane::<2>(new_v_y);
            particles[2].pos.0 = f32x4_extract_lane::<2>(new_pos_x);
            particles[2].pos.1 = f32x4_extract_lane::<2>(new_pos_y);

            particles[3].offset.0 = f32x4_extract_lane::<3>(new_offset_x);
            particles[3].offset.1 = f32x4_extract_lane::<3>(new_offset_y);
            particles[3].v.0 = f32x4_extract_lane::<3>(new_v_x);
            particles[3].v.1 = f32x4_extract_lane::<3>(new_v_y);
            particles[3].pos.0 = f32x4_extract_lane::<3>(new_pos_x);
            particles[3].pos.1 = f32x4_extract_lane::<3>(new_pos_y);

            self.positions[i] = particles[0].pos;
            self.positions[i + 1] = particles[1].pos;
            self.positions[i + 2] = particles[2].pos;
            self.positions[i + 3] = particles[3].pos;
        }

        // Handle remaining particles (scalar processing)
        for i in simd_len..len {
            self.data[i].update_scalar(mouse_x, mouse_y, mouse_speed, scatter_strength, &mut self.rng);
            self.positions[i] = self.data[i].pos;
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
