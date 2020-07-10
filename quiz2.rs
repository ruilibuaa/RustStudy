// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    //string("blue"); //error
    //String_slice("red".to_string()); //error
    string("red".to_string());
    //string_slice(String::from("hi"));  //error
    string(String::from("hi"));
    //string_slice("rust is fun!".to_owned());  //error
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string("nice weather".into());
    //string_slice(format!("Interpolation {}", "Station"));  //error
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    //string(&String::from("abc")[0..1]);   //error
    string_slice("  hello there ".trim());
    //string("  hello there ".trim());  //error
    //string_slice("Happy Monday!".to_string().replace("Mon", "Tues"));  //error
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    //string_slice("mY sHiFt KeY iS sTiCkY".to_lowercase());  //error
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
