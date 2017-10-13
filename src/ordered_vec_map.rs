// Provides an ordered map with a method to query for partial matches.
// This is useful for disambiguation.
pub struct OrderedVecMap<KeyT, ValueT> where KeyT: std::cmp::Ord {
    data: Vec<(KeyT, ValueT)>,
}

impl<KeyT, ValueT> OrderedVecMap<KeyT, ValueT> where KeyT: std::cmp::Ord {
    pub fn new() -> Self {
        OrderedVecMap {
            data: Vec::<(KeyT, ValueT)>::new(),
        }
    }

    // Returns true if the value is inserted, false if overwritten.
    pub fn insert(&mut self, mut datum: (KeyT, ValueT)) -> bool {
        match self.data.binary_search_by(|probe| probe.0.cmp(&datum.0)) {
            Ok(idx) => {
                *self.data.get_mut(idx).unwrap() = datum;
                return false;
            },
            Err(idx) => {
                self.data.insert(idx, datum);
                return true;
            },
        };
    }

    pub fn find(&self, query: &KeyT) -> Option<&ValueT> {
        match self.data.binary_search_by(|probe| probe.0.cmp(query)) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(idx) => None,
        }
    }

    pub fn find_by<'a, F>(&self, f: F) -> Option<&ValueT>
            where F: Fn(&(KeyT, ValueT)) -> std::cmp::Ordering,
                  KeyT: 'a,
                  ValueT: 'a {
        match self.data.binary_search_by(f) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(_) => None
        }
    }
}

impl<KeyT, ValueT> From<Vec<(KeyT, ValueT)>> for OrderedVecMap<KeyT, ValueT>
        where KeyT: std::cmp::Ord {
    fn from(mut data: Vec<(KeyT, ValueT)>) -> OrderedVecMap<KeyT, ValueT> {
        data.sort_by(|a, b| a.0.cmp(&b.0));
        OrderedVecMap {
            data: data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_match() {
    }
}
