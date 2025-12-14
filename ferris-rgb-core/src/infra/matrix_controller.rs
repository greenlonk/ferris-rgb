use std::{env, process::Command};

/* #[cfg(feature = "pi")]
use rpi_led_matrix::{LedColor, LedMatrix};*/

pub struct MatrixController;

impl MatrixController {
    pub fn spawn_clock_process() {
        let root = env::var("RPI_RGB_MATRIX_ROOT").expect("RPI_RGB_MATRIX_ROOT not set");

        let bin = format!("{root}/examples-api-use/clock");
        let font = format!("{root}/fonts/texgyre-27.bdf");

        Command::new("sudo")
            .args([
                &bin,
                "-f",
                &font,
                "--led-rows",
                "64",
                "--led-cols",
                "64",
                "-C",
                "2,0,121",
            ])
            .spawn()
            .expect("failed to start matrix renderer")
            .wait();
    }
}
