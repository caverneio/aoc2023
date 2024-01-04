use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod get_num;

fn main() -> io::Result<()> {
    let mut numbers = HashMap::new();
    numbers.insert("one", "1");
    numbers.insert("two", "2");
    numbers.insert("three", "3");
    numbers.insert("four", "4");
    numbers.insert("five", "5");
    numbers.insert("six", "6");
    numbers.insert("seven", "7");
    numbers.insert("eight", "8");
    numbers.insert("nine", "9");

    let keys: Vec<_> = numbers.keys().cloned().collect();
    let values: Vec<_> = numbers.values().cloned().collect();
    let kv: Vec<&str> = [keys, values].concat();

    println!("TEST");

    let file = File::open("data/main.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let word = line?.to_owned();
        let num = get_num::get_num(&word, &kv, &numbers);
        count += num;
        println!("word: {}, number: {}", word, num);
    }

    println!("count: {}", count);

    Ok(())
}
