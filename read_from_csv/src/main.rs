use csv;
use std::error::Error;
use std::env::args;
use std::path::Path;

fn main(){
    if let Err(e) = read_from_csv(){
        eprintln!("{}", e);
    }
}

fn read_from_csv() -> Result<(), Box<dyn Error>> {
    if args().len() != 2 {
        println!("Usage: read_from_csv <csv_file>");
        return Ok(());
    }
    let path = args().nth(1).unwrap();
    let path = Path::new(&path);

    let mut reader = csv::Reader::from_path(&path)?;

    for result in reader.records(){
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}