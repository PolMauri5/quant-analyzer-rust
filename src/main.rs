mod data;
mod metrics;

fn main() {
    println!("Quant analizer");

    // Leer precios del CSV
    let prices = data::read_prices("data/prices.csv").expect("Error leyendo csv");

    // Calcular los retornos
    let returns = metrics::calculate_log_returns(&prices);

    // Calcular la volatilidad y el shape
    let vol = metrics::annualized_volatility(&returns);
    let shape = metrics::shape_ratio(&returns);

    println!("Volatilidad anualizada: {:.2}", vol * 100.0); // ":" -> empieza la especificacion de un formato, el .2 define 2 decimales max
    println!("Shape ratio: {:.2}", shape);

}
