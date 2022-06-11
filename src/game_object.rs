use cgmath::{ vec3, Vector2, Vector3, Matrix4, Rad };
use cgmath::prelude::*;

use crate::lib::texture::Texture2D;
use crate::lib::sprite_renderer::SpriteRenderer;

struct GameObject {
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
    pub fn new(pos: Vector2<f32>, size: Vector2<f32>, vel: Vector2<f32>, color: Vector3<f32>, rotation: f32, sprite: Texture2D) -> Self {
        let game_object = GameObject {
            position: pos,
            size: size,
            velocity: vel,
            color: color,
            rotation: rotation,
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