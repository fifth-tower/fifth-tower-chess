use bevy::prelude::*;

use crate::config::*;
use crate::piece::*;
use crate::send_event_to_html;
use crate::ChessClientEventIds;
use crate::ChessEvent;
use crate::SoundType;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Update, check_ready.run_if(in_state(AppState::Joined)))
            .add_systems(
                Update,
                create_piece_system.run_if(in_state(AppState::Ready)),
            );
    }
}

pub fn on_click_board(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    pos_query: Query<(&Transform, &Pos, &Mesh2d)>,
    piece_query: Query<(Entity, &Transform, &Piece, &Pos, &PieceColor)>,
    mut game_state: ResMut<GameState>,
) {
    if !game_state.is_turn {
        return;
    }
    let to_result = piece_query.get(trigger.entity());

    let from_piece = game_state.current_pos.is_some()
        && piece_query.iter().any(|(_, _, _, pos, color)| {
            *pos == game_state.current_pos.unwrap() && *color == game_state.one_color
        });
    let to_piece = to_result.is_ok();
    debug!("from_piece:{from_piece}, to_piece:{to_piece}");

    // if !matches!(game_state.status, GameStatus::Started) {
    //     return;
    // }
    let do_mov = |piece: Piece, from: Pos, to: Pos, is_to_piece: bool| {
        let can_move = match piece.get_block_for_move(from.to_vec3(), to.to_vec3()) {
            BlockPos::AllBlock => false,
            BlockPos::NoBlock => true,
            BlockPos::Block(block) => piece_query
                .iter()
                .find(|(_, _, _, pos, _)| pos.0 == block.x && pos.1 == block.y)
                .is_none(),
            BlockPos::CheBlock(is_x, axis_from, axis_to) => {
                !piece_query.iter().any(|(_, _, _, pos, _)| {
                    is_x && pos.0 == from.0 && pos.1 > axis_from && pos.1 < axis_to
                        || !is_x && pos.1 == from.1 && pos.0 > axis_from && pos.0 < axis_to
                })
            }
            BlockPos::PaoBlock(is_x, axis_from, axis_to) => {
                if is_to_piece {
                    piece_query
                        .iter()
                        .filter(|(_, _, _, pos, _)| {
                            is_x && pos.0 == from.0 && pos.1 > axis_from && pos.1 < axis_to
                                || !is_x && pos.1 == from.1 && pos.0 > axis_from && pos.0 < axis_to
                        })
                        .count()
                        == 1
                } else {
                    !piece_query.iter().any(|(_, _, _, pos, _)| {
                        is_x && pos.0 == from.0 && pos.1 > axis_from && pos.1 < axis_to
                            || !is_x && pos.1 == from.1 && pos.0 > axis_from && pos.0 < axis_to
                    })
                }
            }
        };
        debug!("can_move:{can_move}");
        can_move
    };

    let is_jianged = |piece: Piece, from: Pos, one_color: PieceColor| {
        let jiang_pos = piece_query
            .iter()
            .find_map(|(_, _, &p, &pos, &color)| {
                if matches!(p, Piece::Jiang(_)) && color != one_color {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap();
        do_mov(piece, from, jiang_pos, true)
    };
    /*
     *  from     to
     *  none     pos    none
     *  none     piece  set piece
     *  piece    pos    mov
     *  piece    piece  mov  & remove piece
     *  piece =  piece  unset piece
     */
    if from_piece {
        let from = game_state.current_pos.unwrap();
        let (_, _, from_piece, _, from_color) = piece_query
            .iter()
            .find(|(_, _, _, pos, _)| **pos == from)
            .unwrap();
        if to_piece {
            let (_, to_piece, to, to_color) = to_result
                .map(|(to_entity, _, to_piece, to_pos, to_color)| {
                    (to_entity, to_piece, *to_pos, *to_color)
                })
                .unwrap();

            if *from_color != to_color {
                if do_mov(*from_piece, from, to, true) {
                    game_state.current_pos = None;
                    game_state.is_turn = false;

                    //判断是否将军
                    let is_jianged =
                        !to_piece.is_jiang() && is_jianged(*from_piece, to, game_state.one_color);

                    commands.spawn(Action::MovePiece(
                        from,
                        to,
                        true,
                        is_jianged,
                        to_piece.is_jiang(),
                    ));
                    commands.spawn(Packet(Action::MovePiece(
                        from * Vec2::new(1., -1.),
                        to * Vec2::new(1., -1.),
                        true,
                        is_jianged,
                        to_piece.is_jiang(),
                    )));
                    return;
                }
            } else {
                game_state.current_pos = Some(to);
                commands.spawn(SoundType::Choose);
                return;
            }
        } else {
            let to = pos_query
                .get(trigger.entity())
                .map(|(_, pos, _)| *pos)
                .unwrap();

            if do_mov(*from_piece, from, to, false) {
                game_state.current_pos = None;
                game_state.is_turn = false;
                //判断是否将军
                let is_jianged = is_jianged(*from_piece, to, game_state.one_color);

                commands.spawn(Action::MovePiece(from, to, false, is_jianged, false));
                commands.spawn(Packet(Action::MovePiece(
                    from * Vec2::new(1., -1.),
                    to * Vec2::new(1., -1.),
                    false,
                    is_jianged,
                    false,
                )));
                return;
            }
        }
    } else {
        if to_piece {
            let (color, pos) = to_result
                .map(|(_, _, _, to_pos, to_color)| (to_color, to_pos))
                .unwrap();
            if *color == game_state.one_color {
                game_state.current_pos = Some(*pos);
                commands.spawn(SoundType::Choose);
            }
        }
    }
}

fn on_move_over(
    trigger: Trigger<Pointer<Over>>,
    mut piece_query: Query<(&mut Transform, &Piece, Option<&Action>)>,
) {
    if let Ok((mut trans, _, action)) = piece_query.get_mut(trigger.entity()) {
        let Some(_) = action else {
            trans.scale += Vec3::splat(0.1);
            return;
        };
    }
}
fn on_move_out(
    trigger: Trigger<Pointer<Out>>,
    mut piece_query: Query<(&mut Transform, &Piece, Option<&Action>)>,
) {
    if let Ok((mut trans, _, action)) = piece_query.get_mut(trigger.entity()) {
        let Some(_) = action else {
            trans.scale -= Vec3::splat(0.1);
            return;
        };
    }
}

pub fn check_ready(mut app_state: ResMut<NextState<AppState>>, game_state: Res<GameState>) {
    if game_state.one_ready && game_state.two_ready {
        app_state.set(AppState::Ready);
    }
}

pub fn create_piece_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_state: ResMut<GameState>,
    mut event_writer: EventWriter<ChessEvent>,
    opt: Res<Opt>,
    mut piece_query: Query<(Entity, &Piece)>,
    mut client_event_ids: ResMut<ChessClientEventIds>,
) {
    for (e, ..) in piece_query.iter_mut() {
        commands.entity(e).despawn();
    }

    let mut create_pieces = |is_one: bool, color: PieceColor| {
        let texture = asset_server.load("/assets/pieces.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 4, 4, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut create_piece = |pos: Pos, index: usize, piece: Piece| {
            commands
                .spawn((
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index,
                        },
                    ),
                    piece,
                    pos,
                    color,
                    Transform::from_translation(pos.to_vec3() * opt.cell_width)
                        .with_scale(Vec3::new(0.4, 0.4, 0.1)),
                ))
                .observe(on_click_board)
                .observe(on_move_over)
                .observe(on_move_out);
        };

        let mul = if is_one { -1. } else { 1. };
        let ind = if matches!(color, PieceColor::Red) {
            0
        } else {
            8
        };
        //红方
        //将
        create_piece(Pos(0., 4.5 * mul), 0 + ind, Piece::Jiang(JiangMovable {}));
        //士
        create_piece(Pos(-1., 4.5 * mul), 1 + ind, Piece::Shi(ShiMovable {}));
        create_piece(Pos(1., 4.5 * mul), 1 + ind, Piece::Shi(ShiMovable {}));
        //相
        create_piece(Pos(-2., 4.5 * mul), 2 + ind, Piece::Xiang(XiangMovable {}));
        create_piece(Pos(2., 4.5 * mul), 2 + ind, Piece::Xiang(XiangMovable {}));
        //马
        create_piece(Pos(-3., 4.5 * mul), 4 + ind, Piece::Ma(MaMovable {}));
        create_piece(Pos(3., 4.5 * mul), 4 + ind, Piece::Ma(MaMovable {}));
        //车
        create_piece(Pos(-4., 4.5 * mul), 6 + ind, Piece::Che(CheMovable {}));
        create_piece(Pos(4., 4.5 * mul), 6 + ind, Piece::Che(CheMovable {}));
        //炮
        create_piece(Pos(-3., 2.5 * mul), 5 + ind, Piece::Pao(PaoMovable {}));
        create_piece(Pos(3., 2.5 * mul), 5 + ind, Piece::Pao(PaoMovable {}));
        //兵
        create_piece(
            Pos(-4., 1.5 * mul),
            7 + ind,
            Piece::Bing(BingMovable(is_one)),
        );
        create_piece(
            Pos(4., 1.5 * mul),
            7 + ind,
            Piece::Bing(BingMovable(is_one)),
        );
        create_piece(
            Pos(-2., 1.5 * mul),
            7 + ind,
            Piece::Bing(BingMovable(is_one)),
        );
        create_piece(
            Pos(2., 1.5 * mul),
            7 + ind,
            Piece::Bing(BingMovable(is_one)),
        );
        create_piece(
            Pos(0., 1.5 * mul),
            7 + ind,
            Piece::Bing(BingMovable(is_one)),
        );
    };

    match game_state.one_color {
        PieceColor::UnSet => {
            let one_peer = game_state.one_player.clone().unwrap().peer_id;
            let two_peer = game_state.two_player.clone().unwrap().peer_id;
            if one_peer.max(two_peer).eq(&one_peer) {
                game_state.one_color = PieceColor::Red;
                game_state.two_color = PieceColor::Black;

                game_state.is_turn = true;
            } else {
                game_state.one_color = PieceColor::Black;
                game_state.two_color = PieceColor::Red;
            }
        }
        PieceColor::Red => game_state.is_turn = true,
        _ => {}
    }

    create_pieces(true, game_state.one_color);
    create_pieces(false, game_state.two_color);

    app_state.set(AppState::InGame);
    send_event_to_html(
        ChessEvent::in_game(),
        &mut event_writer,
        &mut client_event_ids,
    );
}
