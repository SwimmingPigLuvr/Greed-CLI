use owo_colors::OwoColorize;
use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io;
use std::io::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Player;

// not sure if match values will work for comparing dice values

// 1-6
fn dice_roll() -> (i32, i32) {
    (thread_rng().gen_range(1..7), thread_rng().gen_range(1..7))
}

// 1-12
fn mega_dice_roll() -> (i32, i32) {
    (thread_rng().gen_range(1..13), thread_rng().gen_range(1..13))
}

fn init_roll(f: fn() -> (i32, i32)) -> () {
    let r1 = f().0;
    let r2 = f().1;

    println!(
        "\n{} + {} = {}\n",
        r1.red().on_white().bold(),
        r2.red().on_white().bold(),
        (r1 + r2).bright_green()
    );

    if r1 == 1 && r2 == 1 {
        println!("{}", ("  SNAKE EYES  ").on_bright_magenta());
    }
}

// normal roll turn
pub fn roll() -> () {
    init_roll(dice_roll);
    // check rol
    // go again?
}

// MegaDice
pub fn mega_roll() -> () {
    init_roll(mega_dice_roll)
}

// pub fn get_players() -> Vec<Player>

// pub fn game() -> () {

//     // welcome/instructions
//     // enter player names
//         'game: loop {

//             'turn: loop {

//             }
//         }

// }
