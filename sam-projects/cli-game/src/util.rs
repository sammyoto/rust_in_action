use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("data/names.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut names: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        for name in record.iter() {
            names.push(name.to_string());
        }
    }

    Ok(names)
}