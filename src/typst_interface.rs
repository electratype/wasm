use typst::{World, doc};
use wasm_bindgen::prelude::*;

use js_sys::{Array};

use crate::world::ElectraWorld;
use crate::{log, log_string};

use comemo::{memoize, track, Tracked};

#[wasm_bindgen]
pub struct Typst {
    world: ElectraWorld
}

#[wasm_bindgen]
impl Typst {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Typst {
        Self {
            world: ElectraWorld::new()
        }
    }

    pub fn set_source(&mut self, text: String) {
        self.world.set_source(text);
    }

    pub fn edit_source(&mut self, start: usize, end: usize, with: String) {
        self.world.edit_source(start, end, with);
    }

    pub fn supply_fonts(&mut self, fonts: Array) {
        self.world.supply_fonts(fonts)
    }

    pub fn compile_svg(&mut self) -> Result<Array, JsValue> {

        let output_array = Array::new();

        let mut tracer = typst::eval::Tracer::new();
        let result = typst::compile(&self.world, &mut tracer);

        match result {
            Ok(document) => {
                // Add total number of pages to the array
                output_array.push(&JsValue::from_str(&document.pages.len().to_string().as_str()));

                let cache = self.world.export_cache();
                for (i, frame) in document.pages.iter().enumerate() {

                    if cache.is_cached(i, frame) {
                        continue;
                    }

                    let svg = typst::export::svg(frame);

                    let svg_id = JsValue::from_str(&i.to_string().as_str());
                    let svg_string  = JsValue::from_str(&svg);

                    output_array.push(&svg_id);
                    output_array.push(&svg_string);
                }

                Ok(output_array)
            },
            Err(error) => {
                Ok(output_array)
            }
        }

    }

    /*
    pub fn compile_pdf(&mut self) {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let element = document.get_element_by_id(ELEMENT_ID).unwrap();

        //log("Starting compiling");

        let mut tracer = typst::eval::Tracer::new();
        //log("Tracer initialization successful");

        let result = typst::compile(self, &mut tracer);

        //log_string(format!("{:?}", world.source(typst::syntax::FileId::new(None, typst::syntax::VirtualPath::new(std::path::Path::new("main.typ")))).unwrap().text()));

        match result {
            Ok(result) => {

                //log_string(format!("Length {:?}", result.pages.len()));
                //log_string(format!("Document {:?}", result));

                let doc_bytes = typst::export::pdf(&result);
                let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&doc_bytes) }.into());
                let array = js_sys::Array::new();
                array.push(&uint8arr.buffer());
                let blob = Blob::new_with_u8_array_sequence_and_options(
                    &array,
                    web_sys::BlobPropertyBag::new().type_("application/pdf"),
                ).unwrap();

                let result_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

                element.set_inner_html(
                    &(String::from("<embed src=\"") + &result_url + &String::from("\" type=\"application/pdf\">"))
                );

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
    */
}