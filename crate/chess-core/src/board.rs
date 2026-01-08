use crate::{config::*, on_click_board};
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Startup, (setup_scene, create_board_system).chain());
    }
}

/// set up a simple 3D scene
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn create_board_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    opt: Res<Opt>,
) {
    for x in 0..8 {
        let x = x as f32;
        for y in 0..9 {
            if y == 4 {
                continue;
            }
            let y = y as f32;

            //half_line_width
            let hlw = opt.board_line_width / 2.;
            let center_offset = Vec3::new(0.25, 0.25, 0.);
            //(大格顶点坐标， 小格顶点坐标， 绘制线条)
            let pos_arr = vec![
                (
                    Pos(-4. + x, -4.5 + y),
                    Vec3::splat(0.),
                    Vec3::new(hlw, hlw, 0.),
                    Corner::LeftBottom,
                ),
                (
                    Pos(-3. + x, -4.5 + y),
                    Vec3::new(-0.5, 0., 0.),
                    Vec3::new(0., hlw, 0.),
                    Corner::RightBottom,
                ),
                (
                    Pos(-4. + x, -3.5 + y),
                    Vec3::new(0., -0.5, 0.),
                    Vec3::new(hlw, 0., 0.),
                    Corner::LeftTop,
                ),
                (
                    Pos(-3. + x, -3.5 + y),
                    Vec3::new(-0.5, -0.5, 0.),
                    Vec3::new(0., 0., 0.),
                    Corner::RightTop,
                ),
            ];

            for pos in pos_arr {
                if pos.3.matches(Pos(x, y)) {
                    pos.3
                        .draw(Pos(x, y), &mut commands, &mut meshes, &mut materials, &opt);
                }
                commands
                    .spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            opt.cell_width / 2. - hlw,
                            opt.cell_width / 2. - hlw,
                        ))),
                        MeshMaterial2d(materials.add(Color::from(opt.board_color()))),
                        pos.0,
                        Transform::from_translation(
                            (pos.0.to_vec3() + pos.1 + center_offset).with_z(-1.) * opt.cell_width
                                + pos.2
                                + hlw / 2.,
                        ),
                    ))
                    .observe(on_click_board);
            }
        }
    }

    let cross_lines = [
        vec![Vec3::new(-1., 4.5, 0.), Vec3::new(1., 2.5, 0.)],
        vec![Vec3::new(-1., 2.5, 0.), Vec3::new(1., 4.5, 0.)],
        vec![Vec3::new(-1., -4.5, 0.), Vec3::new(1., -2.5, 0.)],
        vec![Vec3::new(-1., -2.5, 0.), Vec3::new(1., -4.5, 0.)],
    ];
    for line in cross_lines {
        commands.spawn((
            Mesh2d(meshes.add(LineStrip {
                points: line.iter().map(|it| it * opt.cell_width).collect(),
            })),
            MeshMaterial2d(materials.add(Color::from(opt.clear_color()))),
            Transform::from_xyz(0., 0., 0.),
        ));
    }
}

enum Corner {
    LeftBottom,
    RightBottom,
    RightTop,
    LeftTop,
}

impl Corner {
    const LINE_GAP: (f32, f32) = (0.125, 0.25);
    const LEFT_BOTTOM_LINES: [Vec3; 3] = [
        Vec3::new(Corner::LINE_GAP.0, Corner::LINE_GAP.1, 0.),
        Vec3::new(Corner::LINE_GAP.0, Corner::LINE_GAP.0, 0.),
        Vec3::new(Corner::LINE_GAP.1, Corner::LINE_GAP.0, 0.),
    ];
    const BING_PAO_POS: [Vec2; 14] = [
        Vec2::new(-4., 1.5),
        Vec2::new(-2., 1.5),
        Vec2::new(0., 1.5),
        Vec2::new(2., 1.5),
        Vec2::new(4., 1.5),
        Vec2::new(-4., -1.5),
        Vec2::new(-2., -1.5),
        Vec2::new(0., -1.5),
        Vec2::new(2., -1.5),
        Vec2::new(4., -1.5),
        Vec2::new(-3., -2.5),
        Vec2::new(3., -2.5),
        Vec2::new(-3., 2.5),
        Vec2::new(3., 2.5),
    ];
    pub fn draw(
        &self,
        pos: Pos,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        opt: &Res<Opt>,
    ) {
        match self {
            Self::LeftBottom => {
                self.draw_left_bottom(pos, commands, meshes, materials, opt);
            }
            Self::RightBottom => self.draw_right_bottom(pos, commands, meshes, materials, opt),
            Self::RightTop => self.draw_right_top(pos, commands, meshes, materials, opt),
            Self::LeftTop => self.draw_left_top(pos, commands, meshes, materials, opt),
        }
    }

    pub fn draw_left_bottom(
        &self,
        pos: Pos,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        opt: &Res<Opt>,
    ) {
        let (x, y) = (pos.0 - 4., pos.1 - 4.5);

        // Spawn a line strip that goes from point to point
        commands.spawn((
            Mesh2d(
                meshes.add(LineStrip {
                    points: Corner::LEFT_BOTTOM_LINES
                        .iter()
                        .map(|vec3| vec3 * opt.cell_width)
                        .collect(),
                }),
            ),
            MeshMaterial2d(materials.add(Color::from(opt.clear_color()))),
            Transform::from_xyz(x * opt.cell_width, y * opt.cell_width, 0.),
        ));
    }

    pub fn draw_right_bottom(
        &self,
        pos: Pos,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        opt: &Res<Opt>,
    ) {
        let (x, y) = (pos.0 - 4., pos.1 - 4.5);
        // Spawn a line strip that goes from point to point
        commands.spawn((
            Mesh2d(
                meshes.add(LineStrip {
                    points: Corner::LEFT_BOTTOM_LINES
                        .iter()
                        .map(|vec3| {
                            (vec3 * Vec3::new(-1., 1., 0.) + Vec3::new(1., 0., 0.)) * opt.cell_width
                        })
                        .collect(),
                }),
            ),
            MeshMaterial2d(materials.add(Color::from(opt.clear_color()))),
            Transform::from_xyz(x * opt.cell_width, y * opt.cell_width, 0.),
        ));
    }
    pub fn draw_right_top(
        &self,
        pos: Pos,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        opt: &Res<Opt>,
    ) {
        let (x, y) = (pos.0 - 4., pos.1 - 4.5);

        // Spawn a line strip that goes from point to point
        commands.spawn((
            Mesh2d(
                meshes.add(LineStrip {
                    points: Corner::LEFT_BOTTOM_LINES
                        .iter()
                        .map(|vec3| {
                            (vec3 * Vec3::new(-1., -1., 0.) + Vec3::new(1., 1., 0.))
                                * opt.cell_width
                        })
                        .collect(),
                }),
            ),
            MeshMaterial2d(materials.add(Color::from(opt.clear_color()))),
            Transform::from_xyz(x * opt.cell_width, y * opt.cell_width, 0.),
        ));
    }
    pub fn draw_left_top(
        &self,
        pos: Pos,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        opt: &Res<Opt>,
    ) {
        let (x, y) = (pos.0 - 4., pos.1 - 4.5);

        // Spawn a line strip that goes from point to point
        commands.spawn((
            Mesh2d(
                meshes.add(LineStrip {
                    points: Corner::LEFT_BOTTOM_LINES
                        .iter()
                        .map(|vec3| {
                            (vec3 * Vec3::new(1., -1., 0.) + Vec3::new(0., 1., 0.)) * opt.cell_width
                        })
                        .collect(),
                }),
            ),
            MeshMaterial2d(materials.add(Color::from(opt.clear_color()))),
            Transform::from_xyz(x * opt.cell_width, y * opt.cell_width, 0.),
        ));
    }
    pub fn matches(&self, pos: Pos) -> bool {
        let (x, y) = (pos.0 - 4., pos.1 - 4.5);
        match self {
            Self::LeftBottom => Corner::BING_PAO_POS.iter().any(|it| x == it.x && y == it.y),
            Self::RightBottom => Corner::BING_PAO_POS
                .iter()
                .any(|it| x + 1. == it.x && y == it.y),
            Self::RightTop => Corner::BING_PAO_POS
                .iter()
                .any(|it| x + 1. == it.x && y + 1. == it.y),
            Self::LeftTop => Corner::BING_PAO_POS
                .iter()
                .any(|it| x == it.x && y + 1. == it.y),
        }
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}
