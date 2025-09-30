# Quant Analyzer (Rust)

Un pequeño analizador cuantitativo escrito en **Rust** para calcular métricas financieras básicas a partir de precios históricos.

Actualmente soporta:
- Cálculo de **retornos logarítmicos**.
- **Volatilidad anualizada**.
- **Sharpe ratio** (retorno ajustado al riesgo).

---

## Estructura del proyecto
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
Con datos de prueba (`prices.csv`):

```
Quant analizer
Volatilidad anualizada: 32.23
Shape ratio: 0.66
```

---

## 📜 Licencia
MIT License. Libre para usar, modificar y aprender.
