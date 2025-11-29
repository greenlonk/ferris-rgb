mod matrix;

pub struct Uninitialized;

pub struct Matrix<S> {
    state: S,
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
}
