use cgmath::{ vec2, vec3, Vector2, Vector3 };

use crate::game_object::GameObject;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::lib::texture::Texture2D;

pub struct Ball {
    pub game_object: GameObject,

    pub radius: f32,
    pub stuck: bool,
}

impl Ball {
    pub fn new_empty() -> Self {
        let ball = Ball {
            game_object: GameObject::new_empty(),
            radius: 12.5,
            stuck: true,
        };
        
        ball
    }

    pub fn new(pos: Vector2<f32>, radius: f32, velocity: Vector2<f32>, sprite: Texture2D) -> Self {
        let ball = Ball {
            game_object: GameObject::new(
                pos,
                vec2(radius * 2.0, radius * 2.0),
                vec3(1.0, 1.0, 1.0),
                sprite
            ),
            radius: radius,
            stuck: true,
        };

        ball
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        renderer.draw_sprite(&self.game_object.sprite, self.game_object.position, self.game_object.size, self.game_object.rotation, self.game_object.color);
    }
}