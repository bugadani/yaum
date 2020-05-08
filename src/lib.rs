//! This crate provides type-safe basic scientific units and constants for `no_std` programs.
#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(double_precision)]
pub type Base = f32;

#[cfg(not(double_precision))]
pub type Base = f32;

#[macro_export]
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
        rad_per_s: 1.0
    });

    pub type SamplingFrequency = Frequency;

    convert_unit!(Frequency, AngularFrequency, 2.0 * core::f32::consts::PI);
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
        m: 1.0,
        km: 1_000.0
    });
}

pub mod velocity {
    impl_unit!(Velocity, {
        mps: 1.0,
        kph: 1_000.0 / 3_600.0
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
