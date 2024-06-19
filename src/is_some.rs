pub fn is_some_demo() {
    let some_value: Option<i32> = Some(10);
    let none_value: Option<i32> = None;

    println!("some_value is_some: {}", some_value.is_some());
    println!("none_value is_some: {}", none_value.is_some());
}
