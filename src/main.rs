use std::collections::HashMap;

fn main() {
    let arr = [1, 1, 2, 10, 3, 3, 3, 66, 0, 2, 3, 23, 2, 3];

    let mut counts = HashMap::new();

    for num in arr {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }

    let mut sorted_counts = counts.drain().collect::<Vec<_>>();
    sorted_counts.sort_by_key(|&(key, _)| key);

    for (num, count) in sorted_counts {
        println!("Number {}: {}", num, count);
    }
}
