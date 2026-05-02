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

pub struct Cell {
    live: bool,
    index: i32,
}

impl Cell {
    pub fn new(index: i32) -> Self {
        Cell { live: false, index }
    }

    pub fn thrive(&mut self) {
        self.live = true;
    }

    fn perish(&mut self) {
        self.live = false;
    }

    pub fn toggle(&mut self) {
        self.live = !self.live;
    }

    fn sprite(&self) -> SpriteVram {
        if self.live {
            sprites::LIVE.sprite(0).into()
        } else {
            sprites::DEAD.sprite(0).into()
        }
    }

    fn position(&self) -> Vector2D<i32> {
        let slot = vec2(self.index % BOARD_WIDTH, self.index / BOARD_WIDTH);
        let offset = (agb::display::WIDTH - 16 * BOARD_WIDTH) / 2;
        slot * 16 + vec2(offset, 0)
    }

    fn update(&mut self, neighbors: &[Self; 8]) {
        let live_neighbors = neighbors.iter().filter(|cell| cell.live).count();
        match live_neighbors {
            2 if self.live => self.thrive(),
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
