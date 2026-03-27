use std::collections::{BTreeMap, HashMap};

use super::XffValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// An object made up of key-value pairs of XFF values where the order of insertion is preserved.
///
/// Internally uses a `Vec<(String, XffValue)>`.
pub struct OrderedObject {
    /// The underlying key-value pairs
    pub pairs: Vec<(String, XffValue)>,
}

impl OrderedObject {
    /// Creates a new, empty ordered object.
    #[must_use] 
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Creates an ordered object from a vector of pairs.
    #[must_use] 
    pub fn from_vec(pairs: Vec<(String, XffValue)>) -> Self {
        Self { pairs }
    }

    /// Returns the number of elements in the ordered object.
    #[must_use] 
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns `true` if the ordered object is empty.
    #[must_use] 
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Adds a key-value pair to the end of the ordered object.
    ///
    /// If you want to replace an existing key while preserving order, use `insert`.
    pub fn push<K: Into<String>, V: Into<XffValue>>(&mut self, key: K, value: V) {
        self.pairs.push((key.into(), value.into()));
    }

    /// Inserts a key-value pair into the ordered object.
    ///
    /// If the key already exists, the value is updated and the position is preserved.
    /// If the key does not exist, the pair is added to the end.
    pub fn insert<K: Into<String>, V: Into<XffValue>>(&mut self, key: K, value: V) {
        let key = key.into();
        let value = value.into();
        if let Some(pos) = self.pairs.iter().position(|(k, _)| k == &key) {
            self.pairs[pos].1 = value;
        } else {
            self.pairs.push((key, value));
        }
    }

    /// Removes a key-value pair from the ordered object.
    pub fn remove(&mut self, key: &str) -> Option<XffValue> {
        if let Some(pos) = self.pairs.iter().position(|(k, _)| k == key) {
            Some(self.pairs.remove(pos).1)
        } else {
            None
        }
    }

    /// Clears the ordered object.
    pub fn clear(&mut self) {
        self.pairs.clear();
    }

    /// Returns a reference to the value associated with the key, if it exists.
    ///
    /// Note: This is an O(n) operation.
    #[must_use] 
    pub fn get(&self, key: &str) -> Option<&XffValue> {
        self.pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Returns a mutable reference to the value associated with the key, if it exists.
    ///
    /// Note: This is an O(n) operation.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut XffValue> {
        self.pairs.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Returns `true` if the ordered object contains the supplied key.
    #[must_use] 
    pub fn contains_key(&self, key: &str) -> bool {
        self.pairs.iter().any(|(k, _)| k == key)
    }

    /// Returns a reference to the pair at the given index.
    #[must_use] 
    pub fn get_index(&self, index: usize) -> Option<&(String, XffValue)> {
        self.pairs.get(index)
    }

    /// Returns a mutable reference to the pair at the given index.
    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut (String, XffValue)> {
        self.pairs.get_mut(index)
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> std::slice::Iter<'_, (String, XffValue)> {
        self.pairs.iter()
    }

    /// Returns a mutable iterator over the key-value pairs.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (String, XffValue)> {
        self.pairs.iter_mut()
    }
}

impl IntoIterator for OrderedObject {
    type Item = (String, XffValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pairs.into_iter()
    }
}

/// An iterator over the key-value pairs of an `OrderedObject`.
pub struct OrderedObjectIter<'a>(std::slice::Iter<'a, (String, XffValue)>);

impl<'a> Iterator for OrderedObjectIter<'a> {
    type Item = (&'a String, &'a XffValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (k, v))
    }
}

impl<'a> IntoIterator for &'a OrderedObject {
    type Item = (&'a String, &'a XffValue);
    type IntoIter = OrderedObjectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedObjectIter(self.pairs.iter())
    }
}

/// A mutable iterator over the key-value pairs of an `OrderedObject`.
pub struct OrderedObjectIterMut<'a>(std::slice::IterMut<'a, (String, XffValue)>);

impl<'a> Iterator for OrderedObjectIterMut<'a> {
    type Item = (&'a String, &'a mut XffValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (&*k, v))
    }
}

impl<'a> IntoIterator for &'a mut OrderedObject {
    type Item = (&'a String, &'a mut XffValue);
    type IntoIter = OrderedObjectIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedObjectIterMut(self.pairs.iter_mut())
    }
}

impl<S, V> From<Vec<(S, V)>> for OrderedObject
where
    S: Into<String>,
    V: Into<XffValue>,
{
    fn from(pairs: Vec<(S, V)>) -> Self {
        Self {
            pairs: pairs
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl<S, V> From<HashMap<S, V>> for OrderedObject
where
    S: Into<String>,
    V: Into<XffValue>,
{
    fn from(map: HashMap<S, V>) -> Self {
        let mut out = OrderedObject::new();
        for (k, v) in map {
            out.push(k.into(), v.into());
        }
        out
    }
}

impl<S, V> From<BTreeMap<S, V>> for OrderedObject
where
    S: Into<String>,
    V: Into<XffValue>,
{
    fn from(map: BTreeMap<S, V>) -> Self {
        let mut out = OrderedObject::new();
        for (k, v) in map {
            out.push(k.into(), v.into());
        }
        out
    }
}

impl From<OrderedObject> for Vec<(String, XffValue)> {
    fn from(obj: OrderedObject) -> Self {
        obj.pairs
    }
}


impl std::ops::Index<usize> for OrderedObject {
    type Output = (String, XffValue);

    fn index(&self, index: usize) -> &Self::Output {
        &self.pairs[index]
    }
}

impl std::ops::IndexMut<usize> for OrderedObject {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pairs[index]
    }
}

impl std::ops::Index<&str> for OrderedObject {
    type Output = XffValue;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::Index<String> for OrderedObject {
    type Output = XffValue;

    fn index(&self, index: String) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl std::fmt::Display for OrderedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{(ordered) ")?;
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{k}: {v}")?;
        }
        write!(f, "}}")
    }
}
