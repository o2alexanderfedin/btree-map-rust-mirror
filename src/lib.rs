#![allow(unused_imports, dead_code)]

mod src;
mod tests;
use crate::tests::test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    return __main_inner();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn malloc(__size: u64) -> *mut ();
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memset_chk(_: *mut (), _: i32, _: u64, _: u64) -> *mut ();
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn free(_: *mut ()) -> ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8) -> ();
    fn printf(_: *const i8, ...) -> i32;
    fn __builtin_expect(_: i64, _: i64) -> i64;
}
