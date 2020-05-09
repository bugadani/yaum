//! This crate provides type-safe basic scientific units and constants for `no_std` programs.
//!
//! Example
//! =======
//!
//! ```rust
//! use yaum::time::*;
//! use yaum::length::*;
//! use yaum::velocity::*;
//!
//! // Simple arithmetic
//! assert_eq!(1.0 * kph, 1.0 * km / h);
//! assert_eq!(1.0 * km / min, 60.0 * km / h);
//!
//! // Read value in a given unit
//! assert_eq!(60.0, (1.0 * min).s());
//! assert_eq!(1_000.0, (1.0 * km).m());
//! ```
//!
//! Currently supported units:
//! * `time`: Time
//! * `frequency`: Frequency
//! * `length`: Length
//! * `velocity`: Velocity, Acceleration
//! * `digital`: LSB (least significant bits)
//!
//! Define custom units and conversions using the `impl_unit!`, `convert_div!` and `convert_unit!` macros.
//!
//! ```rust
//! #[macro_use] extern crate yaum;
//! use yaum::*;
//! use yaum::time::*;
//!
//! yaum::impl_unit!(ByteSize, {
//!     B: 1.0,
//!     kB: 1024.0,
//!     MB: 1024.0 * 1024.0
//! });
//! yaum::impl_unit!(BitSize, {
//!     b: 1.0,
//!     kb: 1024.0,
//!     Mb: 1024.0 * 1024.0
//! });
//!
//! // define conversion between the two units (1 byte = 8 bits):
//! yaum::convert_unit!(ByteSize, BitSize, 8.0);
//!
//! yaum::impl_unit!(BitSpeed, {
//!     bps: 1.0,
//!     kbps: 1024.0,
//!     Mbps: 1024.0 * 1024.0
//! });
//!
//! // define relationship between units (BitSpeed = BitSize/Time)
//! yaum::convert_div!(BitSize, Time, BitSpeed);
//!
//! fn main() {
//!     assert_eq!(8.0 * b, (1.0 * B).into());
//!     assert_eq!(1.0 * kbps, 1.0 * kb/s);
//! }
//! ```
//!
//! Precision
//! =========
//!
//! By default, units are implemented on top of `f32`. Enable the `double_precision` feature for `f64`.

#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(feature = "double_precision")]
/// Base type. `f64` if `double_precision` is enabled, otherwise `f32`.
pub type Base = f64;

#[cfg(not(feature = "double_precision"))]
/// Base type. `f64` if `double_precision` is enabled, otherwise `f32`.
pub type Base = f32;

#[macro_export]
/// Define a unit. Specify units, constants in brackets.
///
/// # Example:
///
/// ```rust
/// #[macro_use] extern crate yaum;
/// use yaum::*;
///
/// yaum::impl_unit!(BitSize, {
///     b: 1.0,
///     kb: 1024.0,
///     Mb: 1024.0 * 1024.0
/// });
/// # fn main() {}
/// ```
macro_rules! impl_unit {
    ($type:ident) => { crate::impl_unit!($type, {}); };
    ($type:ident, { $( $unit:ident: $value:expr ),* }) => { crate::impl_unit!($type, crate::Base, { $( $unit: $value ),* }); };

    ($type:ident, $basetype:ty, {$( $unit:ident: $value:expr ),*}) => {
        #[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
        pub struct $type($basetype);

        impl $type {
            pub const fn new(value: $basetype) -> Self {
                Self(value)
            }

            pub const fn dimensionless(self) -> $basetype {
                self.0
            }

            $( pub fn $unit(self) -> $basetype {
                self.dimensionless() * $value
            } )*
        }

        impl core::ops::Mul<$basetype> for $type {
            type Output = $type;

            fn mul(self, rhs: $basetype) -> Self::Output {
                $type(self.0 * rhs)
            }
        }

        impl core::ops::Mul<$type> for $basetype {
            type Output = $type;

            fn mul(self, rhs: $type) -> Self::Output {
                $type(self * rhs.0)
            }
        }

        impl core::ops::Add<$type> for $type {
            type Output = $type;

            fn add(self, rhs: $type) -> Self::Output {
                $type(self.0 + rhs.0)
            }
        }

        impl core::ops::Sub<$type> for $type {
            type Output = $type;

            fn sub(self, rhs: $type) -> Self::Output {
                $type(self.0 - rhs.0)
            }
        }

        impl core::ops::Div<$type> for $type {
            type Output = $basetype;

            fn div(self, rhs: $type) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl core::ops::Div<$basetype> for $type {
            type Output = $type;

            fn div(self, rhs: $basetype) -> Self::Output {
                $type(self.0 / rhs)
            }
        }

        $( pub const $unit: $type = $type($value); )*
    };
}

#[macro_export]
/// Specify the result of division of two types.
///
/// # Example
///
/// ```rust
/// #[macro_use] extern crate yaum;
/// use yaum::*;
/// use yaum::time::*;
///
/// yaum::impl_unit!(BitSize, {
///     b: 1.0,
///     kb: 1024.0,
///     Mb: 1024.0 * 1024.0
/// });
///
/// yaum::impl_unit!(BitSpeed, {
///     bps: 1.0,
///     kbps: 1024.0,
///     Mbps: 1024.0 * 1024.0
/// });
///
/// // define relationship between units (BitSpeed = BitSize/Time)
/// yaum::convert_div!(BitSize, Time, BitSpeed);
///
/// # fn main() {}
/// ```
macro_rules! convert_div {
    ($left:ty, $right: ty, $result: ty) => {
        impl core::ops::Div<$right> for $left {
            type Output = $result;

            fn div(self, rhs: $right) -> Self::Output {
                <$result>::new(self.dimensionless() / rhs.dimensionless())
            }
        }
    };
}

#[macro_export]
/// Specify the conversion factor between two types.
///
/// # Example
///
/// ```rust
/// #[macro_use] extern crate yaum;
/// use yaum::*;
/// use yaum::time::*;
///
/// yaum::impl_unit!(BitSize, {
///     b: 1.0,
///     kb: 1024.0,
///     Mb: 1024.0 * 1024.0
/// });
///
/// yaum::impl_unit!(ByteSize, {
///     B: 1.0,
///     kB: 1024.0,
///     MB: 1024.0 * 1024.0
/// });
///
/// // define conversion between the two units (1 byte = 8 bits):
/// yaum::convert_unit!(ByteSize, BitSize, 8.0);
///
/// # fn main() {}
/// ```
macro_rules! convert_unit {
    ($from:ty, $to: ty, $factor: expr) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                <$to>::new(value.0 * $factor)
            }
        }
        impl From<$to> for $from {
            fn from(value: $to) -> Self {
                <$from>::new(value.0 / $factor)
            }
        }
    };
}

pub mod frequency {
    impl_unit!(Frequency, {
        Hz: 1.0,
        kHz: 1_000.0,
        MHz: 1_000_000.0,

        sps: 1.0,
        ksps: 1_000.0
    });
    impl_unit!(AngularFrequency, {
        rad_per_s: 1.0,
        deg_per_s: 0.0174533
    });

    pub type SamplingFrequency = Frequency;

    convert_unit!(Frequency, AngularFrequency, 2.0 * core::f32::consts::PI);
}

pub mod angle {
    impl_unit!(Angle, {
        deg: 0.0174533,
        rad: 1.0
    });

    pub type AngularSpeed = crate::frequency::AngularFrequency;
}

pub mod time {
    impl_unit!(Time, {
        us: 0.000_001,
        ms: 0.001,
        s: 1.0,
        min: 60.0,
        h: 3600.0
    });
}

pub mod length {
    impl_unit!(Length, {
        um: 0.000_001,
        mm: 0.001,
        cm: 0.01,
        m: 1.0,
        km: 1_000.0,

        inch: 0.0254,
        ft: 0.3048,
        yard: 0.9144,
        mile: 1_609.34
    });
}

pub mod velocity {
    impl_unit!(Velocity, {
        mps: 1.0,
        kph: 1_000.0 / 3_600.0,
        mph: 1_609.34 / 3_600.0
    });
    impl_unit!(Acceleration, {
        mps2: 1.0
    });
    pub type Speed = Velocity;

    pub mod consts {
        use super::*;

        pub const c: Velocity = Velocity(299_792_458.0);
        pub const g: Acceleration = Acceleration(9.80665);
    }
}

pub mod conversions {
    use crate::*;

    convert_div!(length::Length, time::Time, velocity::Velocity);
    convert_div!(velocity::Velocity, time::Time, velocity::Acceleration);
    convert_div!(angle::Angle, time::Time, angle::AngularSpeed);

    #[cfg(test)]
    mod tests {
        #[test]
        fn value_readers() {
            use crate::frequency::*;

            assert_eq!(10_000.0, (10.0 * kHz).Hz());
        }

        #[test]
        fn convert_division() {
            use crate::time::*;
            use crate::length::*;
            use crate::velocity::*;

            fn t1(_v: Speed) {
            }

            fn t2(_a: Acceleration) {
            }

            t1(1.0 * m / s);
            t2(1.0 * m / s / s);

            assert_eq!(1.0 * mps2, 1.0 * m / s / s);
            assert_eq!(1.0 * km / min, 60.0 * km / h);
        }

        #[test]
        fn convert_unit() {
            use crate::frequency::*;
            use core::f32::consts::PI;

            let f = 50.0 * Hz;

            assert_eq!(AngularFrequency::new(100.0 * PI), f.into());
        }
    }
}
