//! Extern C ABI

use std::{
    ffi::{CStr, CString},
    os::raw::*,
};

/// Authorization -- C Bind
/// ----------
/// # Parameters
/// - `uid: *const c_char`: uid c-string, UTF-8
/// # Returns
/// - `*mut c_char`: session c-string, UTF-8
/// # Usage
/// ```C
///  char uid[] = "1234567890";
///  char *session = auth(uid);
///  if (session == NULL) {
///    printf("auth error\n");
///    return;
///  } else {
///    printf("session: %s\n", session);
///  }
///  free_c_string(session);
/// ```
///
#[no_mangle]
pub extern "C" fn auth(uid: *const c_char) -> *mut c_char {
    assert!(!uid.is_null());
    let uid = unsafe { c_str_to_str(uid) };

    match crate::auth(uid) {
        Ok((ses, _)) => CString::new(ses).unwrap().into_raw(),
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

#[repr(C)]
#[derive(Default)]
#[allow(non_camel_case_types)]
pub struct ele_info {
    pub status_code: c_int,
    pub total_surplus: c_float,
    pub total_amount: c_float,
    pub surplus: c_float,
    pub surplus_amount: c_float,
    pub subsidy: c_float,
    pub subsidy_amount: c_float,
    pub display_room_name: [c_char; 32],
    pub room_status: [c_char; 32],
}

/// Copy &str to fixed-size c_char array
fn copy_str_to_char_array<const L: usize>(s: &str) -> [c_char; L] {
    let mut c = [0 as c_char; L];
    let len = s.as_bytes().len();
    if len > L - 1 {
        let slice = unsafe { std::slice::from_raw_parts(s.as_ptr() as *mut c_char, L - 1) };
        c[..L - 1].copy_from_slice(slice);
    } else {
        let slice = unsafe { std::slice::from_raw_parts(s.as_ptr() as *mut c_char, len) };
        c[..len].copy_from_slice(slice);
    }
    c
}

/// Query electricity -- C Bind
/// -----------
/// After calling this function,
/// the caller is responsible for using `free_ele_info` to deallocate the memory.
///
/// # Parameters
/// - `session_p`: session string
/// - `session_len`: session string length
///
/// # Returns
/// ## ele_info
/// - `status_code`: status code. 0 for success, others for error
///
/// # Errors (status codes)
/// - `11`: Auth expired
/// - `12`: No bind info
/// - `13`: Other error
///
/// # Usage
/// ```C
/// int main() {
///    ele_info *e = query_ele("00000000-0000-0000-0000-000000000000\0");
///    if (e->status_code == 0) {
///      printf("room: %s\nstatus: %s\ntotal surplus: %f %f\nsurplus: %f "
///             "%f\nsubsidy: %f %f",
///             e->display_room_name, e->room_status, e->total_surplus,
///             e->total_amount, e->surplus, e->surplus_amount, e->subsidy,
///             e->subsidy_amount);
///    } else {
///      printf("error: %d", e->status_code);
///    }
///    free_ele_info(e);
/// }
/// ```
///
#[no_mangle]
pub extern "C" fn query_ele(session: *const c_char) -> *mut ele_info {
    assert!(!session.is_null());
    let session = unsafe { c_str_to_str(session) };

    match crate::query_ele(session) {
        Ok(info) => Box::into_raw(Box::new(ele_info {
            status_code: 0,
            total_surplus: info.soc,
            total_amount: info.total_soc_amount,
            surplus: info.surplus_list[0].surplus,
            surplus_amount: info.surplus_list[0].amount,
            subsidy: info.surplus_list[0].subsidy,
            subsidy_amount: info.surplus_list[0].subsidy_amount,
            display_room_name: copy_str_to_char_array(&info.display_room_name),
            room_status: copy_str_to_char_array(&info.surplus_list[0].room_status),
        })),
        Err(e) => {
            eprintln!("{e}");
            match e {
                crate::error::Error::AuthExpired => Box::into_raw(Box::new(ele_info {
                    status_code: 11,
                    ..ele_info::default()
                })),
                crate::error::Error::NoBind => Box::into_raw(Box::new(ele_info {
                    status_code: 12,
                    ..ele_info::default()
                })),
                _ => Box::into_raw(Box::new(ele_info {
                    status_code: 13,
                    ..ele_info::default()
                })),
            }
        }
    }
}

/// Free ele_info
/// -----------
/// Deallocate the struct to avoid memory leak.
///
/// see `query_ele` for more information.
#[no_mangle]
pub extern "C" fn free_ele_info(ele_info_p: *mut ele_info) {
    assert!(!ele_info_p.is_null());
    unsafe {
        drop(Box::from_raw(ele_info_p));
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct login_handle {
    pub phone_num: [c_char; 12],
    pub device_id: [c_char; 38], //fixed length of 37 with '\0'
}

/// Initialize login handler
fn init_handler(
    handle: *const login_handle,
) -> Result<crate::req::login::LoginHandler, crate::error::Error> {
    assert!(!handle.is_null());

    let phone_num = unsafe {
        let slice = std::slice::from_raw_parts((*handle).phone_num.as_ptr() as *const u8, 11);
        std::str::from_utf8_unchecked(slice)
    };
    let device_id = unsafe {
        let slice = std::slice::from_raw_parts((*handle).device_id.as_ptr() as *const u8, 37);
        std::str::from_utf8_unchecked(slice)
    };

    crate::req::login::LoginHandler::init(phone_num, device_id)
}

/// Generate random device id -- C Bind
/// -----------
///
#[no_mangle]
pub extern "C" fn gen_device_id(handler: *mut login_handle) {
    assert!(!handler.is_null());

    let device_id = crate::req::login::gen_device_id();

    unsafe {
        let slice = std::slice::from_raw_parts(
            device_id.as_ptr() as *mut c_char,
            device_id.as_bytes().len(),
        );
        (*handler).device_id[..37].copy_from_slice(slice);
        (*handler).device_id[37] = '\0' as c_char;
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct security_token_result {
    pub level: c_int,
    pub security_token: *mut c_char,
}

/// Get security token -- C Bind
/// -----------
/// # Returns
/// - `*mut security_token_result`:
/// security token. Return nullptr on error.
/// The caller is responsible for freeing the memory.
#[no_mangle]
pub extern "C" fn get_security_token(handle: *const login_handle) -> *mut security_token_result {
    if let Ok(handler) = init_handler(handle) {
        match handler.get_security_token() {
            Ok(token) => Box::into_raw(Box::new(security_token_result {
                level: token.level as c_int,
                security_token: CString::new(token.security_token).unwrap().into_raw(),
            })),
            Err(_) => std::ptr::null_mut(), // Return nullptr if error
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Free security_token_result
/// -----------
/// Deallocate the struct to avoid memory leak.
#[no_mangle]
pub extern "C" fn free_security_token_result(p: *mut security_token_result) {
    assert!(!p.is_null());
    unsafe {
        drop(CString::from_raw((*p).security_token));
        drop(Box::from_raw(p));
    }
}

/// Free c_string
/// -----------
/// Deallocate c_string to avoid memory leak.
#[no_mangle]
pub extern "C" fn free_c_string(c_string: *mut c_char) {
    assert!(!c_string.is_null());
    unsafe {
        drop(CString::from_raw(c_string));
    }
}

/// Send SMS verification code -- C Bind
/// -----------
///
#[no_mangle]
pub extern "C" fn send_verification_code(
    handle: *const login_handle,
    security_token: *const c_char,
    captcha: *const c_char,
) -> c_int {
    if let Ok(handler) = init_handler(handle) {
        let captcha = if captcha.is_null() {
            None
        } else {
            Some(unsafe { c_str_to_str(captcha) })
        };
        match handler.send_verification_code(unsafe { c_str_to_str(security_token) }, captcha) {
            Ok(v) => {
                if v {
                    0 // Success
                } else {
                    1 // Return 1 if user is not exist
                }
            }
            Err(e) => {
                eprintln!("{e}");
                3 // Return 3 if send verification code error
            }
        }
    } else {
        2 // Init handler error
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct login_result {
    uid: *mut c_char,
    token: *mut c_char,
}

/// Do login with verification code -- C Bind
/// -----------
///
#[no_mangle]
pub extern "C" fn do_login(handle: *const login_handle, code: *const c_char) {
    assert!(!code.is_null());
    if let Ok(handler) = init_handler(handle) {
        match handler.do_login(unsafe { c_str_to_str(code) }) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

/// Convert c-string to &str
/// -----------
/// `unsafe`
unsafe fn c_str_to_str<'a>(c_str: *const c_char) -> &'a str {
    let c_str = CStr::from_ptr(c_str);
    std::str::from_utf8_unchecked(c_str.to_bytes())
}
