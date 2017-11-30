extern crate itertools_num;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pcg_rand;

mod controllers;
mod drawing;
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

#[no_mangle]
pub extern "C" fn update(time: c_double) -> *mut c_char {
    CString::new(_update(time)).unwrap().into_raw()
}

pub fn _update(time: f64) -> String {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update_seconds(time, data.input_controller.actions(), &mut data.state);
    CollisionsController::handle_collisions(&mut data.state);

    // Stats
    let player_pos = data.state.world.player.vector.position;
    format!("Player: {}, {}\nEnemies: {}", player_pos.x, player_pos.y, data.state.world.enemies.len())
}

// Comment this out, as it is definitely Piston-dependent
//mod view;

pub fn main() {
    // Things we need to know from the browser:
    // * Update frame
}

// MVP
// * No display
// * No input
// * 5 updates per second
// * For each update, log some stats to the console

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
