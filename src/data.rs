use chrono::NaiveDate;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PriceData {
    pub date: NaiveDate,
    pub close: f64,
}

// Error es un trait object ya que pueden ser distintos errores
pub fn read_prices(path: &str) -> Result<Vec<PriceData>, Box<dyn Error>> {
    // El ? hace internamente un match, que si es falso da error
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut prices = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let date = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
        let close: f64 = record[4].parse()?; // columna "Close"
        prices.push(PriceData { date, close });
    }

    Ok(prices)
}