use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::io;

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

pub fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);

    let mut response: String = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    response.trim().to_string()
}