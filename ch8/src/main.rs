use std::collections::HashMap;

fn main() {
    ex1(&mut vec![1, 2, 3, 4, 2]);
    ex2(&mut String::from("apple"));
}

fn ex1(arr: &mut Vec<i32>) {
    let mut map = HashMap::new();
    arr.sort();

    let size = arr.len();
    let mut avg: f64 = 0.0;
    for i in arr.iter() {
        avg += *i as f64 / size as f64;
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut most_recent = 0;
    let mut most = 0;
    for i in map.iter() {
        if most < *i.1 {
            most = *i.1;
            most_recent = **i.0;
        }
    }

    println!("평균: {}", avg);
    println!("중앙값: {}", arr[size / 2]);
    println!("최빈값: {}", most_recent);
}

fn ex2(s: &mut String) {
    let mut first: char = '\0';
    for c in s.chars() {
        if first == '\0' {
            if "ayuio".contains(c) {
                print!("{}", c);
            }
            first = c;
        } else {
            print!("{}", c);
        }
    }
    if "ayuio".contains(first) {
        println!("-hay");
    } else {
        println!("-{}ay", first);
    }
}
