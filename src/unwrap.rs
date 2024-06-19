pub fn unwrap_demo(){
    let some_value: Option<i32>=Some(10);
    let none_value: Option<i32>=None;

    let value=some_value.unwrap();
    println!("The value is: {}",value);
}