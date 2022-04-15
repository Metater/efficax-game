use cgmath::{Vector2, Zero};

use super::{physics::collider::PhysicsCollider, zone::MetaitusZone};

pub struct MetaitusEntity {
    id: u32,
    pos: Vector2<f32>,
    current_cell: u32,

    bounds: PhysicsCollider,

    has_vel_epsilon: bool,
    vel_epsilon: f32,

    has_collider: bool,
    collider: PhysicsCollider,

    has_repulsion_radius: bool,
    repulsion_radius: f32,

    has_drag: bool,
    has_linear_drag: bool,
    drag: f32,

    vel: Vector2<f32>,
    cell_indicies: Vec<u32>
}

impl MetaitusEntity {
    pub fn new(id: u32, pos: Vector2<f32>, current_cell: u32) -> Self {
        MetaitusEntity {
            id,
            pos,
            current_cell,

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

            vel: Vector2::zero(),
            cell_indicies: Vec::with_capacity(9)
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl MetaitusEntity {
    pub fn tick(&mut self, zone: &mut MetaitusZone, timestep: f32) {
        self.cell_indicies.clear();
        zone.get_cell_and_surrounding(self.pos, &mut self.cell_indicies);
        
        if self.has_vel_epsilon {
            self.apply_vel_epsilon();
        }

        if !self.vel.is_zero() {

        }
        else {
            // cannot move, velocity is zero
        }

        // check repulsion radius

        // use lerp to step closer to colliders if needed
    }

    fn apply_vel_epsilon(&mut self) {
        if self.vel.x != 0.0 && self.vel.x.abs() < self.vel_epsilon {
            self.vel = Vector2::new(0.0, self.vel.y);
        }
        if self.vel.y != 0.0 && self.vel.y.abs() < self.vel_epsilon {
            self.vel = Vector2::new(self.vel.x, 0.0);
        }
    }

    fn update_pos(&mut self, zone: &mut MetaitusZone, timestep: f32) -> bool {
        if !self.has_collider {
            self.pos = self.get_next_pos(timestep);
            return true
        }

        let nominal_pos = self.get_next_pos(timestep);
        let x_delta = Vector2::new(self.vel.x * timestep, 0.0);
        let y_delta = Vector2::new(0.0, self.vel.y * timestep);

        let can_move_x = !self.vel.x.is_zero();
        let can_move_y = !self.vel.y.is_zero();

        // check bounds

        for &cell_index in &self.cell_indicies {
            let cell = zone

        }

        return can_move_x || can_move_y;
    }

    fn apply_drag(&mut self, timestep: f32) {
        if self.has_drag {
            return
        }
        if self.has_linear_drag {
            self.vel *= 1.0 - (timestep * self.drag);
        }
        else {
            let x_drag = self.drag * ((self.vel.x * self.vel.x) / 2.0);
            let y_drag = self.drag * ((self.vel.y * self.vel.y) / 2.0);
            self.vel.x *= 1.0 - (timestep * x_drag);
            self.vel.y *= 1.0 - (timestep * y_drag);
        }
    }

    fn get_next_pos(&self, timestep: f32) -> Vector2<f32> {
        self.pos + (self.vel * timestep)
    }

    fn get_self_collider(&self) -> PhysicsCollider {
        self.collider.offset(self.pos)
    }
}