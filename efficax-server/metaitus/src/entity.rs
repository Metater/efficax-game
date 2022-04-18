use cgmath::{Vector2, Zero};

use super::{physics::collider::PhysicsCollider};

#[derive(Debug)]
pub struct MetaitusEntity {
    pub id: u32,
    pub pos: Vector2<f32>,
    pub current_cell_index: u32,

    bounds: PhysicsCollider,

    has_vel_epsilon: bool,
    vel_epsilon: f32,

    has_collider: bool,
    collider: PhysicsCollider,

    pub has_repulsion_radius: bool,
    pub repulsion_radius: f32,
    max_repulsion_mag: f32,
    pub repulsion: f32,

    has_drag: bool,
    has_linear_drag: bool,
    drag: f32,

    pub vel: Vector2<f32>,
    pub moved_xy: bool,
    pub tick_count: u32,
}

impl MetaitusEntity {
    pub fn new(id: u32, pos: Vector2<f32>, current_cell_index: u32) -> Self {
        MetaitusEntity {
            id,
            pos,
            current_cell_index,

            bounds: PhysicsCollider::all(),

            has_vel_epsilon: true,
            vel_epsilon: 1.0 / 12.0,

            has_collider: false,
            collider: PhysicsCollider::none(),

            has_drag: false,
            has_linear_drag: false,
            drag: 0.0,

            has_repulsion_radius: false,
            repulsion_radius: 0.0,
            max_repulsion_mag: 0.0,
            repulsion: 0.0,

            vel: Vector2::zero(),
            moved_xy: false,
            tick_count: 0,
        }
    }

    pub fn with_bounds(&mut self, bounds: PhysicsCollider) -> &mut Self {
        self.bounds = bounds;
        self
    }

    pub fn with_vel_epsilon(&mut self, has_vel_epsilon: bool, vel_epsilon: f32) -> &mut Self {
        self.has_vel_epsilon = has_vel_epsilon;
        self.vel_epsilon = vel_epsilon;
        self
    }

    pub fn with_collider(&mut self, has_collider: bool, collider: PhysicsCollider) -> &mut Self {
        self.has_collider = has_collider;
        self.collider = collider;
        self
    }

    pub fn with_drag(&mut self, has_drag: bool, has_linear_drag: bool, drag: f32) -> &mut Self {
        self.has_drag = has_drag;
        self.has_linear_drag = has_linear_drag;
        self.drag = drag;
        self
    }

    pub fn with_repulsion_radius(&mut self, has_repulsion_radius: bool, repulsion_radius: f32, max_repulsion_mag: f32, repulsion: f32) -> &mut Self {
        self.has_repulsion_radius = has_repulsion_radius;
        self.repulsion_radius = repulsion_radius;
        self.max_repulsion_mag = max_repulsion_mag;
        self.repulsion = repulsion;
        self
    }
}

impl MetaitusEntity {
    pub fn add_force(&mut self, force: Vector2<f32>, delta_time: f32) {
        self.vel += force * delta_time;
    }

    pub fn tick(&mut self, delta_time: f32, near_statics: &Vec<PhysicsCollider>, repulsable_entities: &Vec<(Vector2<f32>, f32, f32)>) -> bool {
        self.moved_xy = false;
        
        if self.has_vel_epsilon {
            self.apply_vel_epsilon();
        }

        if !self.vel.is_zero() {
            self.moved_xy = self.update_pos(delta_time, near_statics);
            if self.moved_xy && self.has_drag {
                self.apply_drag(delta_time);
            }
        }
        else {
            // cannot move, velocity is zero
        }

        if repulsable_entities.len() > 0 {
            // if too much repulsion, use avg of vectors again
            //let mut repulsion_vectors_sum = Vector2::zero();
            for (entity_pos, entity_repulsion_radius, entity_repulsion) in repulsable_entities {
                let diff_x = entity_pos.x - self.pos.x;
                let diff_y = entity_pos.y - self.pos.y;
                let sqr_distance = (diff_x * diff_x) + (diff_y * diff_y);
                let radius_sum = entity_repulsion_radius + self.repulsion_radius;
                let sqr_radius_sum = radius_sum * radius_sum;
                if sqr_radius_sum > sqr_distance {
                    let mut repulsion_vector = Vector2::new(0.0, 1.0);
                    if !sqr_distance.is_zero() {
                        let mut repulsion_mag = (entity_repulsion + self.repulsion) * (1.0 / sqr_distance);
                        repulsion_mag = repulsion_mag.clamp(-self.max_repulsion_mag, self.max_repulsion_mag);
                        let distance = sqr_distance.sqrt();
                        repulsion_vector = -Vector2::new(diff_x / distance, diff_y / distance) * repulsion_mag;
                    }
                    //repulsion_vectors_sum += repulsion_vector;
                    self.add_force(repulsion_vector, delta_time);
                }
            }
            //let average_repulsion_vector = repulsion_vectors_sum / repulsable_entities.len() as f32;
            //self.add_force(average_repulsion_vector, delta_time);
        }

        self.tick_count += 1;
        
        self.moved_xy
    }

    fn apply_vel_epsilon(&mut self) {
        if self.vel.x != 0.0 && self.vel.x.abs() < self.vel_epsilon {
            self.vel = Vector2::new(0.0, self.vel.y);
        }
        if self.vel.y != 0.0 && self.vel.y.abs() < self.vel_epsilon {
            self.vel = Vector2::new(self.vel.x, 0.0);
        }
    }

    fn apply_drag(&mut self, timestep: f32) {
        if self.has_linear_drag {
            self.vel *= 1.0 - (self.drag * timestep);
        }
        else {
            let x_drag = self.drag * ((self.vel.x * self.vel.x) / 2.0);
            let y_drag = self.drag * ((self.vel.y * self.vel.y) / 2.0);
            self.vel.x *= 1.0 - (x_drag * timestep);
            self.vel.y *= 1.0 - (y_drag * timestep);
        }
    }

    fn update_pos(&mut self, timestep: f32, near_statics: &Vec<PhysicsCollider>) -> bool {
        if !self.has_collider {
            self.pos = self.get_next_pos(timestep);
            //return (!self.vel.x.is_zero(), !self.vel.y.is_zero())
            return true
        }

        let nominal_pos = self.get_next_pos(timestep);
        let x_delta = Vector2::new(self.vel.x * timestep, 0.0);
        let y_delta = Vector2::new(0.0, self.vel.y * timestep);

        let xy_collider = self.get_self_collider(nominal_pos);
        let x_collider = self.get_self_collider(self.pos + x_delta);
        let y_collider = self.get_self_collider(self.pos + y_delta);

        let mut move_x = !self.vel.x.is_zero();
        let mut move_y = !self.vel.y.is_zero();

        if !xy_collider.intersects(&self.bounds) {
            if move_x && !x_collider.intersects(&self.bounds) {
                move_x = false;
            }
            if move_y && !y_collider.intersects(&self.bounds) {
                move_y = false;
            }
        }

        // use lerp to step closer to colliders if needed
        if move_x || move_y {
            for collider in near_statics {
                if xy_collider.intersects(collider) {
                    if move_x && x_collider.intersects(collider) {
                        move_x = false;
                    }
                    if move_y && y_collider.intersects(collider) {
                        move_y = false;
                    }
                }
            }
        }

        if move_x {
            self.pos += x_delta;
        }
        else {
            self.stop_x_vel();
        }
        if move_y {
            self.pos += y_delta;
        }
        else {
            self.stop_y_vel();
        }

        //(move_x, move_y)
        move_x || move_y
    }

    fn get_next_pos(&self, timestep: f32) -> Vector2<f32> {
        self.pos + (self.vel * timestep)
    }

    fn get_self_collider(&self, pos: Vector2<f32>) -> PhysicsCollider {
        self.collider.offset(pos)
    }

    fn stop_x_vel(&mut self) {
        self.vel = Vector2::new(0.0, self.vel.y);
    }
    fn stop_y_vel(&mut self) {
        self.vel = Vector2::new(self.vel.x, 0.0);
    }
}