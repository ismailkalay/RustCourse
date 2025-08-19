#[allow(dead_code)]
#[allow(unused_variables)]
mod sh;

use std::mem;
const DATA:i8=123;
static mut M:i8=22; //static can be mutable.

fn aritmatik(){
    let mut a = 2+5+3;
    println!("result a: {}",a);
    a+=1; // a=a+1
    println!("mod result {} / {} = {}", a, 4, (a%4));

    let a_pow = i32::pow(a,2);
    println!("pow result:{}",a_pow);

    let b = 2.6;
    let b_cube = f64::powi(b,3);
    let b_cube1 = f64::powf(b, std::f64::consts::PI);
    println!("b_cube: {0} b_cube1: {1}",b_cube,b_cube1);   
}

fn bitwise_logic(){

    let c = 1|2; // | means or
    // & means and
    // ^ means XOR
    // 1 -> 01, 2 -> 10 
    println!("1|2 = {}",c);

    let pi_issmall = std::f64::consts::PI<4.0;
    println!("pi_issmall = {}",pi_issmall);  


}

fn main(){

    //aritmatik();
    //bitwise_logic();
    println!("DATA: {}", DATA);
    unsafe { println!("M: {}", M); }
    sh::stack_and_heap();
}