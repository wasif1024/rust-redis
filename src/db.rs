use bytes::Bytes;
use std::collections::HashMap;
#[derive(Default)]
pub struct Db {
    pub enteries: HashMap<String, Bytes>,
}
/*impl Default for Db {
    fn default() -> Self {
     Self::new()
     }
    }*/
impl Db {
    /*pub fn new() -> Self {
        Db {
            enteries: HashMap::new(),
        }
    }*/
    pub fn write(&mut self, attrs: &[String]) -> Result<&str, &'static str> {
        //dbg!(&attrs);
        let key = &attrs[1];
        let value = &attrs[2];
        let val = value.clone();
        let resp: &Option<Bytes> = &self.enteries.insert(String::from(key), Bytes::from(val));

        match resp {
            Some(_val) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }
    pub fn read(&mut self, arr: &[String]) -> Result<&Bytes, &'static str> {
        dbg!(&arr);
        let key = &arr[1];
        let query_result = self.enteries.get(key);
    
        if let Some(value) = query_result {
            Ok(value)
        } else {
            Err("no such key found")
        }
    }
}
