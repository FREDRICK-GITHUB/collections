use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    //accessing value whose key is supplied
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score for team blue is: {}", score);

    //how to iterate through a Hashmap
    for (key, value) in &scores {
        println!("KEY: {}, VALUE: {}", key, value);
    }

    //ownership in hashmaps
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("The field name: {} and field_value: {} are now owned by the hasmap", field_name, field_value);

    //updating a hashmap
    //updating by overwriting a value
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Blue"),33);

    let correct_score = String::from("Blue");
    println!("The value in the hashmap is: {}", scores1.get( &correct_score ).copied().unwrap_or(0));

    //adding a key and a value only if a key does not exist using entry API
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 13);

    scores2.entry(String::from("Yellow")).or_insert(25);
    scores2.entry(String::from("Blue")).or_insert(65);

    let score2_value = String::from("Blue");
    for (key, value) in &scores2 {
        println!("The values in hashmap are: key-{} value-{}", key, value);
    }


    //updating value based on the old value

    let text = "hello world beautiful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    


}
