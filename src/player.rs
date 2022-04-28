use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::fadeout::create_fadeout;
use crate::simple_tilemap::{EncounterSpawner, SimpleTileCollider};
use crate::{GameState, GLOBAL_SIZE};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::*;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    active: bool,
    just_moved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct EncounterFixedUpdate;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // let mut encounter_stage = SystemStage::parallel();
        // encounter_stage.add_system(encounter_check.run_in_state(GameState::Overworld));

        app.add_system(
            player_move
                .run_in_state(GameState::Overworld)
                .label("player_move"),
        )
        .add_system(
            camera_follow
                .run_in_state(GameState::Overworld)
                .after("player_move"),
        )
        .add_system(
            encounter_check
                .run_in_state(GameState::Overworld)
                .after("player_move"),
        )
        // .add_stage_before(
        //     CoreStage::Update,
        //     EncounterFixedUpdate,
        //     FixedTimestepStage::new(Duration::from_secs_f32(1.0)).with_stage(encounter_stage),
        // )
        .add_enter_system(GameState::Overworld, show_player)
        .add_exit_system(GameState::Overworld, hide_player)
        .add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, sheet: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &sheet,
        2,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * GLOBAL_SIZE, -2.0 * GLOBAL_SIZE, 900.0),
    );
    let bg = spawn_ascii_sprite(
        &mut commands,
        &sheet,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );
    commands.entity(bg).insert(Name::new("Background"));

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            active: true,
            just_moved: false,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true),
        });

    commands.entity(player).push_children(&[bg]);
}

fn player_move(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    wall_q: Query<&Transform, (With<SimpleTileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_q.single_mut();
    let speed = player.speed * time.delta_seconds() * GLOBAL_SIZE;
    player.just_moved = false;

    if !player.active {
        return;
    }

    let mut y_delta = 0.;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        y_delta += speed;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        y_delta -= speed;
    }
    let mut x_delta = 0.;
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        x_delta -= speed;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        x_delta += speed;
    }

    let target = transform.translation + Vec3::new(x_delta, 0., 0.);
    if !wall_q
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if x_delta != 0. {
            player.just_moved = true;
        }
        transform.translation = target;
    }
    let target = transform.translation + Vec3::new(0., y_delta, 0.);
    if !wall_q
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if y_delta != 0. {
            player.just_moved = true;
        }
        transform.translation = target;
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let coll = collide(
        target_player_pos,
        Vec2::splat(GLOBAL_SIZE * 0.9),
        wall_translation,
        Vec2::splat(GLOBAL_SIZE),
    );
    coll.is_some()
}

fn encounter_check(
    mut commands: Commands,
    mut player_q: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_q: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    ascii: Res<AsciiSheet>,
    time: Res<Time>,
) {
    let (mut player, mut encounter_tracker, player_transform) = player_q.single_mut();
    let player_translation = player_transform.translation;
    if player.just_moved
        && encounter_q
            .iter()
            .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.just_finished() {
            println!("changing to combat!");
            player.active = false;
            create_fadeout(&mut commands, Some(GameState::Combat), &ascii);
            //commands.insert_resource(NextState(GameState::Combat));
        }
    }
}

fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player = player_q.single();
    let mut camera = camera_q.single_mut();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

fn show_player(
    mut player_q: Query<(&mut Player, &mut Visibility)>,
    children_q: Query<&Children, With<Player>>,
    mut child_visible_q: Query<&mut Visibility, Without<Player>>,
) {
    let (mut player, mut player_vis) = player_q.single_mut();
    player.active = true;
    player_vis.is_visible = true;

    if let Ok(children) = children_q.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visible_q.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn hide_player(
    mut player_q: Query<&mut Visibility, With<Player>>,
    children_q: Query<&Children, With<Player>>,
    mut child_visible_q: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_q.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_q.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visible_q.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}
