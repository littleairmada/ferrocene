//@ run-pass

pub fn main() {
    let mut i = 0;
    while i < 20 { i += 1; if i == 10 { break; } }
    assert_eq!(i, 10);
    loop { i += 1; if i == 20 { break; } }
    assert_eq!(i, 20);
    let xs = [1, 2, 3, 4, 5, 6];
    for x in &xs {
        if *x == 3 { break; } assert!(*x <= 3);
    }
    i = 0;
    while i < 10 { i += 1; if i % 2 == 0 { continue; } assert!(i % 2 != 0); }
    i = 0;
    loop {
        i += 1; if i % 2 == 0 { continue; } assert!(i % 2 != 0);
        if i >= 10 { break; }
    }
    let ys = vec![1, 2, 3, 4, 5, 6];
    for x in &ys {
        if *x % 2 == 0 { continue; }
        assert!(*x % 2 != 0);
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
