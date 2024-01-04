use std::collections::HashMap;

pub fn get_num(word: &str, kv: &Vec<&str>, numbers: &HashMap<&str, &str>) -> i32 {
    let f = get_first_digit(word, "0", kv, numbers);
    let l = get_last_digit(word, "0", kv, numbers);
    let num = format!("{}{}", f, l);
    let num = num.parse::<i32>().expect("???");
    return num;
}

fn get_first_digit(
    word: &str,
    digit: &str,
    kv: &Vec<&str>,
    numbers: &HashMap<&str, &str>,
) -> String {
    let mut w: String = word.to_owned();
    w.push_str(digit);

    for n in kv {
        if &n == &&digit {
            continue;
        }

        let splitted: Vec<&str> = w.split(n).collect();

        if splitted.len() > 1 {
            return get_first_digit(&splitted.first().expect("Couldn't find last part of the string"), n, kv, numbers);
        }
    }

    let mut d = digit;

    if numbers.get(&digit).is_some() {
        d = numbers.get(&digit).expect("???");
    }

    return d.to_string();
}

fn get_last_digit(
    word: &str,
    digit: &str,
    kv: &Vec<&str>,
    numbers: &HashMap<&str, &str>,
) -> String {
    let mut w: String = digit.to_owned();
    w.push_str(word);

    for n in kv {
        if &n == &&digit {
            continue;
        }

        let splitted: Vec<&str> = w.split(n).collect();

        if splitted.len() > 1 {
            return get_last_digit(&splitted.last().expect("Couldn't find last part of the string"), n, kv, numbers);
        }
    }

    let mut d = digit;

    if numbers.get(digit).is_some() {
        d = numbers.get(digit).expect("???");
    }

    return d.to_string();
}
