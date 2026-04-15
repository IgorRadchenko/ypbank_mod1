// bin/ypbank_compare.rs 

/*
ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
# Output: The transaction records in 'records_example.bin' and 'records_example.csv' are identical.
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut file1: String = "".to_string();
    let mut file2: String = "".to_string();
    let mut format1: String = "".to_string();
    let mut format2: String = "".to_string();

    for i in 1..args.len() {
        //println!("Got: {i}");

        match args[i].as_str() {
            "--file1" => {
                file1 = args[i+1].clone();
            }
            "--format1" => {
                format1 = args[i+1].clone();
            }
            "--file2" => {
                file2 = args[i+1].clone();
            }
            "--format2" => {
                format2 = args[i+1].clone();
            }
            _ => {
                //eprintln!("Неизвестная команда {i}");
            }
        }
    }

    println!("file1: '{file1}'; format1: {format1}");
    println!("file2: '{file2}'; format2: {format2}");
}
