//! Extern C ABI

use std::os::raw::*;

/// Auth C ABI
///
/// # Usage
/// ```C
/// uintptr_t auth(const char *uid_p, uintptr_t uid_len, char *session_p, uintptr_t *session_len);
///
/// int main() {
///  char *session_p = malloc(40);
///  uintptr_t session_len = 40;
///  uintptr_t result = auth("2010000000000000000", 19, session_p, &session_len);
///  if (result == 0) {
///     session_p[session_len] = '\0';
///     printf("session: %s\nlen: %ld\n", session_p, session_len);
///   } else {
///     printf("error: %ld\n", result);
///   }
/// }
/// ```
///
#[no_mangle]
pub extern "C" fn auth(
    uid_p: *const c_char,
    uid_len: usize,
    session_p: *mut c_char,
    session_len: *mut usize,
) -> usize {
    assert!(!uid_p.is_null());

    let uid;

    unsafe {
        let uid_raw = std::slice::from_raw_parts(uid_p as *const u8, uid_len);
        uid = std::str::from_utf8_unchecked(uid_raw);
    }

    let (ses, _) = match crate::auth(uid) {
        Ok((ses, user)) => (ses, user),
        Err(_) => {
            return 1;
        }
    };

    assert!(!session_p.is_null());
    assert!(!session_len.is_null());

    unsafe {
        // Ensure the buffer is large enough
        if ses.as_bytes().len() > *session_len {
            return 2;
        }

        // Copy result to the buffer
        let session = std::slice::from_raw_parts_mut(session_p as *mut u8, *session_len);
        session[..ses.as_bytes().len()].copy_from_slice(ses.as_bytes());

        // Return the number of elements
        *session_len = ses.as_bytes().len();
    }

    return 0;
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

/// Query electricity C ABI
/// -----------
/// After calling this function, the caller is responsible for using `ele_info_free` to free the memory.
///
/// # Input
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
                if info.display_room_name.len() > 32 {
                    c[..32].copy_from_slice(
                        info.display_room_name.as_bytes()[..32]
                            .iter()
                            .map(|&x| x as c_char)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                } else {
                    c[..info.display_room_name.as_bytes().len()].copy_from_slice(
                        info.display_room_name
                            .as_bytes()
                            .iter()
                            .map(|&x| x as c_char)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                }
                c
            },
            room_status: {
                let mut c: [c_char; 32] = [0; 32];
                if info.surplus_list[0].room_status.as_bytes().len() > 32 {
                    c[..32].copy_from_slice(
                        info.surplus_list[0].room_status.as_bytes()[..32]
                            .iter()
                            .map(|&x| x as c_char)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                } else {
                    c[..info.surplus_list[0].room_status.as_bytes().len()].copy_from_slice(
                        info.surplus_list[0]
                            .room_status
                            .as_bytes()
                            .iter()
                            .map(|&x| x as c_char)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
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

/// Free ele_info heap memory
/// -----------
/// Free the struct to avoid memory leak.
///
/// see `query_ele` for more information.
#[no_mangle]
pub extern "C" fn free_ele_info(ele_info_p: *mut ele_info) {
    assert!(!ele_info_p.is_null());
    unsafe {
        Box::from_raw(ele_info_p);
    }
}
