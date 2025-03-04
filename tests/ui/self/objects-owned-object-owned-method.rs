//@ run-pass
// Test invoked `&self` methods on owned objects where the values
// closed over contain managed values. This implies that the boxes
// will have headers that must be skipped over.

trait FooTrait {
    fn foo(self: Box<Self>) -> usize;
}

struct BarStruct {
    x: usize
}

impl FooTrait for BarStruct {
    fn foo(self: Box<BarStruct>) -> usize {
        self.x
    }
}

pub fn main() {
    let foo = Box::new(BarStruct{ x: 22 }) as Box<dyn FooTrait>;
    assert_eq!(22, foo.foo());
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
