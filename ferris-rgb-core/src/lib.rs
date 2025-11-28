mod domain;

#[cfg(test)]
mod tests {
    use crate::domain::Uninitialized;
    use super::*;

    #[test]
    fn test_init_matrix() {
    use crate::domain::Matrix;

    // We want to define the following domain workflow:
    let my_matrix: Matrix<Uninitialized> = Matrix::new();

    }
}