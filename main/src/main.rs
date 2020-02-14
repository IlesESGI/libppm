extern crate ppm;
use colored::*;
use std::io::{self};
use std::path::Path;

// Funcion asking user the path of the image he wants to load
fn ask_path_load() -> String {
    println!("{}", "Enter the path of the image to load :".green().on_black().bold());
    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .ok()
        .expect("Failed to read line");
        path = path.trim().to_string();
    return path;
}

// Funcion asking user the path where he wants to save the image
fn ask_path_save() -> String {
    println!("{}", "Enter the path where you want the image to be saved :".green().on_black().bold());
    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .ok()
        .expect("Failed to read line");
        path = path.trim().to_string();
    return path;
}

// Funcion asking user what kind of file he is loading (plain or binary)
fn ask_type() -> u8 {
    println!("{}", "Do you want to read a plain text or a binary ppm ? \n(Answer by p or b)".green().on_black().bold());
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed to read line");
    match name.trim() {
        "p" => 0,
        "b" => 1,
        _ => 2,
    }
}

// Funcion asking user what kind of file he is saving (plain or binary)
fn ask_type_ii() -> u8 {
    println!("{}", "Do you want to save a plain text or a binary ppm ? \n(Answer by p or b)".green().on_black().bold());
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed to read line");
    match name.trim() {
        "p" => 0,
        "b" => 1,
        _ => 2,
    }
}

// Funcion asking user what kind of operation he wants to do on the image
fn ask_operation() -> u8 {
    println!("{}","What do you want to do with this image ?".green().on_black().bold());
    println!("{}","0 - Invert colors".green().on_black().bold());
    println!("{}","1 - Transform to grayscale".green().on_black().bold());
    println!("{}","2 - Rotate the image by 180°".green().on_black().bold());
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed to read line");
    match name.trim() {
        "0" => 0,
        "1" => 1,
        _ => 2,
    }
}

fn main() {

    println!("\n\n");
    println!("{} {} {}", "Rust Programm for 4IABD".green().on_black().bold(), "\"Big Project\"".black().on_green().bold(), "Project !".green().on_black().bold());
    println!("\n");
    println!("{}", "Written by group 10".green().on_black().bold());
    println!("\n\n");

    let mut type_i = ask_type();

    while type_i == 2 {
        type_i = ask_type(); 
    }

    let mut operation = ask_operation();

    while operation != 0 && operation != 1 && operation != 2 {
        operation = ask_operation(); 
    }

    let load_path = ask_path_load();

    let mut image: ppm::Image;

    if type_i == 1 {
        image = ppm::Image::new_with_file_binary(Path::new(&load_path)).unwrap();
    }
    else {
        image = ppm::Image::new_with_file_plain(Path::new(&load_path)).unwrap();
    }

    if operation == 0 {
        image.invert();
    } else if operation == 1 {
        image.to_grayscale();
    } else {
        image.rotate();
    }

    let mut type_ii = ask_type_ii();

    while type_ii == 2 {
        type_ii = ask_type_ii();
    }

    let save_path = ask_path_save();

    if type_ii == 1 {
        image.save_binary(Path::new(&save_path));
    }
    else {
        image.save(Path::new(&save_path));
    }

    println!("\n{}\n", "Thanks, bye !".green().on_black().bold());

}
