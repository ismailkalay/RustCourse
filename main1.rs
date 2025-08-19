#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;
fn veri(){
    let a = 55;
    println!("a: {}",a);
}


fn main(){
    //veri();

    let a: =44;
    {
        println!("circle inside a: {}",a);
    }
    println!("circle outside a: {}",a);

}
