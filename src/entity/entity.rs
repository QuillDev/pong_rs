use tetra::Context;
use tetra::graphics::{Rectangle, Texture};
use crate::{Vec2, Vec3};

pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        return Entity::with_velocity(texture, position, Vec2::zero());
    }

    pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        return Entity { texture, position, velocity };
    }

    // properties
    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0)
        )
    }

    // physics

    pub fn update(&mut self) {
        self.position += self.velocity;
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    // rendering
    pub fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, self.position);
    }
}