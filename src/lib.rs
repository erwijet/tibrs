mod models;
pub use self::models::*;

mod parse;
pub use self::parse::*;

mod compile;
pub use self::compile::*;

pub fn say_hi() {
    println!("hello, world!");
}
