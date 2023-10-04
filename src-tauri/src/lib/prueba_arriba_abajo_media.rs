use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct ResponseTable {
    mu: f64,
    gamma_value: f64,
    estadistico: f64,
    estadistico_tabla_s: f64,
    estadistico_tabla_i: f64,
    corridas: i64,
}

#[tauri::command(rename_all = "snake_case")]
pub fn prueba_arriba_abajo_media(
    data: BTreeMap<usize, f64>,
    nivel_confianza: f64,
) -> ResponseTable {
    let data_sequence: BTreeMap<usize, bool> = (0..data.len())
        .map(|i| {
            if *data.get(&i).unwrap() >= 0.5 {
                (i, true)
            } else {
                (i, false)
            }
        })
        .collect();
    let mut corridas: i64 = 1;

    let mut n0: i64 = if data_sequence.get(&0).is_some() && *data_sequence.get(&0).unwrap() == false
    {
        1
    } else {
        0
    };
    let mut n1: i64 = if data_sequence.get(&0).is_some() && *data_sequence.get(&0).unwrap() == true
    {
        1
    } else {
        0
    };

    for i in 1..data_sequence.len() {
        if data_sequence[&i] != data_sequence[&(i - 1)] {
            corridas += 1;
        }
        if data_sequence[&i] == true {
            n1 += 1;
        } else if data_sequence[&i] == false {
            n0 += 1
        }
    }

    let n: i64 = data.len() as i64;
    let mu: f64 = (2 * n0 * n1 / n) as f64 - 0.5;
    let gamma_value: f64 =
        (2 * n0 * n1 * (2 * n0 * n1 - n)) as f64 / ((n.pow(2)) as f64 * (n - 1) as f64);
    let estadistico: f64 = (corridas as f64 - mu) / gamma_value.powf(0.5);

    let normal = Normal::new(0.0, 1.0).unwrap();
    let alpha = 1.0 - nivel_confianza;
    let estadistico_tabla_s: f64 = normal.inverse_cdf(alpha / 2.0);
    let estadistico_tabla_i: f64 = -1.0 * normal.inverse_cdf(alpha / 2.0);

    ResponseTable {
        mu,
        gamma_value,
        estadistico,
        estadistico_tabla_s,
        estadistico_tabla_i,
        corridas,
    }
}
