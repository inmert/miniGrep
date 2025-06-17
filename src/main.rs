use std::env;
use std::process;

// Library Crate
use minigrep::{run, Inputs};

fn main() {
    let args:Vec<String> = env::args().collect();
    let input_vals = Inputs::new(&args).unwrap_or_else( |err| {
        println!("Usage <query> <filename> <case_sensitivity> ERROR: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(input_vals){
        println!("Error processing file: {}", e);
        process::exit(1)
    }
}


