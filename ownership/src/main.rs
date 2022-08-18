fn main() {
    let a = vec![1,2,3];

    for i in &a {
        println!("{}", *i);
    }

    println!("{:?}", a);

    let b = true;
    let x;

    if b {
        let y = 5;
        x =  &y;
    }

    println!("{}", x);
}
