use std::ops::Add;

/// 1. 定义一个类型 A，具体有什么业务意义先不管，就是一个代号
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

/// 这就是 A + A ?
impl Add for A {
    type Output = A;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value
        }
    }
}

/// 这就是 A + B ?
impl Add<B> for A {
    type Output = B;

    fn add(self, rhs: B) -> Self::Output {
        B {
            value: self.value * 1000 + rhs.value
        }
    }
}

/// 假设换算规则为 1A = 1000B
/// 2. 然后再定义一个 B，它跟 A 之间有换算规则
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct B {
    pub value: i64,
}

impl B {
    pub fn new(n: i64) -> Self {
        Self {
            value: n
        }
    }
}


/// 这就是 B + B ?
impl Add for B {
    type Output = B;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value
        }
    }
}

/// 这就是 B + A ?
impl Add<A> for B {
    type Output = B;

    fn add(self, rhs: A) -> Self::Output {
        Self {
            value: rhs.value * 1000 + self.value
        }
    }
}




fn main() {
    let a1 = A::new(10);
    let a2 = A::new(20);
    let a3 = A::new(30);
    let a4 = a1 + a2;
    assert_eq!(a3, a4);
    let a5 = a1 + a2;
    assert_eq!(a3, a5);
    println!("{a1:?} {a2:?} {a3:?} {a4:?} {a5:?}");

    let b1 = B::new(10000);
    let b2 = B::new(20000);
    let b3 = B::new(30000);
    let b4 = b1 + b2;
    assert_eq!(b3, b4);
    let b5 = b1 + b2;
    assert_eq!(b3, b5);
    println!("{b1:?} {b2:?} {b3:?} {b4:?} {b5:?}");

    let c1 = A::new(10);
    let c2 = B::new(20000);
    let c3 = B::new(30000);
    let c4 = c1 + c2;
    assert_eq!(c3, c4);
    let c5 = c2 + c1;
    assert_eq!(c3, c5);
    println!("{c1:?} {c2:?} {c3:?} {c4:?} {c5:?}");

}
