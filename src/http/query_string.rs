use std::collections::HashMap;

pub struct QueryString<'buf> {
    // data: HashMap<&str, &str>,
    // note: by default we don't have errors here. Because file is not imported.
    // once we do it, we have the lifetime errors
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        // let mut vec = Vec::new();
                        // vec.push(val);
                        // vec.push(prev_val);

                        // a macro can make it quicker
                        // let mut vec = vec![prev_val, val];

                        // since we are de-referencing the value pointed by 'existing',

                        *existing = Value::Multiple(vec![prev_value, val])
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
