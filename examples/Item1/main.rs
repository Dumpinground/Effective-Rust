use effective_rust::show_readme;

fn main() {
    show_readme(1);
}

struct TextMatch(usize, String);

#[test]
fn whats_text_match() {
    let m = TextMatch(12, "needle".to_owned());
    assert_eq!(m.0, 12);
    println!("{}", m.0);
}

enum HttpResultCode {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
}

#[test]
fn http_code() {
    let code = HttpResultCode::NotFound;

    let msg = match code {
        HttpResultCode::Ok => "Ok",
        HttpResultCode::NotFound => "Not Found",
        HttpResultCode::Teapot => "Teapot",
    };

    assert_eq!(code as i32, 404);
}

#[derive(Debug)]
pub enum Sides {
    Both,
    Single,
}

#[derive(Debug)]
pub enum Output {
    BlackAndWhite,
    Colorful,
}

pub fn print_page(sides: Sides, output: Output) {
    let side = match sides {
        Sides::Both => "both sides",
        Sides::Single => "single side",
    };

    let theme = match output {
        Output::BlackAndWhite => "black and white",
        Output::Colorful => "colorful",
    };

    println!("This page has {side}, with {theme} theme");
}

#[test]
fn which_page() {
    print_page(Sides::Both, Output::BlackAndWhite);
    print_page(Sides::Single, Output::Colorful);

    println!("{:?}, {:?}", Sides::Both, Output::Colorful);
}
