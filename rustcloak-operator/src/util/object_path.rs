use serde_json::Value;

pub trait ObjectPathMut {
    fn path_mut(&mut self, path: &str) -> Option<&mut Value>;
}

impl ObjectPathMut for Value {
    fn path_mut(&mut self, path: &str) -> Option<&mut Value> {
        let mut current = self;
        for key in path.split('.') {
            match current {
                Value::Array(arr) => {
                    let index = key.parse::<usize>().ok()?;
                    current = arr.get_mut(index)?;
                }
                Value::Object(obj) => {
                    if obj.get(key).is_none() {
                        obj.insert(
                            key.to_string(),
                            Value::Object(Default::default()),
                        );
                    }
                    current = obj.get_mut(key)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }
}
