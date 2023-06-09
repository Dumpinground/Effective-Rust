use std::fs;

// present the readme.md in the item
pub fn show_readme(id: i32) {
    let file_name = "readme.md";
    let file_path = format!("examples/item{id}/{file_name}");
    let readme_file = fs::read_to_string(&file_path);
    let text = match readme_file {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open file with path {file_path}:\n{e}");
            "No Infomation Yet".to_owned()
        }
    };
    println!("Item {id}\n  {text}");
}
