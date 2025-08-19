#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main(){

    let a:u8 = 125; // u = unsigned 0-255 8 bit
    println!("a = {}",a);

    let mut b:i8 =0; // i signed -2^(N-1) ----2^(N-1)-1
    //mut mutable
    println!("first b  = {}",b);
    b=22;
    println!("last b  = {}",b);

    let c = 123456789;
    println!("c= {} ve size {} bytes",c,mem::size_of_val(&c));

    let d:isize = -200; //usize for positive
    let d_boyut = mem::size_of_val(&d);
    println!("d = {} ve size {} bytes and computer is {} bits", d, d_boyut, d_boyut*8);

    let e:char ='g'; //.,; can be 
    println!("e = {} ve size {} bytes", e, mem::size_of_val(&e));

    //f32 or f64 should be signed IEEE 754, nan, +- 
    let f:f32 = 2.50000005;
    println!("f = {} ve size {} bytes", f, mem::size_of_val(&f));

    let g:bool = false;
    println!("g = {} ve size {} bytes", g, mem::size_of_val(&g));



}