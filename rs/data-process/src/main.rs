use polars::prelude::*;
use std::fs::File;

fn example() -> Result<DataFrame> {
    let file = File::open("./analysis-20200116.csv").expect("could not read file");
    CsvReader::new(file).has_header(true).finish()
}

fn main() {
    let df = example().unwrap();
    println!("{:?}", df);
}
