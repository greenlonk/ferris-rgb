use crate::domain::{DriverConfig, Matrix, Ready};

pub(crate) struct DisplayService {
    matrix: Matrix<Ready>,
}

impl DisplayService {
    pub fn new() -> Self {
        // Here we would normally initialize the matrix with proper configuration
        // and driver initialization. For simplicity, we'll assume it's already ready.
        let matrix = Matrix::<Ready> {
            state: Ready {
                matrix: Default::default(),
                driver: DriverConfig::default(),
            },
        };
        DisplayService { matrix }
    }

    pub fn render_content(&self, content: &str) {
        // Logic to render content on the matrix display
        println!("Rendering content: {}", content);
    }

    pub fn is_initialized(&self) -> bool {
        // Check if the display service is properly initialized
        true
    }
}
