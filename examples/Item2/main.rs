use std::f64::consts::PI;

use effective_rust::show_readme;

fn main() {
    show_readme(2);
}

enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Self::Circle { radius } => PI * radius * radius,
        }
    }
}

#[test]
fn whats_the_area() {
    let r = Shape::Rectangle {
        width: 7.,
        height: 3.,
    };
    let c = Shape::Circle { radius: 7. };

    println!("r: {}", r.area());
    println!("c: {}", c.area());
}
