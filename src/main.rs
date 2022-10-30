fn main() {
    //declaration of a vector to hold data of a spcific type
    let v0: Vec<i32> = Vec::new();

    //creating a vector without specifying the data type
    let v1 = vec![1, 2, 3];

    println!("the first vector is: {:?}", &v0);

    println!("our second vector has the following data: {:?}", &v1);

    //updating a vector
    let mut v2 = vec![1, 2, 3];

    v2.push(4);
    v2.push(5);
    v2.push(6);

    println!("the updated vector has values: {:?}", v2);

    //reading values of a vector
    let v3 = vec![1, 2, 3, 4, 5];

    //first method using index only
    let third: &i32 = &v3[2];
    println!("the third element in the vector is: {}", third);

    //second method using get. this will not panic if we try accessing a value at an index that does not exist
    let third1:Option<&i32> = v3.get(2);
    match third1 {
        Some(third1) => println!("the third value accessed by get is: {}", third1),
        None => println!("no value found at index 3"),
    }


}
