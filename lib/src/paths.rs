use std::sync::{Arc, Mutex};

use indexmap::IndexSet;
use tower_lsp_server::lsp_types::Uri;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Default, Debug)]
pub struct PathInterner {
    map: Arc<Mutex<IndexSet<Uri>>>,
}

impl PathInterner {
    /// Get the id corresponding to `path`.
    ///
    /// If `path` does not exists in `self`, returns [`None`].
    pub fn get(&self, path: &Uri) -> Option<FileId> {
        let map = self.map.lock().unwrap();
        map.get_index_of(path).map(FileId)
    }

    /// Insert `path` in `self`.
    ///
    /// - If `path` already exists in `self`, returns its associated id;
    /// - Else, returns a newly allocated id.
    pub fn insert(&self, path: Uri) -> FileId {
        let mut map = self.map.lock().unwrap();
        let (id, _added) = map.insert_full(path);
        FileId(id)
    }

    /// Returns the path corresponding to `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exists in `self`.
    pub fn lookup(&self, id: &FileId) -> Uri {
        let map = self.map.lock().unwrap();
        map.get_index(id.0).unwrap().clone()
    }
}
