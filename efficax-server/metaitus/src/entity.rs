use cgmath::{Vector2, Zero};

use crate::zone::MetaitusZone;

use super::collider::MetaitusCollider;

#[derive(Debug)]
pub struct MetaitusEntity {
    pub id: u32,
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub current_cell_index: u32,

    pub has_bounds: bool,
    pub bounds: MetaitusCollider,

    pub has_vel_epsilon: bool,
    pub vel_epsilon: f32,

    pub has_collider: bool,
    pub collider: MetaitusCollider,

    pub has_repulsion_radius: bool,
    pub repulsion_radius: f32,
    pub max_repulsion_mag: f32,
    pub repulsion: f32,

    pub has_drag: bool,
    pub drag: f32,

    pub moved_xy: bool,
    pub moved_on_tick: u32
}

impl MetaitusEntity {
    pub fn new(id: u32, pos: Vector2<f32>) -> Self {
        MetaitusEntity {
            id,
            pos,
            vel: Vector2::zero(),
            current_cell_index: MetaitusZone::get_index_at_pos(pos),

            has_bounds: false,
            bounds: MetaitusCollider::all(),

            has_vel_epsilon: true,
            vel_epsilon: 1.0 / 12.0,

            has_collider: false,
            collider: MetaitusCollider::none(),

            has_drag: false,
            drag: 0.0,

            has_repulsion_radius: false,
            repulsion_radius: 0.0,
            max_repulsion_mag: 0.0,
            repulsion: 0.0,

            moved_xy: false,
            moved_on_tick: 0
        }
    }

    pub fn with_bounds(&mut self, has_bounds: bool, bounds: MetaitusCollider) -> &mut Self {
        self.has_bounds = has_bounds;
        self.bounds = bounds;
        self
    }

    pub fn with_vel_epsilon(&mut self, has_vel_epsilon: bool, vel_epsilon: f32) -> &mut Self {
        self.has_vel_epsilon = has_vel_epsilon;
        self.vel_epsilon = vel_epsilon;
        self
    }

    pub fn with_collider(&mut self, has_collider: bool, collider: MetaitusCollider) -> &mut Self {
        self.has_collider = has_collider;
        self.collider = collider;
        self
    }

    pub fn with_drag(&mut self, has_drag: bool, drag: f32) -> &mut Self {
        self.has_drag = has_drag;
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

    pub fn tick(&mut self, tick_id: u32, delta_time: f32, near_statics: &Vec<MetaitusCollider>) -> bool {
        self.moved_xy = false;
        
        if self.has_vel_epsilon {
            self.apply_vel_epsilon();
        }

        if !self.vel.is_zero() {
            self.moved_xy = self.update_pos(delta_time, near_statics);
            if self.moved_xy {
                self.moved_on_tick = tick_id;
                if self.has_drag {
                    self.apply_drag(delta_time);
                }
            }
        }
        
        self.moved_xy
    }

    fn apply_vel_epsilon(&mut self) {
        if self.vel.x != 0.0 && self.vel.x.abs() < self.vel_epsilon {
            self.vel.x = 0.0;
        }
        if self.vel.y != 0.0 && self.vel.y.abs() < self.vel_epsilon {
            self.vel.y = 0.0;
        }
    }

    fn apply_drag(&mut self, timestep: f32) {
        self.vel *= 1.0 - (self.drag * timestep);
    }

    fn update_pos(&mut self, timestep: f32, near_statics: &Vec<MetaitusCollider>) -> bool {
        if !self.has_collider {
            self.pos = self.get_next_pos(timestep);
            //return (!self.vel.x.is_zero(), !self.vel.y.is_zero())
            return true
        }

        let nominal_pos = self.get_next_pos(timestep);
        let x_delta = Vector2::new(self.vel.x * timestep, 0.0);
        let y_delta = Vector2::new(0.0, self.vel.y * timestep);

        let xy_collider = self.collider.copy_with_offset(nominal_pos);
        let x_collider = self.collider.copy_with_offset(self.pos + x_delta);
        let y_collider = self.collider.copy_with_offset(self.pos + y_delta);

        let mut move_x = !self.vel.x.is_zero();
        let mut move_y = !self.vel.y.is_zero();

        if self.has_bounds && !xy_collider.intersects(&self.bounds) {
            if move_x && !x_collider.intersects(&self.bounds) {
                move_x = false;
            }
            if move_y && !y_collider.intersects(&self.bounds) {
                move_y = false;
            }
        }

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
            self.vel.x = 0.0;
        }
        if move_y {
            self.pos += y_delta;
        }
        else {
            self.vel.y = 0.0;
        }

        move_x || move_y
    }

    fn get_next_pos(&self, timestep: f32) -> Vector2<f32> {
        self.pos + (self.vel * timestep)
    }
}