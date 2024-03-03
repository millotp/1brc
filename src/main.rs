use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Measurement {
    min_t: f64,
    max_t: f64,
    avg: f64,
    count: u64,
}

fn main() {
    let file = File::open("measurements_small.txt").unwrap();
    let reader = BufReader::new(file);

    let mut measurements = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (name, temp) = line.split_once(";").unwrap();
        let value: f64 = temp.parse().unwrap();
        if measurements.contains_key(name) {
            let measurement: &mut Measurement = measurements.get_mut(name).unwrap();
            measurement.min_t = measurement.min_t.min(value);
            measurement.max_t = measurement.max_t.max(value);
            measurement.avg += value;
            measurement.count += 1;
        } else {
            measurements.insert(
                name.clone(),
                Measurement {
                    min_t: value,
                    max_t: value,
                    avg: value,
                    count: 1,
                },
            );
        }
    }

    for (name, measurement) in measurements.iter() {
        println!(
            "{}: min: {}, max: {}, avg: {}",
            name,
            measurement.min_t,
            measurement.max_t,
            measurement.avg / measurement.count as f64
        );
    }
}
