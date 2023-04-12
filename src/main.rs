use std::{error::Error ,env};
use csv;

fn read_from_file (path: &str) -> Result<(), Box<dyn Error>> {
     let mut reader = csv::Reader::from_path(path)?;

     for result in reader.records() {
        let record = result?;

        println!("{:?}", record)
     };
     Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: <./filename>");
        return
    }

    println!("{:?}", args[1]);

    if let Err(e) = read_from_file(&args[1]) {
        eprintln!("{:?}", e);
    };

}
