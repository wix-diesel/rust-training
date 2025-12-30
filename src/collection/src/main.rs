fn main() {
    learn_vectors();
    learn_hashmap();
}

fn learn_vectors() {    
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    println!("The third element is {:?}", third);

    for i in &v {
        println!("{}", i);
    }
}

fn learn_hashmap(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score for {}: {:?}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);
}