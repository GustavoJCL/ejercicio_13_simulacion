use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};
use std::collections::BTreeMap;

use super::csv_to_btreemap::csv_to_data;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct ResponseTable {
    mu: f64,
    sigma: f64,
    estadistico: f64,
    estadistico_tabla: f64,
    corridas: i64,
    normal_inv_value: BTreeMap<usize, (f64, f64)>,
}
#[tauri::command(rename_all = "snake_case")]
pub fn prueba_arriba_abajo(data_string: String, nivel_confianza: f64) -> ResponseTable {
    let data: BTreeMap<usize, f64> = csv_to_data(data_string);
    let data_sequence: BTreeMap<usize, bool> = (1..data.len())
        .map(|i| {
            if *data.get(&i).unwrap() <= *data.get(&(i - 1)).unwrap() {
                (i - 1, false)
            } else {
                (i - 1, true)
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
    let estadistico_tabla: f64 = if alpha > 0.0 {
        normal.inverse_cdf(1.0 - alpha / 2.0)
    } else {
        0.0
    };

    let normal_inv_value: BTreeMap<usize, (f64, f64)> = {
        let mut i = 0.0001;
        let mut count: usize = 0;
        let mut nomr_value: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
        while i < 1.0 {
            let alp = 1.0 - i;
            let inv_cdf = normal.inverse_cdf(1.0 - alp / 2.0);
            nomr_value.insert(count, (i, inv_cdf));
            count += 1;
            i += 0.01;
        }
        nomr_value
    };
    ResponseTable {
        mu,
        sigma,
        estadistico,
        estadistico_tabla,
        corridas,
        normal_inv_value,
    }
}
