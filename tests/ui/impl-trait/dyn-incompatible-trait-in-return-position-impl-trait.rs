trait DynIncompatible {
    fn foo() -> Self;
}

trait DynCompatible {
    fn bar(&self);
}

struct A;
struct B;

impl DynIncompatible for A {
    fn foo() -> Self {
        A
    }
}

impl DynIncompatible for B {
    fn foo() -> Self {
        B
    }
}

impl DynCompatible for A {
    fn bar(&self) {}
}

impl DynCompatible for B {
    fn bar(&self) {}
}

fn can() -> impl DynIncompatible {
    if true {
        return A;
    }
    B //~ ERROR mismatched types
}

fn cat() -> impl DynCompatible {
    if true {
        return A;
    }
    B //~ ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
