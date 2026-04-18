// bin/ypbank_converter.rs 

/*
ypbank_converter \
  --input <input_file> \
  --input-format <format> \
  --output-format <format> \
  > output_file.txt
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut input_file: String = "".to_string();
    let mut input_format: String = "".to_string();
    let mut output_format: String = "".to_string();

    for i in 1..args.len() {
        //println!("Got: {i}");

        match args[i].as_str() {
            "--input" => {
                input_file = args[i+1].clone();
            }
            "--input_format" => {
                input_format = args[i+1].clone();
            }
            "--output-format" => {
                output_format = args[i+1].clone();
            }
            _ => {
                //eprintln!("Неизвестная команда {i}");
            }
        }
    }

    println!("input_file: '{input_file}'");
    println!("input_format: '{input_format}'");
    println!("output_format: '{output_format}'");

}
