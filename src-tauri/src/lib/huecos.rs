use serde::{Deserialize, Serialize};
use statrs::distribution::{ChiSquared, Continuous};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTable {
    frecuencia_observada: BTreeMap<usize, i128>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    estadistico: BTreeMap<usize, f64>,
    h: i128,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
}

#[tauri::command(rename_all = "snake_case")]
pub fn huecos(data: BTreeMap<usize, f64>, nivel_confianza: f64, a: f64, b: f64) -> ResponseTable {
    let binary_sequence: BTreeMap<usize, bool> = data
        .iter()
        .map(|(&key, &value)| {
            let new_value = if value >= a && value <= b {
                true
            } else {
                false
            };
            (key, new_value)
        })
        .collect();
    let mut count: u64 = 0;
    let mut zeros_count: u64 = 0;
    let mut ones_count: u64 = 0;
    let mut num_huecos: BTreeMap<usize, i128> = BTreeMap::new();
    for i in 0..binary_sequence.len() {
        if binary_sequence[&i] == false && ones_count > 0 {
            num_huecos.insert(count as usize, 0);
            count += 1;
            zeros_count = 0;
            for mut j in i..binary_sequence.len() {
                if binary_sequence[&j] == true {
                    num_huecos.insert(count as usize, zeros_count as i128);
                    count += 1;
                    j = binary_sequence.len();
                } else if binary_sequence[&j] == false {
                    zeros_count += 1;
                }
            }
        } else if binary_sequence[&i] == true {
            ones_count += 1;
        }
    }

    let frecuencia_observada: BTreeMap<usize, i128> =
        num_huecos
            .values()
            .fold(BTreeMap::new(), |mut acc, &value| {
                *acc.entry(value as usize).or_insert(0) += 1;
                acc
            });

    let h: i128 = num_huecos.iter().map(|(_, value)| value).sum();

    let frecuencia_esperada: BTreeMap<usize, f64> = num_huecos
        .iter()
        .map(|(key, _)| {
            (
                *key,
                (h as f64) * ((b - a) * (1.0 - b + a).powf(*key as f64)),
            )
        })
        .collect();
    let mut estadistico: BTreeMap<usize, f64> = BTreeMap::new();
    let mut total_estadistico: f64 = 0.0;

    for i in 0..num_huecos.len() {
        let estadistico_value = (frecuencia_observada[&i] as f64 - frecuencia_esperada[&i])
            .powf(2.0)
            / frecuencia_esperada[&i];
        total_estadistico += estadistico_value;
        estadistico.insert(i, estadistico_value);
    }

    let chi_square = ChiSquared::new(num_huecos.len() as f64).unwrap();

    let total_estadistico_tabla = chi_square.pdf(1.0 - nivel_confianza);

    ResponseTable {
        frecuencia_observada,
        frecuencia_esperada,
        estadistico,
        h,
        total_estadistico,
        total_estadistico_tabla,
    }
}
