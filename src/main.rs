#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, vec2},
    input::{Button, ButtonController},
    println,
};
use core::array;

use crate::cell::Cell;

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

mod cell;
mod sprites;

const BOARD_WIDTH: i32 = 12;
const BOARD_HEIGHT: i32 = 8;

enum GameState {
    Intro,
    Setup,
    Simulation,
}

type World = [Cell; (BOARD_WIDTH * BOARD_HEIGHT) as usize];

struct Game {
    state: GameState,
    world: World,
    setup: Setup,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Setup,
            world: array::from_fn(|i| Cell::new(i as i32)),
            setup: Setup::new(),
        }
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        match self.state {
            GameState::Intro => (),
            GameState::Setup => self.setup.show(frame),
            GameState::Simulation => (),
        }
    }

    fn update(&mut self, btn_controller: &ButtonController) {
        match self.state {
            GameState::Setup => self.setup.update(btn_controller),
            _ => (),
        }
    }
}

struct Intro {
    button_controller: ButtonController,
}

struct Setup {
    world: World,
    selection: u16,
}

impl Setup {
    fn new() -> Self {
        Setup {
            world: array::from_fn(|i| Cell::new(i as i32)),
            selection: 0,
        }
    }

    fn slot(&self) -> Vector2D<i32> {
        vec2(
            self.selection as i32 % BOARD_WIDTH,
            self.selection as i32 / BOARD_WIDTH,
        )
    }

    fn set_selection(&mut self, slot: &Vector2D<i32>) {
        self.selection =
            (slot.x.rem_euclid(BOARD_WIDTH) + slot.y.rem_euclid(BOARD_HEIGHT) * BOARD_WIDTH) as u16;
    }

    fn update(&mut self, btn_controller: &ButtonController) {
        if btn_controller.is_just_pressed(Button::A) {
            self.world[self.selection as usize].toggle();
        }
        if btn_controller.is_just_pressed(Button::Right) {
            let slot = self.slot() + vec2(1, 0);
            self.set_selection(&slot);
        }
        if btn_controller.is_just_pressed(Button::Left) {
            let slot = self.slot() + vec2(-1, 0);
            self.set_selection(&slot);
        }
        if btn_controller.is_just_pressed(Button::Down) {
            let slot = self.slot() + vec2(0, 1);
            self.set_selection(&slot);
        }
        if btn_controller.is_just_pressed(Button::Up) {
            let slot = self.slot() + vec2(0, -1);
            self.set_selection(&slot);
        }
    }

    fn sprite(&self) -> SpriteVram {
        sprites::sprites::TARGET.sprite(0).into()
    }

    fn position(&self) -> Vector2D<i32> {
        let offset = (agb::display::WIDTH - 16 * BOARD_WIDTH) / 2;
        self.slot() * 16 + vec2(offset, 0)
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        for cell in self.world.iter() {
            cell.show(frame);
        }
        Object::new(self.sprite())
            .set_pos(self.position())
            .set_priority(agb::display::Priority::P0)
            .show(frame);
    }
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    let mut btn_controller = ButtonController::new();
    let mut game = Game::new();
    loop {
        btn_controller.update();
        let mut frame = gfx.frame();
        game.update(&btn_controller);
        game.show(&mut frame);
        frame.commit();
    }
}
