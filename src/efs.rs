use std::{path::Path, ops::Range};

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
            file_source: Some(Source::new(id, "source".to_string())),
            file_id: id,
        }
    }

    pub fn set_source(&mut self, source: String) {
        //self.file_source = Some(Source::new(self.file_id, source));

        let s_slice: &str = &*source;  // s  : String 
        self.file_source.as_mut().unwrap().replace(s_slice);
    }

    pub fn edit_source(&mut self, start: usize, end: usize, with: String) {

        let range = Range{start: start, end: end};
        let s: &str = &*with;

        self.file_source.as_mut().unwrap().edit(range, s);
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