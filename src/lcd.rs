use lcd_display::{GPIO_Pin, LCD, lcd::LCD_Mode};

fn init() {
    // Control pins
    let rs = GPIO_Pin::new("/dev/gpiochip1", 03).expect("Error while opening RS pin!");
    let en = GPIO_Pin::new("/dev/gpiochip1", 04).expect("Error while opening EN pin!");

    // Data pins
    let d4 = GPIO_Pin::new("/dev/gpiochip0", 12).expect("Error while opening D4 pin!");
    let d5 = GPIO_Pin::new("/dev/gpiochip3", 26).expect("Error while opening D5 pin!");
    let d6 = GPIO_Pin::new("/dev/gpiochip0", 14).expect("Error while opening D6 pin!");
    let d7 = GPIO_Pin::new("/dev/gpiochip1", 01).expect("Error while opening D7 pin!");

    let data_pins = vec![d4, d5, d6, d7];

    let mut lcd = LCD::new(rs, en, data_pins, LCD_Mode::FourBit);

    lcd.begin(16, 2);

    lcd.set_cursor(0, 0);
    lcd.print("Hello, world!");

    lcd.set_cursor(0, 1);
    lcd.print("This is an LCD");

    std::thread::sleep(std::time::Duration::from_secs(30));

    lcd.clear();
}

pub fn write(line: i32, text: &str) {
    println!("line {}: {}", line, text);
}
