use cgmath::{Vector2, Zero, Bounded};

#[derive(Copy, Clone, Debug)]
pub struct PhysicsCollider {
    pub id: u64,
    pub min: Vector2<f32>,
    pub max: Vector2<f32>
}

impl PhysicsCollider {
    pub fn new(id: u64, min: Vector2<f32>, max: Vector2<f32>) -> Self {
        PhysicsCollider {
            id,
            min,
            max
        }
    }

    pub fn none() -> Self {
        PhysicsCollider::new(0, Vector2::zero(), Vector2::zero())
    }

    pub fn all() -> Self {
        PhysicsCollider::new(0, Vector2::min_value(), Vector2::max_value())
    }

    pub fn is_static(&self) -> bool {
        !self.id.is_zero()
    }

    pub fn copy_with_id(&self, id: u64) -> Self {
        PhysicsCollider::new(id, self.min, self.max)
    }

    pub fn offset(&self, offset: Vector2<f32>) -> Self {
        PhysicsCollider::new(
            self.id,
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