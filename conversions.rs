fn main() {
    
    //Example 1
    let text = "this is a test &string";

    let _text2 = String::from(text);
    let _text3 = text.to_string();
    let _text4 = text.to_owned();

    //Example 2
    let _bytes: Vec<u8> = text.into();

    let string = "Hello".to_string();
    string.as_bytes(); //(string to vec[u8])

    //Example 3
    let buffer = vec![1, 2, 3, 5, 8];
    buffer.as_slice(); //equivalent to  &buffer[..]

    //Example 4
    "44".to_string().as_bytes();
    let s: Box<[i32]> = Box::new([10, 40, 30]);
    let _x = s.into_vec();
    //Converts self into a vector without clones or allocation.
    //The resulting vector can be converted back into a box via Vec<T>'s into_boxed_slice method.

    //Example 5
    let s = [10, 40, 30];
    let _x = s.to_vec();
    // Here, `s` and `_x` can be modified independently.

    ////Example 6
    let _string_number = 47.8.to_string();

    //Example 7
    //Converts text string to a raw number

    match "47.8".parse::<f64>() {
        Ok(number) => println!("Number {:?}", number + 45.0),
        Err(_) => (),
    }

    //Example 8
    //Converts a vector to a String

    String::from_utf8_lossy(&buffer);

    let _message = vec!["a", "b", "c"];
    let result: String = String::from_utf8_lossy(&buffer).into();
    let _result: &str = &result[..];
}
