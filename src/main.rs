extern crate itertools_num;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pcg_rand;

mod controllers;
mod game_state;
mod geometry;
mod models;
mod util;

use std::ffi::CString;
use std::mem;
use std::os::raw::{c_double, c_char, c_void};
use std::sync::Mutex;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use self::game_state::GameState;
use self::geometry::Size;
use self::controllers::{InputController, TimeController, CollisionsController};

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(GameData {
        state: GameState::new(Size::new(1024.0, 600.0)),
        input_controller: InputController::new(),
        time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
    });
}

struct GameData {
    state: GameState,
    input_controller: InputController,
    time_controller: TimeController<Pcg32Basic>
}

extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_enemy(_: c_double, _: c_double);
}

#[no_mangle]
pub extern "C" fn draw() {
    use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    unsafe {
        clear_screen();
        draw_player(data.state.world.player.x(), data.state.world.player.y(), data.state.world.player.direction());
        for enemy in &data.state.world.enemies {
            draw_enemy(enemy.x(), enemy.y());
        }
    }
}

#[no_mangle]
pub extern "C" fn update(time: c_double) {
    _update(time)
}

pub fn _update(time: f64) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update_seconds(time, data.input_controller.actions(), &mut data.state);
    CollisionsController::handle_collisions(&mut data.state);

    // Stats
    //let player_pos = data.state.world.player.vector.position;
    //format!("Player: {}, {}\nEnemies: {}", player_pos.x, player_pos.y, data.state.world.enemies.len())
}

pub fn main() {}

// The usual boilerplate
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
