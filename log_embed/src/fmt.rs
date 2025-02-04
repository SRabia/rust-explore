#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

#[collapse_debuginfo(yes)]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        #[cfg(feature = "rtt")]
        rtt_target::rprintln!($s $(, $x)*);

        #[cfg(feature = "defmt")]
        defmt::trace!($s $(, $x)*);

        #[cfg(not(any(feature = "rtt", feature = "defmt")))]
        let _ = ($(& $x ), *);
    };
}

#[collapse_debuginfo(yes)]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        #[cfg(feature = "rtt")]
        rtt_target::rprintln!($s $(, $x)*);

        #[cfg(feature = "defmt")]
        defmt::debug!($s $(, $x)*);

        #[cfg(not(any(feature = "rtt", feature = "defmt")))]
        let _ = ($(& $x ), *);
    };
}

#[collapse_debuginfo(yes)]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        #[cfg(feature = "rtt")]
        rtt_target::rprintln!($s $(, $x)*);

        #[cfg(feature = "defmt")]
        defmt::info!($s $(, $x)*);

        #[cfg(not(any(feature = "rtt", feature = "defmt")))]
        let _ = ($(& $x ), *);
    };
}

pub fn init() {
    #[cfg(feature = "rtt")]
    rtt_init_print!();
}
