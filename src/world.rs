use core::array;

use crate::cell::Cell;
use crate::{BOARD_HEIGHT, BOARD_WIDTH};
use agb::fixnum::{Vector2D, vec2};

pub type World = [Cell; (BOARD_WIDTH * BOARD_HEIGHT) as usize];

pub fn empty_world() -> World {
    array::from_fn(|i| Cell::new(i as i32))
}

fn get_slot(index: i32) -> Vector2D<i32> {
    vec2(index % BOARD_WIDTH, index / BOARD_WIDTH)
}

pub fn get_index(slot: &Vector2D<i32>) -> usize {
    (slot.x.rem_euclid(BOARD_WIDTH) + slot.y.rem_euclid(BOARD_HEIGHT) * BOARD_WIDTH)
        .try_into()
        .unwrap()
}

pub fn update_status(world: &mut World) {
    for cell in world.iter_mut() {
        cell.update_status();
    }
}

pub fn next_gen(world: &World) -> World {
    let mut next = world.clone();
    for (i, cell) in next.iter_mut().enumerate() {
        let slot = get_slot(i.try_into().unwrap());
        let neighbors = {
            let idx_tl = get_index(&(slot + vec2(-1, -1)));
            let idx_tc = get_index(&(slot + vec2(0, -1)));
            let idx_tr = get_index(&(slot + vec2(1, -1)));
            let idx_ml = get_index(&(slot + vec2(-1, 0)));
            let idx_mr = get_index(&(slot + vec2(1, 0)));
            let idx_bl = get_index(&(slot + vec2(-1, 1)));
            let idx_bc = get_index(&(slot + vec2(0, 1)));
            let idx_br = get_index(&(slot + vec2(1, 1)));
            [
                &world[idx_tl],
                &world[idx_tc],
                &world[idx_tr],
                &world[idx_ml],
                &world[idx_mr],
                &world[idx_bl],
                &world[idx_bc],
                &world[idx_br],
            ]
        };
        cell.update(&neighbors);
    }
    next
}
