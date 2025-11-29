pub struct Uninitialized;

pub struct Matrix<S> {
    pub(crate) state: S,
}

impl<S> Matrix<S> {
    pub fn map_state<T>(self, f: impl FnOnce(S) -> T) -> Matrix<T> {
        Matrix { state: f(self.state) }
    }
    pub fn into_inner(self) -> S {
        self.state
    }
}

impl Matrix<Uninitialized> {
    pub fn new() -> Self {
        Matrix { state: Uninitialized }
    }

    pub fn configure(self, config: MatrixConfig) -> Matrix<Configured> {
        self.map_state(|_| Configured { config })
    }
}

pub struct MatrixConfig {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        MatrixConfig {
            width: 64,
            height: 64,
            refresh_rate: 60,
        }
    }
}

pub struct Configured {
    pub config: MatrixConfig,
}

impl Matrix<Configured> {
    pub fn get_height(&self) -> u32 {
        self.state.config.height
    }

    pub fn get_width(&self) -> u32 {
        self.state.config.width
    }

    pub fn get_refresh_rate(&self) -> u32 {
        self.state.config.refresh_rate
    }

    pub fn init_driver(self, driver_config: DriverConfig) -> Matrix<Ready> {
        self.map_state(|configured| Ready {
            matrix: configured.config,
            driver: driver_config,
        })
    }
}
pub struct DriverConfig {
    pub hardware_mapping: String,
    pub rgb_sequence: String,
    pub brightness: u8,
}

impl DriverConfig {
    pub fn new(hardware_mapping: String, rgb_sequence: String, brightness: u8) -> Self {
        DriverConfig {
            hardware_mapping,
            rgb_sequence,
            brightness,
        }
    }

    pub fn default() -> Self {
        DriverConfig::new("adafruit-hat".to_string(), "RGB".to_string(), 128)
    }
}

pub struct Ready {
    pub matrix: MatrixConfig,
    pub driver: DriverConfig,
}
