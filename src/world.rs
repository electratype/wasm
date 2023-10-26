/*
    To be able to compile input by Typst, one needs to provide implementation of 'World' trait defined in typst standard library.
*/

use std::{mem, cell::OnceCell, path::Path, fs};

use js_sys::{Array, ArrayBuffer};
use wasm_bindgen::prelude::*;
use typst::{World, doc::Frame, diag::FileError};

use chrono::{Local, DateTime, TimeZone, FixedOffset, };

use comemo::{Prehashed, track};
use web_sys::Blob;

use crate::{ELEMENT_ID, log, log_string};

use typst::{
    eval::{Library, Bytes, Datetime},
    font::{FontBook, Font},
    syntax::{FileId, Source, PackageSpec},
    diag::{FileResult},
    util::hash128,
};

use typst_library::{
    prelude::{EcoString}
};

use crate::efs::*;

/// Holds details about the location of a font and lazily the font itself.
struct FontSlot {
    buffer: Bytes,
    index: u32,
    font: OnceCell<Option<Font>>
}


#[wasm_bindgen]
pub struct ElectraWorld {
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    fs: ElectraFileSystem,
    fonts: Vec<FontSlot>,
    export_cache: ExportCache
}

#[wasm_bindgen]
impl ElectraWorld {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ElectraWorld {
        Self {
            library: Prehashed::new(typst_library::build()),
            book: Prehashed::new(FontBook::new()),
            fs: ElectraFileSystem::new(),
            fonts: vec![],
            export_cache: ExportCache::new()
        }
    }

    pub fn set_source(&mut self, text: String) {
        self.fs.set_source(text);
    }

    pub fn edit_source(&mut self, start: usize, end: usize, with: String) {
        self.fs.edit_source(start, end, with);
    }

    pub fn supply_fonts(&mut self, fonts: Array) {
        let hashed_book = mem::replace(&mut self.book, Prehashed::default());
        let mut book: FontBook = hashed_book.into_inner();
        fonts.for_each(&mut |font: JsValue, _, _| {
            let bytes: ArrayBuffer = font.dyn_into().unwrap();
            let bytes = js_sys::Uint8Array::new(&bytes).to_vec();
            let buffer = Bytes::from(&bytes[..]);
            for (i, font) in Font::iter(buffer.clone()).enumerate() {
                book.push(font.info().clone());
                self.fonts.push(FontSlot {
                    buffer: buffer.clone(),
                    index: i as u32,
                    font: OnceCell::from(Some(font)),
                });
            }
        });
        self.book = Prehashed::new(book);
    }
}

impl ElectraWorld {
    pub fn export_cache(&mut self) -> &mut ExportCache {
        &mut self.export_cache
    }
}

#[track]
impl World for ElectraWorld {

    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) ->  &Prehashed<FontBook> {
        &self.book
    }

    fn file(&self, id:FileId) -> FileResult<Bytes> {
        self.fs.file(id)
    }

    fn main(&self) -> Source {
        self.fs.source()
    }

    fn font(&self, index:usize) -> Option<Font> {
        let slot = &self.fonts[index];
        slot.font
            .get_or_init(|| Font::new(slot.buffer.clone(), slot.index))
            .clone()
    }
    

    fn packages(&self) ->  &[(PackageSpec, Option<EcoString>)] {
        &[]
    }

    fn source(&self, id:FileId) -> FileResult<Source> {
        Ok(self.fs.source())
    }

    fn today(&self, offset:Option<i64>) -> Option<Datetime> {

        /*
        let now = DateTime::from_naive_utc_and_offset(Local::now(), TimeZone::from_offset(offset))

        let tz_offset = FixedOffset::east_opt(i32::from(offset.unwrap()));


        let naive_utc = now.naive_utc();
        let offset = now.offset().clone();
        // Serialize, pass through FFI... and recreate the `DateTime`:
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        */

        Datetime::from_ymd_hms(0, 0, 0, 0, 0, 0)
        
    }

}

pub struct ExportCache {
    /// The hashes of last compilation's frames.
    pub cache: Vec<u128>,
}

impl ExportCache {
    /// Creates a new export cache.
    pub fn new() -> Self {
        Self { cache: Vec::with_capacity(32) }
    }

    /// Returns true if the entry is cached and appends the new hash to the
    /// cache (for the next compilation).
    pub fn is_cached(&mut self, i: usize, frame: &Frame) -> bool {
        let hash = hash128(frame);

        if i >= self.cache.len() {
            self.cache.push(hash);
            return false;
        }

        std::mem::replace(&mut self.cache[i], hash) == hash
    }
}