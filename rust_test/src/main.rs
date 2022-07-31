use std::collections::VecDeque;
fn test1(){
    println!("Hello, world!");
    let mut buf = VecDeque::with_capacity(15);
    buf.extend(0..4);
    println!("{:?}", buf);
    // println!("{:?}", buf[4]);
    buf.push_back(6);
    println!("{:?}", buf);

    let mut a = (1,2);
    a.0 = 2;
    // let mut a = [1,3];
    // a[0] = 3
    println!("{:?}", a);

    for b in buf.iter_mut(){
        println!("{}", b);
        *b = 10;
    }
    println!("{:?}", buf);
}
fn test2(){
    let mut buf = VecDeque::with_capacity(15);
    buf.push_back((1,2));
    buf.push_back((3,4));
    println!("{:?}", buf);
    for b in buf.iter_mut(){
        println!("{:?}", b);
        b.0 = 10;
    }
    println!("{:?}", buf);
    println!("{:?}", buf[0]);
    println!("{:?}", buf[1]);
    buf[0] = (2, 4);
    println!("{:?}", buf);
    buf.remove(1);
    println!("{:?}", buf);
}
fn main() {
    test2()
}
