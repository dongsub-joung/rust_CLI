mod service;
use std::io::{self, BufRead, Stdin, stdin};

pub fn run(){
    let std= stdin();
    let mut inputing= std.lock().lines();
    let str_v: Vec<&str>= inputing.next().unwrap().unwrap().split_whitespace().map(|f| f).collect();
    

}