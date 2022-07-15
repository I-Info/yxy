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
