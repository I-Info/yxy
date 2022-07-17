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
/// ```
///
#[no_mangle]
pub extern "C" fn auth(uid: *const c_char) -> *mut c_char {
    assert!(!uid.is_null());

    let uid_c = unsafe { CStr::from_ptr(uid) };
    let uid = unsafe { std::str::from_utf8_unchecked(uid_c.to_bytes()) };

    match crate::auth(uid) {
        Ok((ses, _)) => CString::new(ses).unwrap().into_raw(),
        Err(e) => {
            eprintln!("{}", e);
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

/// Query electricity -- C Bind
/// -----------
/// After calling this function, the caller is responsible for using `ele_info_free` to free the memory.
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
/// - `11`: Session expired
/// - `12`: Other query error
///
/// # Usage
/// ```C
/// int main() {
///    ele_info *e = query_ele("00000000-0000-0000-0000-000000000000", 36);
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
pub extern "C" fn query_ele(session_p: *const c_char, session_len: usize) -> *mut ele_info {
    assert!(!session_p.is_null());

    let session;
    unsafe {
        let session_raw = std::slice::from_raw_parts(session_p as *const u8, session_len);
        session = std::str::from_utf8_unchecked(session_raw);
    }

    match crate::query_ele(session) {
        Ok(info) => Box::into_raw(Box::new(ele_info {
            status_code: 0,
            total_surplus: info.soc,
            total_amount: info.total_soc_amount,
            surplus: info.surplus_list[0].surplus,
            surplus_amount: info.surplus_list[0].amount,
            subsidy: info.surplus_list[0].subsidy,
            subsidy_amount: info.surplus_list[0].subsidy_amount,
            display_room_name: {
                let mut c: [c_char; 32] = [0; 32];
                if info.display_room_name.as_bytes().len() > 31 {
                    let slice = unsafe {
                        std::slice::from_raw_parts(
                            info.display_room_name.as_ptr() as *mut c_char,
                            31,
                        )
                    };
                    c[..31].copy_from_slice(slice);
                } else {
                    let slice = unsafe {
                        std::slice::from_raw_parts(
                            info.display_room_name.as_ptr() as *mut c_char,
                            info.display_room_name.as_bytes().len(),
                        )
                    };
                    c[..info.display_room_name.as_bytes().len()].copy_from_slice(slice);
                }
                c
            },
            room_status: {
                let mut c: [c_char; 32] = [0; 32];
                let len = info.surplus_list[0].room_status.as_bytes().len();
                if len > 31 {
                    let slice = unsafe {
                        std::slice::from_raw_parts(
                            info.surplus_list[0].room_status.as_ptr() as *mut c_char,
                            31,
                        )
                    };
                    c[..31].copy_from_slice(slice);
                } else {
                    let slice = unsafe {
                        std::slice::from_raw_parts(
                            info.surplus_list[0].room_status.as_ptr() as *mut c_char,
                            len,
                        )
                    };
                    c[..len].copy_from_slice(slice);
                }
                c
            },
        })),
        Err(e) => {
            // println!("{:?}", e);
            match e {
                crate::error::Error::AuthExpired => Box::into_raw(Box::new(ele_info {
                    status_code: 11,
                    ..ele_info::default()
                })),
                _ => Box::into_raw(Box::new(ele_info {
                    status_code: 12,
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
    assert!(!handle.is_null());

    let phone_num = unsafe {
        let slice = std::slice::from_raw_parts((*handle).phone_num.as_ptr() as *const u8, 11);
        std::str::from_utf8_unchecked(slice)
    };
    let device_id = unsafe {
        let slice = std::slice::from_raw_parts((*handle).device_id.as_ptr() as *const u8, 37);
        std::str::from_utf8_unchecked(slice)
    };

    if let Ok(handler) = crate::req::login::LoginHandler::init(phone_num, device_id) {
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
