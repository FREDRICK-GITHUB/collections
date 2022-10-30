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

}
