use std::collections::HashMap;
use rand::Rng;


fn main() {
    let mut list: Vec<i64> = Vec::new();
    let mut mode_list: HashMap<i64, i64> = HashMap::new();
    let mut average: i64 = 0;

    println!("1");

    //Let's generate a list of 100 random i32 integers
    for _i in 0..100 {
        let random_number = rand::thread_rng().gen_range(-200, 200);
        list.push(random_number)
    }

    println!("2");

    //mean value (average)
    for i in &list {
        println!("{}", i);
        average += i;
    }

    println!("3");

    average = average/100;
    println!("The average is: {}", average);

    //median (sorted middle value)
    list.sort();
    let &median = &list[49];
    println!("The median is: {}", &median);

    //mode (most occuring value)
    for v in &list {
        let count = mode_list.entry(*v).or_insert(0);
        *count += 1;
    }

    let mut most_occuring: i64 = match mode_list.get(&list[0]) {
        Some(value) => *value, //dereferencing
        None => 0,
    };

    for (v, _o) in mode_list {
        if v > most_occuring { most_occuring = v; }
    }

    println!("The most occuring value is: {}", most_occuring);

}
