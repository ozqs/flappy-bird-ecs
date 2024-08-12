use bevy_ecs::prelude::*;
use macroquad::prelude::*;
#[derive(Component, Debug)]
pub struct Gravity;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

/// Rend a sprite centered at the position
#[derive(Component, Debug)]
pub struct TextureRender {
    pub texture: Texture2D,
    pub params: DrawTextureParams,
}
impl TextureRender {
    pub fn new(texture: Texture2D, params: DrawTextureParams) -> Self {
        Self {
            texture,
            params
        }
    }
}

#[derive(Component, Debug)]
pub struct RectangleCollider {
    pub width: f32,
    pub height: f32,
}

impl RectangleCollider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}

#[derive(Component, Debug)]
pub struct Flapable {
    pub velocity_sub: f32,
    pub key: KeyCode
}

impl Flapable {
    pub fn new(velocity_sub: f32, key: KeyCode) -> Self {
        Self {
            velocity_sub,
            key,
        }
    }
}

#[derive(Component, Debug)]
pub struct DieWhenOutOfScreen;
