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

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn function_pointer() {
    let op: fn(i32, i32) -> i32 = sum;
    println!("{}", op(1, 2));

    let op1 = op;
    let op2 = op;
    assert!(op1 == op2);
    println!("op = {:p}", op);
}

pub fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
    for value in data {
        *value = mutator(*value);
    }
}

fn add2(v: u32) -> u32 {
    v + 2
}

#[test]
fn modify_data_by_2() {
    let mut data = vec![1, 2, 3];
    modify_all(&mut data, add2);
    assert_eq!(data, vec![3, 4, 5]);
    println!("{:?}", data);
}

#[test]
fn add_by_n() {
    let amount_to_add = 3;
    let add_n = |y| y + amount_to_add;
    let z = add_n(5);
    assert_eq!(z, 8);
}
