use core::ffi::{c_char, c_int};
use std::ffi::CString;

/// Consumes a `Vec<String>` and saves some helper data, so it can provide
/// the `*const *const c_char` (`argv`) and `*const c_char` (`argv[0]`)
/// when the user needs them i. e. for [`libc::execvp()`] library call.
pub struct Argv {
    argv: Vec<CString>,
    argv_ptr: Vec<*const c_char>,
}

impl Argv {
    /// Creates a new `Argv` structure.
    pub fn new(args: Vec<String>) -> Self {
        let argv: Vec<_> = args
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
        let mut argv_ptr: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();
        argv_ptr.push(std::ptr::null());
        Self { argv, argv_ptr }
    }

    /// Returns the C language's `argv` (`*const *const c_char`).
    pub fn get_argv(&self) -> *const *const c_char {
        self.argv_ptr.as_ptr()
    }

    /// Returns the C language's `argv[0]` (`*const c_char`).
    pub fn get_argv0(&self) -> *const c_char {
        self.argv_ptr[0]
    }

    /// Gets total length of the `argv` array (excluding the last null pointer).
    pub fn get_argc(&self) -> *const c_int {
        debug_assert_eq!(self.argv.len() + 1, self.argv_ptr.len());
        &(self.argv.len() as i32)
    }
}

#[cfg(test)]
mod test {
    use super::Argv;

    #[test]
    fn exec_args() {
        let args = vec!["ls".to_string(), "-l".to_string(), "-h".to_string()];
        let argv = Argv::new(args.clone());
        unsafe {
            libc::execvp(argv.get_argv0(), argv.get_argv());
        }
    }

    #[test]
    fn get_argc() {
        let args = vec!["ls".to_string(), "-l".to_string(), "-h".to_string()];
        let argv = Argv::new(args.clone());
        assert_eq!(unsafe { *argv.get_argc() }, 3);
    }
}
