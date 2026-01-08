use bevy::prelude::*;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sound);
        app.add_systems(Update, play_sound);
    }
}

#[derive(Resource)]
struct SoundHolder {
    choose: Handle<AudioSource>,
    eat: Handle<AudioSource>,
    mv: Handle<AudioSource>,
    jiang: Handle<AudioSource>,
    lost: Handle<AudioSource>,
    win: Handle<AudioSource>,
    he: Handle<AudioSource>,
}

impl SoundHolder {
    pub fn get_sound(&self, sound_type: &SoundType) -> Handle<AudioSource> {
        match sound_type {
            SoundType::Choose => self.choose.clone(),
            SoundType::Eat => self.eat.clone(),
            SoundType::Move => self.mv.clone(),
            SoundType::Jiang => self.jiang.clone(),
            SoundType::Lost => self.lost.clone(),
            SoundType::Win => self.win.clone(),
            SoundType::He => self.he.clone(),
        }
    }
}
#[derive(Component, Debug)]
pub enum SoundType {
    Choose,
    Eat,
    Move,
    Jiang,
    Lost,
    Win,
    He,
}

fn setup_sound(asset_server: Res<AssetServer>, mut commands: Commands) {
    let choose = asset_server.load::<AudioSource>("/assets/sound/choose.ogg");
    let eat = asset_server.load::<AudioSource>("/assets/sound/eat.ogg");
    let mv = asset_server.load::<AudioSource>("/assets/sound/move.ogg");
    let jiang = asset_server.load::<AudioSource>("/assets/sound/trouble.ogg");
    let lost = asset_server.load::<AudioSource>("/assets/sound/over.ogg");
    let win = asset_server.load::<AudioSource>("/assets/sound/victory.ogg");
    let he = asset_server.load::<AudioSource>("/assets/sound/victory.ogg");

    commands.insert_resource(SoundHolder {
        choose,
        eat,
        mv,
        jiang,
        lost,
        win,
        he,
    });
}

fn play_sound(
    mut commands: Commands,
    sounds: Res<SoundHolder>,
    mut sound_type: Query<(&SoundType, Entity)>,
) {
    for (sound_type, entity) in sound_type.iter_mut() {
        debug!("play sound: {:?}", sound_type);
        commands.spawn((
            AudioPlayer(sounds.get_sound(sound_type)),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        ));
        commands.entity(entity).despawn();
    }
}
