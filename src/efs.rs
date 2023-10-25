use std::path::Path;

use typst::{syntax::{VirtualPath, FileId, Source}, diag::FileResult, eval::Bytes};
use wasm_bindgen::prelude::*;

pub const FILE_NAME: &str = "main.typ";

pub struct ElectraFileSystem {

    file_source: Option<Source>,
    file_id: FileId,

}

impl ElectraFileSystem {

    pub fn new() -> ElectraFileSystem {

        let file_path = VirtualPath::new(Path::new(FILE_NAME));

        let id = FileId::new(None, file_path);

        Self {
            file_source: None,
            file_id: id,
        }
    }

    pub fn set_source(&mut self, source: String) {
        self.file_source = Some(Source::new(self.file_id, source));
    }
    
}

impl ElectraFileSystem {

    pub fn file(&self, id: FileId) -> FileResult<Bytes> {

        let file_source = self.file_source.as_ref().unwrap();
        let file_text = file_source.text();
        let file_bytes = file_text.as_bytes();
        Ok(Bytes::from(file_bytes))

    }

    pub fn source(&self) -> Source {
        self.file_source.clone().expect("File source has not been set!")
    }
}