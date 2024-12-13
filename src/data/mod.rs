pub mod writing_prompt_repository;

use rocksdb::DB;
use std::sync::Arc;

trait VecExtensions {
    fn to_i64(&self) -> i64;
}

impl VecExtensions for Vec<u8> {
    fn to_i64(&self) -> i64 {
        let mut array = [0u8; 8]; 
        let len = std::cmp::min(self.len(), 8); 
        array[..len].copy_from_slice(&self[..len]); 
        i64::from_be_bytes(array) 
    }
}

trait DBExtensions {
    fn put_i64_cf<S: AsRef<str>>(&self, cf: S, key: S, value: i64);
    fn get_i64_cf<S: AsRef<str>>(&self, cf: S, key: S) -> i64;

    fn put_string_cf<S: AsRef<str>>(&self, cf: S, key: S, value: S);
    fn get_string_cf<S: AsRef<str>>(&self, cf: S, key: S) -> String;
}

impl DBExtensions for Arc<DB> {
    fn put_i64_cf<S: AsRef<str>>(&self, cf: S, key: S, value: i64) {
        let cf_handle = self.cf_handle(cf.as_ref()).unwrap();
        self.put_cf(cf_handle, key.as_ref(), value.to_be_bytes()).unwrap();
    }

    fn get_i64_cf<S: AsRef<str>>(&self, cf: S, key: S) -> i64 {
        let cf_handle = self.cf_handle(cf.as_ref()).unwrap();
        let value = self.get_cf(cf_handle, key.as_ref()).unwrap().unwrap_or_else(|| vec![0,0,0,0,0,0,0,0]);
        value.to_i64()
    }

    fn put_string_cf<S: AsRef<str>>(&self, cf: S, key: S, value: S) {
        let cf_handle = self.cf_handle(cf.as_ref()).unwrap();
        self.put_cf(cf_handle, key.as_ref(), value.as_ref().as_bytes()).unwrap();
    }

    fn get_string_cf<S: AsRef<str>>(&self, cf: S, key: S) -> String {
        let cf_handle = self.cf_handle(cf.as_ref()).unwrap();
        let value = self.get_cf(cf_handle, key.as_ref()).unwrap().unwrap_or_else(|| vec![]);
        String::from_utf8(value).unwrap_or_else(|_| String::new())
    }
}