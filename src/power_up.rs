use cgmath::{vec2, vec3, Vector2, Vector3};

use crate::game_object::GameObject;
use crate::lib::texture::Texture2D;

const SIZE: Vector2<f32> = vec2(60.0, 20.0);
const VELOCITY: Vector2<f32> = vec2(0.0, 150.0);

pub struct PowerUp {
    pub game_object: GameObject,
    // powerup state
    pub pw_type: String,
    pub duration: f32,
    pub activated: bool,
}

impl PowerUp {
    pub fn new_empty() -> Self {
        PowerUp {
            game_object: GameObject::new_empty(),
            pw_type: String::default(),
            duration: 0.0,
            activated: false
        }
    }

    pub fn new(pos: Vector2<f32>, color: Vector3<f32>, sprite: Texture2D, pw_type: &str, duration: f32, activated: bool) -> Self {
        PowerUp {
            game_object: GameObject::new(
                pos, 
                SIZE, 
                VELOCITY, 
                color, 
                sprite
            ),
            pw_type: pw_type.to_string(),
            duration: duration,
            activated: activated            
        }
    }
}