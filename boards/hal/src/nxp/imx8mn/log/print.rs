//! Printing.

use super::{console, console::Write};
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    console::console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::nxp::imx8mn::log::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::nxp::imx8mn::log::print::_print(format_args_nl!($($arg)*));
    })
}

/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        #[allow(unused_imports)]
        use $crate::nxp::imx8mn::arch::timer::*;

        let timestamp = time_manager().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::nxp::imx8mn::log::print::_print(format_args_nl!(
            concat!("[  {:>3}.{:03}{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use $crate::nxp::imx8mn::arch::timer::*;

        let timestamp = time_manager().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::nxp::imx8mn::log::print::_print(format_args_nl!(
            concat!("[  {:>3}.{:03}{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000,
            $($arg)*
        ));
    })
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        #[allow(unused_imports)]
        use $crate::nxp::imx8mn::arch::timer::*;

        let timestamp = time_manager().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::nxp::imx8mn::log::print::_print(format_args_nl!(
            concat!("[W {:>3}.{:03}{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use $crate::nxp::imx8mn::arch::timer::*;

        let timestamp = time_manager().uptime();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::nxp::imx8mn::log::print::_print(format_args_nl!(
            concat!("[W {:>3}.{:03}{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000,
            $($arg)*
        ));
    })
}
