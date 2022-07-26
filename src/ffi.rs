//! Extern C ABI
//! -----------
//! This module contains the Rust FFI bindings for the C API.
//!
//! # Examples
//!
//! ## Query electricity
//! ```c
//! void query() {
//!     char uid[] = "123456789\0";
//!     char *session = auth(uid);
//!     if (session == NULL) {
//!       printf("auth error\n");
//!       return;
//!     } else {
//!       printf("session: %s\n", session);
//!     }
//!   
//!     ele_result *e;
//!     int code = query_ele(session, &e);
//!     if (code) {
//!       printf("query error: %d\n", code);
//!       return;
//!     }
//!   
//!     free_c_string(session);
//!   
//!     printf("room: %s\nstatus: %s\ntotal surplus: %f %f\nsurplus: %f "
//!            "%f\nsubsidy: %f %f\n",
//!            e->display_room_name, e->room_status, e->total_surplus,
//!            e->total_amount, e->surplus, e->surplus_amount, e->subsidy,
//!            e->subsidy_amount);
//!   
//!     free_ele_result(e);
//! }
//! ```
//!
//! ## App login (query uid)
//! ```c
//! void login() {
//!     login_handle handle;
//!   
//!     gen_device_id(&handle);
//!     printf("device_id: %s\n", handle.device_id);
//!   
//!     handle.phone_num = "18888888888\0";
//!   
//!     security_token_result *sec_token;
//!     int code = get_security_token(&handle, &sec_token);
//!   
//!     if (code) {
//!       printf("fail to get security token: %d\n", code);
//!       return;
//!     }
//!     printf("security token: %s\nlevel: %d\n", sec_token->token, sec_token->level);
//!     
//!     if (sec_token->level != 0) {
//!         char *image;
//!         code = get_captcha_image(&handle, sec_token->token, &image);
//!         if (code) {
//!             printf("fail to get captcha image: %d\n", code);
//!             return;
//!         }
//!         char captcha_code[5] = {0};
//!         printf("Captcha image: %s\n", image);
//!         printf("Please solve the image captcha and input the code: ");
//!         scanf("%4s", captcha_code);
//!         code = send_verification_code(&handle, sec_token->token, captcha_code);
//!     } else {
//!         code = send_verification_code(&handle, sec_token->token, NULL);
//!     }
//!
//!     if (code == 0 || code == 1) {
//!       printf("send verification code success\n");
//!     } else {
//!       printf("fail to send verification code: %d\n", code);
//!       return;
//!     }
//!     free_security_token_result(sec_token);
//!   
//!     char verification_code[7] = {0};
//!     printf("please input verification code:");
//!     scanf("%6s", verification_code);
//!   
//!     login_result *l_result;
//!     code = do_login(&handle, verification_code, &l_result);
//!     if (code) {
//!       printf("fail to login: %d\n", code);
//!       return;
//!     }
//!     printf("login success\n");
//!     printf("uid: %s\ntoken: %s\ndivice_id: %s\nbind_card_status: %d\n",
//!            l_result->uid, l_result->token, l_result->device_id,
//!            l_result->bind_card_status);
//!   
//!     free_login_result(l_result); // remember to free the memory
//!     free_c_string(handle.device_id);
//! }
//! ```
//!
//! # Error code reference
//! - `0`: Success
//! - `101`: Unhandled error
//! - `201`: Authentication expired
//! - `202`: No bind info
//! - `203`: Initialization handler error, please check the input parameters
//! - `204`: Bad phone number
//! - `205`: Limit of SMS verification code sent
//! - `206`: Bad(Wrong) verification code
//! - `207`: Get captcha image failed

use std::{
    ffi::{CStr, CString},
    os::raw::*,
};

/// Authorization -- C Bind
/// ----------
/// # Inputs
/// - `uid: *const c_char`: uid c-string, UTF-8
/// # Returns
/// - `*mut c_char`: session c-string, UTF-8. Return nullptr on error.
#[no_mangle]
pub extern "C" fn auth(uid: *const c_char) -> *mut c_char {
    assert!(!uid.is_null());
    let uid = unsafe { c_string_to_str(uid) };

    match crate::auth(uid) {
        Ok((ses, _)) => CString::new(ses).unwrap().into_raw(),
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ele_result {
    pub total_surplus: c_float,
    pub total_amount: c_float,
    pub surplus: c_float,
    pub surplus_amount: c_float,
    pub subsidy: c_float,
    pub subsidy_amount: c_float,
    pub display_room_name: *mut c_char,
    pub room_status: *mut c_char,
}

/// Copy `&str` to fixed-size `c_char` array
pub fn copy_str_to_char_array<const L: usize>(s: &str) -> [c_char; L] {
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
/// the caller is responsible for using `free_ele_result` to deallocate the memory.
///
/// # Inputs
/// - `session: *const c_char`: session c-string
/// - `result: *mut *mut ele_result`: second-level pointer for return pointer of `ele_result` struct
///
/// # Returns
/// - `c_int`: 0 on success, otherwise error code
/// - `result`: ele_result
///
/// # Errors (status codes)
/// - `201`: Auth expired
/// - `202`: No bind info
/// - `101`: Other error
#[no_mangle]
pub extern "C" fn query_ele(session: *const c_char, result: *mut *mut ele_result) -> c_int {
    assert!(!session.is_null());
    assert!(!result.is_null());

    let session = unsafe { c_string_to_str(session) };

    match crate::query_ele(session) {
        Ok(mut info) => unsafe {
            let surplus = info.surplus_list.swap_remove(0);

            (*result) = Box::into_raw(Box::new(ele_result {
                total_surplus: info.soc,
                total_amount: info.total_soc_amount,
                surplus: surplus.surplus,
                surplus_amount: surplus.amount,
                subsidy: surplus.subsidy,
                subsidy_amount: surplus.subsidy_amount,
                display_room_name: CString::new(info.display_room_name).unwrap().into_raw(),
                room_status: CString::new(surplus.room_status).unwrap().into_raw(),
            }));

            0 // Return 0 for success
        },
        Err(e) => {
            eprintln!("{e}");
            match e {
                crate::error::Error::AuthExpired => 201,
                crate::error::Error::NoBind => 202,
                _ => 101,
            }
        }
    }
}

/// Free ele_result
/// -----------
/// Deallocate the struct to avoid memory leak.
#[no_mangle]
pub extern "C" fn free_ele_result(p: *mut ele_result) {
    assert!(!p.is_null());
    unsafe {
        drop(Box::from_raw(p));
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct login_handle {
    pub phone_num: *mut c_char, // Fixed length of 12 with '\0'
    pub device_id: *mut c_char, // Fixed length of 37 with '\0'
}

/// Initialize login handler
fn init_handler(
    handle: *const login_handle,
) -> Result<crate::req::login::LoginHandler, crate::error::Error> {
    assert!(!handle.is_null());

    let phone_num = unsafe {
        // let slice = std::slice::from_raw_parts((*handle).phone_num as *const u8, 11);
        // std::str::from_utf8_unchecked(slice)
        c_string_to_str((*handle).phone_num)
    };
    let device_id = unsafe {
        // let slice = std::slice::from_raw_parts((*handle).device_id as *const u8, 37);
        // std::str::from_utf8_unchecked(slice)
        c_string_to_str((*handle).device_id)
    };

    crate::req::login::LoginHandler::init(phone_num, device_id)
}

/// Generate random device id -- C Bind
/// -----------
/// Generate random device into struct `login_handle`.
///
/// The caller is responsible for using `free_c_string` to deallocate
/// the string in the structure.
///
/// # Inputs
/// - `handle: *const login_handle`: Pointer of Login handle
#[no_mangle]
pub extern "C" fn gen_device_id(handler: *mut login_handle) {
    assert!(!handler.is_null());

    let device_id = crate::req::login::gen_device_id();

    unsafe {
        // let slice = std::slice::from_raw_parts(
        //     device_id.as_ptr() as *mut c_char,
        //     device_id.as_bytes().len(),
        // );
        // (*handler).device_id[..37].copy_from_slice(slice);
        // (*handler).device_id[37] = '\0' as c_char;
        (*handler).device_id = CString::new(device_id).unwrap().into_raw();
    }
}

/// Security token result
/// -----------
/// - `token: *mut c_char`: token c-string
/// - `level: c_int`: security level, 0: no captcha required, 1: captcha required
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct security_token_result {
    pub level: c_int,       // 0: no captcha required, 1: captcha required
    pub token: *mut c_char, // c-string of token
}

/// Get security token -- C Bind
/// -----------
/// Using filled handle to query security token.
/// # Inputs
/// - `handle: *const login_handle`: Pointer of Login handle
/// - `result: *mut *mut security_token_result`: second-level pointer for return pointer of `security_token_result` struct
///
/// # Returns
/// - `c_int`: 0 on success, otherwise error code
/// - `result`: security_token_result
///
/// # Errors
/// - `203`: Initialize login handler failed
/// - `101: Other errors
#[no_mangle]
pub extern "C" fn get_security_token(
    handle: *const login_handle,
    result: *mut *mut security_token_result,
) -> c_int {
    if let Ok(handler) = init_handler(handle) {
        match handler.get_security_token() {
            Ok(token) => unsafe {
                (*result) = Box::into_raw(Box::new(security_token_result {
                    level: token.level as c_int,
                    token: CString::new(token.security_token).unwrap().into_raw(),
                }));

                0 // Return 0 for success
            },
            Err(_) => 101, // Return nullptr if error
        }
    } else {
        203 // Return error code if init_handler failed
    }
}

/// Free security_token_result
/// -----------
/// Deallocate the struct to avoid memory leak.
#[no_mangle]
pub extern "C" fn free_security_token_result(p: *mut security_token_result) {
    assert!(!p.is_null());
    unsafe {
        drop(CString::from_raw((*p).token));
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

/// Get captcha image -- C Bind
/// -----------
/// Get captcha image in base64 format.
///
/// Like `data:image/jpeg;base64,xxxxxxxxxxxxxx`
///
/// # Inputs
/// - `handle: *const login_handle`: Pointer of Login handle
/// - `security_token: *const c_char`: security token
/// - `result: *mut *mut c_char`: second-level pointer for result
///
/// # Returns
/// - `result: *mut *mut c_char`: captcha image in base64
///
/// # Errors
/// - `207`: Get captcha image failed
/// - `101: Other errors
///
/// # Usage
///
#[no_mangle]
pub extern "C" fn get_captcha_image(
    handle: *const login_handle,
    security_token: *const c_char,
    result: *mut *mut c_char,
) -> c_int {
    assert!(!handle.is_null());
    assert!(!security_token.is_null());

    let handler = init_handler(handle).unwrap();
    let security_token = unsafe { c_string_to_str(security_token) };
    match handler.get_captcha_image(security_token) {
        Ok(image) => {
            unsafe {
                (*result) = CString::new(image).unwrap().into_raw();
            }

            0 // Return 0 for success
        }

        Err(e) => {
            eprintln!("{}", e);
            match e {
                crate::error::Error::Runtime(_) => {
                    207 // Return 207 for get captcha image error
                }
                _ => 101, // Return 101 for other errors
            }
        }
    }
}

/// Send SMS verification code -- C Bind
/// -----------
/// # Inputs
/// - `handle: *const login_handle`: Pointer of Login handle
/// - `security_token: *const c_char`: c-string of security token
/// - `captcha: *const c_char`: c-string of captcha.
/// If captcha input `NULL`, it means no captcha is required.
/// # Returns
/// - `c_int`: `0` on success, `1` on user is not exist(registered), otherwise error code
/// # Errors
/// - `203`: Initialize login handler failed
/// - `204`: Bad phone number
/// - `205`: Limit of SMS verification code sent
/// - `101: Other errors
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
            Some(unsafe { c_string_to_str(captcha) })
        };
        match handler.send_verification_code(unsafe { c_string_to_str(security_token) }, captcha) {
            Ok(v) => {
                if v {
                    0 // Success
                } else {
                    1 // Return 1 if user is not exist
                }
            }
            Err(e) => {
                eprintln!("{e}");
                match e {
                    crate::error::Error::BadPhoneNumber => 204, // Bad phone number
                    crate::error::Error::VerificationLimit => 205, // Too may requests, limited
                    _ => 101,                                   // Return 101 on other error
                }
            }
        }
    } else {
        203 // Init handler error
    }
}

/// Login result
/// -----------
/// - `bind_card_status: c_int`: 0: not bind, 1: bound
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct login_result {
    uid: *mut c_char,
    token: *mut c_char,
    device_id: *mut c_char,
    bind_card_status: c_int, //0: not bind, 1: bind
}

/// Do login with verification code -- C Bind
/// -----------
/// Do login to get uid, token(APP), device id, and bind card status.
/// # Inputs
/// - `handle: *const login_handle`: Pointer of Login handle
/// - `code: *const c_char`: c-string of verification code
/// - `result: *mut *mut login_result`: second-level pointer for return pointer of `login_result` struct
/// # Returns
/// - `c_int`: 0 on success, otherwise error code
/// # Errors
/// - `203`: Initialize login handler failed
/// - `206`: Bad(Wrong) verification code
/// - `101: Other errors
#[no_mangle]
pub extern "C" fn do_login(
    handle: *const login_handle,
    code: *const c_char,
    result: *mut *mut login_result,
) -> c_int {
    assert!(!code.is_null());
    assert!(!result.is_null());

    if let Ok(handler) = init_handler(handle) {
        match handler.do_login(unsafe { c_string_to_str(code) }) {
            Ok(v) => unsafe {
                (*result) = Box::into_raw(Box::new(login_result {
                    uid: CString::new(v.id).unwrap().into_raw(),
                    token: CString::new(v.token).unwrap().into_raw(),
                    device_id: CString::new(v.device_id).unwrap().into_raw(),
                    bind_card_status: v.bind_card_status as c_int,
                }));

                0 // Success
            },
            Err(e) => {
                eprintln!("{e}");
                match e {
                    crate::error::Error::BadVerificationCode => 206, // Wrong verification code
                    _ => 101,
                }
            }
        }
    } else {
        203 // Initialize login handler failed
    }
}

/// Free login_result
/// ---------
/// Deallocate the struct to avoid memory leak.
#[no_mangle]
pub extern "C" fn free_login_result(p: *mut login_result) {
    assert!(!p.is_null());
    unsafe {
        drop(CString::from_raw((*p).uid));
        drop(CString::from_raw((*p).token));
        drop(CString::from_raw((*p).device_id));
        drop(Box::from_raw(p));
    }
}

/// Convert c-string to &str
/// -----------
/// `unsafe`: unchecked
unsafe fn c_string_to_str<'a>(c_str: *const c_char) -> &'a str {
    let c_str = CStr::from_ptr(c_str);
    std::str::from_utf8_unchecked(c_str.to_bytes())
}
