use serde::{Deserialize, Serialize};
use statrs::distribution::{ChiSquared, Continuous, ContinuousCDF};
use std::collections::BTreeMap;

use super::csv_to_btreemap::csv_to_data;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTable {
    frecuencia_observada: BTreeMap<usize, i128>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    estadistico: BTreeMap<usize, f64>,
    h: i128,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
    chi_square_values: BTreeMap<usize, (f64, f64)>,
}

#[tauri::command(rename_all = "snake_case")]
pub fn huecos(data_string: String, nivel_confianza: f64, a: f64, b: f64) -> ResponseTable {
    let data: BTreeMap<usize, f64> = csv_to_data(data_string);
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
    {
        let mut i = 0;
        while i < binary_sequence.len() {
            if binary_sequence[&i] == false && ones_count > 0 {
                if ones_count >= 2 {
                    num_huecos.insert(count as usize, 0);
                    ones_count = 0;
                    count += 1;
                }
                zeros_count = 0;
                let mut j = i;
                while j < binary_sequence.len() {
                    if binary_sequence[&j] == true {
                        num_huecos.insert(count as usize, zeros_count as i128);
                        count += 1;
                        i = j;
                        ones_count += 1;
                        break;
                    } else if binary_sequence[&j] == false {
                        zeros_count += 1;
                    }
                    j += 1;
                }
            } else if binary_sequence[&i] == true {
                ones_count += 1;
            }
            i += 1;
        }
    }
    // let frecuencia_observada: BTreeMap<usize, i128> =
    //     num_huecos
    //         .values()
    //         .fold(BTreeMap::new(), |mut acc, &value| {
    //             *acc.entry(value as usize).or_insert(0) += 1;
    //             acc
    //         });
    let frecuencia_observada: BTreeMap<usize, i128> = {
        let mut observada: BTreeMap<usize, i128> =
            num_huecos
                .values()
                .fold(BTreeMap::new(), |mut acc, &value| {
                    *acc.entry(value as usize).or_insert(0) += 1;
                    acc
                });
        let max_index = observada.keys().max().cloned().unwrap_or(0);
        for i in 0..=max_index {
            observada.entry(i).or_insert(0);
        }
        observada
    };

    let h: i128 = num_huecos.iter().map(|(_, value)| value).sum();

    let frecuencia_esperada: BTreeMap<usize, f64> = frecuencia_observada
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

    println!("{:#?}", binary_sequence);
    println!("{:#?}", num_huecos);
    println!("{:#?}", frecuencia_observada);
    println!("{:#?}", frecuencia_esperada);
    for i in 0..frecuencia_observada.len() {
        // println!("{}", frecuencia_observada[&i]);
        let frec_esp = frecuencia_esperada[&i];
        let frec_obs = frecuencia_observada[&i] as f64;
        // println!("{}, {}", frec_esp, frec_obs);
        let estadistico_value: f64 = (frec_esp - frec_obs).powf(2.0) / frec_esp;
        total_estadistico += estadistico_value;
        estadistico.insert(i, estadistico_value);
    }

    // println!("{}", num_huecos.len());
    let grados_de_libertad = frecuencia_observada.len() as f64 - 1.0;
    let mut total_estadistico_tabla = 0.0;
    let mut chi_square_values: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    if grados_de_libertad > 0.0 {
        let chi_square = ChiSquared::new(grados_de_libertad).unwrap();
        total_estadistico_tabla = chi_square.inverse_cdf(nivel_confianza);

        chi_square_values = {
            let mut i = 0.0001;
            let mut count: usize = 0;
            let mut chi_inv_value: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
            while i < nivel_confianza * 1.25 {
                let inv_cdf = chi_square.inverse_cdf(i);
                chi_inv_value.insert(count, (i, inv_cdf));
                count += 1;
                i += 0.01;
            }
            chi_inv_value
        };
    }

    ResponseTable {
        frecuencia_observada,
        frecuencia_esperada,
        estadistico,
        h,
        total_estadistico,
        total_estadistico_tabla,
        chi_square_values,
    }
}
