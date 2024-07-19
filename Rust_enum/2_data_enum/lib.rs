/*
rustc --crate-type=cdylib lib.rs -o libcomplex_color.so
*/
#[repr(C)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum ComplexColor {
    Red,
    Green,
    Blue,
    Custom(String),
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
pub extern "C" fn print_complex_color(tag: i32, custom_data: *const i8) {
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
    } else {
        println!("Cannot map complex color to C++ Color");
    }
}
