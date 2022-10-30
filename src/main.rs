fn main() {
    //iterating over an immutable vector
    let v0 = vec![100, 73, 32];

    for i in &v0 {
     println!("{}",&i);
    }

    //iterating over a mutable vector
    let mut v1 = vec![100, 73, 32];
    for i in &mut v1 {
        *i += 10;
        println!("{}",&i);
    }

    /* a vector only stores values of the same type. in case we need to store values of different types,
    we have to use an enum. the enum type will be taken as the value type in the vector as shown below */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(10.12), SpreadsheetCell::Text(String::from("blue"))];

    println!("the contents of the vector that uses an enum to cover diferent data type values are: {:?}", &row);

}
