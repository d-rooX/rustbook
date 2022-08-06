use std::collections::HashMap;

fn main() {
    let mut data: Vec<i32> = vec![1, 1, 10, 23, 22, 22, 2, 1, 4, 5, 6, 7, 10, 10, 10, 10, 22, 23, 25, 60];
    data.sort();

    println!("Vector: {:?}", data);
    println!("Average: {}", average(&data));
    println!("Median: {}", median(&data));
    println!("Most common: {}", most_common(&data));
}


fn average(v: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum as f32 / v.len() as f32
}

fn median(v: &Vec<i32>) -> i32 {
    v[v.len() / 2]
}

fn most_common(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max_key = 0;
    let mut max_count = 0;
    for &i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_key = i;
        }
    };
    max_key
}
