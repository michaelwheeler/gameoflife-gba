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

#[derive(Clone, PartialEq)]
enum Status {
    Live,
    Dead,
}

#[derive(Clone)]
pub struct Cell {
    status: Status,
    index: i32,
}

impl Cell {
    pub fn new(index: i32) -> Self {
        Cell {
            status: Status::Dead,
            index,
        }
    }

    fn thrive(&mut self) {
        self.status = Status::Live;
    }

    fn perish(&mut self) {
        self.status = Status::Dead;
    }

    fn is_live(&self) -> bool {
        self.status == Status::Live
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            Status::Live => Status::Dead,
            Status::Dead => Status::Live,
        }
    }

    fn sprite(&self) -> SpriteVram {
        if self.is_live() {
            sprites::LIVE.sprite(0).into()
        } else {
            sprites::DEAD.sprite(0).into()
        }
    }

    fn position(&self) -> Vector2D<i32> {
        let slot = vec2(self.index % BOARD_WIDTH, self.index / BOARD_WIDTH);
        let offset = (agb::display::WIDTH - 16 * BOARD_WIDTH) / 2;
        slot * 16 + vec2(offset, 0 + 8)
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
