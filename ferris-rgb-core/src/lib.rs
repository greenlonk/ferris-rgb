mod domain;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Matrix, Uninitialized};

    #[test]
    fn test_init_matrix() {
        use crate::domain::Matrix;

        // We want to define the following domain workflow:
        let my_matrix: Matrix<Uninitialized> = Matrix::new();

        // Compile-time assertion (no-op at runtime, just linter sanity)
        let _: Matrix<Uninitialized> = my_matrix;

        assert_eq!(std::mem::size_of::<Matrix<Uninitialized>>(), 0); // This is a zero-sized type

        let _inner = my_matrix.into_inner();
    }

    #[test]
    fn test_config_matrix() {
        use crate::domain::matrix::{Configured, Matrix, MatrixConfig};

        let my_matrix: Matrix<Uninitialized> = Matrix::new();

        let matrix_config: MatrixConfig = MatrixConfig {
            width: 64,
            height: 64,
            refresh_rate: 60,
        };

        let configured_matrix: Matrix<Configured> = my_matrix.configure(matrix_config);

        assert_eq!(configured_matrix.get_height(), 64);
        assert_eq!(configured_matrix.get_width(), 64);
        assert_eq!(configured_matrix.get_refresh_rate(), 60);
    }

    #[test]
    fn test_default_config_matrix() {
        use crate::domain::matrix::{Configured, Matrix, MatrixConfig};

        let my_matrix: Matrix<Uninitialized> = Matrix::new();
        let default_config: MatrixConfig = MatrixConfig::default();
        let configured_matrix: Matrix<Configured> = my_matrix.configure(default_config);

        assert_eq!(configured_matrix.get_height(), 64);
        assert_eq!(configured_matrix.get_width(), 64);
        assert_eq!(configured_matrix.get_refresh_rate(), 60);
    }

    #[test]
    fn test_configured_matrix_init_driver() {
        use crate::domain::matrix::*;

        let my_matrix: Matrix<Uninitialized> = Matrix::new();
        let matrix_config: MatrixConfig = MatrixConfig::default();

        let configured_matrix: Matrix<Configured> = my_matrix.configure(matrix_config);

        let driver_config = DriverConfig::default();

        let ready_matrix: Matrix<Ready> = configured_matrix.init_driver(driver_config);

        assert_eq!(ready_matrix.state.driver.hardware_mapping, "adafruit-hat");
    }
}
