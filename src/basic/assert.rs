use crate::basic::value::Value;
use std::hash::{Hash, Hasher};

/// Represents a comparison assertion between [`Value`] instances.
///
/// `Assert` is used throughout the GOAP system to define preconditions for actions
/// and goals for the planner. It provides various comparison operators that can
/// be used to evaluate world state values.
///
/// # Examples
/// ```
/// use rust_goap::prelude::*;
///
/// // Create assertions for different comparison types
/// let eq_assert = Assert::eq(42);          // Value must equal 42
/// let ne_assert = Assert::not_eq(0);       // Value must not equal 0
/// let gt_assert = Assert::gt(10);          // Value must be greater than 10
/// let lt_assert = Assert::lt(100);         // Value must be less than 100
/// let gte_assert = Assert::gt_eq(50);      // Value must be greater than or equal to 50
/// let lte_assert = Assert::lt_eq(200);     // Value must be less than or equal to 200
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Assert {
    /// Value must equal the specified value
    Equals(Value),
    /// Value must not equal the specified value
    NotEquals(Value),
    /// Value must be greater than the specified value
    GreaterThan(Value),
    /// Value must be greater than or equal to the specified value
    GreaterThanEquals(Value),
    /// Value must be less than the specified value
    LessThan(Value),
    /// Value must be less than or equal to the specified value
    LessThanEquals(Value),
}

impl Assert {
    /// Creates an equality assertion.
    ///
    /// # Arguments
    /// * `value` - The value to compare against
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::eq(42);
    /// assert!(matches!(assert, Assert::Equals(Value::I64(42))));
    /// ```
    pub fn eq(value: impl Into<Value>) -> Self {
        Assert::Equals(value.into())
    }

    /// Creates a non-equality assertion.
    ///
    /// # Arguments
    /// * `value` - The value that must not be equal
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::not_eq(0);
    /// assert!(matches!(assert, Assert::NotEquals(Value::I64(0))));
    /// ```
    pub fn not_eq(value: impl Into<Value>) -> Self {
        Assert::NotEquals(value.into())
    }

    /// Creates a greater-than-or-equal assertion.
    ///
    /// # Arguments
    /// * `value` - The minimum value (inclusive)
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::gt_eq(50);
    /// assert!(matches!(assert, Assert::GreaterThanEquals(Value::I64(50))));
    /// ```
    pub fn gt_eq(value: impl Into<Value>) -> Self {
        Assert::GreaterThanEquals(value.into())
    }

    /// Creates a less-than-or-equal assertion.
    ///
    /// # Arguments
    /// * `value` - The maximum value (inclusive)
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::lt_eq(100);
    /// assert!(matches!(assert, Assert::LessThanEquals(Value::I64(100))));
    /// ```
    pub fn lt_eq(value: impl Into<Value>) -> Self {
        Assert::LessThanEquals(value.into())
    }

    /// Creates a greater-than assertion.
    ///
    /// # Arguments
    /// * `value` - The value that must be exceeded
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::gt(10);
    /// assert!(matches!(assert, Assert::GreaterThan(Value::I64(10))));
    /// ```
    pub fn gt(value: impl Into<Value>) -> Self {
        Assert::GreaterThan(value.into())
    }

    /// Creates a less-than assertion.
    ///
    /// # Arguments
    /// * `value` - The value that must not be reached
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::lt(200);
    /// assert!(matches!(assert, Assert::LessThan(Value::I64(200))));
    /// ```
    pub fn lt(value: impl Into<Value>) -> Self {
        Assert::LessThan(value.into())
    }
}

impl Assert {
    /// Extracts the comparison value from an assertion.
    ///
    /// Returns the [`Value`] that this assertion is comparing against,
    /// regardless of the comparison operator.
    ///
    /// # Examples
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let assert = Assert::eq(42);
    /// let value = assert.value();
    /// assert!(matches!(value, Value::I64(42)));
    ///
    /// let assert2 = Assert::gt(10.5);
    /// let value2 = assert2.value();
    /// assert!(matches!(value2, Value::F64(10.5)));
    /// ```
    pub fn value(&self) -> Value {
        match self {
            Assert::Equals(v)
            | Assert::NotEquals(v)
            | Assert::GreaterThan(v)
            | Assert::LessThan(v)
            | Assert::GreaterThanEquals(v)
            | Assert::LessThanEquals(v) => *v,
        }
    }
}

impl Hash for Assert {
    /// Implements hashing for `Assert`.
    ///
    /// The hash includes both the assertion type (encoded as a u8 discriminant)
    /// and the hash of the comparison value. This ensures that different
    /// assertion types with the same value hash to different values.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Assert::Equals(v) => {
                0_u8.hash(state);
                v.hash(state);
            }
            Assert::NotEquals(v) => {
                1_u8.hash(state);
                v.hash(state);
            }
            Assert::GreaterThanEquals(v) => {
                2_u8.hash(state);
                v.hash(state);
            }
            Assert::LessThanEquals(v) => {
                3_u8.hash(state);
                v.hash(state);
            }
            Assert::GreaterThan(v) => {
                4_u8.hash(state);
                v.hash(state);
            }
            Assert::LessThan(v) => {
                5_u8.hash(state);
                v.hash(state);
            }
        }
    }
}

/// Compares a value against an assertion.
///
/// This is the core evaluation function that checks whether a given [`Value`]
/// satisfies the condition specified by an [`Assert`].
///
/// # Arguments
/// * `comparison` - The assertion to evaluate against
/// * `value` - The value to check
///
/// # Returns
/// `true` if the value satisfies the assertion, `false` otherwise.
///
/// # Examples
/// ```
/// use rust_goap::prelude::*;
///
/// let value = Value::I64(42);
/// let assert = Assert::eq(42);
/// assert!(compare_values(&assert, &value));
///
/// let assert2 = Assert::gt(40);
/// assert!(compare_values(&assert2, &value));
///
/// let assert3 = Assert::lt(30);
/// assert!(!compare_values(&assert3, &value));
/// ```
pub fn compare_values(comparison: &Assert, value: &Value) -> bool {
    match comparison {
        Assert::Equals(v) => value == v,
        Assert::NotEquals(v) => value != v,
        Assert::GreaterThanEquals(v) => value >= v,
        Assert::LessThan(v) => value < v,
        Assert::GreaterThan(v) => value > v,
        Assert::LessThanEquals(v) => value <= v,
    }
}
