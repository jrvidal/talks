extern crate cfg_if;
extern crate exif;
extern crate js_sys;
extern crate wasm_bindgen;

mod utils;

use exif::{Error as ExifError, Value as ExifValue};
use std::io::{BufRead, Cursor};
use wasm_bindgen::prelude::*;

/// Parses metadata of a JPG image (passed as a Uint8Array) and calls (synchronously)
/// `callback` with each of the `tag`, `value` pairs (as strings), or throws if it
/// encounters an error.
#[wasm_bindgen]
pub fn get_exif(source: &[u8], callback: &js_sys::Function) {
    let fields = match get_exif_from_source(Cursor::new(source)) {
        Ok(result) => result,
        Err(e) => wasm_bindgen::throw_str(&format!("{:?}", e)),
    };

    let this = JsValue::NULL;

    for field in fields {
        let tag = JsValue::from_str(&field.tag);
        let value = JsValue::from_str(&field.value);
        if let Err(e) = callback.call2(&this, &tag, &value) {
            wasm_bindgen::throw_val(e);
        }
    }
}

/// A simple, owned, already-parsed Tiff field
#[derive(Debug)]
struct ParsedField {
    tag: String,
    value: String,
}

/// The generic version of `get_exif`.
fn get_exif_from_source<R: BufRead>(
    mut source: R,
) -> Result<Vec<ParsedField>, ExifError> {
    let raw = exif::get_exif_attr_from_jpeg(&mut source)?;

    let (fields, _) = exif::parse_exif(&raw[..])?;

    let mut parsed_fields = Vec::new();

    for field in fields {
        let tag = match field.tag.description() {
            Some(desc) => desc,
            None => continue,
        };

        let parsed_value = match parse_exif_field_value(field.value) {
            Some(result) => result,
            None => continue,
        };

        parsed_fields.push(ParsedField {
            tag: tag.to_owned(),
            value: parsed_value,
        });
    }

    Ok(parsed_fields)
}

/// Attempts to parse a Tiff field value (it does not accept all Tiff value types)
fn parse_exif_field_value(value: ExifValue) -> Option<String> {
    let parsed = match value {
        ExifValue::Ascii(slices) => slices
            .into_iter()
            .filter_map(|b| std::str::from_utf8(&b).ok())
            .fold(String::new(), |mut acc, piece| {
                acc += piece;
                acc
            }),
        _ => return None,
    };

    Some(parsed)
}
