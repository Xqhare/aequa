use super::{Object, XffValue};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A metadata object for XFF files.
/// 
/// Provides high-level context about the file, such as creator, source, and license.
pub struct Metadata {
    /// The underlying data storage
    pub map: Object,
}

impl Metadata {
    /// Creates a new, empty Metadata object
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the creator of the file
    pub fn set_creator(&mut self, creator: String) {
        self.map.insert("creator", XffValue::String(creator));
    }

    /// Gets the creator of the file
    pub fn get_creator(&self) -> Option<String> {
        self.map.get("creator")?.into_string()
    }

    /// Sets the creation timestamp (milliseconds since epoch)
    pub fn set_created_at(&mut self, timestamp: u64) {
        self.map.insert("created_at", XffValue::DateTime(timestamp));
    }

    /// Gets the creation timestamp
    pub fn get_created_at(&self) -> Option<u64> {
        if let Some(XffValue::DateTime(dt)) = self.map.get("created_at") {
            Some(*dt)
        } else {
            None
        }
    }

    /// Sets the source of the data
    pub fn set_source(&mut self, source: String) {
        self.map.insert("source", XffValue::String(source));
    }

    /// Gets the source of the data
    pub fn get_source(&self) -> Option<String> {
        self.map.get("source")?.into_string()
    }

    /// Sets a human-readable summary
    pub fn set_description(&mut self, description: String) {
        self.map.insert("description", XffValue::String(description));
    }

    /// Gets the description
    pub fn get_description(&self) -> Option<String> {
        self.map.get("description")?.into_string()
    }

    /// Sets the license
    pub fn set_license(&mut self, license: String) {
        self.map.insert("license", XffValue::String(license));
    }

    /// Gets the license
    pub fn get_license(&self) -> Option<String> {
        self.map.get("license")?.into_string()
    }

    /// Sets an arbitrary metadata key-value pair.
    /// 
    /// To adhere to the XFF v3 standard, metadata should be flat.
    /// Use `is_flat_value()` to check if a value is suitable for metadata.
    pub fn set_custom<S: Into<String>, V: Into<XffValue>>(&mut self, key: S, value: V) {
        self.map.insert(key, value);
    }

    /// Checks if the metadata adheres to the XFF v3 "no nested parents" requirement.
    /// 
    /// Metadata can contain primitives or a single level of parent types (Array, Object, Table),
    /// but those parent types cannot contain further nested parents.
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
    pub fn is_flat_value(value: &XffValue) -> bool {
        match value {
            XffValue::String(_) |
            XffValue::Number(_) |
            XffValue::Boolean(_) |
            XffValue::DateTime(_) |
            XffValue::Duration(_) |
            XffValue::Uuid(_) |
            XffValue::NaN |
            XffValue::Infinity |
            XffValue::NegInfinity |
            XffValue::Null => true,
            
            // Parent types and legacy types are not "flat"
            _ => false,
        }
    }

    /// Gets an arbitrary metadata value
    pub fn get_custom(&self, key: &str) -> Option<&XffValue> {
        self.map.get(key)
    }

    /// Returns the number of metadata entries
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if there are no metadata entries
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
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
