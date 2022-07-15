//! Extern C ABI

use std::os::raw::*;

/// Auth ABI
#[no_mangle]
extern "C" fn auth(
    uid_p: *const c_uchar,
    uid_len: usize,
    session_p: *mut c_uchar,
    session_len: *mut usize,
) -> usize {
    assert!(!uid_p.is_null());

    let uid;

    unsafe {
        let uid_raw = std::slice::from_raw_parts(uid_p, uid_len);
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
        let session = std::slice::from_raw_parts_mut(session_p, *session_len);
        session[..ses.as_bytes().len()].copy_from_slice(ses.as_bytes());

        // Return the number of elements
        *session_len = ses.as_bytes().len();
    }

    return 0;
}
