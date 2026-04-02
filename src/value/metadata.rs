use super::{Object, XffValue};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A metadata object for XFF files.
///
/// Can be created with `Metadata::new()` or `Metadata::from()`.
///
/// Provides high-level context about the file, such as creator, source, and license.
///
/// Most functionality needed for interacting with the underlying object is provided trough the struct itself.
///
/// Access to the underlying object is provided through the `as_object()` method.
///
/// `Metadata` implements `From<Object>`.
///
/// # Examples
/// ```rust
/// use aequa::Metadata;
///
/// let mut meta = Metadata::new();
/// meta.set_creator("Xqhare".to_string());
///
/// assert_eq!(meta.get_creator(), Some("Xqhare".to_string()));
/// ```
pub struct Metadata {
    /// The underlying data storage
    pub map: Object,
}

impl Metadata {
    /// Creates a new, empty Metadata object
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let meta = Metadata::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the creator of the file
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_creator("Xqhare".to_string());
    /// ```
    pub fn set_creator(&mut self, creator: String) {
        self.map.insert("creator", XffValue::from(creator));
    }

    /// Gets the creator of the file
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_creator("Xqhare".to_string());
    /// assert_eq!(meta.get_creator(), Some("Xqhare".to_string()));
    /// ```
    #[must_use]
    pub fn get_creator(&self) -> Option<String> {
        self.map.get("creator")?.into_string()
    }

    /// Sets the creation timestamp (milliseconds since epoch)
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_created_at(1647081600000);
    /// ```
    pub fn set_created_at(&mut self, timestamp: u64) {
        self.map.insert("created_at", XffValue::from_unix_timestamp_millis(timestamp));
    }

    /// Gets the creation timestamp
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_created_at(1647081600000);
    /// assert_eq!(meta.get_created_at(), Some(1647081600000));
    /// ```
    #[must_use]
    pub fn get_created_at(&self) -> Option<u64> {
        if let Some(XffValue::DateTime(dt)) = self.map.get("created_at") {
            Some(dt.0)
        } else {
            None
        }
    }

    /// Sets the source of the data
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_source("https://example.com".to_string());
    /// ```
    pub fn set_source(&mut self, source: String) {
        self.map.insert("source", XffValue::from(source));
    }

    /// Gets the source of the data
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_source("https://example.com".to_string());
    /// assert_eq!(meta.get_source(), Some("https://example.com".to_string()));
    /// ```
    #[must_use]
    pub fn get_source(&self) -> Option<String> {
        self.map.get("source")?.into_string()
    }

    /// Sets a human-readable summary
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_description("A collection of data".to_string());
    /// ```
    pub fn set_description(&mut self, description: String) {
        self.map.insert("description", XffValue::from(description));
    }

    /// Gets the description
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_description("A collection of data".to_string());
    /// assert_eq!(meta.get_description(), Some("A collection of data".to_string()));
    /// ```
    #[must_use]
    pub fn get_description(&self) -> Option<String> {
        self.map.get("description")?.into_string()
    }

    /// Sets the license
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_license("MIT".to_string());
    /// ```
    pub fn set_license(&mut self, license: String) {
        self.map.insert("license", XffValue::from(license));
    }

    /// Gets the license
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_license("MIT".to_string());
    /// assert_eq!(meta.get_license(), Some("MIT".to_string()));
    /// ```
    #[must_use]
    pub fn get_license(&self) -> Option<String> {
        self.map.get("license")?.into_string()
    }

    /// Sets an arbitrary metadata key-value pair.
    ///
    /// To adhere to the XFF v3 standard, metadata should be flat.
    /// Use `is_flat_value()` to check if a value is suitable for metadata.
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_custom("key", "value");
    /// ```
    pub fn set_custom<S: Into<String>, V: Into<XffValue>>(&mut self, key: S, value: V) {
        self.map.insert(key, value);
    }

    /// Checks if the metadata adheres to the XFF v3 "no nested parents" requirement.
    ///
    /// Metadata can contain primitives or a single level of parent types (Array, Object, Table),
    /// but those parent types cannot contain further nested parents.
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_custom("key", "value");
    /// assert!(meta.is_strict_v3_compliant());
    /// ```
    #[must_use]
    pub fn is_strict_v3_compliant(&self) -> bool {
        self.map.iter().all(|(_, v)| {
            if Self::is_flat_value(v) {
                true
            } else {
                // If it's a parent, it must not contain any other parents
                match v {
                    XffValue::Array(a) => a.values.iter().all(Self::is_flat_value),
                    XffValue::Object(o) => o.map.values().all(Self::is_flat_value),
                    XffValue::OrderedObject(o) => o.iter().all(|(_, val)| Self::is_flat_value(val)),
                    XffValue::Table(t) => t.rows.iter().flatten().all(Self::is_flat_value),
                    _ => false, // Metadata variant inside metadata is forbidden
                }
            }
        })
    }

    /// Helper to check if a value is a "flat" (primitive/specialized) type.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{Metadata, XffValue};
    /// assert!(Metadata::is_flat_value(&XffValue::from(1)));
    /// ```
    #[must_use]
    pub fn is_flat_value(value: &XffValue) -> bool {
        match value {
            XffValue::String(_)
            | XffValue::Number(_)
            | XffValue::Boolean(_)
            | XffValue::DateTime(_)
            | XffValue::Duration(_)
            | XffValue::Uuid(_)
            | XffValue::NaN
            | XffValue::Infinity
            | XffValue::NegInfinity
            | XffValue::Null => true,

            // Parent types and legacy types are not "flat"
            _ => false,
        }
    }

    /// Gets an arbitrary metadata value
    ///
    /// # Example
    /// ```rust
    /// use aequa::{Metadata, XffValue};
    /// let mut meta = Metadata::new();
    /// meta.set_custom("key", "value");
    /// assert_eq!(meta.get_custom("key"), Some(&XffValue::from("value")));
    /// ```
    #[must_use]
    pub fn get_custom(&self, key: &str) -> Option<&XffValue> {
        self.map.get(key)
    }

    /// Returns the number of metadata entries
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// meta.set_creator("Xqhare".to_string());
    /// assert_eq!(meta.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if there are no metadata entries
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let meta = Metadata::new();
    /// assert!(meta.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the underlying object
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let meta = Metadata::new();
    /// let obj = meta.into_object();
    /// ```
    pub fn into_object(self) -> Object {
        self.map
    }

    /// Returns a reference to the underlying object
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let meta = Metadata::new();
    /// let obj = meta.as_object();
    /// ```
    pub fn as_object(&self) -> &Object {
        &self.map
    }

    /// Returns a mutable reference to the underlying object
    ///
    /// # Example
    /// ```rust
    /// use aequa::Metadata;
    /// let mut meta = Metadata::new();
    /// let obj = meta.as_mut_object();
    /// ```
    pub fn as_mut_object(&mut self) -> &mut Object {
        &mut self.map
    }
}

impl From<Object> for Metadata {
    fn from(map: Object) -> Self {
        Self { map }
    }
}

impl From<Metadata> for Object {
    fn from(meta: Metadata) -> Self {
        meta.map
    }
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Metadata({})", self.map)
    }
}
