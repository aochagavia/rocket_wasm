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

pub fn loop_(time: f64) -> String {
    let data = DATA.lock().unwrap();
    format!("Hello world")
}

// Comment this out, as it is definitely Piston-dependent
//mod view;

pub fn main() {
    // Things we need to know from the browser:
    // * Update frame

    // Things to keep in static memory
    // * Game state
    // * InputController
    // * TimeController
}

// MVP
// * No display
// * No input
// * 5 updates per second
// * For each update, log some stats to the console
