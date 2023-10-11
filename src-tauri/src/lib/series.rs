use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use statrs::distribution::{ChiSquared, ContinuousCDF};

use super::csv_to_btreemap::csv_to_data;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTable {
    frecuencia_observada: BTreeMap<usize, i64>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    estadistico: BTreeMap<usize, f64>,
    m: i64,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
    chi_square_values: BTreeMap<usize, (f64, f64)>,
}
#[tauri::command(rename_all = "snake_case")]
pub fn series(data_string: String, nivel_confianza: f64) -> ResponseTable {
    let data: BTreeMap<usize, f64> = csv_to_data(data_string);
    let m: i64 = (data.len() as f64 + 1_f64).powf(0.5) as i64;
    let mut ordered_pair: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    for i in 1..data.len() {
        ordered_pair.insert(i - 1, (data[&(i - 1)], data[&i]));
    }
    let box_division: f64 = 1.0 / m as f64;
    let mut num_division: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    let mut count: usize = 0;
    {
        let mut i = 0.0;
        while i <= 1.0 {
            let mut j = 0.0;
            while j <= 1.0 {
                num_division.insert(count, (i, j));
                count += 1;
                j += box_division
            }
            i += box_division
        }
        // num_division.insert(count, (1.0, 1.0));
    }
    println!("ordered_pair: {:#?}", ordered_pair);
    println!("m: {:#?}", m);
    // for i in (0..1).step_by(box_division) {
    //     for j in (0..1).step_by(box_division) {
    //         num_division.insert(count, (i, j));
    //         count += 1;
    //     }
    // }

    let mut frecuencia_observada: BTreeMap<usize, i64> = BTreeMap::new();
    count = 0;
    {
        let num_division_len = num_division.len();
        for i in 1..num_division_len {
            let i_value = num_division[&i];
            let i_minus_1_value = num_division.get(&(i - 1));
            // } else {
            //     Some(&(0.0, 0.0))
            // };

            for j in 1..num_division_len {
                let j_value = num_division[&j];
                let j_minus_1_value = num_division.get(&(j - 1));
                // } else {
                //     Some(&(0.0, 0.0))
                // };

                let mut count_frec: i64 = 0;
                for k in 0..ordered_pair.len() {
                    if ordered_pair[&k].0 <= i_value.0
                        && ordered_pair[&k].1 <= j_value.1
                        && i_minus_1_value.is_some()
                        && j_minus_1_value.is_some()
                        && i_minus_1_value.unwrap().0 < ordered_pair[&k].0
                        && j_minus_1_value.unwrap().1 < ordered_pair[&k].1
                    {
                        count_frec += 1;
                    }
                    // ...
                }
                frecuencia_observada.insert(count, count_frec);
                count += 1;
            }
        }

        let max_index = frecuencia_observada.keys().max().cloned().unwrap_or(0);
        for i in 0..=max_index {
            frecuencia_observada.entry(i).or_insert(0);
        }
    }

    let frecuencia_esperada_value: f64 =
        (data.len() as f64 - 1.0) / frecuencia_observada.len() as f64;
    let frecuencia_esperada: BTreeMap<usize, f64> = (0..frecuencia_observada.len())
        .map(|i| (i, frecuencia_esperada_value))
        .collect();
    let mut total_estadistico: f64 = 0.0;
    let estadistico: BTreeMap<usize, f64> = frecuencia_observada
        .iter()
        .map(|(&key, &value)| {
            let est: f64 =
                (frecuencia_esperada_value - value as f64).powf(2.0) / frecuencia_esperada_value;
            total_estadistico += est;
            (key, est)
        })
        .collect();

    println!("num div: {:#?}", num_division);
    // println!("frecuencia_observada: {:#?}", frecuencia_observada);
    // println!("frecuencia_esperada: {:#?}", frecuencia_esperada);
    // println!("estadistico: {:#?}", estadistico);
    let degrees_of_freedom: f64 = frecuencia_observada.len() as f64 - 1.0;
    let mut total_estadistico_tabla = 0.0;
    let chi_square: ChiSquared;
    let mut chi_square_values: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    if degrees_of_freedom > 0.0 {
        chi_square = ChiSquared::new(degrees_of_freedom).unwrap();
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
    // println!("chi_square_values: {:#?}", chi_square_values);
    // println!("total_estadistico_tabla: {:#?}", total_estadistico_tabla);

    ResponseTable {
        frecuencia_observada,
        frecuencia_esperada,
        estadistico,
        m,
        total_estadistico,
        total_estadistico_tabla,
        chi_square_values,
    }
}
