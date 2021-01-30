extern crate ssq_sys;
#[macro_use]
extern crate bitflags;
#[repr(u32)]
pub enum Zone {
    UsaEast = ssq_sys::SSQ_USA_EAST,
    UsaWest = ssq_sys::SSQ_USA_WEST,
    SouthAmerica = ssq_sys::SSQ_SOUTH_AMERICA,
    Europe = ssq_sys::SSQ_EUROPE,
    Asia = ssq_sys::SSQ_ASIA,
    Australia = ssq_sys::SSQ_AUSTRALIA,
    MiddleEast = ssq_sys::SSQ_MIDDLE_EAST,
    Africa = ssq_sys::SSQ_AFRICA,
    World = ssq_sys::SSQ_WORLD,
}

#[repr(u32)]
pub enum CallbackReplyType {
    BatchReplyCallback = ssq_sys::SSQ_BATCH_REPLY_CALLBACK,
    LogReplyCallback = ssq_sys::SSQ_LOG_REPLY_CALLBACK,
    RconReplyCallback = ssq_sys::SSQ_RCON_REPLY_CALLBACK,
    RulesReplyCallback = ssq_sys::SSQ_RULES_REPLY_CALLBACK,
    LogThreadNotify = ssq_sys::SSQ_LOG_THREAD_NOTIFY,
}

bitflags! {
    struct TimeoutFlag:u32 {
        const GS = ssq_sys::SSQ_GS_TIMEOUT;
        const LOG = ssq_sys::SSQ_LOG_TIMEOUT;
        const MS = ssq_sys::SSQ_MS_TIMEOUT;
        const RCON = ssq_sys::SSQ_RCON_TIMEOUT;
    }
}

const TRUE: i32 = ssq_sys::true_ as i32;
const FALSE: i32 = ssq_sys::false_ as i32;

static mut SSQ_EXIST: bool = false;

pub struct SSQ {
    is_initialized: bool,
    is_connected: bool,
}

impl SSQ {
    pub fn new() -> Self {
        if unsafe { SSQ_EXIST } {
            panic!("Only one instance of SSQ should exist at once");
        }
        unsafe {
            SSQ_EXIST = true;
        }

        SSQ {
            is_initialized: false,
            is_connected: false,
        }
    }

    pub fn initialize(&mut self) -> bool {
        let val = unsafe { ssq_sys::SSQ_Initialize(TRUE) } == TRUE;
        self.is_initialized = val;
        val
    }

    pub fn set_timeout(&mut self, timeout_type: TimeoutFlag, timeout_value: std::time::Duration) {

        }
}

impl Drop for SSQ {
    fn drop(&mut self) {
        if self.is_initialized {
            unsafe { ssq_sys::SSQ_Initialize(FALSE) };
        }
        unsafe {
            SSQ_EXIST = false;
        }
    }
}
