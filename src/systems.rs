use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::*;

pub fn update_velocity_by_gravity(mut query: Query<&mut Velocity, With<Gravity>>) {
    for mut velocity in &mut query {
        velocity.y += G * get_frame_time();
        velocity.y = clamp(velocity.y, -MAX_SPEED, MAX_SPEED);
    }
}

pub fn update_position_by_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x * get_frame_time();
        position.y += velocity.y * get_frame_time();
        if velocity.y.abs() > 0.01 {
            position.y = position.y.max(0.);
        }
    }
}

pub fn draw_texture_in_position(query: Query<(&TextureRender, &Position)>) {
    for (texture, position) in &query {
        draw_texture_ex(
            &texture.texture,
            position.x - texture.params.dest_size.unwrap_or(texture.texture.size()).x / 2.,
            position.y - texture.params.dest_size.unwrap_or(texture.texture.size()).y / 2.,
            WHITE,
            texture.params.clone(),
        )
    }
}

pub fn handle_collide(
    mut commands: Commands,
    query: Query<(Entity, &RectangleCollider, &Position)>,
) {
    for (_, collider, position) in &query {
        let rect = Rect::new(
            position.x - collider.width / 2.,
            position.y - collider.height / 2.,
            collider.width,
            collider.height,
        );
        for (_, collider, position) in &query {
            let rect2 = Rect::new(
                position.x - collider.width / 2.,
                position.y - collider.height / 2.,
                collider.width,
                collider.height,
            );
            if rect == rect2 {
                continue;
            }
            if rect.overlaps(&rect2) {
                for (entity, _, _) in &query {
                    commands.entity(entity).despawn();
                }
                return;
            }
        }
    }
}

pub fn handle_flap(mut query: Query<(&mut Velocity, &Flapable)>) {
    for (mut velocity, flap) in &mut query {
        if is_key_down(flap.key) {
            velocity.y -= flap.velocity_sub;
            velocity.y = clamp(velocity.y, -MAX_SPEED, MAX_SPEED);
        }
    }
}

pub fn despawn_out_of_bounds_system(
    mut commands: Commands,
    query: Query<(Entity, &Position, &RectangleCollider), With<DieWhenOutOfScreen>>,
) {
    for (entity, transform, collider) in query.iter() {
        let width = collider.width / 2.;
        let height = collider.height / 2.;
        // 检查实体是否在屏幕边缘之外
        if transform.x < 0.0 - width
            // || transform.x > screen_width() + width
            // || transform.y < 0.0 - height
            || transform.y > screen_height() + height
        {
            // 删除实体
            commands.entity(entity).despawn();
        }
    }
}
