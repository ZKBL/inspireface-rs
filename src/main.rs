#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");
use std::ffi::CString;
fn main() {
    let path = CString::new(
        "/home/arch/inspireface-rs/InspireFace/build/inspireface-linux/test_res/pack/Megatron",
    )
    .unwrap();
    let res = unsafe { HFLaunchInspireFace(path.as_ptr()) };
    if res != HSUCCEED as i64 {
        println!("加载资源失败：{}", res);
        println!("res:{}", res);
    }
    let option = unsafe {
        let option: HOption =
            (HF_ENABLE_QUALITY | HF_ENABLE_MASK_DETECT | HF_ENABLE_LIVENESS) as i32;
        option
    };
    let det_mode = unsafe {
        let det_mode: HFDetectMode = HFDetectMode_HF_DETECT_MODE_ALWAYS_DETECT;
        det_mode
    };
    let max_detect_num = unsafe {
        let max_detet_num: HInt32 = 5;
        max_detet_num
    };
    let session = unsafe {
        let session: HFSession = HFSession ;
    };
}
