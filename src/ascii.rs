use crate::GLOBAL_SIZE;
use bevy::prelude::*;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = server.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    sheet: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);

    sprite.custom_size = Some(Vec2::splat(GLOBAL_SIZE));
    sprite.color = color;

    let bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: sheet.0.clone(),
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(bundle).id()
}
