use crate::BOARD_WIDTH;
use agb::{
    display::{
        GraphicsFrame,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, vec2},
    include_aseprite,
};

include_aseprite!(
    mod sprites,
    "src/gfx/cell.aseprite"
);

#[derive(Clone, PartialEq, Debug)]
enum Status {
    Live(usize),
    Dead(usize),
}

#[derive(Clone)]
pub struct Cell {
    status: Status,
    index: i32,
}

impl Cell {
    const GROW_FRAMES: usize = 8;
    const SHRINK_FRAMES: usize = 4;

    pub fn new(index: i32) -> Self {
        Cell {
            status: Status::Dead(0),
            index,
        }
    }

    fn thrive(&mut self) {
        self.status = match self.status {
            Status::Live(count) => Status::Live(count),
            Status::Dead(_) => Status::Live(Cell::GROW_FRAMES),
        };
    }

    fn perish(&mut self) {
        self.status = match self.status {
            Status::Dead(count) => Status::Dead(count),
            Status::Live(_) => Status::Dead(Cell::SHRINK_FRAMES),
        }
    }

    pub fn is_live(&self) -> bool {
        match self.status {
            Status::Live(_) => true,
            Status::Dead(_) => false,
        }
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            Status::Live(_) => Status::Dead(Cell::SHRINK_FRAMES),
            Status::Dead(_) => Status::Live(Cell::GROW_FRAMES),
        }
    }

    fn sprite(&self) -> SpriteVram {
        match self.status {
            Status::Live(count) => sprites::LIVE
                .animation_frame(&mut (Cell::GROW_FRAMES - count), 0)
                .into(),
            Status::Dead(count) => sprites::DEAD
                .animation_frame(&mut (Cell::SHRINK_FRAMES - count), 0)
                .into(),
        }
    }

    fn position(&self) -> Vector2D<i32> {
        let slot = vec2(self.index % BOARD_WIDTH, self.index / BOARD_WIDTH);
        let offset = (agb::display::WIDTH - 16 * BOARD_WIDTH) / 2;
        slot * 16 + vec2(offset, 0 + 8)
    }

    pub fn update_status(&mut self) {
        self.status = match self.status {
            Status::Live(0) => Status::Live(0),
            Status::Live(count) => Status::Live(count - 1),
            Status::Dead(0) => Status::Dead(0),
            Status::Dead(count) => Status::Dead(count - 1),
        };
    }

    pub fn update(&mut self, neighbors: &[&Self; 8]) {
        let live_neighbors = neighbors.iter().filter(|cell| cell.is_live()).count();
        match live_neighbors {
            2 if self.is_live() => self.thrive(),
            3 => self.thrive(),
            _ => self.perish(),
        }
    }

    pub fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite())
            .set_pos(self.position())
            .set_priority(agb::display::Priority::P1)
            .show(frame);
    }
}
