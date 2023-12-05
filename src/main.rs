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
        .add_row([3, day4::first_part(), day4::second_part()])
        .column_iter_mut()
        .for_each(|col| col.set_cell_alignment(CellAlignment::Right));
    println!("{table}");
}
