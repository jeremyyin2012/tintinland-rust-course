use std::ops::{Add, Deref};

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

/// 假设有一个概念叫 W
pub trait W {
    fn w(&self) -> Box<dyn W>;
}

/// 给A实现下W，返回也是实现了W的即可
impl W for A {
    fn w(&self) -> Box<dyn W> {
        let v = &self.value * 1000 * 5;
        let b = B { value: v };
        Box::new(b)
    }
}

impl W for B {
    fn w(&self) -> Box<dyn W> {
        let v = &self.value * 5;
        let b = B { value: v };
        Box::new(b)
    }
}

/// 然后有一个函数，支持给实现了W的东西做w计算
pub fn do_w(obj: &dyn W) -> Box<dyn W> {
    obj.w()
}

/// 据说只要是实现了 Add 的 W 也都可以相加
impl Add for Box<dyn W> {
    type Output = Box<dyn W>;

    fn add(self, rhs: Self) -> Self::Output {
        self.w() + rhs.w()
    }
}

impl Add<Box<dyn W + 'static>> for Box<dyn W> {
    type Output = Box<dyn W>;

    fn add(self, rhs: Box<dyn W>) -> Self::Output {
        todo!()
    }
}

/// 那么如果现在有一个 X
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct X {
    pub value: i64,
}

impl X {
    pub fn new(n: i64) -> Self {
        Self {
            value: n
        }
    }
}

/// 还有一个 Y
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Y {
    pub value: i64,
}

impl Y {
    pub fn new(n: i64) -> Self {
        Self {
            value: n
        }
    }
}

/// 它们都只知道去实现 W
impl W for X {
    fn w(&self) -> Box<dyn W> {
        Box::new(Self { value: &self.value * 5 })
    }
}

impl W for Y {
    fn w(&self) -> Box<dyn W> {
        Box::new(Self { value: &self.value * 1000 * 5 })
    }
}


/// 然后两个分别实现了 W 的东西可以分别调用 w
/// 泛型的写法
pub fn do_w2<T, S>(obj1: T, obj2: S) -> Box<dyn W>
    where
        T: W,
        S: W {
    obj1.w() + obj2.w()
}

/// Trait Object 的写法
pub fn do_w3(obj1: Box<dyn W>, obj2: Box<dyn W>) -> Box<dyn W> {
    obj1.w() + obj2.w()
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


    let x = X::new(1);
    let y = Y::new(2);
    let z = do_w2(x, y);
    let z2 = do_w3(Box::new(x), Box::new(y));
}
