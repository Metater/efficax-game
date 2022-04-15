use cgmath::{Vector2, Zero, Bounded};

#[derive(Copy, Clone)]
pub struct PhysicsCollider {
    min: Vector2<f32>,
    max: Vector2<f32>
}

impl PhysicsCollider {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        PhysicsCollider {
            min,
            max
        }
    }

    pub fn none() -> Self {
        PhysicsCollider::new(Vector2::zero(), Vector2::zero())
    }

    pub fn all() -> Self {
        PhysicsCollider::new(Vector2::min_value(), Vector2::max_value())
    }

    pub fn offset(&self, offset: Vector2<f32>) -> Self {
        PhysicsCollider::new(
            self.min + offset,
            self.max + offset
        )
    }

    pub fn intersects(&self, other: &PhysicsCollider) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false
        }
        return true
    }

    //pub fn intersects_line(&self, other: )

    //pub fn contains_point
}