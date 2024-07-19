// lib.rs
#[repr(C)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[no_mangle]
pub extern "C" fn print_color(color: Color) {
    match color {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }
}
