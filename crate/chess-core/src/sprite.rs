use bevy::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite);
        app.add_systems(Update, (display_sprite, hide_sprite));
    }
}

#[derive(Component, Debug)]
pub enum SpriteType {
    Eat,
    Jiang,
    Lost,
    Win,
    He,
}

#[derive(Component)]
struct SpriteTime(Timer);

#[derive(Resource)]
struct SpriteHolder {
    eat: Handle<Image>,
    jiang: Handle<Image>,
    lost: Handle<Image>,
    win: Handle<Image>,
    he: Handle<Image>,
}

impl SpriteHolder {
    fn get_sprite(&self, sprite_type: &SpriteType) -> Handle<Image> {
        match sprite_type {
            SpriteType::Eat => self.eat.clone(),
            SpriteType::Jiang => self.jiang.clone(),
            SpriteType::Lost => self.lost.clone(),
            SpriteType::Win => self.win.clone(),
            SpriteType::He => self.he.clone(),
        }
    }
}

fn setup_sprite(asset_server: Res<AssetServer>, mut commands: Commands) {
    let eat = asset_server.load::<Image>("/assets/chi.png");
    let jiang = asset_server.load::<Image>("/assets/jiang.png");
    let lost = asset_server.load::<Image>("/assets/jue.png");
    let win = asset_server.load::<Image>("/assets/jue.png");
    let he = asset_server.load::<Image>("/assets/jue.png");

    commands.insert_resource(SpriteHolder {
        eat,
        jiang,
        lost,
        win,
        he,
    });
    commands.spawn(SpriteTime(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn display_sprite(
    mut commands: Commands,
    sprites: Res<SpriteHolder>,
    mut sprite_type: Query<(&SpriteType, Entity)>,
) {
    for (sprite_type, entity) in sprite_type.iter_mut() {
        debug!("display sprite: {:?}", sprite_type);
        commands.spawn((
            Sprite::from_image(sprites.get_sprite(sprite_type)),
            Transform::from_xyz(0., 0., 2.),
            SpriteTime(Timer::from_seconds(1.0, TimerMode::Once)),
        ));
        commands.entity(entity).despawn();
    }
}

fn hide_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteTime)>,
) {
    for (entity, mut sprite_timer) in query.iter_mut() {
        // timers gotta be ticked, to work
        sprite_timer.0.tick(time.delta());

        // if it finished, despawn the bomb
        if sprite_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
