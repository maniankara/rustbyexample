use std::mem;
use std::string;

fn main() {

    let str: & 'static str = "This is a simple static string with long text in it";
    let string = "This is a simple static string with long text in it";


    println!("Size of str is: {}", mem::size_of_val(str)); // always 16 with & and actual 51
    println!("Size of str is: {}", mem::size_of_val(string)); // always 16 with & and actual 51



}