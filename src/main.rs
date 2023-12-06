//! # Advent of Code 2023
//!
//! ## Français
//! 
//! Ceci est ma contribution à l’Advent of Code.
//! Tu devrais essayer de résoudre les problèmes par toi-même avant d’utilise
//!   ce code pour gagner des étoiles.
//! 
//! ## Esperanto
//! 
//! Tio estas mia kontribuo al la Advent of Code.
//! Vi provu solvi la problemojn sole antaŭ ol uzi ĉi tion kodon por akiri
//!   stelojn.
//! 
//! ## English
//!
//! This is my contribution to the Advent of Code.
//! You should try to resolve the problems by yourself before using this code
//!   to obtain stars.
//! 

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{CellAlignment, Table};

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(["Day", "Part 1", "Part 2"])
        .add_row([1, day1::first_part(), day1::second_part()])
        .add_row([2, day2::first_part(), day2::second_part()])
        .add_row([3, day3::first_part(), day3::second_part()])
        .add_row([4, day4::first_part(), day4::second_part()])
        .add_row([5, day5::first_part()])
        .column_iter_mut()
        .for_each(|col| col.set_cell_alignment(CellAlignment::Right));
    println!("{table}");
}
