use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_matchbox::prelude::*;
use leptos_bevy_canvas::prelude::*;

mod action_process;
mod board;
mod chat;
mod config;
mod event;
mod event_process;
mod game;
mod piece;
mod sound;
mod sprite;

pub use action_process::*;
pub use board::*;
pub use chat::*;
pub use config::*;
pub use event::*;
pub use event_process::*;
pub use game::*;
pub use piece::*;
pub use sound::*;
pub use sprite::*;

pub fn init_chess(opt: Opt, duplex: BevyEventDuplex<ChessEvent>) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "中国象棋".into(),
                    resolution: (540., 600.).into(),
                    canvas: Some("#bevy_canvas".into()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        MeshPickingPlugin,
        BoardPlugin,
        GamePlugin,
        ActionPlugin,
        EventPlugin { duplex },
        SoundPlugin,
        SpritePlugin,
    ))
    .init_state::<AppState>()
    .insert_resource(ClearColor(Color::from(opt.clear_color())))
    .insert_resource(opt)
    .add_systems(OnEnter(AppState::Init), start_matchbox_socket)
    .add_systems(Update, lobby_system)
    .add_systems(Update, (send_packet_system, receive_packet_system));
    app
}

fn start_matchbox_socket(
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    opt: Res<Opt>,
) {
    commands.insert_resource(MatchboxSocket::new_unreliable(opt.server_url.clone()));
    next_app_state.set(AppState::Lobby);
}

fn lobby_system(
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut socket: ResMut<MatchboxSocket>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    opt: Res<Opt>,
) {
    // regularly call update_peers to update the list of connected peers
    let Ok(peer_changes) = socket.try_update_peers() else {
        warn!("socket dropped");
        return;
    };
    for (peer, new_state) in peer_changes {
        // you can also handle the specific dis(connections) as they occur:
        match new_state {
            PeerState::Connected => {
                let player = Player {
                    peer_id: socket.id().unwrap(),
                    avatar: opt.avatar,
                    nickname: opt.nickname.clone(),
                    user_id: opt.user_id,
                };
                game_state.one_player = Some(player.clone());
                game_state.one_color = PieceColor::UnSet;

                commands.spawn(Packet(Action::JoinLobby(player.clone())));
                info!("client {} connected,{:?}", peer, player);
            }
            PeerState::Disconnected => {
                commands.spawn(Action::LeaveLobby);
                info!("client {peer} disconnected");
            }
        }
    }

    let connected_peers = socket.connected_peers().count();
    if connected_peers < opt.play_num - 1 || !matches!(app_state.get(), AppState::Lobby) {
        return;
    }
    info!("All clients have joined, going in-game");

    // transition to in-game state
    next_app_state.set(AppState::Joined);
}

fn send_packet_system(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    packets: Query<(Entity, &Packet)>,
) {
    for (e, packet) in packets.iter() {
        debug!("send:{:?}", packet);
        let packet = bincode::serialize(packet).unwrap().into_boxed_slice();

        let peer_id = socket.connected_peers().next();
        if peer_id.is_some() {
            socket.channel_mut(0).send(packet.clone(), peer_id.unwrap());
            commands.entity(e).despawn();
        }
    }
}

fn receive_packet_system(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
    for (_, packet) in socket.channel_mut(0).receive() {
        let packet: Packet = bincode::deserialize(&packet[..]).unwrap();
        debug!("recevied:{:?}", packet);

        commands.spawn(packet.0);
    }
}
