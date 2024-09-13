fn main() {
    println!("Hello, world!");

    let array = [1, 2, 3, 4, 5];
    println!("Array length: {}", array.len());
    for i in array {
        println!("{}", i);
    }
    assert_eq!(array.len(), 5);
}