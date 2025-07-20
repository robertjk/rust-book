fn main() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("v1: {:?}", v1);

    let second = &v1[1];
    println!("The second element in v1 is: {}", second);
    let third = v1.get(2);
    match third {
        Some(third) => println!("The third element in v1 is: {}", third),
        None => println!("There is no third element"),
    }

    let mut v2 = vec![1, 2, 3];

    println!("v2: {:?}", v2);

    for i in &mut v2 {
        *i += 10;
    }
    for i in &v2 {
        println!("{i}");
    }
}

fn strings() {
    let mut s1 = String::new();
    s1.push_str("initial contents");

    let s2_input = "initial contents";
    let s2 = s2_input.to_string();

    let mut s3 = String::from("initial contents");
    s3.push('s');

    let s4_first = String::from("initial");
    let s4_second = String::from("contents");
    let s4 = s4_first + " " + &s4_second;

    let s5_first = String::from("initial");
    let s5_second = String::from("contents");
    let s5 = format!("{s5_first} {s5_second}");

    let hello1 = String::from("Hola");
    let hello2 = String::from("Здравствуйте");
    println!("Length of '{}' is {}", hello1, hello1.len());
    println!("Length of '{}' is {}", hello2, hello2.len());

    for char in hello2.chars() {
        println!("{char}");
    }
}

fn hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // field_name;

    let text = "hello world wonderful world";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{word_count:?}");
}
