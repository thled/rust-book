use std::collections::HashMap;

fn main() {
    let integers = vec![38, 3, 32, 10, 38];
    let sum: usize = integers.iter().sum();
    let count = integers.len();
    let mean = sum / count;
    println!("Mean: {}", mean);

    let mut integers = vec![38, 3, 32, 10, 38];
    integers.sort_unstable();
    let middle = integers.len() / 2;
    let median = integers[middle];
    println!("Median: {}", median);

    let integers = vec![38, 3, 32, 10, 38];
    let mut map = HashMap::new();
    for i in &integers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut highest_count = 0;
    for (k, &v) in map.iter() {
        if v > highest_count {
            highest_count = v;
            mode = **k;
        }
    }
    println!("Mode: {}", mode);
}
