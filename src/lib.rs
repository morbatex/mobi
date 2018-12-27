extern crate mobi_sys as ff;
extern crate libc;

use std::ffi::{CStr,CString};

pub fn load_file(path : std::path::PathBuf) -> Result<MobiData,ff::MOBI_RET> {
    unsafe{
        let mobi_data = ff::mobi_init();
        match path.to_str() {
            Some(path) => {
                match CString::new(path) {
                    Ok(path) => {
                        let ret = ff::mobi_load_filename(mobi_data,path.as_ptr());
                        match ret {
                            ff::MOBI_RET_MOBI_SUCCESS => {
                                Ok(MobiData{ptr: mobi_data})
                            },
                            e => {
                                ff::mobi_free(mobi_data);
                                Err(e)
                            },
                        }
                    },
                    Err(_) => {
                        ff::mobi_free(mobi_data);
                        Err(ff::MOBI_RET_MOBI_PARAM_ERR)
                    }, 
                }
            },
            None => {
                ff::mobi_free(mobi_data);
                Err(ff::MOBI_RET_MOBI_PARAM_ERR)
            },
        }
    }
}

pub fn get_title(book: &MobiData) -> Result<String,String> {
    get_meta(book,ff::mobi_meta_get_title) 
}

pub fn get_author(book: &MobiData) -> Result<String,String> {
    get_meta(book,ff::mobi_meta_get_author) 
}

pub fn get_publisher(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_publisher)
}

pub fn get_imprint(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_imprint)
}

pub fn get_description(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_description)
}

pub fn get_isbn(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_isbn)
}

pub fn get_subject(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_subject)
}

pub fn get_publish_date(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_publishdate)
}

pub fn get_review(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_review)
}

pub fn get_contributer(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_contributor)
}

pub fn get_copyright(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_copyright)
}

pub fn get_asin(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_asin)
}

pub fn get_language(book: &MobiData) -> Result<String,String> {
    get_meta(book, ff::mobi_meta_get_language)
}

fn get_meta(book: &MobiData, f: unsafe extern "C" fn(*const ff::MOBIData) -> *mut std::os::raw::c_char) -> Result<String, String> {
    unsafe {    
        let ptr = f(book.ptr);
        if ptr.is_null() {
            libc::free(ptr as *mut libc::c_void);
            Err(String::from("Call to libmobi was not successfull"))
        } else {
            let text = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr as *mut libc::c_void);
            Ok(text)
        }
    }
}


pub fn get_text(book: &MobiData) -> Result<String, String> {
    unsafe {
        let max_size = ff::mobi_get_text_maxsize(book.ptr);
        let text_ptr = libc::malloc(std::mem::size_of::<i8>() * max_size) as *mut i8;
        if text_ptr.is_null() {
            libc::free(text_ptr as *mut libc::c_void);
            return Err(String::from("Could not allocate memory"));
        }
        let max_size = Box::into_raw(Box::new(max_size));
        let ret = ff::mobi_get_rawml(book.ptr,text_ptr, max_size);
        drop(Box::from_raw(max_size));
        match ret {
            ff::MOBI_RET_MOBI_SUCCESS => {
                let text = CStr::from_ptr(text_ptr).to_string_lossy().into_owned();
                libc::free(text_ptr as *mut libc::c_void);
                Ok(text)
            },
            _ => {    
                libc::free(text_ptr as *mut libc::c_void);
                Err(String::from("Call to libmobi was not successfull"))
            },
        }
    }
}

pub struct MobiData {
    ptr: *mut ff::MOBIData,
}

impl Drop for MobiData {
    fn drop(&mut self) {
        unsafe {
            ff::mobi_free(self.ptr)
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn get_text_test() {
        let mut path = std::path::PathBuf::new();
        path.push("samples/rust.mobi");
        let res = crate::load_file(path);
        match res {
            Ok(book) => {
                match crate::get_text(&book) {
                    Ok(text) => {
                        assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
                        assert!(text.contains("Finally, we’ll talk about Cargo"));
                        assert!(text.ends_with("margin: 0 0 0 1em
    }"));
                    },
                    Err(_) => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_author_test() {
        let mut path = std::path::PathBuf::new();
        path.push("samples/rust.mobi");
        let res = crate::load_file(path);
        match res {
            Ok(book) => {
                match crate::get_author(&book) {
                    Ok(author) => assert_eq!(String::from("The Rust Team"),author),
                    Err(_) => assert!(false),
                }
            },
            Err(_) => assert!(false),
        }
    }


    #[test]
    fn get_title_test() {
        let mut path = std::path::PathBuf::new();
        path.push("samples/rust.mobi");
        let res = crate::load_file(path);
        match res {
            Ok(book) => {
                match crate::get_title(&book) {
                    Ok(title) => assert_eq!(String::from("The Rust Programming Language"),title),
                    Err(_) => assert!(false),
                }
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn load_file_path_empty() {
        let res = crate::load_file(std::path::PathBuf::new());
        match res {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(ff::MOBI_RET_MOBI_FILE_NOT_FOUND,e),                
        }
    }

    #[test] 
    fn load_file_existing() {
        let mut path = std::path::PathBuf::new();
        path.push("samples/rust.mobi");
        let res = crate::load_file(path);
        match res {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
