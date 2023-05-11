//This is a simple command line app for logging inventory
use std::io;
use std::env;

 

fn add_item(args: Vec<String>) {
 
    let item_name = args_1;
    let item_quantity = args_2;
    let item_location = args_3;
}

fn remove_item(args: Vec<String>) {
    let item_name = args_1;
    let item_quantity = args_2;
    let item_location = args_3;
}

fn list_items(args: Vec<String>) {
    
}

fn help() {
    dbg!("Help");
}

fn arg_selector(selector_name: &str, args: Vec<String>) {
    match selector_name {
        "--add" => add_item(args), 
        "--remove" => remove_item(args),
        "--list" => list_items(args),
        "--help" => help(),
        _ => println!("Invalid selector. Please use --help for more information"),

    }
}


fn main() {
   let args: Vec<String> = env::args().collect(); 
   arg_selector(&args[1], args[2..].to_vec());
}
