use std::fmt::Formatter;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;

#[derive(Clone)]
struct Matrix<T> ((T, T), (T, T));

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul for Matrix<T> {
    type Output = Matrix<T>;
    
    fn mul(self, that: Matrix<T>) -> Matrix<T> {
        Matrix::<T>(
            (self.0.0 * that.0.0 + self.0.1 * that.1.0, self.0.0 * that.0.1 + self.0.1 * that.1.1),
            (self.1.0 * that.0.0 + self.1.1 * that.1.0, self.1.0 * that.0.1 + self.1.1 * that.1.1),
        )
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[[{}, {}],\n [{}, {}]]", self.0.0, self.0.1, self.1.0, self.1.1)
    }
}

impl<T: Copy> Copy for Matrix<T> {}

fn main() {
    let m = Matrix((1, 1), (1, 0));
    let mut m2 = Matrix((1, 0), (0, 1));
    println!("a");
    for _ in 0..10 {
        println!("{}", m2);
        m2 = m2 * m;
    }
}
