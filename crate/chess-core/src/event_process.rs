use bevy::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::*;
pub struct EventPlugin {
    pub duplex: BevyEventDuplex<ChessEvent>,
}

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChessClientEventIds>()
            .add_duplex_leptos_event(self.duplex.to_owned())
            .add_systems(Update, read_event_from_leptos);
    }
}

fn read_event_from_leptos(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut client_event_ids: ResMut<ChessClientEventIds>,
    mut event_reader: EventReader<ChessEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut opt: ResMut<Opt>,
) {
    for (event, id) in event_reader.read_with_id() {
        if client_event_ids.contains(&id) {
            client_event_ids.retain(|e| *e != id);
            continue;
        }
        match &event.kind {
            ChessEventType::Ready => {
                game_state.one_ready = true;

                commands.spawn(Packet(Action::Ready));
            }
            ChessEventType::Chat(data) => {
                match data {
                    ChessChatData::Lost(_) => {
                        commands.spawn(Action::Winner(game_state.two_color));
                        commands.spawn(Packet(Action::Winner(game_state.two_color)));
                    }
                    ChessChatData::Reply(_, is_agree) => {
                        //now only for request he
                        if *is_agree {
                            commands.spawn(Action::Winner(PieceColor::UnSet));
                            commands.spawn(Packet(Action::Winner(PieceColor::UnSet)));
                        }
                    }
                    _ => {}
                };
                commands.spawn(Packet(Action::Chat(data.clone())));
            }
            ChessEventType::QuickMatch(server_url) => {
                opt.server_url = server_url.clone();
                next_app_state.set(AppState::Init);
            }
            _ => {}
        }
    }
}
