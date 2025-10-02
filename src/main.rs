mod data;
mod metrics;
mod graph;

fn main() {
    println!("Quant analizer");

    // Leer precios del CSV
    let prices_appl = data::read_prices("data/sp500.csv").expect("Error leyendo csv");
    let prices_two = data::read_prices("data/sp500.csv").expect("Error");

    // Calcular los retornos
    let returns = metrics::calculate_log_returns(&prices_appl);
    let returns_two = metrics::calculate_log_returns(&prices_two);

    // Calcular la volatilidad y el sharpe
    let vol = metrics::annualized_volatility(&returns);
    let sharpe = metrics::sharpe_ratio(&returns);
    let sortino_ratio = metrics::sortino_ratio(&returns);
    let cumulative_return = metrics::cumulative_return(&prices_appl);
    let equity_curve = metrics::cumulative_return_log_series(&returns);
    let calmar_ratio = metrics::calmar_ratio(&returns);
    // let eq_curve = metrics::equity_curve(&returns, 1000.0);
    let recovery_time = metrics::recovery_time(&returns);
    let corelacion = metrics::corelation_between_actives(&returns, &returns_two);
    let beta = metrics::beta(&returns, &returns_two);
    let jensen = metrics::jensen_alpha(&returns, &returns_two, 1.0);
    println!("Volatilidad anualizada: {:.2}", vol * 100.0); // ":" -> empieza la especificacion de un formato, el .2 define 2 decimales max
    println!("sharpe ratio: {:.2}", sharpe);
    println!("Sortino ratio: {:.2}", sortino_ratio);
    println!("Retorno acumulado: {:.2}%", cumulative_return);
    println!("Calmar ratio: {:.2}", calmar_ratio);
    println!("Corelacion entre activos {:.2}", corelacion);
    // println!("Equity curve como capital total: {:?}", eq_curve);
    println!("Recovery times in days from biggest bearish trend: {}", recovery_time);
    println!("Jensen: {}", jensen);
    println!("Beta vs benchmark: {}", beta);
    println!("Curva de capital relativa:");
    for val in &equity_curve[0..5] {
        println!("{:.2}", val);
    }

}
