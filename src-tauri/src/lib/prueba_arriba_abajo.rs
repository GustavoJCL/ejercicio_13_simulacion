use rug::{ops::Pow, Float, Integer};
use statrs::distribution::{ContinuousCDF, Normal};

struct ResponseTable {
    mu: Float,
    gamma: Float,
    estadistico: Float,
    estadistico_tabla: Float,
    corridas: Integer,
}
#[tauri::command(rename_all = "snake_case")]
pub fn prueba_arriba_abajo(data: BTreeMap<usize, bool>, nivel_confianza: f64) -> ResponseTable {
    let data_sequence: BTreeMap<usize, Float> = (1..data.len())
        .map(|i| {
            if data[&i] <= data[&i - 1] {
                (i, 0)
            } else {
                (i, 1)
            }
        })
        .collect();
    let corridas: Integer = 1;
    for i in 1..data_sequence.len() {
        if data_sequence[&i] != data_sequence[&i - 1] {
            corridas += 1;
        }
    }

    let mu: Float = (2 * n - 1) / 3;
    let sigma: Float = ((16 * n - 29) / 90);
    let estadistico: Float = ((corridas - mu) / sigma.pow(0.5)).abs();

    let normal = Normal::new(0.0, 1.0).unwrap();
    let alpha = (1.0 - nivel_confianza);
    let estadistico_tabla: Float = normal.inverse_cdf(alpha / 2.0);
    ResponseTable {
        mu,
        gamma,
        estadistico,
        estadistico_tabla,
        corridas,
    }
}
