use std::{fs, path::Path};

// This is the main function.
fn main() {
    // hello_world()
    // add_numbers()
    // for_loop()
    // reverse_string()
    read_file()
}

// fn hello_world() {
//     println!("Hello World")
// }

// fn add_numbers() {
//     let a: i32 = 2;
//     let b: i32 = 2;
//     println!("The sum is {}", a + b)
// }

// fn for_loop(){
//     // =can be used to include end number
//     for x in 1..=5{
//         println!("the loop is {x}")
//     }
// }

// fn reverse_string(){
//     let name: String = "shreyas".to_string();
//     let reversed_name: String = name.chars().rev().collect();
//     println!("{reversed_name}");
// }

fn read_file() {
    let path = Path::new("src/assets/sample_text_file.tx");
    let text = fs::read_to_string(path).expect("invalid file");
    println!("{text}");
}
