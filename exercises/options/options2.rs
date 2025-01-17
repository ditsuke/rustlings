// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

fn main() {
    let optional_word = Some(String::from("rustlings"));
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // You can stack `Option<T>`'s into while let and if let
    while let Some(maybe_integer) = optional_integers_vec.pop() {
        if let Some(integer) = maybe_integer {
            println!("current value: {}", integer);
        } else {
            println!("no value :'(");
        }
    }
}
