/* automatically generated by rust-bindgen 0.61.0 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const W_MAX_PATH: u32 = 260;
pub const W_SUCCESS: u32 = 0;
pub const W_FAILURE: i32 = -1;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct w_vigem_client_t {
    _unused: [u8; 0],
}
pub type w_vigem_client = *mut w_vigem_client_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xinput_gamepad_t {
    pub buttons: u16,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub thumb_lx: i16,
    pub thumb_ly: i16,
    pub thumb_rx: i16,
    pub thumb_ry: i16,
}
pub type xinput_gamepad = *mut xinput_gamepad_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xinput_state_t {
    pub packet_number: u16,
    pub gamepad: xinput_gamepad_t,
}
pub type xinput_state = *mut xinput_state_t;
extern "C" {
    #[doc = " initialize the ViGEm client and connect to the bus"]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @returns zero on success"]
    pub fn w_vigem_client_init(p_vigem: *mut w_vigem_client) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " clear gamepad state"]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @param p_index, the gamepad state"]
    #[doc = " @returns zero on success"]
    pub fn w_vigem_clear_gamepad_state(
        p_vigem: w_vigem_client,
        p_index: usize,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " add a gamepad"]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @param p_index, the the index of new gamepad"]
    #[doc = " @returns zero on success"]
    pub fn w_vigem_add_gamepad(
        p_vigem: w_vigem_client,
        p_index: *mut usize,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " remove a gamepad"]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @param p_index, index of gamepad"]
    #[doc = " @returns zero on success"]
    pub fn w_vigem_remove_gamepad(
        p_vigem: w_vigem_client,
        p_index: usize,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " add a gamepad"]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @param p_index, the index of gamepad"]
    #[doc = " @param p_xinput, the xinput data"]
    #[doc = " @returns zero on success"]
    pub fn w_vigem_send_input(
        p_vigem: w_vigem_client,
        p_index: usize,
        p_xinput: xinput_state,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " get number of gamepads."]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @returns number of gamepads"]
    pub fn w_vigem_get_number_of_gamepads(p_vigem: w_vigem_client) -> usize;
}
extern "C" {
    #[doc = " disconnects from the ViGEm bus device and resets the driver object state and release the ViGEm client resources."]
    #[doc = " @param p_vigem, the vigem client object"]
    #[doc = " @returns void"]
    pub fn w_vigem_client_fini(p_vigem: *mut w_vigem_client);
}
