#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn while_and_loop(){
    let mut x = 1;
    while x<1000 {
        x*=2;
        if x==128 {continue}
        println!("x={}",x);
    }
    println!("loop:");
    let mut y =1;
    loop //while true  
    {
        y*=2;
        println!("y={}",y);
        if y==1<<10 {break;}

    }
    
    
}

fn for_loop(){
    for x in 1..11{
        if x==3 {continue;}
        if x==8 {break;}
        println!("{}",x);
    }
    println!("---enumerate");
    for (poz,y) in (50..61).enumerate(){
        println!("poz:{} y:{}",poz,y);
    }
}

fn match_case(){
    let country_code =1000;
    let country = match country_code{
        44=>"UK",
        46=>"Sweden",
        7=>"Russia",
        90=>"Turkey",
        1..=1000=>"Unknown",
        _=>"invalid"
    };
    println!("country code {} is {}",country_code,country);

}

 

fn main(){
    //while_and_loop();
    //for_loop();
    match_case();


}