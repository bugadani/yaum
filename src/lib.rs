//! This crate provides type-safe basic scientific units and constants for `no_std` programs.
#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]

#[cfg(double_precision)]
pub type Base = f32;

#[cfg(not(double_precision))]
pub type Base = f32;

#[macro_export]
macro_rules! impl_unit {
    ($type:ident) => { crate::impl_unit!($type, crate::Base); };

    ($type:ident, $basetype:ty) => {
        #[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
        pub struct $type(pub $basetype);

        impl $type {
            pub fn new(value: $basetype) -> Self {
                Self(value)
            }
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
    };
}

#[macro_export]
macro_rules! convert_div {
    ($left:ty, $right: ty, $result: ty) => {
        impl core::ops::Div<$right> for $left {
            type Output = $result;

            fn div(self, rhs: $right) -> Self::Output {
                <$result>::new(self.0 / rhs.0)
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
    impl_unit!(Frequency);
    impl_unit!(AngularFrequency);

    pub const Hz:  Frequency = Frequency(1.0);
    pub const kHz: Frequency = Frequency(1_000.0);
    pub const MHz: Frequency = Frequency(1_000_000.0);

    pub type SamplingFrequency = Frequency;

    pub const sps:  Frequency = Frequency(1.0);
    pub const ksps: Frequency = Frequency(1_000.0);

    convert_unit!(Frequency, AngularFrequency, 2.0 * core::f32::consts::PI);
}

pub mod time {
    impl_unit!(Time);

    pub const us: Time = Time(0.000_001);
    pub const ms: Time = Time(0.001);
    pub const s: Time = Time(1.0);
    pub const min: Time = Time(60.0);
    pub const h: Time = Time(3600.0);
}

pub mod length {
    impl_unit!(Length);

    pub const m: Length = Length(1.0);
    pub const km: Length = Length(1_000.0);
}

pub mod velocity {
    impl_unit!(Velocity);
    impl_unit!(Acceleration);
    pub type Speed = Velocity;

    pub mod consts {
        use super::*;

        pub const c: Velocity = Velocity(299_792_458.0);
        pub const g: Acceleration = Acceleration(9.80665);
    }
}

pub mod digital {
    impl_unit!(LSB, usize);
}

pub mod conversions {
    use crate::*;

    convert_div!(length::Length, time::Time, velocity::Velocity);
    convert_div!(velocity::Velocity, time::Time, velocity::Acceleration);

    #[cfg(test)]
    mod tests {
        #[test]
        fn convert_division() {
            use crate::time::*;
            use crate::length::*;
            use crate::velocity::*;

            fn t(_v: Speed) {
            }

            t(1.0 * m / s);
        }

        #[test]
        fn convert_unit() {
            use crate::frequency::*;
            use core::f32::consts::PI;

            let f = 50.0 * Hz;

            assert_eq!(AngularFrequency(100.0 * PI), f.into());
        }
    }
}
