use std::ffi::{c_char, CString};
use std::{slice, str};

use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::FontMgr;

#[no_mangle]
pub extern "C" fn init_font_provider() -> *mut TypefaceFontProvider {
    let font_provider = TypefaceFontProvider::new();
    Box::into_raw(Box::new(font_provider))
}

#[no_mangle]
pub extern "C" fn get_font_family_names(font_provider: *mut TypefaceFontProvider) -> *mut c_char {
    let font_provider = unsafe { &mut *font_provider };
    let font_family_names = font_provider.family_names();

    let mut font_family_names_vec = Vec::new();
    for font_family_name in font_family_names {
        font_family_names_vec.push(font_family_name);
    }

    let array_str = font_family_names_vec.join("\n");
    CString::new(array_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn check_font_exists(
    font_provider: *mut TypefaceFontProvider,
    font_family: *const u8,
    font_family_length: usize,
) -> bool {
    let font_provider = unsafe { &mut *font_provider };

    let font_family_slice = unsafe { slice::from_raw_parts(font_family, font_family_length) };

    let font_family_str = match str::from_utf8(font_family_slice) {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };

    let font_family_names = font_provider.family_names();

    for font_family_name in font_family_names {
        if font_family_name == font_family_str {
            return true;
        }
    }

    false
}

#[no_mangle]
pub extern "C" fn registry_font(
    font_provider: *mut TypefaceFontProvider,
    font_family: *const u8,
    font_family_length: usize,
    font_data: *const u8,
    font_data_length: usize,
) -> bool {
    let font_provider = unsafe { &mut *font_provider };

    let font_family_slice = unsafe { slice::from_raw_parts(font_family, font_family_length) };

    let font_family_str = match str::from_utf8(font_family_slice) {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };

    let font_data_slice = unsafe { slice::from_raw_parts(font_data, font_data_length) };

    // let skia_data = Data::new_copy(font_data_slice);
    // let type_face = Typeface::from_data(skia_data, None).unwrap();
    // font_provider.register_typeface(type_face, Some(font_family_str));

    let font_mgr = FontMgr::new();
    let type_face = font_mgr.new_from_data(font_data_slice, None).unwrap();
    font_provider.register_typeface(type_face, Some(font_family_str));

    true
}

#[no_mangle]
pub extern "C" fn get_svg_path(
    content: *const u8,
    content_length: usize,
    font_family: *const u8,
    font_family_length: usize,
    font_size: f32,
    line_height: f32,
    letter_spacing: f32,
) -> *mut c_char {
    let svg_path = "";
    CString::new(svg_path).unwrap().into_raw()
}

fn main() {
    // nothing to do here
}
