use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents a typed value that can be stored in a WorldState.
///
/// `Value` is the fundamental data type used throughout the GOAP (Goal-Oriented Action Planning) system
/// to represent state variables. It supports three primitive types:
/// - Boolean values for logical conditions (e.g., `is_hungry`, `has_weapon`)
/// - 64-bit integers for discrete quantities (e.g., `health`, `ammo_count`)
/// - 64-bit floating-point numbers for continuous values (e.g., `distance`, `time_remaining`)
///
/// # Examples
/// ```
/// use rust_goap::prelude::*;
///
/// let bool_value: Value = true.into();      // Value::Bool(true)
/// let int_value: Value = 42.into();         // Value::I64(42)
/// let float_value: Value = 3.14.into();     // Value::F64(3.14)
/// ```
#[derive(Clone, Debug, PartialOrd, Copy)]
pub enum Value {
    /// Boolean value, typically used for logical state flags
    Bool(bool),
    /// 64-bit signed integer value, used for discrete quantities
    I64(i64),
    /// 64-bit floating-point value, used for continuous measurements
    F64(f64),
}

impl From<i64> for Value {
    /// Converts an `i64` to a `Value::I64`.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let value: Value = 42.into();
    /// assert!(matches!(value, Value::I64(42)));
    /// ```
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<f64> for Value {
    /// Converts an `f64` to a `Value::F64`.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let value: Value = 3.14.into();
    /// assert!(matches!(value, Value::F64(3.14)));
    /// ```
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<bool> for Value {
    /// Converts a `bool` to a `Value::Bool`.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let value: Value = true.into();
    /// assert!(matches!(value, Value::Bool(true)));
    /// ```
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl Hash for Value {
    /// Implements hashing for `Value`.
    ///
    /// This allows `Value` to be used as keys in hash-based collections.
    /// The hash includes the enum discriminant to ensure different variants
    /// hash to different values, even if their contained values might be equal
    /// when interpreted differently (e.g., `1` as `i64` vs `1.0` as `f64`).
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Value::Bool(b) => b.hash(state),
            Value::I64(i) => i.hash(state),
            Value::F64(f) => f.to_bits().hash(state),
        }
    }
}

impl PartialEq for Value {
    /// Compares two `Value` instances for equality.
    ///
    /// Values are only considered equal if they are of the same variant
    /// and contain equal inner values. Different variants (e.g., `I64` vs `F64`)
    /// are never equal, even if their numerical values would be equal.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let a: Value = 42.into();
    /// let b: Value = 42.into();
    /// let c: Value = 42.0.into();
    ///
    /// assert_eq!(a, b);     // Both are I64(42)
    /// assert_ne!(a, c);     // I64(42) != F64(42.0)
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            _ => false,
        }
    }
}

/// Marker trait indicating that `Value` has total equality.
///
/// Since `PartialEq` is implemented and all variants are comparable,
/// `Value` can safely implement `Eq`.
impl Eq for Value {}

impl Value {
    /// Calculates the distance between two values of the same type.
    ///
    /// This method is used by the GOAP planner to estimate how far a current
    /// state is from a desired goal state. The distance metric varies by type:
    /// - For `Bool`: 0 if equal, 1 if different
    /// - For `I64`: absolute difference as unsigned 64-bit integer
    /// - For `F64`: absolute difference converted to unsigned 64-bit integer
    ///
    /// # Panics
    /// Panics if the two values are of different variants (e.g., comparing `Bool` with `I64`).
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let a: Value = 10.into();
    /// let b: Value = 15.into();
    /// let c: Value = true.into();
    ///
    /// assert_eq!(a.distance(&b), 5);     // |10 - 15| = 5
    /// assert_eq!(c.distance(&c), 0);     // true == true
    /// ```
    pub fn distance(&self, other: &Value) -> u64 {
        match (self, other) {
            (Value::Bool(lhs), Value::Bool(rhs)) => {
                if lhs == rhs {
                    0
                } else {
                    1
                }
            },
            (Value::I64(lhs), Value::I64(rhs)) => (lhs - rhs).unsigned_abs(),
            (Value::F64(lhs), Value::F64(rhs)) => (lhs - rhs).abs() as u64,
            _ => panic!("Cannot calculate distance between different Value types"),
        }
    }
}

impl Display for Value {
    /// Formats the `Value` for display purposes.
    ///
    /// The output format includes the variant name and the contained value,
    /// making it clear what type of value is being displayed.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let bool_val: Value = true.into();
    /// let int_val: Value = 42.into();
    /// let float_val: Value = 3.14.into();
    ///
    /// assert_eq!(format!("{}", bool_val), "Value:Bool(true)");
    /// assert_eq!(format!("{}", int_val), "Value:I64(42)");
    /// assert_eq!(format!("{}", float_val), "Value:F64(3.14)");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => {
                write!(f, "Value:Bool({v})")
            },
            Self::I64(v) => {
                write!(f, "Value:I64({v})")
            },
            Self::F64(v) => {
                write!(f, "Value:F64({v})")
            },
        }
    }
}

impl Add for Value {
    type Output = Value;

    /// Adds two `Value` instances together.
    ///
    /// Supports addition between values of the same numeric type:
    /// - `I64 + I64` → `I64`
    /// - `F64 + F64` → `F64`
    ///
    /// Boolean values do not support addition.
    ///
    /// # Panics
    /// Panics if:
    /// - The values are of different variants
    /// - Either value is `Bool` (booleans don't support arithmetic)
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let a: Value = 10.into();
    /// let b: Value = 5.into();
    /// let result = a + b;
    ///
    /// assert!(matches!(result, Value::I64(15)));
    /// ```
    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(a), Value::I64(b)) => Value::I64(a + b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a + b),
            _ => panic!("Unsupported addition between Datum variants, {self:?} - {other:?}"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    /// Subtracts one `Value` from another.
    ///
    /// Supports subtraction between values of the same numeric type:
    /// - `I64 - I64` → `I64`
    /// - `F64 - F64` → `F64`
    ///
    /// Boolean values do not support subtraction.
    ///
    /// # Panics
    /// Panics if:
    /// - The values are of different variants
    /// - Either value is `Bool` (booleans don't support arithmetic)
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let a: Value = 10.into();
    /// let b: Value = 5.into();
    /// let result = a - b;
    ///
    /// assert!(matches!(result, Value::I64(5)));
    /// ```
    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(a), Value::I64(b)) => Value::I64(a - b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a - b),
            _ => panic!("Unsupported negation between Datum variants, {self:?} - {other:?}"),
        }
    }
}

impl AddAssign for Value {
    /// Adds another `Value` to this one in-place.
    ///
    /// This is the in-place version of the `Add` trait, allowing `+=` syntax.
    /// Only supports same-type numeric operations.
    ///
    /// # Panics
    /// Currently panics with "unimplemented" for unsupported operations.
    /// In the future, this should be changed to panic with a clearer error message.
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(v1) => match rhs {
                Self::I64(v2) => {
                    *v1 += v2;
                },
                _ => unimplemented!("Unimplemented! Tried to add {self:?} to {rhs:?}"),
            },
            Self::F64(v1) => match rhs {
                Self::F64(v2) => {
                    *v1 += v2;
                },
                _ => unimplemented!("Unimplemented! Tried to add {self:?} to {rhs:?}"),
            },
            _ => unimplemented!("Unimplemented! Tried to add {self:?} to {rhs:?}"),
        }
    }
}

impl SubAssign for Value {
    /// Subtracts another `Value` from this one in-place.
    ///
    /// This is the in-place version of the `Sub` trait, allowing `-=` syntax.
    /// Only supports same-type numeric operations.
    ///
    /// # Panics
    /// Currently panics with "unimplemented" for unsupported operations.
    /// In the future, this should be changed to panic with a clearer error message.
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(v1) => match rhs {
                Self::I64(v2) => {
                    *v1 -= v2;
                },
                _ => unimplemented!("Unimplemented! Tried to subtract {rhs:?} from {self:?}"),
            },
            Self::F64(v1) => match rhs {
                Self::F64(v2) => {
                    *v1 -= v2;
                },
                _ => unimplemented!("Unimplemented! Tried to subtract {rhs:?} from {self:?}"),
            },
            _ => unimplemented!("Unimplemented! Tried to subtract {rhs:?} from {self:?}"),
        }
    }
}
