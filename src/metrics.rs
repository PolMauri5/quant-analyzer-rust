// Formulas financieras (retornos, sharpes, etc..)
use crate::data::PriceData;

// Retornos logaritmicos
pub fn calculate_log_returns(prices: &[PriceData]) -> Vec<f64> {
    let mut returns = Vec::new();

    for i in 1..prices.len() {
        let prev = prices[i - 1].apertura;
        let curr = prices[i].apertura;
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

// sharpe ratio es una medida para defini que tan buena és una inversion en relacion al riesgo que estas tomando
pub fn sharpe_ratio(returns: &[f64]) -> f64 {
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;

    //Retorno anualizado
    let annualized_return = (mean * 252.0).exp() - 1.0;

    let vol = annualized_volatility(returns);

    // sharpe ratio
    annualized_return / vol
}

pub fn cumulative_return(returns: &[PriceData]) -> f64 {
    let first_price = returns.first().unwrap().apertura;
    let last_price = returns.last().unwrap().apertura;

    ((last_price / first_price) - 1.0) * 100.0
}

pub fn cumulative_return_log_series(returns: &[f64]) -> Vec<f64> {
    let mut equity_curve = Vec::new();
    let mut cum_sum = 0.0;

    for &r in returns {
        cum_sum += r; // Acumula los returns
        equity_curve.push(cum_sum.exp()); // Exp es para exponenciar el capital relativo
    }

    equity_curve
}

pub fn sortino_ratio(returns: &[f64]) -> f64 {
    // Media de todos los valores
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    // Solo los valores negativos del array
    let negatives: Vec<f64> = returns.iter().cloned().filter(|r| *r < 0.0).collect();

    if negatives.is_empty() {
        return f64::INFINITY;
    }
    // Promedio de los cuadrados de los numeros negativos
    let downside_variance = negatives.iter().map(|r| r.powi(2)).sum::<f64>() / negatives.len() as f64;
    // Sacamos la desviacion downside (raiz cuadrada del promedio de negativos)
    let downside_variation = downside_variance.sqrt();

    //Anualizamos
    let annualized_return  = (mean * 252.0).exp() - 1.0;
    let anualized_downside_variation = downside_variation * (252_f64).sqrt();

    // Sortino ratio
    annualized_return / anualized_downside_variation
}

pub fn max_drawdon(returns: &[f64]) -> f64 {
    let mut equity = Vec::new();
    let mut cum_sum = 0.0;
    for &r in returns {
        cum_sum += r;
        equity.push(cum_sum.exp());
    }

    let mut peak = equity[0];
    let mut max_dd = 0.0;

    //Por cada valor del arr de acumulados
    for &val in &equity {
        // Si el val es mayor que el mas grande
        if val > peak {
            // Se transforma en el mas grande
            peak = val
        }
        // Por cada valor, se divide entre el mayor
        let dd = (val / peak) - 1.0;
        if dd < max_dd {
            max_dd = dd;
        }
    }
    max_dd.abs() // Trasfromar en negativo
}

// El calmar ratio es el (retorno anualizado / (caida mas grande des de un hh a un ll)
pub fn calmar_ratio(returns: &[f64]) -> f64 {
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;

    let annualized_returns = (mean * 252.0).exp() - 1.0;
    let max_dd = max_drawdon(returns);

    if max_dd == 0.0 {
        return f64::INFINITY;
    }

    annualized_returns / max_dd
}

// Recordatorio que siempre me olvido: para calcular logs, solo hace falta el valor con log.exp()
#[allow(dead_code)]
pub fn equity_curve(returns: &[f64], init_capital: f64) -> Vec<f64> {
    // With capacity le dice directamente al vec cuanta memoria va a usar asi que optimiza el programa
    let mut curve = Vec::with_capacity(returns.len());
    let mut cum_sum = 0.0;

    for &r in returns {
        cum_sum += r;
        curve.push(init_capital * cum_sum.exp());
    }

    curve
}

pub fn recovery_time(returns: &[f64]) -> i32 {
    let mut equity = Vec::new();
    let mut cum_size = 0.0;

    for &r in returns {
        cum_size += r;
        equity.push(cum_size.exp());
    }

    let mut peak = equity[0];
    let mut days_in_dd = 0;
    let mut max_recovery = 0;

    for &val in &equity {
        if val >= peak {
            // Hemos vuelto al pico o superado
            peak = val;
            if  days_in_dd > max_recovery {
                max_recovery = days_in_dd;
            }
            days_in_dd = 0;
        } else {
            days_in_dd += 1;
        }
    }
    max_recovery
}

pub fn corelation_between_actives(returns: &[f64], returns_two: &[f64]) -> f64 {
    // Aseguramos que tengan la misma longitud
    assert_eq!(returns.len(), returns_two.len());
    
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let mean_two = returns_two.iter().sum::<f64>() / returns_two.len() as f64;

    // Varinaza
    let mut varianza = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..returns.len() {
        let dx = returns[i] - mean;
        let dy = returns_two[i] - mean_two;
        varianza += dx * dy;

        var_x += dx.powi(2);
        var_y += dy.powi(2);
    } 
    varianza / (var_x.sqrt() * var_y.sqrt())
}

// Mida la sensibilidad del activo sobre el mercado a lo largo de todo el periodo, da igual que el precio final sea igual
// Ejemplo resultado: 3, si el mercado sube 1 tu activo tiende a subir 3, si el mercado baja 1 tu activo tiende a bajar 3 
// Como mas alto mas riesgo
pub fn beta(asset_return: &[f64], market_return: &[f64]) -> f64 {
    assert_eq!(asset_return.len(), market_return.len());

    let media = asset_return.iter().sum::<f64>() / asset_return.len() as f64;
    let media_mercado = market_return.iter().sum::<f64>() / market_return.len() as f64;

    // caovarianza entre activo y mercado
    let mut varianza = 0.0;
    let mut var_x = 0.0;

    for i in 0..asset_return.len() {
        let dx = asset_return[i] - media;
        let dy = market_return[i] - media_mercado;

        varianza += dx * dy;

        var_x += dy.powi(2);
    }
    varianza / var_x
}

// Minimo retorno que deberia de dar segun el riesgo 0=neutral | >0 = buen resultado segun el riesgo | <0 = mal resultado segun el riesgo
pub fn jensen_alpha(asset_return: &[f64], market_return: &[f64], risk_free: f64) -> f64 {
    let mean_asset = asset_return.iter().sum::<f64>() / asset_return.len() as f64;
    let mean_market = market_return.iter().sum::<f64>() / market_return.len() as f64; 

    let beta_val = beta(asset_return, market_return);

    mean_asset - (risk_free + beta_val * (mean_market - risk_free))
}
