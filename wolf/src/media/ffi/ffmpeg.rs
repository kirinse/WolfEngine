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
pub struct AVPacket {
    _unused: [u8; 0],
}
pub type w_av_packet = *mut AVPacket;
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_av_packet, the AVPacket object"]
    #[doc = " @returns zero on success"]
    pub fn w_av_packet_init(
        p_av_packet: *mut w_av_packet,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " get packet data"]
    #[doc = " @param p_av_packet, the AVPacket object"]
    #[doc = " @param p_data, the data"]
    #[doc = " @returns size of packet in bytes"]
    pub fn w_av_packet_get(p_av_packet: w_av_packet, p_data: *mut u8) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " release resources of avpacket"]
    #[doc = " @param p_av_frame, the ffmpeg AVFrame"]
    pub fn w_av_packet_fini(p_av_packet: *mut w_av_packet);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFrame {
    _unused: [u8; 0],
}
pub type w_av_frame = *mut AVFrame;
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_avframe, the ffmpeg AVFrame"]
    #[doc = " @param p_pixel_format, the pixel format of ffmpeg AVFrame"]
    #[doc = " @param p_width, the width of ffmpeg AVFrame"]
    #[doc = " @param p_height, the height of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the alignment"]
    #[doc = " @param p_data, the initial data of ffmpeg AVFrame"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_frame_init(
        p_avframe: *mut w_av_frame,
        p_pixel_format: u32,
        p_width: u32,
        p_height: u32,
        p_alignment: u32,
        p_data: *const u8,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_avframe, the ffmpeg AVFrame"]
    #[doc = " @param p_data, the initial data of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the alignment"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_set_data(
        p_avframe: w_av_frame,
        p_data: *const u8,
        p_alignment: u32,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " get data of ffmpeg AVFrame"]
    #[doc = " @param p_avframe, the avframe"]
    #[doc = " @param p_index, the index of data pointer (0 - 7)"]
    #[doc = " @param p_data, the pointer to the data"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_get_data(
        p_avframe: w_av_frame,
        p_index: usize,
        p_data: *mut u8,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " get data of ffmpeg AVFrame"]
    #[doc = " @param p_avframe, the avframe"]
    #[doc = " @param p_index, the index of data pointer (0 - 7)"]
    #[doc = " @returns the required linesize"]
    pub fn w_av_get_data_linesize(
        p_avframe: w_av_frame,
        p_index: usize,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_pixel_format, the pixel format of ffmpeg AVFrame"]
    #[doc = " @param p_width, the width of ffmpeg AVFrame"]
    #[doc = " @param p_height, the height of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the aligmnet which is usually 1"]
    #[doc = " @returns the size of buffer"]
    pub fn w_av_get_required_buffer_size(
        p_pixel_format: u32,
        p_width: u32,
        p_height: u32,
        p_alignment: u32,
    ) -> usize;
}
extern "C" {
    #[doc = " convert the ffmpeg AVFrame"]
    #[doc = " @param p_src_avframe, the source AVFrame"]
    #[doc = " @param p_dst_avframe, the destination AVFrame"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_frame_convert(
        p_src_avframe: w_av_frame,
        p_dst_avframe: *mut w_av_frame,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " release resources of avframe"]
    #[doc = " @param p_av_frame, the ffmpeg AVFrame"]
    pub fn w_av_frame_fini(p_avframe: *mut w_av_frame);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodeOptions {
    pub gop: ::std::os::raw::c_int,
    pub level: ::std::os::raw::c_int,
    pub max_b_frames: ::std::os::raw::c_int,
    pub refs: ::std::os::raw::c_int,
    pub thread_count: ::std::os::raw::c_int,
    pub fps: u32,
    pub bitrate: u64,
}
pub const AVSetOptionValueType_STRING: AVSetOptionValueType = 0;
pub const AVSetOptionValueType_INTEGER: AVSetOptionValueType = 1;
pub const AVSetOptionValueType_DOUBLE: AVSetOptionValueType = 2;
pub type AVSetOptionValueType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVSetOption {
    pub name: *const ::std::os::raw::c_char,
    pub value_type: AVSetOptionValueType,
    pub value_str: *const ::std::os::raw::c_char,
    pub value_int: ::std::os::raw::c_int,
    pub value_double: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct w_ffmpeg_t {
    _unused: [u8; 0],
}
pub type w_ffmpeg = *mut w_ffmpeg_t;
extern "C" {
    #[doc = " initialize the ffmpeg context"]
    #[doc = " @param p_ffmpeg, ffmpeg object"]
    #[doc = " @param p_frame, the av frame"]
    #[doc = " @param p_mode, zero for encoder and non-zero for decoder"]
    #[doc = " @param p_avcodec_id, av codec id"]
    #[doc = " @param p_codec_opt, the av codec options"]
    #[doc = " @param p_av_opt_sets, the av set options"]
    #[doc = " @param p_av_opt_sets_size, the av set options size"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @return int the result of encoding the frame"]
    pub fn w_ffmpeg_init(
        p_ffmpeg: *mut w_ffmpeg,
        p_frame: w_av_frame,
        p_mode: u32,
        p_avcodec_id: *const ::std::os::raw::c_char,
        p_codec_opt: *mut AVCodeOptions,
        p_av_opt_sets: *const AVSetOption,
        p_av_opt_sets_size: u32,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " encode the ffmpeg avframe"]
    #[doc = " @param p_ffmpeg, ffmpeg object"]
    #[doc = " @param p_av_frame, the av frame"]
    #[doc = " @param p_packet, the output packet"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns an int which is the result of encoding the frame"]
    pub fn w_ffmpeg_encode(
        p_ffmpeg: w_ffmpeg,
        p_av_frame: w_av_frame,
        p_packet: *mut w_av_packet,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " decode the ffmpeg avframe"]
    #[doc = " @param p_ffmpeg, the ffmpeg object"]
    #[doc = " @param p_packet, the av packet"]
    #[doc = " @param p_av_frame, the avframe"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns an int which is the result of encoding the frame"]
    pub fn w_ffmpeg_decode(
        p_ffmpeg: w_ffmpeg,
        p_packet: w_av_packet,
        p_av_frame: *mut w_av_frame,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @param p_ffmpeg, ffmpeg object"]
    #[doc = " release all ffmpeg resources"]
    pub fn w_ffmpeg_fini(p_ffmpeg: *mut w_ffmpeg);
}
