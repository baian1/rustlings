// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

fn main() {
    //宏需要先定义
    //再使用
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    my_macro!();
}
