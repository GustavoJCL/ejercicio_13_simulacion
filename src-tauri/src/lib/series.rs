use std::collections::BTreeMap;

use rug::{Float, Integer};
use statrs::distribution::ChiSquared;

struct ResponseTable {
    frecuencia_observada: BTreeMap<usize, Integer>,
    frecuencia_esperada: BTreeMap<usize, Float>,
    estadistico: BTreeMap<usize, Float>,
    m: Integer,
    total_estadistico: Float,
    total_estadistico_tabla: Float,
}
#[tauri::command(rename_all = "snake_case")]
pub fn series(data: BTreeMap<usize, Float>, nivel_confianza: f64) -> ResponseTable {
    let m: Integer = (data.len() + 1).pow(0.5);
    let ordered_pair: BTreeMap<usize, (Float, Float)> = BTreeMap::new();
    for i in 1..data.len() {
        ordered_pair.insert(i - 1, (data[&i - 1], data[&i]));
    }
    let box_division: Float = 1 / m;
    let num_division: BTreeMap<usize, (Float, Float)> = BTreeMap::new();
    let count: Integer = 0;
    for i in (0..1).step_by(box_division as usize) {
        for j in (0..1).step_by(box_division as usize) {
            num_division.insert(count, (i, j));
            count += 1;
        }
    }

    let frecuencia_observada: BTreeMap<usize, Integer> = BTreeMap::new();
    count = 0;
    for i in 0..num_division.len() {
        for j in 0..num_division.len() {
            let count_frec: Integer = 0;
            for k in 0..ordered_pair.len() {
                if ordered_pair[&k].1 .0 <= num_division[&i].1 .0
                    && ordered_pair[&k].1 .1 <= num_division[&j].1 .1
                {
                    count_frec += 1;
                }
            }
            frecuencia_observada.insert(count, count_frec);
            count += 1;
        }
    }
    let frecuencia_esperada_value: Float = (data.len() - 1) / m;
    let frecuencia_esperada: BTreeMap<usize, Float> = (0..frecuencia_observada.len())
        .map(|i| (i, frecuencia_esperada_value))
        .collect();
    //     for i in 0..frecuencia_observada.len() {
    //     frecuencia_esperada.insert(i, frecuencia_esperada_value);
    // }
    let total_estadistico: Float = 0;
    let estadistico: BTreeMap<usize, Float> = frecuencia_observada
        .iter()
        .map(|(&key, &value)| {
            let est: Float =
                (frecuencia_esperada_value - value as Float).pow(2) / frecuencia_esperada_value;
            total_estadistico += est;
            (key, est)
        })
        .collect();

    let chi_square = ChiSquared::new(m - 1).unwrap();

    let total_estadistico_tabla = chi_square.pdf(1 - nivel_confianza) as Float;

    ResponseTable {
        frecuencia_observada,
        frecuencia_esperada,
        estadistico,
        m,
        total_estadistico,
        total_estadistico_tabla,
    }
}
