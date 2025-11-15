use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T, U>(x: T, y: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        Self {
            real: x.into(),
            imag: y.into(),
        }
    }
    fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other = other.into();
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}
impl From<f64> for Complex {
    fn from(x: f64) -> Self {
        Self {
            real: (x),
            imag: (0.0),
        }
    }
}
impl From<i32> for Complex {
    fn from(x: i32) -> Self {
        Self {
            real: (x.into()),
            imag: (0.0),
        }
    }
}
impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other = other.into();
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}
impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let other = other.into();
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let real = self.real;
        let imag = self.imag;

        if real.abs() < 0.001 && imag.abs() < 0.001 {
            return write!(f, "0");
        }

        if real.abs() < 0.001 {
            return write!(f, "{}i", imag);
        }

        if imag.abs() < 0.001 {
            return write!(f, "{}", real);
        }

        if imag > 0.0 {
            write!(f, "{}+{}i", real, imag)
        } else {
            write!(f, "{}{}i", real, imag)
        }
    }
}

//exercise example(do not touch)
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
