#[allow(unused_imports)]
use crate::third::say_hello as say_hello_third;

#[allow(dead_code)]
pub fn say_hello() {
    println!("Hello from first!");

    say_hello_third();
}

#[allow(dead_code)]
mod second{
    pub mod third{
        pub fn say_hello(){
            super::super::say_hello();
        }
    }
}
