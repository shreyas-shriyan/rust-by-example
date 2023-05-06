// use std::{fs, path::Path};

// This is the main function.
fn main() {
    // hello_world()
    // add_numbers()
    // for_loop()
    // reverse_string()
    // read_file()
    // read_user_input()
    arrays_and_slices()
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

// fn read_file() {
//     let path = Path::new("src/assets/sample_text_file.tx");
//     let text = fs::read_to_string(path).expect("invalid file");
//     println!("{text}");
// }

// fn read_user_input(){
//    let mut line = String::new();
//    println!("Enter your name :");
//    std::io::stdin().read_line(&mut line).unwrap();
//    println!("Hello, {}", line);
// }

fn arrays_and_slices() {
    // arrays cannot be modified
    // they are of fixed size
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for item in array {
        println!("item is {}", item)
    }

    // slices can be are part of an array
    let slice = &array[1..3];
    for item in slice {
        println!("slice item is {item}")
    }
}
