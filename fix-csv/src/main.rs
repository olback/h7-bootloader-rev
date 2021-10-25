use std::fs;

fn main() {
    let data = fs::read_to_string("cap2.csv").unwrap();
    let new_data = data
        .split("\n")
        .skip(2)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts = line.split(",").skip(1).collect::<Vec<_>>();
            format!(
                "{},{}",
                if parts[0].parse::<f64>().unwrap() > 1.8 {
                    1
                } else {
                    0
                },
                if parts[1].parse::<f64>().unwrap() > 1.8 {
                    1
                } else {
                    0
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("cap2-fixed.csv", new_data).unwrap();
}
