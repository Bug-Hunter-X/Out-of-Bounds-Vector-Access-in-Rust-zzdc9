fn main() {    let mut vec = Vec::new();    vec.push(1);    vec.push(2);    vec.push(3);    let index = 10;    // This will panic at runtime because index is out of bounds    let element = vec.get(index).unwrap();    println!("Element at index {} is {}", index, element);}