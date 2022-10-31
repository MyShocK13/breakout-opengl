use std::sync::MutexGuard;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use cgmath::{vec2, vec3};

use crate::game_object::GameObject;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::resource_manager::ResourceManager;

pub struct GameLevel {
    // level state
    pub bricks: Vec<GameObject>,
}

impl GameLevel {
    pub fn new() -> Self {
        GameLevel { bricks: Vec::new() }
    }

    pub fn load(&mut self, resources: MutexGuard<ResourceManager<'static>>, file_path: &str, level_width: u32, level_height: u32) {
        // clear old data
        self.bricks.clear();
        // load from file
        let mut tile_data: Vec<Vec<u32>> = Vec::new();

        let level_file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", file_path));

        let reader = BufReader::new(level_file);
        for line in reader.lines() {
            let mut row: Vec<u32> = Vec::new();
            for number in line.unwrap().split_whitespace() {
                row.push(number.parse::<u32>().unwrap());
            }
            tile_data.push(row);
        }

        if tile_data.len() > 0 {
            // calculate dimensions
            let height = tile_data.len();
            let width = tile_data[0].len();
            let unit_width = level_width / width as u32;
            let unit_height = level_height / height as u32;

            // initialize level tiles based on tileData
            for (y, row) in tile_data.iter().enumerate() {
                for (x, brick) in row.iter().enumerate() {
                    if *brick == 0 {
                        continue;
                    }

                    let pos = vec2((unit_width * x as u32) as f32, (unit_height * y as u32) as f32);
                    let size = vec2(unit_width as f32, unit_height as f32);
                    let color = match *brick {
                        1 => vec3(0.8, 0.8, 0.7),
                        2 => vec3(0.2, 0.6, 1.0),
                        3 => vec3(0.0, 0.7, 0.0),
                        4 => vec3(0.8, 0.8, 0.4),
                        5 => vec3(1.0, 0.5, 0.0),
                        _ => vec3(1.0, 1.0, 1.0)
                    };
                    // check block type from level data (2D level array)
                    let sprite = match *brick {
                        1 => resources.get_texture("block_solid"),
                        _ => resources.get_texture("block"),
                    };

                    let mut obj = GameObject::new(pos, size, vec2(0.0, 0.0), color, sprite);
                    obj.is_solid = *brick == 1;
                    self.bricks.push(obj);
                }
            }
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        for brick in self.bricks.iter() {
            if !brick.destroyed {
                brick.draw(renderer);
            }
        }
    }
}