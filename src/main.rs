#[derive(PartialEq)]
struct Foo<'a>(i32,&'a str);

fn main() {
    println!("Hello, world!");
    let a = Box::new(Foo( 88, "Hello"));
    let b = Box::new(Foo( 88, "Hello"));

    if a == b {
        println!("They are equal!");
    }
}
