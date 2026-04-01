use std::collections::{BTreeMap, HashMap};

pub use array::Array;
pub use boolean::Boolean;
pub use cmd_char::CommandCharacter;
pub use data::Data;
pub use datetime::DateTime;
pub use duration::Duration;
pub use metadata::Metadata;
pub use num::Number;
pub use object::Object;
pub use ordered_object::OrderedObject;
pub use string_val::XffString;
pub use table::Table;
pub use uuid::Uuid;

/// Contains the `Array` type, representing a list of `XffValue`s.
pub mod array;
/// Contains the `Boolean` type.
pub mod boolean;
/// Contains the `CommandCharacter` type, used in v0 (deprecated).
pub mod cmd_char;
/// Contains the `Data` type, wrapping arbitrary bytes.
pub mod data;
/// Contains the `DateTime` type.
pub mod datetime;
/// Contains the `Duration` type.
pub mod duration;
/// Contains the `Metadata` type.
pub mod metadata;
/// Contains the `Number` type, capable of storing various precisions.
pub mod num;
/// Contains the `Object` type, a string-to-XffValue mapping.
pub mod object;
/// Contains the `OrderedObject` type, a key-value mapping that preserves insertion order.
pub mod ordered_object;
/// Contains the `XffString` type.
pub mod string_val;
/// Contains the `Table` type, a schema-based data structure.
pub mod table;
/// Contains the `Uuid` type, a 128-bit unique identifier wrapper.
pub mod uuid;

mod tests;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// An enum for the different types of XFF values.
///
/// Many From traits are implemented for convenience on `XffValue` directly.
///
/// Directly stored data, `String`, `Booleans` and `Null` have convenience
/// functions implemented on `XffValue` directly.
///
/// `Data`, `Array`, `Object`, and `Number` have convenience functions implemented on their respective types.
///
/// All variants of `XffValue` are clone-able and have `is_` functions implemented.
/// E.g. `is_string()`, `is_number()`, etc.
///
/// All variants have also `into_` functions implemented to retrieve the wrapped data inside.
/// E.g. `into_string()`, `into_array()`, etc.
///
/// For more information please refer to the readme, or the documentation of the function or type.
///
/// Deprecated and kept for compatibility with v0:
///
/// `CommandCharacter` is an enum representing a single ASCII command or control character
/// `ArrayCmdChar` is a list of `CommandCharacter`s and seldom used in writing XFF files, but never in reading them.
///
/// # Example
/// ```rust
/// use aequa::{XffValue, Number, Array, Object, Data, Metadata};
///
/// let string_val = XffValue::from("hello mom!");
/// let num_val = XffValue::from(42.69);
/// let array_val = XffValue::from(
///     vec![
///         XffValue::from("hi mom!"),
///         XffValue::from(42.69)
///     ]
/// );
/// let object_val = XffValue::from(
///     Object::from(
///        vec![
///            ("keyA".to_string(), XffValue::from("hi mom!")),
///            ("keyB".to_string(), XffValue::from(42.69))
///        ]
///     )
/// );
/// let data_val = XffValue::from(Data::from(vec![1, 2, 3]));
/// let boolean_val = XffValue::from(true);
/// let null_val = XffValue::Null;
/// let mut meta = Metadata::new();
/// meta.set_creator("Xqhare".to_string());
/// let meta_val = XffValue::from(meta);
///
/// assert!(string_val.is_string());
/// assert!(num_val.is_number());
/// assert!(array_val.is_array());
/// assert!(object_val.is_object());
/// assert!(data_val.is_data());
/// assert!(boolean_val.is_boolean());
/// assert!(null_val.is_null());
/// assert!(meta_val.is_metadata());
///
/// let string: String = string_val.into_string().unwrap();
/// let num: Number = num_val.into_number().unwrap();
/// let array: Array = array_val.into_array().unwrap();
/// let object: Object = object_val.into_object().unwrap();
/// let data: Data = data_val.into_data().unwrap();
/// let boolean: bool = boolean_val.into_boolean().unwrap();
/// let null: Option<()> = null_val.into_null();
/// let metadata: Metadata = meta_val.into_metadata().unwrap();
/// ```
pub enum XffValue {
    /// A string value
    String(XffString),
    /// A numeric value
    Number(Number),
    /// An array of XFF values of arbitrary length
    Array(Array),
    /// An object of string keys and `XffValue` values
    Object(Object),
    /// A sequence of Key-Value pairs where order is preserved
    OrderedObject(OrderedObject),
    /// A schema-based table
    Table(Table),
    /// A metadata object
    Metadata(Metadata),
    /// A data value, holding arbitrary bytes
    Data(Data),
    /// A boolean value, true or false
    Boolean(Boolean),
    /// Date and Time (milliseconds since epoch)
    DateTime(DateTime),
    /// Duration in milliseconds
    Duration(Duration),
    /// 128-bit UUID
    Uuid(Uuid),
    /// Not a Number
    NaN,
    /// Positive Infinity
    Infinity,
    /// Negative Infinity
    NegInfinity,
    /// A null value, a.k.a. `None`, `Nill` or `nothing`
    Null,
    /// Deprecated
    /// Only used in v0, needed for legacy usage
    /// A command character is represented by the `CommandCharacter` enum
    CommandCharacter(CommandCharacter),
    /// Deprecated
    /// Only used in v0, needed for legacy usage
    /// An array of `CommandCharacter`s
    ArrayCmdChar(Vec<CommandCharacter>),
}

impl Default for XffValue {
    fn default() -> Self {
        XffValue::Null
    }
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl XffValue {
    /// Returns the value as a string
    ///
    /// Only works on `XffValue::String` and `XffValue::Number`.
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::{XffValue, Number, Data};
    ///
    /// let string_value = XffValue::from("hello mom!");
    /// let num_value = XffValue::from(42.69);
    /// let data_value = XffValue::from(Data::from(vec![1, 2]));
    ///
    /// assert_eq!(string_value.into_string(), Some("hello mom!".to_string()));
    /// assert_eq!(num_value.into_string(), Some("42.69".to_string()));
    /// assert_eq!(data_value.into_string(), None);
    /// ```
    #[must_use]
    pub fn into_string(&self) -> Option<String> {
        match self {
            XffValue::String(s) => Some(s.value.clone()),
            XffValue::Number(n) => Some(n.as_string()),
            _ => None,
        }
    }

    /// Returns the value as a number if it is a `XffValue::Number`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::{XffValue, Number};
    ///
    /// let num_value = XffValue::from(42.69);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert_eq!(string_value.into_number(), None);
    /// assert_eq!(num_value.into_number(), Some(Number::from(42.69)));
    /// ```
    #[must_use]
    pub fn into_number(&self) -> Option<Number> {
        match self {
            XffValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the value as an array if it is a `XffValue::Array`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let vec_value = XffValue::from(vec![XffValue::from("hello mom!"), XffValue::from(42.69)]);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_array(), None);
    /// assert_eq!(vec_value.into_array(), XffValue::from(vec![XffValue::from("hello mom!"), XffValue::from(42.69)]).into_array());
    /// ```
    #[must_use]
    pub fn into_array(&self) -> Option<Array> {
        match self {
            XffValue::Array(a) => Some(a.clone()),
            _ => None,
        }
    }

    /// Returns the value as an object if it is a `XffValue::Object`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use aequa::{XffValue, Number};
    ///
    /// let map = BTreeMap::from([
    ///     ("key0".to_string(), XffValue::from("value0")),
    ///     ("key1".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// let map_value = XffValue::from(map.clone());
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_object(), None);
    /// assert_eq!(map_value.into_object(), XffValue::from(map).into_object());
    ///
    /// ```
    #[must_use]
    pub fn into_object(&self) -> Option<Object> {
        match self {
            XffValue::Object(o) => Some(o.clone()),
            _ => None,
        }
    }

    /// Returns the value as an ordered object if it is a `XffValue::OrderedObject`
    #[must_use]
    pub fn into_ordered_object(&self) -> Option<OrderedObject> {
        match self {
            XffValue::OrderedObject(o) => Some(o.clone()),
            _ => None,
        }
    }

    /// Returns the value as a table if it is a `XffValue::Table`
    #[must_use]
    pub fn into_table(&self) -> Option<Table> {
        match self {
            XffValue::Table(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Returns the value as a metadata object if it is a `XffValue::Metadata`
    #[must_use]
    pub fn into_metadata(&self) -> Option<Metadata> {
        match self {
            XffValue::Metadata(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// If the value is a Table, returns the specific row as an `XffValue::OrderedObject`.
    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<XffValue> {
        match self {
            XffValue::Table(t) => t.get_row(index),
            _ => None,
        }
    }

    /// Returns the value as a UUID if it is a `XffValue::Uuid`
    #[must_use]
    pub fn into_uuid(&self) -> Option<Uuid> {
        match self {
            XffValue::Uuid(u) => Some(*u),
            _ => None,
        }
    }

    /// Returns the value as a data value if it is a `XffValue::Data`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::{XffValue, Data};
    ///
    /// let data_value = XffValue::from(vec![1, 2, 3]);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_data(), None);
    /// assert_eq!(data_value.into_data(), XffValue::from(vec![1, 2, 3]).into_data());
    /// ```
    #[must_use]
    pub fn into_data(&self) -> Option<Data> {
        match self {
            XffValue::Data(d) => Some(d.clone()),
            _ => None,
        }
    }

    /// Returns the value as a boolean if it is a `XffValue::Boolean`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let bool_value_true = XffValue::from(true);
    /// let bool_value_false = XffValue::from(false);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_boolean(), None);
    /// assert_eq!(bool_value_true.into_boolean(), Some(true));
    /// assert_eq!(bool_value_false.into_boolean(), Some(false));
    /// ```
    #[must_use]
    pub fn into_boolean(&self) -> Option<bool> {
        match self {
            XffValue::Boolean(b) => Some(b.0),
            _ => None,
        }
    }

    /// Returns the value as a `DateTime` (milliseconds since epoch) if it is a `XffValue::DateTime`
    #[must_use]
    pub fn into_datetime(&self) -> Option<u64> {
        match self {
            XffValue::DateTime(dt) => Some(dt.0),
            _ => None,
        }
    }

    /// Returns the value as a UNIX timestamp (seconds since epoch) if it is a `XffValue::DateTime`
    #[must_use]
    pub fn into_unix_timestamp(&self) -> Option<f64> {
        match self {
            XffValue::DateTime(dt) => Some(dt.0 as f64 / 1000.0),
            _ => None,
        }
    }

    /// Returns the value as a Duration (milliseconds) if it is a `XffValue::Duration`
    #[must_use]
    pub fn into_duration(&self) -> Option<u64> {
        match self {
            XffValue::Duration(d) => Some(d.0),
            _ => None,
        }
    }

    /// Returns the value as a Duration in seconds if it is a `XffValue::Duration`
    #[must_use]
    pub fn into_duration_seconds(&self) -> Option<f64> {
        match self {
            XffValue::Duration(d) => Some(d.0 as f64 / 1000.0),
            _ => None,
        }
    }

    /// Returns the value as a `std::time::Duration` if it is a `XffValue::Duration`
    #[must_use]
    pub fn into_std_duration(&self) -> Option<std::time::Duration> {
        match self {
            XffValue::Duration(d) => Some(std::time::Duration::from_millis(d.0)),
            _ => None,
        }
    }

    /// Checks if the value is a `DateTime`
    #[must_use]
    pub fn is_datetime(&self) -> bool {
        matches!(self, XffValue::DateTime(_))
    }

    /// Checks if the value is a Duration
    #[must_use]
    pub fn is_duration(&self) -> bool {
        matches!(self, XffValue::Duration(_))
    }

    /// Creates a new `XffValue::DateTime` from milliseconds since epoch
    #[must_use]
    pub fn from_unix_timestamp_millis(ms: u64) -> Self {
        XffValue::DateTime(DateTime(ms))
    }

    /// Creates a new `XffValue::DateTime` from a UNIX timestamp (seconds since epoch)
    #[must_use]
    pub fn from_unix_timestamp(seconds: f64) -> Self {
        // Remember, DateTime is in milliseconds
        XffValue::DateTime(DateTime((seconds * 1000.0) as u64))
    }

    /// Creates a new `XffValue::Duration` from milliseconds
    #[must_use]
    pub fn from_duration_millis(ms: u64) -> Self {
        XffValue::Duration(Duration(ms))
    }

    /// Creates a new `XffValue::Duration` from seconds
    #[must_use]
    pub fn from_duration_seconds(seconds: f64) -> Self {
        // Remember, Duration is in milliseconds
        XffValue::Duration(Duration((seconds * 1000.0) as u64))
    }

    /// Returns the value as a reference to a metadata object if it is a `XffValue::Metadata`
    #[must_use]
    pub fn as_metadata(&self) -> Option<&Metadata> {
        match self {
            XffValue::Metadata(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the value as a reference to a string
    #[must_use]
    pub fn as_string(&self) -> Option<&String> {
        match self {
            XffValue::String(s) => Some(&s.value),
            _ => None,
        }
    }

    /// Returns the value as a reference to a number
    #[must_use]
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            XffValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the value as a reference to an array
    #[must_use]
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            XffValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the value as a reference to an object
    #[must_use]
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            XffValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns the value as a reference to an ordered object
    #[must_use]
    pub fn as_ordered_object(&self) -> Option<&OrderedObject> {
        match self {
            XffValue::OrderedObject(o) => Some(o),
            _ => None,
        }
    }

    /// Returns the value as a reference to a table
    #[must_use]
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            XffValue::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the value as a reference to data
    #[must_use]
    pub fn as_data(&self) -> Option<&Data> {
        match self {
            XffValue::Data(d) => Some(d),
            _ => None,
        }
    }

    /// Returns the value as a reference to a boolean
    #[must_use]
    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            XffValue::Boolean(b) => Some(&b.0),
            _ => None,
        }
    }

    /// Returns the value as a reference to a datetime
    #[must_use]
    pub fn as_datetime(&self) -> Option<&u64> {
        match self {
            XffValue::DateTime(dt) => Some(&dt.0),
            _ => None,
        }
    }

    /// Returns the value as a reference to a duration
    #[must_use]
    pub fn as_duration(&self) -> Option<&u64> {
        match self {
            XffValue::Duration(d) => Some(&d.0),
            _ => None,
        }
    }

    /// Returns the value as a reference to a UUID
    #[must_use]
    pub fn as_uuid(&self) -> Option<&Uuid> {
        match self {
            XffValue::Uuid(u) => Some(u),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a string
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            XffValue::String(s) => Some(&mut s.value),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a number
    pub fn as_number_mut(&mut self) -> Option<&mut Number> {
        match self {
            XffValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to an array
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match self {
            XffValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to an object
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            XffValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to an ordered object
    pub fn as_ordered_object_mut(&mut self) -> Option<&mut OrderedObject> {
        match self {
            XffValue::OrderedObject(o) => Some(o),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a table
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            XffValue::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a metadata object
    pub fn as_metadata_mut(&mut self) -> Option<&mut Metadata> {
        match self {
            XffValue::Metadata(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to data
    pub fn as_data_mut(&mut self) -> Option<&mut Data> {
        match self {
            XffValue::Data(d) => Some(d),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a boolean
    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            XffValue::Boolean(b) => Some(&mut b.0),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a datetime
    pub fn as_datetime_mut(&mut self) -> Option<&mut u64> {
        match self {
            XffValue::DateTime(dt) => Some(&mut dt.0),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a duration
    pub fn as_duration_mut(&mut self) -> Option<&mut u64> {
        match self {
            XffValue::Duration(d) => Some(&mut d.0),
            _ => None,
        }
    }

    /// Returns the value as a mutable reference to a UUID
    pub fn as_uuid_mut(&mut self) -> Option<&mut Uuid> {
        match self {
            XffValue::Uuid(u) => Some(u),
            _ => None,
        }
    }

    /// Returns null if it is a `XffValue::Null`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let null_value = XffValue::Null;
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_null(), Some(()));
    /// assert_eq!(null_value.into_null(), None);
    /// ```
    #[must_use]
    pub fn into_null(&self) -> Option<()> {
        match self {
            XffValue::Null => None,
            _ => Some(()),
        }
    }

    /// Checks if the value is a string, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let string_value = XffValue::from("hello mom!");
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert!(!num_value.is_string());
    /// assert!(string_value.is_string());
    /// ```
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, XffValue::String(_))
    }

    /// Checks if the value is a number, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let number_value = XffValue::from(42.69);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_number());
    /// assert!(number_value.is_number());
    /// ```
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, XffValue::Number(_))
    }

    /// Checks if the value is an array, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let array_value = XffValue::from(vec![XffValue::from("hello mom!"), XffValue::from(42.69)]);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_array());
    /// assert!(array_value.is_array());
    /// ```
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, XffValue::Array(_))
    }

    /// Checks if the value is an object, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{XffValue, Object};
    ///
    /// let object_value = XffValue::from(Object::from(vec![("key0".to_string(), XffValue::from("hello mom!")), ("key1".to_string(), XffValue::from(vec![1, 2, 3]))]));
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_object());
    /// assert!(object_value.is_object());
    /// ```
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, XffValue::Object(_))
    }

    /// Checks if the value is an ordered object
    #[must_use]
    pub fn is_ordered_object(&self) -> bool {
        matches!(self, XffValue::OrderedObject(_))
    }

    /// Checks if the value is a table
    #[must_use]
    pub fn is_table(&self) -> bool {
        matches!(self, XffValue::Table(_))
    }

    /// Checks if the value is a metadata object
    #[must_use]
    pub fn is_metadata(&self) -> bool {
        matches!(self, XffValue::Metadata(_))
    }

    /// Checks if the value has metadata attached (for Table, Object, or Metadata types)
    #[must_use]
    pub fn has_metadata(&self) -> bool {
        matches!(self, XffValue::Metadata(_))
    }

    /// Checks if the value is a UUID
    #[must_use]
    pub fn is_uuid(&self) -> bool {
        matches!(self, XffValue::Uuid(_))
    }

    /// Checks if the value is data, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{XffValue, Data};
    ///
    /// let data_value = XffValue::from(Data::from(vec![1, 2, 3]));
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_data());
    /// assert!(data_value.is_data());
    /// ```
    #[must_use]
    pub fn is_data(&self) -> bool {
        matches!(self, XffValue::Data(_))
    }

    /// Checks if the value is a boolean, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let boolean_value = XffValue::from(true);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_boolean());
    /// assert!(boolean_value.is_boolean());
    /// ```
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        matches!(self, XffValue::Boolean(_))
    }

    /// Checks if the value is both a boolean and true, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let boolean_value_true = XffValue::from(true);
    /// let boolean_value_false = XffValue::from(false);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_true());
    /// assert!(!boolean_value_false.is_true());
    /// assert!(boolean_value_true.is_true());
    /// ```
    #[must_use]
    pub fn is_true(&self) -> bool {
        matches!(self, XffValue::Boolean(Boolean(true)))
    }

    /// Checks if the value is both a boolean and false, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let boolean_value_true = XffValue::from(true);
    /// let boolean_value_false = XffValue::from(false);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_false());
    /// assert!(!boolean_value_true.is_false());
    /// assert!(boolean_value_false.is_false());
    /// ```
    #[must_use]
    pub fn is_false(&self) -> bool {
        matches!(self, XffValue::Boolean(Boolean(false)))
    }

    /// Checks if the value is null, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use aequa::XffValue;
    ///
    /// let null_value = XffValue::Null;
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_null());
    /// assert!(null_value.is_null());
    /// ```
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, XffValue::Null)
    }

    /// Checks if the value is NaN
    #[must_use]
    pub fn is_nan(&self) -> bool {
        matches!(self, XffValue::NaN)
    }

    /// Checks if the value is Infinity
    #[must_use]
    pub fn is_infinity(&self) -> bool {
        matches!(self, XffValue::Infinity)
    }

    /// Checks if the value is Negative Infinity
    #[must_use]
    pub fn is_neg_infinity(&self) -> bool {
        matches!(self, XffValue::NegInfinity)
    }
}

// -----------------------------------------------------------
//                     Into implementations
// -----------------------------------------------------------

#[allow(clippy::from_over_into)]
impl Into<Vec<XffValue>> for XffValue {
    fn into(self) -> Vec<XffValue> {
        vec![self]
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl From<Object> for XffValue {
    fn from(c: Object) -> Self {
        XffValue::Object(c)
    }
}

impl From<Table> for XffValue {
    fn from(c: Table) -> Self {
        XffValue::Table(c)
    }
}

impl From<Uuid> for XffValue {
    fn from(c: Uuid) -> Self {
        XffValue::Uuid(c)
    }
}

impl From<Metadata> for XffValue {
    fn from(c: Metadata) -> Self {
        XffValue::Metadata(c)
    }
}

impl From<OrderedObject> for XffValue {
    fn from(c: OrderedObject) -> Self {
        XffValue::OrderedObject(c)
    }
}

impl From<Array> for XffValue {
    fn from(c: Array) -> Self {
        XffValue::Array(c)
    }
}

impl<V> From<Vec<V>> for XffValue
where
    V: Into<XffValue>,
{
    fn from(c: Vec<V>) -> Self {
        XffValue::Array(Array::from(c))
    }
}

impl From<CommandCharacter> for XffValue {
    fn from(c: CommandCharacter) -> Self {
        XffValue::CommandCharacter(c)
    }
}

impl From<Data> for XffValue {
    fn from(c: Data) -> Self {
        XffValue::Data(c)
    }
}

impl From<bool> for XffValue {
    fn from(c: bool) -> Self {
        XffValue::Boolean(Boolean::from(c))
    }
}

impl From<Boolean> for XffValue {
    fn from(c: Boolean) -> Self {
        XffValue::Boolean(c)
    }
}

impl From<XffString> for XffValue {
    fn from(c: XffString) -> Self {
        XffValue::String(c)
    }
}

impl From<DateTime> for XffValue {
    fn from(c: DateTime) -> Self {
        XffValue::DateTime(c)
    }
}

impl From<Duration> for XffValue {
    fn from(c: Duration) -> Self {
        XffValue::Duration(c)
    }
}

// -----------------------------------------------------------
//                     Index implementations
// -----------------------------------------------------------

impl std::ops::Index<&str> for XffValue {
    type Output = XffValue;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            XffValue::Object(o) => &o[index],
            XffValue::OrderedObject(o) => &o[index],
            _ => panic!("Cannot index into non-object XffValue"),
        }
    }
}

impl std::ops::Index<usize> for XffValue {
    type Output = XffValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            XffValue::Array(a) => &a[index],
            XffValue::OrderedObject(o) => &o[index].1,
            _ => panic!("Cannot index into non-array XffValue"),
        }
    }
}

impl<S, V> From<BTreeMap<S, V>> for XffValue
where
    S: Into<String>,
    V: Into<XffValue>,
{
    fn from(c: BTreeMap<S, V>) -> Self {
        XffValue::Object(Object::from(c))
    }
}

impl<S, V> From<HashMap<S, V>> for XffValue
where
    S: Into<String>,
    V: Into<XffValue>,
{
    fn from(c: HashMap<S, V>) -> Self {
        XffValue::Object(c.into())
    }
}

impl<S> From<(S, u8)> for XffValue
where
    S: Into<String>,
{
    fn from(c: (S, u8)) -> Self {
        match c.1 {
            0 => {
                let string = c.0.into();
                let check_usize = &string.parse::<usize>();
                if check_usize.is_ok() {
                    XffValue::Number(Number::from(check_usize.as_ref().unwrap()))
                } else {
                    let check_isize = &string.parse::<isize>();
                    if check_isize.is_ok() {
                        XffValue::Number(Number::from(check_isize.as_ref().unwrap()))
                    } else {
                        let check_float = &string.parse::<f64>();
                        if check_float.is_ok() {
                            XffValue::Number(Number::from(check_float.as_ref().unwrap()))
                        } else {
                            XffValue::String(XffString::from(string))
                        }
                    }
                }
            }
            1 => XffValue::String(XffString::from(c.0.into())),
            _ => unreachable!(),
        }
    }
}

impl From<String> for XffValue {
    fn from(c: String) -> Self {
        XffValue::String(XffString::from(c))
    }
}

impl From<&String> for XffValue {
    fn from(c: &String) -> Self {
        XffValue::String(XffString::from(c.clone()))
    }
}

impl From<&str> for XffValue {
    fn from(c: &str) -> Self {
        XffValue::String(XffString::from(c.to_string()))
    }
}

impl From<char> for XffValue {
    fn from(c: char) -> Self {
        XffValue::String(XffString::from(c.to_string()))
    }
}

impl From<usize> for XffValue {
    fn from(c: usize) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<isize> for XffValue {
    fn from(c: isize) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<f64> for XffValue {
    fn from(c: f64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u64> for XffValue {
    fn from(c: u64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i64> for XffValue {
    fn from(c: i64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<f32> for XffValue {
    fn from(c: f32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u32> for XffValue {
    fn from(c: u32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i32> for XffValue {
    fn from(c: i32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u16> for XffValue {
    fn from(c: u16) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i16> for XffValue {
    fn from(c: i16) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u8> for XffValue {
    fn from(c: u8) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i8> for XffValue {
    fn from(c: i8) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<std::time::Duration> for XffValue {
    fn from(c: std::time::Duration) -> Self {
        XffValue::Duration(Duration::from(c.as_millis() as u64))
    }
}

impl From<std::time::SystemTime> for XffValue {
    fn from(c: std::time::SystemTime) -> Self {
        match c.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => XffValue::DateTime(DateTime::from(d.as_millis() as u64)),
            Err(_) => XffValue::DateTime(DateTime::from(0)),
        }
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for XffValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            XffValue::String(s) => write!(f, "{s}"),
            XffValue::Number(n) => write!(f, "{n}"),
            XffValue::Array(a) => write!(f, "{a}"),
            XffValue::Object(o) => write!(f, "{o}"),
            XffValue::OrderedObject(o) => {
                write!(f, "{{(ordered) ")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k}: {v}")?;
                }
                write!(f, "}}")
            }
            XffValue::Table(t) => write!(f, "{t}"),
            XffValue::Metadata(m) => write!(f, "{m}"),
            XffValue::Data(d) => write!(f, "{d}"),
            XffValue::Boolean(b) => write!(f, "{b}"),
            XffValue::DateTime(dt) => write!(f, "{dt}"),
            XffValue::Duration(d) => write!(f, "{d}"),
            XffValue::Uuid(u) => write!(f, "{u}"),
            XffValue::NaN => write!(f, "NaN"),
            XffValue::Infinity => write!(f, "Infinity"),
            XffValue::NegInfinity => write!(f, "NegInfinity"),
            XffValue::Null => write!(f, "Null"),

            // Legacy - v0 only - debug will suffice
            XffValue::CommandCharacter(cmd) => write!(f, "{cmd:?}"),
            XffValue::ArrayCmdChar(acmd) => write!(f, "{acmd:?}"),
        }
    }
}
