use std::collections::HashMap;

fn main() {
    let numbers = vec![59, 82, 64, 32, 54, 49, 18, 83, 81, 10, 80, 75, 65, 66, 8, 83];

    println!("{:?}", numbers);
    println!("mean: {}", mean(&numbers));
    println!("median: {}", median(&numbers));
    println!("mode: {:?}", mode(&numbers));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in numbers {
        sum += i;
    }
    sum / (numbers.len() as i32)
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut temp = numbers.clone();
    temp.sort();

    let index = temp.len() / 2;
    temp[index]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    for i in numbers {
        let count = hash.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut number: i32 = 0;

    for (num, count) in hash {
        if count > max_count {
            max_count = count;
            number = *num;
        }
    }

    number
}