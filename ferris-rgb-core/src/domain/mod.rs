use std::ops::Bound::Unbounded;

mod matrix;


pub struct Uninitialized;


pub struct Matrix<S> {
    state: S
}

impl <S> Matrix<S> {
    pub fn map_state<T>(self, f: impl FnOnce(S) -> T) -> Matrix<T> {
        Matrix {
            state: f(self.state),
        }
    }
    pub fn into_inner(self) -> S {
        self.state
    }
}

impl Matrix<Uninitialized> {
    pub fn new() -> Self {
        Matrix {
            state: Uninitialized
        }
    }
}