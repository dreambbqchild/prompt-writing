use std::{path::Path, sync::{Arc, RwLock}, time::Duration};
use chrono::{DateTime, Utc};
use rand::prelude::*;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};

use super::DBExtensions;

#[derive(Clone)]
pub struct WritingPromptRepository {
    db: Arc<DB>,
    prompt_count_lock: Arc<RwLock<i32>>
}

static SESSION_TIMEOUT: &str = "session_timeouts";
static SESSION_CURRENT_PROMPT: &str = "session_current_prompt";
static PROMPTS: &str = "prompts";
static METADATA: &str = "metadata";

static PROMPT_COUNT : &str = "prompt_count";

impl WritingPromptRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> WritingPromptRepository {
        let session_timeout_cf = ColumnFamilyDescriptor::new(SESSION_TIMEOUT, Options::default());
        let session_current_prompt_cf = ColumnFamilyDescriptor::new(SESSION_CURRENT_PROMPT, Options::default());
        let prompts_cf = ColumnFamilyDescriptor::new(PROMPTS, Options::default());
        let metadata_cf = ColumnFamilyDescriptor::new(METADATA, Options::default());

        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);

        let db = Arc::new(DB::open_cf_descriptors(&db_opts, path, vec![session_timeout_cf, session_current_prompt_cf, prompts_cf, metadata_cf]).unwrap());
        let prompt_count_lock = Arc::new(RwLock::new(0));

        WritingPromptRepository {
            db, prompt_count_lock
        }
    }

    pub fn set_timelimit<S: AsRef<str>>(&self, key: S, seconds: u64) -> DateTime<Utc> {
        let end_at = Utc::now() + Duration::from_secs(seconds) + Duration::from_millis(500);
        self.db.put_i64_cf(SESSION_TIMEOUT, key.as_ref(), end_at.timestamp());
        end_at
    }

    pub fn extract_datetime<S: AsRef<str>>(&self, key: S) -> DateTime<Utc> {
        let value = self.db.get_i64_cf(SESSION_TIMEOUT, key.as_ref());
        DateTime::from_timestamp(value, 0).unwrap()
    }

    pub fn random_prompt_for_session(&self, session_key: &String) -> Result<String, &str> {
        let _unused = self.prompt_count_lock.read().unwrap();
        let count = self.db.get_i64_cf(METADATA, PROMPT_COUNT);
        if count == 0 {
            Err("Prompt Database is Empty")
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..count);
            let prompt_key = format!("{index}");
            let prompt = self.db.get_string_cf(PROMPTS, &prompt_key);

            self.db.put_string_cf(SESSION_CURRENT_PROMPT, session_key, &prompt);

            Ok(prompt)
        }
    }

    pub fn get_current_prompt_for_session(&self, session_key: &String) -> String {
        self.db.get_string_cf(SESSION_CURRENT_PROMPT, session_key)
    }

    pub fn add_prompt<S: AsRef<str>>(&self, value: S) -> i64 {
        let _unused = self.prompt_count_lock.write().unwrap();
        let mut count = self.db.get_i64_cf(METADATA, PROMPT_COUNT);
        let key = format!("{count}");

        self.db.put_string_cf(PROMPTS, &key, value.as_ref());
        
        count = count + 1;
        self.db.put_i64_cf(METADATA, PROMPT_COUNT, count);

        count
    }

    pub fn get_prompt<S: AsRef<str>>(&self, key: S) -> String {
        let _unused = self.prompt_count_lock.read().unwrap();
        self.db.get_string_cf(PROMPTS, key.as_ref())
    }

    pub fn get_prompt_count(&self) -> i64 {
        let _unused = self.prompt_count_lock.read().unwrap();
        self.db.get_i64_cf(METADATA, PROMPT_COUNT)
    }
}