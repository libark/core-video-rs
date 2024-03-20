use crate::libc::c_double;

extern "C" {
    pub fn CVGetCurrentHostTime() -> u64;
    pub fn CVGetHostClockFrequency() -> c_double;
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}
