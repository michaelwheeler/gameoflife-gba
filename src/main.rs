#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        GraphicsFrame,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, vec2},
    input::{Button, ButtonController},
};

use world::World;

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

mod cell;
mod sprites;
mod world;

const BOARD_WIDTH: i32 = 12;
const BOARD_HEIGHT: i32 = 8;

#[derive(PartialEq)]
enum GameState {
    Intro,
    Setup,
    Simulation,
}

struct Game {
    state: GameState,
    setup: Setup,
    simulation: Simulation,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Setup,
            setup: Setup::new(),
            simulation: Simulation::new(),
        }
    }

    fn update(&mut self, btn_controller: &ButtonController) {
        if btn_controller.is_just_pressed(Button::B) {
            if self.state == GameState::Setup {
                self.simulation.setup(self.setup.world.clone());
            }
            if self.state == GameState::Simulation {
                self.simulation.running = false;
            }
            self.state = match self.state {
                GameState::Intro => GameState::Setup,
                GameState::Setup => GameState::Simulation,
                GameState::Simulation => GameState::Setup,
            }
        }
        match self.state {
            GameState::Setup => self.setup.update(btn_controller),
            GameState::Simulation => self.simulation.update(btn_controller),
            _ => (),
        }
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        match self.state {
            GameState::Intro => (),
            GameState::Setup => self.setup.show(frame),
            GameState::Simulation => self.simulation.show(frame),
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
            world: world::empty_world(),
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
        self.selection = world::get_index(slot) as u16;
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

struct Simulation {
    world: World,
    counter: u32,
    running: bool,
}

impl Simulation {
    const STEP_FRAMES: u32 = 30;

    fn new() -> Self {
        Self {
            world: world::empty_world(),
            counter: 0,
            running: false,
        }
    }

    fn setup(&mut self, new_world: World) {
        self.world = new_world;
        self.running = true;
    }

    fn update(&mut self, button_controller: &ButtonController) {
        if button_controller.is_just_pressed(Button::Start) {
            self.running = !self.running;
        }
        if !self.running {
            return;
        }
        self.counter += 1;
        if self.counter == Simulation::STEP_FRAMES {
            self.world = world::next_gen(&self.world);
            self.counter = 0;
        }
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        for cell in self.world.iter() {
            cell.show(frame);
        }
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
