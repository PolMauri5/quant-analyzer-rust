# Quant Analyzer (Rust)

Un pequeño analizador cuantitativo escrito en **Rust** para calcular métricas financieras básicas a partir de precios históricos.

Actualmente soporta:
- Cálculo de **retornos logarítmicos**.
- **Volatilidad anualizada**.
- **Sharpe ratio** (retorno ajustado al riesgo).

---

##  Estructura del proyecto
```
src/
 ├── main.rs      # Punto de entrada, orquesta la lectura de datos y cálculos
 ├── data.rs      # Lectura de CSV y definición de la struct PriceData
 └── metrics.rs   # Funciones de métricas financieras (retornos, volatilidad, Sharpe)
data/
 └── prices.csv   # Archivo de precios de ejemplo (Date, Open, High, Low, Close, Volume)
```

---

## Ejemplo de CSV (`data/prices.csv`)
```csv
Date,Open,High,Low,Close,Volume
2025-01-02,100,101,99,100.5,1200000
2025-01-03,100.5,102,100,101.8,1350000
2025-01-06,101.8,103,101,102.5,1100000
2025-01-07,102.5,104,102,103.7,1450000
```

> El programa usa la columna `Date` y la columna `Close`.  
> El resto se ignora, pero se deja para compatibilidad con fuentes reales (Yahoo Finance, Stooq, etc).

---

## Ejecución

Clona el repositorio y ejecuta con:

```bash
git clone https://github.com/PolMauri5/quant-analyzer-rust.git
cd quant-analyzer-rust
cargo run
```

---

## Ejemplo de salida
Con datos reales de Apple (últimos 5 años):

```
Quant analizer
Volatilidad anualizada: 32.23
Shape ratio: 0.66
```

---

## ¿Qué significan estos valores?

### Volatilidad anualizada
Mide cuánto varían los retornos alrededor de su media, es decir, el **riesgo** o la **incertidumbre** de la inversión.

Fórmula de la varianza muestral diaria:  
![Varianza](https://latex.codecogs.com/png.latex?\bg_white\sigma^2%20=%20\frac{1}{N-1}%20\sum_{t=1}^N(r_t-\bar{r})^2)

Volatilidad diaria:  
![VolDaily](https://latex.codecogs.com/png.latex?\bg_white\sigma_{daily}%20=%20\sqrt{\sigma^2})

Volatilidad anualizada (suponiendo 252 días de trading):  
![VolAnnual](https://latex.codecogs.com/png.latex?\bg_white\sigma_{annual}%20=%20\sigma_{daily}%20\times%20\sqrt{252})

---

### Sharpe ratio
Mide la **rentabilidad ajustada al riesgo**: cuánto retorno adicional obtiene una inversión por cada unidad de riesgo asumido.

![Sharpe](https://latex.codecogs.com/png.latex?\bg_white%20Sharpe%20=%20\frac{E[R]-%20R_f}{\sigma})

- **E[R]** = retorno medio anualizado de la inversión.  
- **Rf** = tasa libre de riesgo (en este programa asumida como 0).  
- **σ** = volatilidad anualizada.  

Interpretación práctica:
- **Sharpe < 1** → riesgo demasiado alto para el retorno.  
- **1 – 2** → aceptable.  
- **2 – 3** → muy bueno.  
- **> 3** → excelente (muy raro en mercados reales).   

---

## Licencia
MIT License. Libre para usar, modificar y aprender 🚀.
