use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

const LOGO_SIZE: Vec2 = Vec2::new(520.0, 130.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "DVD Bevy Screensaver".to_owned(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_event::<BounceEvent>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_logo)
        .add_system(check_bounce)
        .add_system(move_logo.after(check_bounce))
        .add_system(bounce_handling.after(check_bounce))
        .run();
}

struct BounceEvent;

#[derive(Component)]
struct Logo;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Direction(Vec2);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_logo_dark.png"),
            ..default()
        },
        Logo,
        Speed(100.0),
        Direction(Vec2::ONE),
    ));
}

fn move_logo(
    time: Res<Time>,
    mut logo_query: Query<(&mut Transform, &Direction, &Speed), With<Logo>>,
) {
    let (mut transform, direction, speed) = logo_query.single_mut();
    let velocity = direction.0 * speed.0;

    transform.translation += velocity.extend(0.0) * time.delta_seconds();
}

fn check_bounce(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut logo_query: Query<(&Transform, &mut Direction), With<Logo>>,
    mut bounce_event: EventWriter<BounceEvent>,
) {
    let (camera, global_transform) = camera_query.single();
    let (transform, mut direction) = logo_query.single_mut();

    let offset = (direction.0 * LOGO_SIZE / 2.0).extend(0.0);

    if let Some(position) = camera.world_to_ndc(global_transform, transform.translation + offset) {
        if position.x.abs() > 1.0 {
            direction.0.x *= -1.0;
            bounce_event.send(BounceEvent);
        }

        if position.y.abs() > 1.0 {
            direction.0.y *= -1.0;
            bounce_event.send(BounceEvent);
        }
    }
}

fn bounce_handling(
    mut bounce_event: EventReader<BounceEvent>,
    mut logo_query: Query<&mut Sprite, With<Logo>>,
) {
    if bounce_event.iter().next().is_some() {
        let mut rng = rand::thread_rng();
        let mut sprite = logo_query.single_mut();

        sprite.color = Color::hsl(rng.gen_range(0.0..=360.0), 0.75, 0.5);
    }
}
