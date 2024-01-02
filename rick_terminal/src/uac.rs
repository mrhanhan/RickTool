
struct A {

}
impl Drop for A {
    fn drop(&mut self) {
        println!("Drop A")
    }
}

fn hello(a: A) -> A {
    println!("Hello");
    a
}

fn main() {
    let a = A{};
    {
        hello(hello(a));
    }
    println!("Hello World");
}