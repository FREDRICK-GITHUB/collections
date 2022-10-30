fn main() {
    //declaration of a vector to hold data of a spcific type
    let v0: Vec<i32> = Vec::new();

    //creating a vector without specifying the data type
    let mut v1 = vec![1, 2, 3];

    println!("the first vector is: {:?}", &v0);

    println!("our second vector has the following data: {:?}", &v1);
}
