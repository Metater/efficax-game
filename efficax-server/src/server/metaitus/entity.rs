use std::ops::Deref;

use cgmath::{Vector2, Zero};

use super::{physics::collider::PhysicsCollider};

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

    has_drag: bool,
    has_linear_drag: bool,
    drag: f32,

    vel: Vector2<f32>
}

impl MetaitusEntity {
    pub fn new(id: u32, pos: Vector2<f32>, current_cell_index: u32) -> Self {
        MetaitusEntity {
            id,
            pos,
            current_cell_index,

            bounds: PhysicsCollider::all(),

            has_vel_epsilon: true,
            vel_epsilon: 1.0 / 16.0,

            has_collider: false,
            collider: PhysicsCollider::none(),

            has_drag: false,
            has_linear_drag: false,
            drag: 0.0,

            has_repulsion_radius: false,
            repulsion_radius: 0.0,

            vel: Vector2::zero()
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

    pub fn with_repulsion_radius(&mut self, has_repulsion_radius: bool, repulsion_radius: f32) -> &mut Self {
        self.has_repulsion_radius = has_repulsion_radius;
        self.repulsion_radius = repulsion_radius;
        self
    }
}

impl MetaitusEntity {
    pub fn tick(&mut self, timestep: f32, near_statics: &Vec<PhysicsCollider>, repulsable_entities: &Vec<(Vector2<f32>, f32)>) -> bool {
        let mut moved_xy = false;
        
        if self.has_vel_epsilon {
            self.apply_vel_epsilon();
        }

        if !self.vel.is_zero() {
            moved_xy = self.update_pos(timestep, near_statics);
            if moved_xy && self.has_drag {
                self.apply_drag(timestep);
            }
        }
        else {
            // cannot move, velocity is zero
        }

        // check repulsion radius
        for (entity_pos, entity_repulsion_radius) in repulsable_entities {
            let sqr_distance = entity_pos.deref()
        }

        moved_xy
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