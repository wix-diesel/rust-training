fn main() {
    function44();
    println!("---");
    function45();
    println!("---");
    function49();
}


fn function44(){
    let s = String::from("hello");

    take_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn function45(){
    let s1 = give_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn function49(){
    let mys_string = String::from("hello world");

    let word = first_word(&mys_string[..]);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);
}