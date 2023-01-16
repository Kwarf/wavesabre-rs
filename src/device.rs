pub type Device = *mut core::ffi::c_void;

extern "C" {
    pub fn falcon() -> Device;
    pub fn slaughter() -> Device;
    pub fn thunder() -> Device;
    pub fn scissor() -> Device;
    pub fn leveller() -> Device;
    pub fn crusher() -> Device;
    pub fn echo() -> Device;
    pub fn smasher() -> Device;
    pub fn chamber() -> Device;
    pub fn twister() -> Device;
    pub fn cathedral() -> Device;
    pub fn adultery() -> Device;
    pub fn specimen() -> Device;
}

#[repr(C)]
pub enum DeviceId {
    Falcon,
    Slaughter,
    Thunder,
    Scissor,
    Leveller,
    Crusher,
    Echo,
    Smasher,
    Chamber,
    Twister,
    Cathedral,
    Adultery,
    Specimen,
}
