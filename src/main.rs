enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Color::Yellow => return true,
            Color::Blue => return true,
            Color::Green => return false,
            Color::Red => return false,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blues"),
        Color::Yellow => println!("yellow")
    }
}

fn main() {
    let foo = Color::Green;

    println!("{:?}", foo.is_green());
}
