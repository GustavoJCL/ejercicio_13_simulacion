use std::collections::BTreeMap;

pub fn csv_to_data(csv_data: String) -> BTreeMap<usize, f64> {
    let mut map: BTreeMap<usize, f64> = BTreeMap::new();

    for line in csv_data.lines() {
        let line = line.trim_end_matches(','); // Remove trailing comma
        let parts: Vec<&str> = line.split(',').collect();
        if let (Some(index), Some(value)) = (parts.get(0), parts.last()) {
            if let (Ok(index), Ok(value)) = (index.parse::<usize>(), value.parse::<f64>()) {
                map.insert(index - 1, value);
            }
        }
    }

    // for (key, value) in &map {
    //     println!("{}: {}", key, value);
    // }
    // println!("{:#?}", map);
    //     println!("{} ", map.len());
    map
}
