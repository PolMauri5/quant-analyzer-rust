use chrono::NaiveDate;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PriceData {
    pub fecha: NaiveDate,
    pub apertura: f64,
    pub cierre: f64,
}

fn parse_number(s: &str) -> Result<f64, Box<dyn Error>> {
    let normalized = s.replace(".", "").replace(",", ".");
    Ok(normalized.parse::<f64>()?)
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
        let fecha = NaiveDate::parse_from_str(&record[0], "%d.%m.%Y")?;
        let apertura = parse_number(&record[2])?;
        let cierre   = parse_number(&record[1])?;
        prices.push(PriceData { fecha, apertura, cierre });
    }
    prices.sort_by_key(|p| p.fecha);
    Ok(prices)
}