use effective_rust::show_readme;

fn main() {
    show_readme(3);
}

struct S {
    field: Option<i32>,
}

#[test]
fn access_s_field() {
    let s = S { field: Some(42) };

    // match &s.field {
    //     Some(i) => println!("field is {i}"),
    //     &None => {}
    // }

    if let Some(i) = &s.field {
        println!("field is {}", i);
    }
}
