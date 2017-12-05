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

use std::os::raw::{c_double, c_int};
use std::sync::Mutex;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use self::game_state::GameState;
use self::geometry::Size;
use self::controllers::{Actions, TimeController, CollisionsController};

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    state: GameState,
    actions: Actions,
    time_controller: TimeController<Pcg32Basic>
}

fn new_game_data(width: f64, height: f64) -> GameData {
    GameData {
        state: GameState::new(Size::new(width, height)),
        actions: Actions::default(),
        time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
    }
}

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_player(_: c_double, _: c_double, _: c_double);
    fn draw_enemy(_: c_double, _: c_double);
    fn draw_bullet(_: c_double, _: c_double);
    fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_score(_: c_double);
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    use geometry::{Advance, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;

    clear_screen();
    for particle in &world.particles {
        draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
    }

    for bullet in &world.bullets {
        draw_bullet(bullet.x(), bullet.y());
    }

    for enemy in &world.enemies {
        draw_enemy(enemy.x(), enemy.y());
    }

    draw_player(world.player.x(), world.player.y(), world.player.direction());
    draw_score(data.state.score as f64);
}

#[no_mangle]
pub extern "C" fn update(time: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update_seconds(time, &data.actions, &mut data.state);
    CollisionsController::handle_collisions(&mut data.state);
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn toggle_shoot(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.shoot = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_boost(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.boost = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_turn_left(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_left = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_turn_right(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_right = int_to_bool(b);
}
