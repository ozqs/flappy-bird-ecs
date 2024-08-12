mod components;
mod systems;
use std::f32::consts::PI;

use bevy_ecs::prelude::*;
use components::*;
use macroquad::prelude::*;
use systems::*;
use rand::gen_range;

const BIRD_FLAP: f32 = 100.;
pub const G: f32 = 490.;
pub const MAX_SPEED: f32 = 500.;
const BARRIER_LEFT_SPEED: f32 = 100.;

enum GameMode {
    MainMenu,
    Playing,
    GameOverMenu,
}

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut world = World::new();

    let bird = load_texture("bird.png").await.unwrap();
    let barrier = load_texture("barrier.png").await.unwrap();
    bird.set_filter(FilterMode::Nearest);
    let size = bird.size() * 3.;
    let params = DrawTextureParams {
        dest_size: Some(size),
        ..Default::default()
    };

    let bird = world
        .spawn((
            Position::new(100., 100.),
            TextureRender::new(bird, params),
            RectangleCollider::new(size.x, size.y),
            Flapable::new(BIRD_FLAP, KeyCode::Space),
            DieWhenOutOfScreen,
            Velocity::new(0., 0.),
            Gravity,
        ))
        .id();

    let mut schedule = Schedule::default();

    // Add our system to the schedule
    schedule.add_systems(update_velocity_by_gravity);
    schedule.add_systems(update_position_by_velocity);
    schedule.add_systems(draw_texture_in_position);
    schedule.add_systems(handle_collide);
    schedule.add_systems(handle_flap);
    schedule.add_systems(despawn_out_of_bounds_system);

    // let mut lastx: f32 = 1000.;
    let (_, mut latest) = spawn_barrier_randomly(1000., &mut world, &barrier);

    loop {
        clear_background(SKYBLUE);

        // println!("Loop hard.");

        let mut lastx = world.entity(latest).get::<Position>().unwrap().x;
        while lastx < screen_width() {
            lastx += gen_range(200., 500.);
            (_, latest) = spawn_barrier_randomly(lastx, &mut world, &barrier);
        }

        schedule.run(&mut world);
        if world.get_entity(bird).is_none() {
            break;
        }
        next_frame().await;
    }
}

fn spawn_barrier_randomly(x: f32, world: &mut World, barrier: &Texture2D) -> (Entity, Entity) {
    let gap_radius = screen_height() / gen_range(5., 7.);
    spawn_barrier(
        x,
        gen_range(gap_radius, screen_height() - gap_radius),
        gap_radius,
        world,
        barrier,
    )
}

fn spawn_barrier(
    x: f32,
    gap_pos: f32,
    gap_radius: f32,
    world: &mut World,
    barrier: &Texture2D,
) -> (Entity, Entity) {
    let size = barrier.size();

    (
        world
            .spawn((
                Position::new(x, gap_pos - gap_radius - size.y / 2.),
                TextureRender::new(
                    barrier.clone(),
                    DrawTextureParams {
                        rotation: PI,
                        ..Default::default()
                    },
                ),
                RectangleCollider::new(size.x, size.y),
                Velocity::new(-BARRIER_LEFT_SPEED, 0.),
            ))
            .id(),
        world
            .spawn((
                Position::new(x, gap_pos + gap_radius + size.y / 2.),
                TextureRender::new(barrier.clone(), Default::default()),
                RectangleCollider::new(size.x, size.y),
                Velocity::new(-BARRIER_LEFT_SPEED, 0.),
            ))
            .id(),
    )
}
