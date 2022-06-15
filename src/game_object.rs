use cgmath::{ vec2, vec3, Vector2, Vector3 };

use crate::lib::texture::Texture2D;
use crate::lib::sprite_renderer::SpriteRenderer;

pub struct GameObject {
    // object state
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub color: Vector3<f32>,
    pub rotation: f32,
    pub is_solid: bool,
    pub destroyed: bool,
    // render state
    pub sprite: Texture2D,
}

impl GameObject {
    pub fn new_empty() -> Self {
        let game_object = GameObject {
            position: vec2(0.0, 0.0),
            size: vec2(1.0, 1.0),
            velocity: vec2(0.0, 0.0),
            color: vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite: Texture2D::default(),
        };

        game_object
    }

    pub fn new(pos: Vector2<f32>, size: Vector2<f32>, velocity: Vector2<f32>, color: Vector3<f32>, sprite: Texture2D) -> Self {
        let game_object = GameObject {
            position: pos,
            size: size,
            velocity: velocity,
            color: color,
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite: sprite
        };

        game_object
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        renderer.draw_sprite(&self.sprite, self.position, self.size, self.rotation, self.color);
    }
}