#![allow(unused)]
use std::fmt::format;


pub fn hello() -> String {
    let hello = "Hello";
    let rust = "Rust";
    format!("{} {}",hello,rust)
}

pub fn greet(name: &str) -> String {
    format!("Hello {}",name)
}

pub fn append(mut s: String) -> String {
    s+="!";
    s
}
