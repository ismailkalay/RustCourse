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
    let mtx:[[f32;3];2] =[
        [1.0,0.0,0.0], [0.0,1.0,0.0]];

    println!("{:?} ",mtx);

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            println!("mtx[{}][{}]= {}",i,j,mtx[i][j]);
        }

    }

}

fn use_slice(slice:&mut[i32]){
    println!("new data slice first member {}, member number {}.",slice[0],slice.len());
    slice[0]=3333;
}
fn slices(){
    let mut data:[i32;5] = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}",data);
}

fn topla_ve_carp(x:i32,y:i32)-> (i32,i32){
    (x+y,x*y)

}

fn tuples(){
    let x =3;
    let y =4;
    let sonuclar = topla_ve_carp(x,y);
    println!("results : {:?}", sonuclar);
    println!("{0} + {1} = {2} and {0}*{1}={3}",x,y,sonuclar.0,sonuclar.1);

    let elemanlar = (true, 24.1, -1i8);
    println!("members :{:?}",elemanlar);



}

struct  Point <T,V> {
    x:T,
    y:V
}

struct Line <T,V,S,K>{
    start:Point<T,V>,
    end:Point<S,K>
}

fn main(){
    
    //arrays();
    //matris();
    //slices();
    //tuples();

    let a = Point{x:4.5,y:7.5};
    let b = Point{x:4,y:7};
    let cizgi = Line{start:Point{x:2,y:2.4}, end:Point{x:4,y:2.3}};

    println!("a = ({},{})",a.x,a.y);
    println!("b = ({},{})",b.x,b.y);


}