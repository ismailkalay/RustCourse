#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn arrays(){
    let mut a:[i32;5] =[1,2,3,4,5];
    println!("a length {} and  a: {}",a.len(),a[0]);

    a[0]=22;
    println!("a length {} and  a: {}",a.len(),a[0]);
    assert_eq!(a,[22,2,3,4,5]); //control

    if a!=[1,2,3,4,5]{
        println!("arrays not equal.");
    }

    let b=[1u16;10]; //u16, u32 than half
    for i in 0..b.len(){
        println!("{}. value {}",i,b[i]);
    }
    println!("---------------");
    println!("{:?}",b);

    println!("b array {} bytes", std::mem::size_of_val(&b));


}
fn matris(){

}

fn main(){
    
    //arrays();
    matris();
    


}