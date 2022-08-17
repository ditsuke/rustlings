// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// either this **
#[macro_use]
mod macros {
    // or this **
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // or even this (latest thing?) **
    pub(crate) use my_macro;
}

fn main() {
    my_macro!();
}
