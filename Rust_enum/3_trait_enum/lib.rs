/*
rustc --crate-type=cdylib lib.rs -o libdescribable_color.so
*/

pub trait Describable {
    fn describe(&self) -> String;
}

pub enum ComplexColor {
    Red,
    Green,
    Blue,
    Custom(String),
}

impl Describable for ComplexColor {
    fn describe(&self) -> String {
        match self {
            ComplexColor::Red => String::from("Red"),
            ComplexColor::Green => String::from("Green"),
            ComplexColor::Blue => String::from("Blue"),
            ComplexColor::Custom(color) => format!("Custom color: {}", color),
        }
    }
}

#[repr(C)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl ComplexColor {
    fn to_c_color(&self) -> Option<Color> {
        match self {
            ComplexColor::Red => Some(Color::Red),
            ComplexColor::Green => Some(Color::Green),
            ComplexColor::Blue => Some(Color::Blue),
            _ => None,
        }
    }
}

#[no_mangle]
pub extern "C" fn print_color(color: Color) {
    match color {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }
}

#[no_mangle]
pub extern "C" fn print_complex_color_with_description(tag: i32, custom_data: *const i8) {
    let color = match tag {
        0 => ComplexColor::Red,
        1 => ComplexColor::Green,
        2 => ComplexColor::Blue,
        3 => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(custom_data) };
            let str_slice = c_str.to_str().unwrap();
            ComplexColor::Custom(str_slice.to_string())
        },
        _ => panic!("Invalid tag"),
    };

    if let Some(c_color) = color.to_c_color() {
        print_color(c_color);
        println!("{}", color.describe());
    } else {
        println!("Cannot map complex color to C++ Color");
        println!("{}", color.describe());
    }
}
