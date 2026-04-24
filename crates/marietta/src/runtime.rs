// Runtime support for Marietta
// Provides C-compatible extern functions for Marietta programs to call

use std::sync::atomic::{AtomicI64, Ordering};
use std::alloc::{alloc, dealloc, Layout};

// ---------------------------------------------------------------------------
// String type (low-level implementation)
// ---------------------------------------------------------------------------
// Marietta strings use a refcounted header with (refcount, len, ptr) tuple.
// Layout: [refcount (i64) | len (u64) | ptr (*const u8)] followed by data

/// String header: reference-counted with length.
/// Stored at the beginning of allocated memory.
#[repr(C)]
pub struct MStrHeader {
    /// Atomic reference count. When it reaches 0, the allocation is freed.
    pub ref_count: i64,
    /// Length of the string data in bytes.
    pub len: u64,
    /// Pointer to the string data (may point to inline data or separate allocation).
    pub ptr: *const u8,
}

impl MStrHeader {
    /// Create a new string from a Rust string slice.
    /// Allocates header + data together, sets refcount to 1.
    pub fn new(s: &str) -> *mut MStrHeader {
        let bytes = s.as_bytes();
        let len = bytes.len();
        
        // Allocate space for header + string data
        let total_size = std::mem::size_of::<MStrHeader>() + len;
        let total_layout = Layout::from_size_align(total_size, 8).unwrap();
        
        unsafe {
            let ptr = alloc(total_layout) as *mut MStrHeader;
            if ptr.is_null() {
                return std::ptr::null_mut();
            }
            
            (*ptr).ref_count = 1;
            (*ptr).len = len as u64;
            
            // Data pointer: right after the header
            let data_ptr = (ptr as *mut u8).add(std::mem::size_of::<MStrHeader>());
            (*ptr).ptr = data_ptr;
            
            // Copy string data
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data_ptr, len);
            
            ptr
        }
    }

    /// Get the string as a Rust str (without lifetime safety).
    pub unsafe fn as_str(&self) -> &'static str {
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        unsafe { std::str::from_utf8_unchecked(slice) }
    }

    /// Atomically increment the reference count.
    pub fn inc_ref(&mut self) {
        // SAFETY: ref_count is i64 which fits in an aligned location
        unsafe {
            let ref_ptr = &mut self.ref_count as *mut i64;
            let atomic = &*(ref_ptr as *const AtomicI64);
            atomic.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Atomically decrement the reference count.
    /// Returns true if refcount reached 0 (allocation should be freed).
    pub fn dec_ref(&mut self) -> bool {
        unsafe {
            let ref_ptr = &mut self.ref_count as *mut i64;
            let atomic = &*(ref_ptr as *const AtomicI64);
            let new_count = atomic.fetch_sub(1, Ordering::AcqRel) - 1;
            new_count <= 0
        }
    }
}

// ---------------------------------------------------------------------------
// Print functions for numeric types
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn print_i64(value: i64) {
    println!("{}", value);
}

#[unsafe(no_mangle)]
pub extern "C" fn print_u64(value: u64) {
    println!("{}", value);
}

#[unsafe(no_mangle)]
pub extern "C" fn print_bool(value: i8) {
    let b = if value != 0 { "true" } else { "false" };
    println!("{}", b);
}

// ---------------------------------------------------------------------------
// String functions
// ---------------------------------------------------------------------------

/// Create a string from a C string (null-terminated).
#[unsafe(no_mangle)]
pub extern "C" fn mstr_from_cstr(cstr: *const u8) -> *mut MStrHeader {
    if cstr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let cstr = std::ffi::CStr::from_ptr(cstr as *const _);
        if let Ok(s) = cstr.to_str() {
            MStrHeader::new(s)
        } else {
            std::ptr::null_mut()
        }
    }
}

/// Print a Marietta string.
#[unsafe(no_mangle)]
pub extern "C" fn print_str(ptr: *mut MStrHeader) {
    if let Some(header) = unsafe { ptr.as_ref() } {
        let s = unsafe { header.as_str() };
        println!("{}", s);
    }
}

/// Clone a string by incrementing its reference count.
#[unsafe(no_mangle)]
pub extern "C" fn mstr_clone(ptr: *mut MStrHeader) -> *mut MStrHeader {
    if let Some(header) = unsafe { ptr.as_mut() } {
        header.inc_ref();
    }
    ptr
}

/// Free a string by decrementing its reference count.
/// If refcount reaches 0, deallocates the header+data.
#[unsafe(no_mangle)]
pub extern "C" fn mstr_free(ptr: *mut MStrHeader) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        if let Some(header) = ptr.as_mut() {
            if header.dec_ref() {
                // Refcount reached 0, deallocate
                let total_size = std::mem::size_of::<MStrHeader>() + header.len as usize;
                let layout = Layout::from_size_align(total_size, 8).unwrap();
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

/// Get the length of a string in bytes.
#[unsafe(no_mangle)]
pub extern "C" fn mstr_len(ptr: *const MStrHeader) -> i64 {
    unsafe {
        if let Some(header) = ptr.as_ref() {
            header.len as i64
        } else {
            0
        }
    }
}

/// Concatenate two strings and return a new one.
#[unsafe(no_mangle)]
pub extern "C" fn mstr_concat(s1: *const MStrHeader, s2: *const MStrHeader) -> *mut MStrHeader {
    unsafe {
        if let (Some(h1), Some(h2)) = (s1.as_ref(), s2.as_ref()) {
            let str1 = h1.as_str();
            let str2 = h2.as_str();
            let concat = format!("{}{}", str1, str2);
            MStrHeader::new(&concat)
        } else {
            std::ptr::null_mut()
        }
    }
}
