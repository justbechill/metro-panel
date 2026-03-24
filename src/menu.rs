use crate::lcd;

// Struct for an individual setting in the menu
struct Setting {
    name: String,
    init: fn(),
    increase: fn() -> i32,
    decrease: fn() -> i32,
}

const FIELDS: [&str; 3] = ["Metro Panel!", "On/Off", "Brightness"];

static mut SELECTED_INDEX: usize = 0;
static mut FIELD_SELECTED: bool = false;

fn init() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    // If any field is not currently selected
    let next_index = if SELECTED_INDEX != FIELDS.len() - 1 {
        SELECTED_INDEX + 1
    } else {
        0
    };

    println!("{}", next_index);

    lcd::write(0, FIELDS[SELECTED_INDEX]);
    lcd::write(1, FIELDS[next_index]);

    Ok(())
}

pub fn down() {
    SELECTED_INDEX += 1;
}
