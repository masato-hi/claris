use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub trait YamlExt {
    fn f64_val(&self, key: &str) -> Option<f64>;
    fn f32_val(&self, key: &str) -> Option<f32>;
    fn i64_val(&self, key: &str) -> Option<i64>;
    fn i32_val(&self, key: &str) -> Option<i32>;
    fn bool_val(&self, key: &str) -> Option<bool>;
    fn str_val(&self, key: &str) -> Option<&str>;
    fn string_val(&self, key: &str) -> Option<String>;
    fn array_val(&self, key: &str) -> Option<&Vec<Yaml>>;
    fn hash_val(&self, key: &str) -> Option<&Hash>;
    fn entry(&self, key: &str) -> Option<&Yaml>;
}

impl YamlExt for Yaml {
    fn f64_val(&self, key: &str) -> Option<f64> {
        match self[key].as_i64() {
            Some(x) => Some(x as f64),
            None => match self[key].as_f64() {
                Some(x) => Some(x),
                None => None,
            },
        }
    }

    fn f32_val(&self, key: &str) -> Option<f32> {
        match self.f64_val(key) {
            Some(x) => Some(x as f32),
            None => None,
        }
    }

    fn i64_val(&self, key: &str) -> Option<i64> {
        self[key].as_i64()
    }

    fn i32_val(&self, key: &str) -> Option<i32> {
        match self.i64_val(key) {
            Some(x) => Some(x as i32),
            None => None,
        }
    }

    fn bool_val(&self, key: &str) -> Option<bool> {
        self[key].as_bool()
    }

    fn str_val(&self, key: &str) -> Option<&str> {
        match &self[key] {
            Yaml::String(x) => Some(x.as_str()),
            _ => None,
        }
    }

    fn string_val(&self, key: &str) -> Option<String> {
        match &self[key] {
            Yaml::String(x) => Some(x.clone()),
            _ => None,
        }
    }

    fn array_val(&self, key: &str) -> Option<&Vec<Yaml>> {
        match &self[key] {
            Yaml::Array(x) => Some(x),
            _ => None,
        }
    }

    fn hash_val(&self, key: &str) -> Option<&Hash> {
        match &self[key] {
            Yaml::Hash(x) => Some(x),
            _ => None,
        }
    }

    fn entry(&self, key: &str) -> Option<&Yaml> {
        Some(&self[key])
    }
}

impl YamlExt for Hash {
    fn f64_val(&self, key: &str) -> Option<f64> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            match self[k].as_i64() {
                Some(x) => Some(x as f64),
                None => match self[k].as_f64() {
                    Some(x) => Some(x),
                    None => None,
                },
            }
        } else {
            None
        }
    }

    fn f32_val(&self, key: &str) -> Option<f32> {
        match self.f64_val(key) {
            Some(x) => Some(x as f32),
            None => None,
        }
    }

    fn i64_val(&self, key: &str) -> Option<i64> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            self[k].as_i64()
        } else {
            None
        }
    }

    fn i32_val(&self, key: &str) -> Option<i32> {
        match self.i64_val(key) {
            Some(x) => Some(x as i32),
            None => None,
        }
    }

    fn bool_val(&self, key: &str) -> Option<bool> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            self[k].as_bool()
        } else {
            None
        }
    }

    fn str_val(&self, key: &str) -> Option<&str> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            match &self[k] {
                Yaml::String(x) => Some(x.as_str()),
                _ => None,
            }
        } else {
            None
        }
    }

    fn string_val(&self, key: &str) -> Option<String> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            match &self[k] {
                Yaml::String(x) => Some(x.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    fn array_val(&self, key: &str) -> Option<&Vec<Yaml>> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            match &self[k] {
                Yaml::Array(x) => Some(x),
                _ => None,
            }
        } else {
            None
        }
    }

    fn hash_val(&self, key: &str) -> Option<&Hash> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            match &self[k] {
                Yaml::Hash(x) => Some(x),
                _ => None,
            }
        } else {
            None
        }
    }

    fn entry(&self, key: &str) -> Option<&Yaml> {
        let k = &Yaml::from_str(key);
        if self.contains_key(k) {
            Some(&self[k])
        } else {
            None
        }
    }
}
