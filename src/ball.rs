use cgmath::{vec2, vec3, Vector2};

use crate::game_object::GameObject;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::lib::texture::Texture2D;

#[derive(Copy, Clone)]
pub struct Ball {
    pub game_object: GameObject,
    pub radius: f32,
    pub stuck: bool,
    pub sticky: bool,
    pub passthrough: bool,
}

impl Ball {
    pub fn new_empty() -> Self {
        Ball {
            game_object: GameObject::new_empty(),
            radius: 12.5,
            stuck: true,
            sticky: false,
            passthrough: false,
        }
    }

    pub fn new(pos: Vector2<f32>, radius: f32, velocity: Vector2<f32>, sprite: Texture2D) -> Self {
        Ball {
            game_object: GameObject::new(
                pos,
                vec2(radius * 2.0, radius * 2.0),
                velocity,
                vec3(1.0, 1.0, 1.0),
                sprite
            ),
            radius: radius,
            stuck: true,
            sticky: false,
            passthrough: false,
        }
    }

    pub fn move_ball(&mut self, dt: f32, window_width: u32) -> Vector2<f32> {
        // if not stuck to player board
        if !self.stuck {
            // move the ball
            self.game_object.position += self.game_object.velocity * dt;
            // check if outside window bounds; if so, reverse velocity and restore at correct position
            if self.game_object.position.x <= 0.0 {
                self.game_object.velocity.x = -self.game_object.velocity.x;
                self.game_object.position.x = 0.0;
            } else if self.game_object.position.x + self.game_object.size.x >= window_width as f32 {
                self.game_object.velocity.x = -self.game_object.velocity.x;
                self.game_object.position.x = window_width as f32 - self.game_object.size.x;
            }

            if self.game_object.position.y <= 0.0 {
                self.game_object.velocity.y = -self.game_object.velocity.y;
                self.game_object.position.y = 0.0;
            }
        }

        self.game_object.position
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        renderer.draw_sprite(&self.game_object.sprite, self.game_object.position, self.game_object.size, self.game_object.rotation, self.game_object.color);
    }

    pub fn reset(&mut self, position: Vector2<f32>, velocity: Vector2<f32>) {
        self.game_object.position = position;
        self.game_object.velocity = velocity;
        self.game_object.color = vec3(1.0, 1.0, 1.0);
        self.stuck = true;
        self.sticky = false;
        self.passthrough = false;
    }
}