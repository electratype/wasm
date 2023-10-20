/*
    To be able to compile input by Typst, one needs to provide implementation of 'World' trait defined in typst standard library.
*/

use std::{mem, cell::OnceCell};

use js_sys::{Array, ArrayBuffer};
use wasm_bindgen::prelude::*;
use typst::World;

use chrono::{Local, DateTime, TimeZone, FixedOffset, };

use comemo::Prehashed;

use crate::{ELEMENT_ID, log, log_string};

use typst::{
    eval::{Library, Bytes, Datetime},
    font::{FontBook, Font},
    syntax::{FileId, Source, PackageSpec},
    diag::{FileResult}
};

use typst_library::{
    prelude::{EcoString}
};

use crate::efs::*;

/// Holds details about the location of a font and lazily the font itself.
struct FontSlot {
    buffer: Bytes,
    index: u32,
    font: OnceCell<Option<Font>>,
}

#[wasm_bindgen]
pub struct ElectraWorld {
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    fs: ElectraFileSystem,
    fonts: Vec<FontSlot>,
}

#[wasm_bindgen]
impl ElectraWorld {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ElectraWorld {
        Self {
            library: Prehashed::new(typst_library::build()),
            book: Prehashed::new(FontBook::new()),
            fs: ElectraFileSystem::new(),
            fonts: vec![]
        }
    }

    pub fn set_source(&mut self, text: String) {
        self.fs.set_source(text);
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

    pub fn compile(&mut self) {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let element = document.get_element_by_id(ELEMENT_ID).unwrap();

        log("Starting compiling");

        let mut tracer = typst::eval::Tracer::new();
        log("Tracer initialization successful");

        let result = typst::compile(self, &mut tracer);

        //log_string(format!("{:?}", world.source(typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new(std::path::Path::new("main.typ")))).unwrap().text()));

        match result {
            Ok(result) => {

                log_string(format!("Length {:?}", result.pages.len()));
                log_string(format!("Document {:?}", result));

                for frame in result.pages.iter() {

                    log_string(format!("Page {:?}", frame));

                    let svg = typst::export::svg(frame);
                    
                    log_string(svg.clone());

                    element.set_inner_html(&svg);
                }
                log("Done rendering");

                for v in tracer.values().iter() {

                    log_string(v.clone().display().plain_text().to_string());

                }

            },
            Err(error) => {
                panic!("{:?}", error)
            }
        }

        /*
        match result {
            Ok(document) => {
                let fill = Color::WHITE;
                let images = document
                    .pages
                    .into_iter()
                    .map(|page| typst::export::render(&page, 144.0, fill))
                    .map(|image| image.encode_png().expect("Could not encode as PNG"))
                    .collect();
                Ok(images)
            }
            Err(errors) => Err(errors
                .into_iter()
                .map(|error| JsValue::from_str(&error.message))
                .collect::<Array>()
                .into()),
            }
        */
    }
}

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