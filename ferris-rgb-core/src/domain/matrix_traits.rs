use crate::domain::matrix::{Configured, DriverConfig, Matrix, MatrixConfig, Ready};
pub trait Configurable {
    fn configure(self, config: MatrixConfig) -> Matrix<Configured>;
}

pub trait DriverInitializable {
    fn init_driver(self, driver_config: DriverConfig) -> Matrix<Ready>;
}
