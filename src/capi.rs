use super::*;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct otaws_state {
    taws_ptr: Box<crate::TawsState>,
    alarms: *const otaws_alarm,
    alarms_count: u8,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct otaws_alarm {
    name: *const u8,
    is_important: u8,
}

#[no_mangle]
pub extern "C" fn otaws_create() -> Box<otaws_state> {
    let taws = TawsState::default();
    Box::new(otaws_state {
        taws_ptr: Box::new(taws),
        alarms: std::ptr::null(),
        alarms_count: 0,
    })
}

#[no_mangle]
pub extern "C" fn otaws_delete(otaws_ptr: Box<otaws_state>) {
    std::mem::drop(otaws_ptr);
}
