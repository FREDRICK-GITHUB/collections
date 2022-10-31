fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s1 = data.to_string();

    let s2 = "initial contents".to_string();

    println!("we have an empty string: {} ",s );

    //creating a string from a string literal
    let s3 = String::from("initial contents");

    //appending to a string using push_str
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("our new string is: {}", s4);

    //appending to a string using push. Push takes a single character and adds it to a string
    let mut s5 = String::from("lo");
    s5.push('l');
    println!("the string with a new character is: {}", s5);

    //concatenation using + operator
    let s6 = String::from("Hello ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7;
    println!("{}", s8);

    //concatenation using the format! Macro
    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = t1 + "-" + &t2 + "-" + &t3;
    println!("{}", t);


    let tt1 = String::from("tic");
    let tt2 = String::from("tac");
    let tt3 = String::from("toe");
    let tt4 = format!("{}-{}-{}", tt1, tt2, tt3);
    println!("{}", tt4);

    /* Indexing into Strings brings in issues especially for UTF-8 
    this is because a character in a string could be taking more than one byte. 
    This makes it hard to know where a character starts and where it ends */


}
