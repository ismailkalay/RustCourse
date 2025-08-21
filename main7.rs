#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

struct Point{
    x:f64,
    y:f64
}
struct Line{
    start:Point,
    end:Point
}

fn structs(){
    let p = Point{x:4.0,y:3.0};
    println!("point p is ({}, {})",p.x,p.y);
    let p2 = Point{x:5.0, y:10.0};
    let myLine = Line{start:p,end:p2};
    println!(" my line ({},{}) - ({},{})",myLine.start.x,myLine.start.y,myLine.end.x,myLine.end.y);


}

enum Color{
    Red,
    Blue,
    Green,
    RGBColor(u8,u8,u8), //tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
}

fn enums(){
    let c = Color::Blue;
    let c = Color::RGBColor(10,0,0);
    match c {
        Color::Blue=> println!("b"),
        Color::Red=> println!("r"),
        Color::Green=> println!("g"),
        Color::RGBColor(0,0,0)=> println!("black"),
        Color::Cmyk {cyan:_, magenta:_, yellow:_, black:255}=>println!("black"),
        Color::RGBColor(r,g,b)=> println!("rgb({},{},{})",r,g,b),
        _=> println!("the other color")
    }
}

union IntOrFloat {
    i:i32,
    f:f32
}

fn process_value(iof:IntOrFloat) {
    unsafe {
        match iof { //union 2 type is enough
            IntOrFloat{i:44}=>{
                println!("deger: 44 ");
            }
            IntOrFloat{f}=>{
                println!("second state deger: {}",f);
            }

        }
    }
}





fn main(){

    //structs();
    //enums();
    /*let mut iof = IntOrFloat{i:122};
    iof.i = 221;
    let value = unsafe {iof.i};
    println!(" value:{}",value);

    let iof2 = IntOrFloat{f:55.0};
    process_value(iof2);

    let iof3 = IntOrFloat{i:1};
    process_value(iof3);//force to second value */

    let x = 2.0;
    let y = 1.0;
    //option -> Some(c) | None
    let result = if y!=0.0 { Some(x/y) } else {None};
    
    match result {
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("y is zero")
    }

}