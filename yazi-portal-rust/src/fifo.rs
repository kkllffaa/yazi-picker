use std::ffi::CString;
use std::path::Path;

mod sys {
	unsafe extern "C" {
		pub fn mkfifo(path: *const i8, mode: u32) -> i32;
	}
}

pub fn mkfifo(path: &Path) {
	let c_path = CString::new(path.to_str().unwrap()).unwrap();

	unsafe {
		sys::mkfifo(c_path.as_ptr(), 0o644);
	}
}
