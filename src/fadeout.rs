use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ascii::AsciiSheet, GameState};

pub struct FadeoutPlugin;

#[derive(Component)]
struct ScreenFade {
    pub alpha: f32,
    pub sent: bool,
    next_state: Option<GameState>,
    timer: Timer,
}

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fadeout);
    }
}

fn fadeout(
    mut commands: Commands,
    mut fade_q: Query<(Entity, &mut ScreenFade, &mut TextureAtlasSprite)>,
    //mut state: ResMut<GameState>,
    time: Res<Time>,
) {
    for (entity, mut fade, mut sprite) in fade_q.iter_mut() {
        fade.timer.tick(time.delta());

        if fade.timer.percent() < 0.5 {
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fade.timer.percent() > 0.5 && !fade.sent {
            if let Some(next_state) = fade.next_state {
                commands.insert_resource(NextState(next_state));
            }
            fade.sent = true;
        }

        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_fadeout(
    commands: &mut Commands,
    next_state: Option<GameState>,
    ascii: &Res<AsciiSheet>,
) {
    println!("create_fadeout() called");
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(100000.0));

    let bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 999.0),
            ..Default::default()
        },
        ..Default::default()
    };
    let screenfade = ScreenFade {
        alpha: 0.0,
        sent: false,
        next_state,
        timer: Timer::from_seconds(1.0, false),
    };

    commands
        .spawn_bundle(bundle)
        .insert(screenfade)
        .insert(Name::new("Fadeout"));
}
