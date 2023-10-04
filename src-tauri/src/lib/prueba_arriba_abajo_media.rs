use std::collections::BTreeMap;

use rug::{ops::Pow, Float, Integer};
use statrs::distribution::Normal;

struct ResponseTable {
    mu: Float,
    gamma_value: Float,
    estadistico: Float,
    estadistico_tabla_s: Float,
    estadistico_tabla_i: Float,
    corridas: Integer,
}

#[tauri::command(rename_all = "snake_case")]
pub fn prueba_arriba_abajo_media(data: BTreeMap<usize, Float>, nivel_confianza: f64) {
    let data_sequence: BTreeMap<usize, Float> = (0..data.len())
        .map(|i| if data[&i] >= 0.5 { (i, 1) } else { (i, 0) })
        .collect();
    let corridas: Integer = 1;
    let n0: Float = if (data_sequence[&0] == 0) { 1 } else { 0 };
    let n1: Float = if (data_sequence[&0] == 1) { 1 } else { 0 };
    for i in 1..data_sequence.len() {
        if data_sequence[&i] != data_sequence[&i - 1] {
            corridas += 1;
        }
        if data_sequence[&i] == 1 {
            n1 += 1;
        } else if data_sequence[&i] == 0 {
            n0 += 1
        }
    }

    let n: Integer = data.len();
    let mu: Float = (2 * n0 * n1 / n) - 0.5;
    let gamma_value: Float = (2 * n0 * n1 * (2 * n0 * n1 - n)) / (n.pow(2) * (n - 1));
    let estadistico: Float = (corridas - mu) / gamma_value.pow(0.5);

    let normal = Normal::new(0.0, 1.0).unwrap();
    let alpha = (1.0 - nivel_confianza);
    let estadistico_tabla_s: Float = normal.inverse_cdf(alpha / 2.0);
    let estadistico_tabla_i: Float = -1.0 * normal.inverse_cdf(alpha / 2.0);

    ResponseTable {
        mu,
        gamma_value,
        estadistico,
        estadistico_tabla_s,
        estadistico_tabla_i,
        corridas,
    }
}
