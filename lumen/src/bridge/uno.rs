use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::LumenAgent;

static AGENT: Lazy<Mutex<Option<LumenAgent>>> = Lazy::new(|| Mutex::new(None));

// Callback type for Rust to ask C++ for document data
type GetDocContentFn = extern "C" fn() -> *mut c_char;
static mut GET_DOC_CONTENT_CB: Option<GetDocContentFn> = None;

#[no_mangle]
pub extern "C" fn lumen_set_get_content_callback(cb: GetDocContentFn) {
    unsafe { GET_DOC_CONTENT_CB = Some(cb); }
}

pub fn get_remote_document_content() -> String {
    unsafe {
        if let Some(cb) = GET_DOC_CONTENT_CB {
            let ptr = cb();
            if !ptr.is_null() {
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                lumen_free_string(ptr);
                return s;
            }
        }
    }
    "Unable to retrieve document content".to_string()
}

#[no_mangle]
pub extern "C" fn lumen_init(api_key: *const c_char) -> i32 {
    let c_str = unsafe {
        if api_key.is_null() { return -1; }
        CStr::from_ptr(api_key)
    };
    
    let key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };

    let mut agent = AGENT.lock().unwrap();
    *agent = Some(LumenAgent::new(key));
    0
}

#[no_mangle]
pub extern "C" fn lumen_query(query: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        if query.is_null() { return std::ptr::null_mut(); }
        CStr::from_ptr(query)
    };

    let query_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let agent_lock = AGENT.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        // Phase 3: Update context before processing query
        // This is a simplified "perception" step
        let content = get_remote_document_content();
        // In a real impl, we'd update agent.context_engine here

        match runtime.block_on(agent.process_query(query_str)) {
            Ok(response) => {
                let c_res = CString::new(response).unwrap();
                c_res.into_raw()
            },
            Err(_) => {
                let c_err = CString::new("Error processing query").unwrap();
                c_err.into_raw()
            }
        }
    } else {
        let c_err = CString::new("Lumen not initialized").unwrap();
        c_err.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn lumen_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        let _ = CString::from_raw(s);
    }
}
