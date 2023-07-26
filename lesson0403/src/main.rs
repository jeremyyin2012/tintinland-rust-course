use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct A {
    pub value: i64,
}

impl A {
    pub fn new(n: i64) -> Self {
        Self {
            value: n
        }
    }
}

impl Add for A {
    type Output = A;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value
        }
    }
}


fn main() {
    let a1 = A::new(10);
    let a2 = A::new(20);
    let a3 = a1 + a2;
    let a4 = A::new(30);
    assert_eq!(a3, a4);
    println!("{a1:?} {a2:?} {a3:?} {a4:?}");
}
