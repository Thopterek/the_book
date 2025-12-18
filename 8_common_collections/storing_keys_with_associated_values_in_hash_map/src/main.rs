// type of HashMap<K, V>
use std::collections::HashMap;

fn main() {
    println!("Hello, Hash Maps!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    let blue_score = scores.get(&"Blue".to_string()).copied().unwrap_or(0);
    println!("Blue team score is -> {blue_score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // for non copy trait types the values will be moved
    let another_team = String::from("Yellow");
    scores.insert(another_team, 0);
    let yellow_score = scores.get(&"Yellow".to_string()).copied().unwrap_or(0);
    println!("Yellow team score is -> {yellow_score}");
    // then overwriting
    scores.insert(String::from("Yellow"), 12);
    let yellow_score = scores.get(&"Yellow".to_string()).copied().unwrap_or(0);
    println!("Yellow team overwritten score is -> {yellow_score}");
    {
        // adding key & value only if key isn't present
        println!("{scores:?}");
        println!("trying to add Yellow with entry");
        scores.entry(String::from("Yellow")).or_insert(50);
        println!("{scores:?}");
        println!("and now Brown");
        scores.entry(String::from("Brown")).or_insert(50);
        println!("{scores:?}");
    }
    // updating based on the old value
    let text = "hello world I repeat hello world hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns &mut V
        let count = map.entry(word).or_insert(0);
        // dereference to assing
        *count += 1;
        // count getting dropped here
    }
    println!("{map:?}");
    // hash maps use SipHash as a resistence to DoS
    // another crates can have different hashers
}
