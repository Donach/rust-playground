

// UFCS - Universal function call syntax

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);

}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Bar");
    }
}


fn main() {
    let b: Baz = Baz;
    Foo::f(&b);
    Bar::f(&b);
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);

    ["hello", "world"].into_iter().map(str::to_string).collect::<Vec<_>>();
}