use std::{collections::HashMap, fs, io::Result, sync::{Arc, Mutex}};

pub trait FS: Sync + Send {
    fn read(&self, path: &str) -> Result<String>;
    fn write(&self, path: &str, content: &str) -> Result<()>;
}

pub struct MemoryFS {
    files: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryFS {
    pub fn new() -> Self {
        MemoryFS {
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl FS for MemoryFS {
    fn read(&self, path: &str) -> Result<String> {
        let files = self.files.lock().unwrap();
        Ok(files.get(path).unwrap().clone())
    }

    fn write(&self, path: &str, content: &str) -> Result<()> {
        let mut files = self.files.lock().unwrap();
        files.insert(path.to_string(), content.to_string());

        Ok(())
    }
}

pub struct LocalFs {}

impl LocalFs {
    pub fn new() -> Self {
        LocalFs {}
    }
}

impl FS for LocalFs {
    fn read(&self, path: &str) -> Result<String> {
        fs::read_to_string(path)
    }

    fn write(&self, path: &str, content: &str) -> Result<()> {
        fs::write(path, content)
    }
}
