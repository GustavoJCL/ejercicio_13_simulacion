use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use statrs::distribution::{ChiSquared, ContinuousCDF};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTable {
    frecuencia_observada: BTreeMap<usize, i64>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    estadistico: BTreeMap<usize, f64>,
    m: i64,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
}
#[tauri::command(rename_all = "snake_case")]
pub fn series(data: BTreeMap<usize, f64>, nivel_confianza: f64) -> ResponseTable {
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
        while i < 1.0 {
            let mut j = 0.0;
            while j < 1.0 {
                num_division.insert(count, (i, j));
                count += 1;
                j += box_division
            }
            i += box_division
        }
        num_division.insert(count, (1.0, 1.0));
    }
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
        for i in 0..num_division_len {
            let i_value = num_division[&i];
            let i_minus_1_value = num_division.get(&(i - 1));

            for j in 0..num_division_len {
                let j_value = num_division[&j];
                let j_minus_1_value = num_division.get(&(j - 1));

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
    }
    // for i in 0..num_division.len() {
    //     for j in 0..num_division.len() {
    //         let count_frec: i64 = 0;
    //         for k in 0..ordered_pair.len() {
    //             if ordered_pair[&k].0 <= num_division[&i].0
    //                 && ordered_pair[&k].1 <= num_division[&j].1
    //                 && Some(num_division[&(i - 1)])
    //                 && Some(num_division[&(j - 1)])
    //                 && num_division[&(i - 1)].0 < ordered_pair[&k].0
    //                 && num_division[&(j - 1)].0 < ordered_pair[&k].0
    //             {
    //                 count_frec += 1;
    //             }
    //             // if ordered_pair[&k].1 <= num_division[&i].1
    //             //     && ordered_pair[&k].1 <= num_division[&j].1
    //             // {
    //             //     count_frec += 1;
    //             // }
    //         }
    //         frecuencia_observada.insert(count, count_frec);
    //         count += 1;
    //     }
    // }
    let frecuencia_esperada_value: f64 = (data.len() as f64 - 1.0) / m as f64;
    let frecuencia_esperada: BTreeMap<usize, f64> = (0..frecuencia_observada.len())
        .map(|i| (i, frecuencia_esperada_value))
        .collect();
    //     for i in 0..frecuencia_observada.len() {
    //     frecuencia_esperada.insert(i, frecuencia_esperada_value);
    // }
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

    let degrees_of_freedom: f64 = m as f64 - 1.0;
    let chi_square = ChiSquared::new(degrees_of_freedom).unwrap();

    let total_estadistico_tabla = chi_square.inverse_cdf(1.0 - nivel_confianza);

    ResponseTable {
        frecuencia_observada,
        frecuencia_esperada,
        estadistico,
        m,
        total_estadistico,
        total_estadistico_tabla,
    }
}
