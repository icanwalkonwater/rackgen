use std::os::raw::c_char;
use std::ptr;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::borrow::Cow;
use crate::gen::gen_js;

pub mod search;
pub mod sig;
pub mod gen;

#[no_mangle]
extern "C" fn gen_js_interface_for_rack(so: *const c_char, pretty_name: *const c_char) -> *const c_char {
    if so.is_null() {
        return ptr::null();
    }
    let so = unsafe { CStr::from_ptr(so) }.to_string_lossy();

    if let Ok(js) = gen_js_result(so, pretty_name) {
        let c_str = CString::new(js).unwrap();
        c_str.into_raw()
    } else {
        ptr::null()
    }
}

#[no_mangle]
extern "C" fn free_gen_js(js: *mut c_char) {
    if js.is_null() {
        return;
    }

    unsafe {
        CString::from_raw(js);
    }
}

fn gen_js_result(so: Cow<str>, pretty_name: *const c_char) -> anyhow::Result<String> {
    let funcs = search::search_api_funcs(&so)?;
    let sigs = search::search_api_fun_sig(&so, &funcs)?;

    let input_file = Path::new(so.as_ref());
    let libname = input_file
        .file_name()
        .ok_or(anyhow::Error::msg("F u"))?
        .to_string_lossy()
        .into_owned();

    let pretty_class = if pretty_name.is_null() {
        // Strip extension
        let ext_len = input_file.extension().map_or(0, |e| e.len());
        String::from(&libname[..libname.len() - ext_len - 1])
    } else {
        unsafe {CStr::from_ptr(pretty_name)}.to_string_lossy().into_owned()
    };


    Ok(gen_js(&libname, &pretty_class, &sigs))
}
