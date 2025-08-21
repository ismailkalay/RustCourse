#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn if_statement(){
    let sicaklik = 9;
    if sicaklik>35
    {
        println!("very hot.");
    }
    else if sicaklik<10{
        println!("cold");
    }

    let day = if sicaklik>20 {"sunny"} else {"cloudy"};
    println!("today {}",day)

    
}


fn main(){
    if_statement();



}