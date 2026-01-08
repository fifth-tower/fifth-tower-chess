use bevy::prelude::*;

pub enum BlockPos {
    AllBlock,
    NoBlock,
    Block(Vec3),
    //(isX, from, to)
    CheBlock(bool, f32, f32),

    //(isX, from, to)
    PaoBlock(bool, f32, f32),
}
pub trait Movable {
    ///不满足移动条件，返回AllBlock
    ///满足移动条件，无block点时，返回(0,0,0)
    /// 满足移动条件，有block点时，返回block位置
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos;

    fn maybe_pos(&self, to: Vec3) -> bool {
        true
    }
}

#[derive(Clone, Copy, Component)]
pub enum Piece {
    Jiang(JiangMovable),
    Shi(ShiMovable),
    Xiang(XiangMovable),
    Ma(MaMovable),
    Che(CheMovable),
    Pao(PaoMovable),
    Bing(BingMovable),
}

impl Piece {
    pub fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        match self {
            Piece::Jiang(movable) => movable.get_block_for_move(from, to),
            Piece::Shi(movable) => movable.get_block_for_move(from, to),
            Piece::Xiang(movable) => movable.get_block_for_move(from, to),
            Piece::Ma(movable) => movable.get_block_for_move(from, to),
            Piece::Che(movable) => movable.get_block_for_move(from, to),
            Piece::Pao(movable) => movable.get_block_for_move(from, to),
            Piece::Bing(movable) => movable.get_block_for_move(from, to),
        }
    }

    pub fn is_jiang(&self) -> bool {
        if let Self::Jiang(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
pub struct JiangMovable;
impl Movable for JiangMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }
        let Vec3 { x, y, .. } = to - from;
        if x == 0. && y.abs() == 1. {
            return BlockPos::NoBlock;
        }
        if y == 0. && x.abs() == 1. {
            return BlockPos::NoBlock;
        }
        BlockPos::AllBlock
    }
    fn maybe_pos(&self, to: Vec3) -> bool {
        let Vec3 { x, y, .. } = to;
        if x.abs() > 1. {
            return false;
        }
        if y.abs() < 2.5 {
            return false;
        }
        if y.abs() > 4.5 {
            return false;
        }
        true
    }
}

#[derive(Clone, Copy)]
pub struct ShiMovable;
impl Movable for ShiMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }
        let Vec3 { x, y, .. } = to - from;
        if x.abs() == 1. && y.abs() == 1. {
            return BlockPos::NoBlock;
        }
        BlockPos::AllBlock
    }
    fn maybe_pos(&self, to: Vec3) -> bool {
        let Vec3 { x, y, .. } = to;
        if x.abs() == 1. && (y.abs() == 2.5 || y.abs() == 4.5) {
            return true;
        }
        if x == 0. && y.abs() == 3.5 {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy)]
pub struct XiangMovable;
impl Movable for XiangMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }

        let Vec3 { x, y, .. } = to - from;
        if x.abs() == 2. && y.abs() == 2. {
            return BlockPos::Block(from + Vec3::new(x / 2., y / 2., 0.));
        }
        BlockPos::AllBlock
    }

    fn maybe_pos(&self, to: Vec3) -> bool {
        let Vec3 { x, y, .. } = to;
        match (x.abs(), y.abs()) {
            (0., 2.5) => true,
            (2., 0.) => true,
            (4., 2.5) => true,
            (2., 0.5) => true,
            (2., 4.5) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MaMovable;
impl Movable for MaMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }

        let Vec3 { x, y, .. } = to - from;
        if x.abs() == 1. && y.abs() == 2. {
            return BlockPos::Block(from + Vec3::new(0., y / 2., 0.));
        }
        if x.abs() == 2. && y.abs() == 1. {
            return BlockPos::Block(from + Vec3::new(x / 2., 0., 0.));
        }
        BlockPos::AllBlock
    }
}

#[derive(Clone, Copy)]
pub struct CheMovable;
impl Movable for CheMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }

        let Vec3 { x, y, .. } = to - from;
        if x == 0. {
            return BlockPos::CheBlock(true, from.y.min(to.y), from.y.max(to.y));
        }
        if y == 0. {
            return BlockPos::CheBlock(false, from.x.min(to.x), from.x.max(to.x));
        }
        BlockPos::AllBlock
    }
}

#[derive(Clone, Copy)]
pub struct PaoMovable;
impl Movable for PaoMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }

        let Vec3 { x, y, .. } = to - from;
        if x == 0. {
            return BlockPos::PaoBlock(true, from.y.min(to.y), from.y.max(to.y));
        }
        if y == 0. {
            return BlockPos::PaoBlock(false, from.x.min(to.x), from.x.max(to.x));
        }
        BlockPos::AllBlock
    }
}

//Bing(is_one)
#[derive(Clone, Copy)]
pub struct BingMovable(pub bool);
impl Movable for BingMovable {
    fn get_block_for_move(&self, from: Vec3, to: Vec3) -> BlockPos {
        if !self.maybe_pos(to) {
            return BlockPos::AllBlock;
        }
        let Vec3 { x, y, .. } = to - from;
        debug!("from:{from},to:{to}={x},{y}");
        match self.0 {
            true => match (x.abs(), y) {
                (0., 1.) => BlockPos::NoBlock,
                (1., 0.) if from.y > 0. => BlockPos::NoBlock,
                _ => BlockPos::AllBlock,
            },
            false => match (x.abs(), y) {
                (0., 1.) => BlockPos::NoBlock,
                (1., 0.) if from.y < 0. => BlockPos::NoBlock,
                _ => BlockPos::AllBlock,
            },
        }
    }

    fn maybe_pos(&self, to: Vec3) -> bool {
        let Vec3 { x, y, .. } = to;

        match self.0 {
            true => {
                if y > 0. {
                    return true;
                }
                match (x.abs(), y) {
                    (0., -1.5) => true,
                    (2., -1.5) => true,
                    (4., -1.5) => true,
                    (0., -0.5) => true,
                    (2., -0.5) => true,
                    (4., -0.5) => true,
                    _ => false,
                }
            }
            false => {
                if y < 0. {
                    return true;
                }
                match (x.abs(), y) {
                    (0., 1.5) => true,
                    (2., 1.5) => true,
                    (4., 1.5) => true,
                    (0., 0.5) => true,
                    (2., 0.5) => true,
                    (4., 0.5) => true,
                    _ => false,
                }
            }
        }
    }
}
