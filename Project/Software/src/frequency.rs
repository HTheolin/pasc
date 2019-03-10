//! Definition of bus frequency details for f4.
macro_rules! frequency {
    ($FREQUENCY:expr) => {
        use crate::time::*;

        /// Frequency
        pub static mut FREQUENCY: u32 = $FREQUENCY;

        /// Set Frequency
        pub fn set_frequency(f: u32) {
            unsafe {FREQUENCY = f };
        }

        /// Unit of time
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct Ticks(pub u32);

        impl Ticks {
            /// Applies the function `f` to the inner value
            fn map<F>(self, f: F) -> Self
                where F: FnOnce(u32) -> u32,
            {
                Ticks(f(self.0))
            }
        }

        impl From<Ticks> for Microseconds {
            fn from(ticks: Ticks) -> Self {
                Microseconds(ticks.0 / (unsafe { FREQUENCY }/ 1_000_000))
            }
        }

        impl From<Ticks> for Milliseconds {
            fn from(ticks: Ticks) -> Self {
                Milliseconds(ticks.0 / (unsafe { FREQUENCY }/ 1_000))
            }
        }

        impl From<Ticks> for Seconds {
            fn from(ticks: Ticks) -> Self {
                Seconds(ticks.0 / unsafe { FREQUENCY })
            }
        }

        impl From<IHertz> for Ticks {
            fn from(ihz: IHertz) -> Ticks {
                Ticks(unsafe { FREQUENCY } / ihz.0)
            }
        }

        impl From<Microseconds> for Ticks {
            fn from(us: Microseconds) -> Ticks {
                Ticks(us.0 * (unsafe { FREQUENCY } / 1_000_000))
            }
        }

        impl From<Milliseconds> for Ticks {
            fn from(ms: Milliseconds) -> Ticks {
                Ticks(ms.0 * (unsafe { FREQUENCY } / 1_000))
            }
        }

        impl From<Seconds> for Ticks {
            fn from(s: Seconds) -> Ticks {
                Ticks(s.0 * unsafe { FREQUENCY })
            }
        }

        impl Into<u32> for Ticks {
            fn into(self) -> u32 {
                self.0
            }
        }
    }
}

/// Advance High-performance Bus (AHB1)
pub mod ahb1 {
    frequency!(16_000_000);
}

/// Advance High-performance Bus (AHB2)
pub mod ahb2 {
    frequency!(16_000_000);
}

/// Advance Peripheral Bus 1 (APB1)
pub mod apb1 {
    frequency!(16_000_000);
}

/// Advance Peripheral Bus 2 (APB2)
pub mod apb2 {
    frequency!(16_000_000);
}