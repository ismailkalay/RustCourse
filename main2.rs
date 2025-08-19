#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn veri(){
    let a = 55;
    println!("a: {}",a);
}

fn main(){

    let a:u8 = 125; // u = unsigned 0-255 8 bit
    println!("a = {}",a);

    let mut b:i8 =0; // i signed -2^(N-1) ----2^(N-1)-1
    println!("first b  = {}",b);
    b=22;
    println!("last b  = {}",b);

    let c = 123456789;
    println!("c= {} ve size {} bytes",c,mem::size_of_val(&c));



}