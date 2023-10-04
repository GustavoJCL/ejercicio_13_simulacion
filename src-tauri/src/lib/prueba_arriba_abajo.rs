use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct ResponseTable {
    mu: f64,
    sigma: f64,
    estadistico: f64,
    estadistico_tabla: f64,
    corridas: i64,
}
#[tauri::command(rename_all = "snake_case")]
pub fn prueba_arriba_abajo(data: BTreeMap<usize, f64>, nivel_confianza: f64) -> ResponseTable {
    let data_sequence: BTreeMap<usize, bool> = (1..data.len())
        .map(|i| {
            if *data.get(&i).unwrap() <= *data.get(&(i - 1)).unwrap() {
                (i, false)
            } else {
                (i, true)
            }
        })
        .collect();
    let mut corridas: i64 = 1;
    for i in 1..data_sequence.len() {
        if data_sequence[&i] != data_sequence[&(i - 1)] {
            corridas += 1;
        }
    }

    let n: f64 = data.len() as f64;
    let mu: f64 = (2.0 * n - 1.0) / 3.0;
    let sigma: f64 = (16.0 * n - 29.0) / 90.0;
    let estadistico: f64 = ((corridas as f64 - mu) / sigma.powf(0.5)).abs();

    let normal = Normal::new(0.0, 1.0).unwrap();
    let alpha = 1.0 - nivel_confianza;
    let estadistico_tabla: f64 = normal.inverse_cdf(alpha / 2.0);
    ResponseTable {
        mu,
        sigma,
        estadistico,
        estadistico_tabla,
        corridas,
    }
}
