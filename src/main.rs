mod shapes;

use crate::shapes::{rect::Rect, circle::Circle, area::Area};

fn main() {
    let rect = Rect::default();
    println!("{}", rect);

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 4.0,
    };

    println!("{}", circ.area());
    println!("{}", rect.area());

}
