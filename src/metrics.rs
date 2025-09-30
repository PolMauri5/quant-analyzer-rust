// Formulas financieras (retornos, shapes, etc..)
use crate::data::PriceData;

// Retornos logaritmicos
pub fn calculate_log_returns(prices: &[PriceData]) -> Vec<f64> {
    let mut returns = Vec::new();

    for i in 1..prices.len() {
        let prev = prices[i - 1].close;
        let curr = prices[i].close;
        let ret = (curr / prev).ln(); // significa logaritmo natural
        returns.push(ret);
    }

    returns
}

pub fn annualized_volatility(returns: &[f64]) -> f64 {
    // Calcular la media de los retornos diaria
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;

    // Calcular la varianza (promedio de (r - media)^2)
    let variance = returns.iter()
        // map transforma cada elemento en otro valorm
        // |r| es un clousure, funcion anonima
        // r es cada resultado de la iteracion
        .map(|r| (r - mean).powi(2))
        .sum::<f64>() / (returns.len() as f64 - 1.0);

    // Calcular desviacion estandar diaria = raiz cuadrada de la varianza
    let daily_vol = variance.sqrt();

    // Calcular volatilidad anualizada = volatilidad diaria * sqrt(252) -> dias de trading al año
    daily_vol * (252.0_f64).sqrt()
}

// Shape ratio es una medida para defini que tan buena és una inversion en relacion al riesgo que estas tomando
pub fn shape_ratio(returns: &[f64]) -> f64 {
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;

    //Retorno anualizado
    let annualized_return = mean * 252.0;

    let vol = annualized_volatility(returns);

    // Shape ratio
    annualized_return / vol
}
