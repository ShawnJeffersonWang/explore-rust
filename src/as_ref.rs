pub fn as_ref() {
    let some_value: Option<String> = Some(String::from("hello"));

    if let Some(ref_value) = some_value.as_ref() {
        println!("The value is: {}", ref_value);
    }

    println!("Original value: {:?}", some_value);
}
