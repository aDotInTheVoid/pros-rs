#![allow(non_camel_case_types)]
#![no_std]

//! Binding of rust types to c types for the cortex A9 processor

pub use core::ffi::c_void;

#[cfg(target_arch = "arm")]
mod cortex_types {

    // Floats, confirmed by elipsons in float.h
    // https://doc.rust-lang.org/src/core/num/f64.rs.html#30
    /// `f64`
    pub type c_double = f64;
    // https://doc.rust-lang.org/src/core/num/f32.rs.html#30
    /// `f32`
    pub type c_float = f32;

    // Confirmed by limits.h <type>_MAX
    /// `u8`
    pub type c_char = u8;
    /// `i32`
    pub type c_int = i32;

    //Confirmed by limits.h `typedef`

    /// `i32`
    ///
    /// `typedef long int __int32_t;`
    pub type c_long = i32;
    /// `i64`
    ///
    /// `typedef long long int __int64_t;`
    pub type c_longlong = i64;
    /// `i8`
    ///
    /// `typedef signed char __int8_t;`
    pub type c_schar = i8; // signed char
    /// `i16`
    ///
    /// `typedef short int __int16_t;`
    pub type c_short = i16; // short int
    /// `u8`
    ///
    /// `typedef unsigned char __uint8_t;`
    pub type c_uchar = u8; // unsigned char

    // Unsigned versions of knowns
    /// `u32`
    pub type c_ulong = u32; // unsigned long int
    /// `u64`
    pub type c_ulonglong = u64; // unsigned long long int
    /// `u16`
    pub type c_ushort = u16; // unsigned short int
    /// `u32`
    pub type c_uint = u32; // unsigned int

}

// Major hack to allow x86 tests to work
#[cfg(target_arch = "arm")]
pub use cortex_types::*;

#[cfg(not(target_arch = "arm"))]
pub use libc::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
