use crate::config::*;
use crate::piece::*;
use crate::ChessChatData;
use crate::ChessClientEventIds;
use crate::ChessEvent;
use crate::SoundType;
use crate::SpriteType;
use bevy::prelude::*;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_action);
    }
}

fn process_action(
    mut commands: Commands,
    step_query: Query<(Entity, &Action)>,
    mut piece_query: Query<(Entity, &mut Transform, &Piece, &mut Pos, &PieceColor)>,
    mut game_state: ResMut<GameState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_writer: EventWriter<ChessEvent>,
    mut client_event_ids: ResMut<ChessClientEventIds>,
    opt: Res<Opt>,
) {
    for (step_entity, action) in step_query.iter() {
        match action {
            Action::JoinLobby(player) => {
                game_state.two_player = Some(player.clone());
                game_state.two_color = PieceColor::UnSet;

                send_event_to_html(
                    ChessEvent::connected(player.user_id, player.nickname.clone(), player.avatar),
                    &mut event_writer,
                    &mut client_event_ids,
                );
            }
            Action::LeaveLobby => {
                game_state.reset(false);
                next_app_state.set(AppState::Init);
                send_event_to_html(
                    ChessEvent::lobby(),
                    &mut event_writer,
                    &mut client_event_ids,
                );
            }
            Action::Ready => {
                game_state.two_ready = true;
            }
            Action::MovePiece(from, to, is_eat, is_jianged, is_end) => {
                //处理动画、声音
                if *is_eat {
                    let piece = piece_query
                        .iter()
                        .find_map(|(entity, _, _, &pos, &color)| {
                            if pos == *to {
                                Some((entity, color))
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    commands.entity(piece.0).insert(Action::RemovePiece);

                    if *is_end {
                        commands.spawn(Action::Winner(piece.1));
                    } else {
                        if *is_jianged {
                            commands.spawn(SoundType::Jiang);
                            commands.spawn(SpriteType::Jiang);
                        } else {
                            commands.spawn(SoundType::Eat);
                            commands.spawn(SpriteType::Eat);
                        }
                    }
                } else {
                    if *is_jianged {
                        commands.spawn(SoundType::Jiang);
                        commands.spawn(SpriteType::Jiang);
                    } else {
                        commands.spawn(SoundType::Move);
                    }
                }
                //将棋子移到位置to
                match piece_query
                    .iter_mut()
                    .find(|(_, _, _, pos, _)| **pos == *from)
                {
                    None => {}
                    Some((_, mut trans, _, mut pos, &color)) => {
                        *pos = *to;
                        let to_xyz: Vec3 = to.to_vec3() * opt.cell_width;
                        trans.translation.x = to_xyz.x;
                        trans.translation.y = to_xyz.y;

                        if *is_end {
                            game_state.is_turn = false;
                        } else {
                            if color == game_state.one_color {
                                game_state.is_turn = false;
                                send_event_to_html(
                                    ChessEvent::turn(game_state.two_color),
                                    &mut event_writer,
                                    &mut client_event_ids,
                                );
                            } else {
                                game_state.is_turn = true;
                                send_event_to_html(
                                    ChessEvent::turn(game_state.one_color),
                                    &mut event_writer,
                                    &mut client_event_ids,
                                );
                            }
                        }
                    }
                }
            }
            Action::RemovePiece => {}
            Action::Winner(color) => {
                match color {
                    PieceColor::UnSet => {
                        commands.spawn(SoundType::He);
                        commands.spawn(SpriteType::He);
                    }
                    _ => {
                        if *color == game_state.one_color {
                            commands.spawn(SoundType::Win);
                            commands.spawn(SpriteType::Win);
                        } else {
                            commands.spawn(SoundType::Lost);
                            commands.spawn(SpriteType::Lost);
                        }
                    }
                };
                game_state.reset(true);
                next_app_state.set(AppState::Joined);
                send_event_to_html(
                    ChessEvent::joined(),
                    &mut event_writer,
                    &mut client_event_ids,
                );
            }
            Action::Chat(data) => {
                send_chat_event(
                    data.clone(),
                    &game_state,
                    &mut event_writer,
                    &mut client_event_ids,
                );
            }
        }
        commands.entity(step_entity).despawn();
    }
}

pub(crate) fn send_chat_event(
    chat_data: ChessChatData,
    game_state: &ResMut<GameState>,
    event_writer: &mut EventWriter<ChessEvent>,
    client_event_ids: &mut ResMut<ChessClientEventIds>,
) {
    let two_player = game_state.two_player.clone();
    if let Some(two_player) = two_player {
        let event_id = event_writer.send(ChessEvent::chat(
            chat_data,
            two_player.user_id,
            two_player.nickname,
            two_player.avatar,
        ));
        client_event_ids.push(event_id);
    }
}

pub(crate) fn send_event_to_html(
    event: ChessEvent,
    event_writer: &mut EventWriter<ChessEvent>,
    client_event_ids: &mut ResMut<ChessClientEventIds>,
) {
    let event_id = event_writer.send(event);
    client_event_ids.push(event_id);
}
