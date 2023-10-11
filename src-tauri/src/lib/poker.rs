use serde::{Deserialize, Serialize};
use statrs::distribution::ChiSquared;
use statrs::distribution::Continuous;
use statrs::distribution::ContinuousCDF;
use std::collections::{BTreeMap, HashMap};

use super::csv_to_btreemap::csv_to_data;
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTable {
    frecuencia_observada: BTreeMap<PokerTable, i128>,
    frecuencia_esperada: BTreeMap<PokerTable, f64>,
    estadistico: BTreeMap<usize, f64>,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
    chi_square_values: BTreeMap<usize, (f64, f64)>,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PokerTable {
    TD,
    P1,
    P2,
    TP,
    T,
    P,
    Q,
}
impl std::fmt::Display for PokerTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TD => write!(f, "TD"),
            Self::P1 => write!(f, "P1"),
            Self::P2 => write!(f, "P2"),
            Self::TP => write!(f, "TP"),
            Self::T => write!(f, "T"),
            Self::P => write!(f, "P"),
            Self::Q => write!(f, "Q"),
        }
    }
}
impl PokerTable {
    fn new() -> Self {
        Self::TD
    }
}

struct Poker3Decimales {
    td: f32,
    p1: f32,
    t: f32,
}
impl Poker3Decimales {
    fn new() -> Self {
        Poker3Decimales {
            td: 0.72,
            p1: 0.27,
            t: 0.01,
        }
    }
}
struct Poker4Decimales {
    td: f32,
    p1: f32,
    p2: f32,
    t: f32,
    p: f32,
}
impl Poker4Decimales {
    fn new() -> Self {
        Poker4Decimales {
            td: 0.5040,
            p1: 0.4320,
            p2: 0.0270,
            t: 0.0360,
            p: 0.0010,
        }
    }
}
struct Poker5Decimales {
    td: f32,
    p1: f32,
    p2: f32,
    tp: f32,
    t: f32,
    p: f32,
    q: f32,
}
impl Poker5Decimales {
    fn new() -> Self {
        Poker5Decimales {
            td: 0.3024,
            p1: 0.5040,
            p2: 0.1080,
            tp: 0.0090,
            t: 0.0720,
            p: 0.0045,
            q: 0.0001,
        }
    }
}
#[tauri::command(rename_all = "snake_case")]
pub fn poker(data_string: String, nivel_confianza: f64, cifras: i32) -> ResponseTable {
    // let poker_table: PokerTable;
    let data: BTreeMap<usize, f64> = csv_to_data(data_string);

    let poker_3_decimales = Poker3Decimales::new();
    let poker_4_decimales = Poker4Decimales::new();
    let poker_5_decimales = Poker5Decimales::new();

    let multiplier = 10_f64.powi(cifras as i32);
    let data_poker: BTreeMap<usize, PokerTable> = data
        .iter()
        .map(|(&key, &value)| {
            let mut poker_table: PokerTable = PokerTable::new();
            let truncated_value = (value * multiplier).trunc() / multiplier;

            let mut truncated_value_str = truncated_value
                .to_string()
                .trim_start_matches("0.")
                .to_string();
            if truncated_value_str.len() - 1 < cifras as usize {
                let zeros_to_add = cifras as usize - truncated_value_str.len();
                truncated_value_str.push_str(&"0".repeat(zeros_to_add));
            }
            let mut max_repeated_value = HashMap::new();
            for c in truncated_value_str.chars() {
                *max_repeated_value.entry(c).or_insert(0) += 1;
            }
            let mut ordered_max_repeated: Vec<(&char, &i128)> = max_repeated_value.iter().collect();
            ordered_max_repeated.sort_by(|a, b| b.1.cmp(a.1));
            // let repeated_value = max_repeated_value.iter().max().unwrap().1 as i32;
            if cifras == 3 {
                match ordered_max_repeated[0].1 {
                    1 => poker_table = PokerTable::TD,
                    2 => poker_table = PokerTable::P1,
                    3 => poker_table = PokerTable::T,
                    _ => poker_table = PokerTable::T,
                };
            } else if cifras == 4 {
                match *ordered_max_repeated[0].1 {
                    1 => poker_table = PokerTable::TD,
                    2 => {
                        if *ordered_max_repeated[0].1 == 2 {
                            poker_table = PokerTable::P2;
                        } else {
                            poker_table = PokerTable::P1;
                        }
                    }
                    3 => poker_table = PokerTable::T,
                    4 => poker_table = PokerTable::P,
                    _ => poker_table = PokerTable::P,
                }
            } else if cifras == 5 {
                match *ordered_max_repeated[0].1 {
                    1 => poker_table = PokerTable::TD,
                    2 => {
                        if *ordered_max_repeated[1].1 == 2 {
                            poker_table = PokerTable::P2;
                        } else {
                            poker_table = PokerTable::P1;
                        }
                    }
                    3 => {
                        if *ordered_max_repeated[1].1 == 2 {
                            poker_table = PokerTable::TP;
                        } else {
                            poker_table = PokerTable::T;
                        }
                    }
                    4 => poker_table = PokerTable::P,
                    5 => poker_table = PokerTable::Q,
                    _ => poker_table = PokerTable::Q,
                }
            }

            (key, poker_table)
        })
        .collect();
    let mut frecuencia_observada: BTreeMap<PokerTable, i128> = BTreeMap::new();
    for (&_key, value) in data_poker.iter() {
        frecuencia_observada.insert(
            value.clone(),
            frecuencia_observada.get(&value).unwrap_or(&0) + 1,
        );
        // frecuencia_observada.insert(
        //     key,
        //     frecuencia_observada.contains_key(&key).unwrap_or(&0) + 1,
        // );
    }
    let n: i128 = data.len() as i128 + 1;
    let frecuencia_esperada: BTreeMap<PokerTable, f64> = frecuencia_observada
        .iter()
        .map(|(key, _value)| {
            let mut frec_esp: f64 = 0.0;

            if cifras == 3 {
                frec_esp = match key {
                    PokerTable::TD => poker_3_decimales.td,
                    PokerTable::P1 => poker_3_decimales.p1,
                    PokerTable::T => poker_3_decimales.t,
                    _ => 0.0, // Provide a default value
                } as f64
                    * n as f64;
            } else if cifras == 4 {
                frec_esp = match key {
                    PokerTable::TD => poker_4_decimales.td,
                    PokerTable::P2 => poker_4_decimales.p2,
                    PokerTable::P1 => poker_4_decimales.p1,
                    PokerTable::T => poker_4_decimales.t,
                    _ => 0.0, // Provide a default value
                } as f64
                    * n as f64;
            } else if cifras == 5 {
                frec_esp = match key {
                    PokerTable::TD => poker_5_decimales.td,
                    PokerTable::P2 => poker_5_decimales.p2,
                    PokerTable::P1 => poker_5_decimales.p1,
                    PokerTable::TP => poker_5_decimales.tp,
                    PokerTable::T => poker_5_decimales.t,
                    PokerTable::P => poker_5_decimales.p,
                    PokerTable::Q => poker_5_decimales.q,
                    _ => 0.0, // Provide a default value
                } as f64
                    * n as f64;
            }
            (key.clone(), frec_esp)
        })
        .collect();
    let mut total_estadistico: f64 = 0.0;
    let mut key_estadistico: usize = 0;
    let estadistico: BTreeMap<usize, f64> = frecuencia_observada
        .iter()
        .map(|(key, _value)| {
            let frec_esperada = frecuencia_esperada.get(&key).unwrap();
            let frec_observada = frecuencia_observada.get(&key).unwrap();

            let est: f64 = (frec_esperada - *frec_observada as f64).powf(2.0) / frec_esperada;
            total_estadistico += est;
            key_estadistico += 1;
            (key_estadistico - 1, est)

            // match (frec_esperada, frec_observada) {
            //     (Some(&esperada), Some(&observada)) => {
            //         if esperada != 0.0 {
            //             let est: f64 = (esperada - observada as f64).powf(2.0) / esperada;
            //             total_estadistico += est;
            //             Some((key, est))
            //         } else {
            //             None
            //         }
            //     } // _ => None,
            // }
        })
        .collect();
    let grados_libertad = if cifras == 3 {
        2
    } else if cifras == 4 {
        4
    } else if cifras == 5 {
        6
    } else {
        0
    };
    let mut total_estadistico_tabla = 0.0;
    let mut chi_square_values: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    if grados_libertad > 0 {
        let chi_square = ChiSquared::new(grados_libertad as f64).unwrap();
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
        total_estadistico,
        total_estadistico_tabla,
        chi_square_values,
    }
}
