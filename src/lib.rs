#![no_std]

use core::{mem::size_of, time::Duration};

use device::{Device, DeviceId};

type IPlayer = *mut core::ffi::c_void;

pub mod device;

extern "C" {
    fn wsc_render(
        factory: unsafe extern "C" fn(DeviceId) -> Device,
        indata: *const u8,
        samples: *mut u16,
    ) -> i32;
    fn wsc_play(factory: unsafe extern "C" fn(DeviceId) -> Device, indata: *const u8) -> IPlayer;
    fn wsc_free(player: IPlayer);
}

pub struct Song {
    ptr: *const u16,
    len: i32,
}

impl core::ops::Deref for Song {
    type Target = [u16];

    fn deref(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len as usize) }
    }
}

impl Drop for Song {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut libc::c_void) };
    }
}

pub unsafe fn render(factory: unsafe extern "C" fn(DeviceId) -> Device, data: &[u8]) -> Song {
    let buffer = core::ptr::null_mut();
    let num_samples = wsc_render(factory, data.as_ptr(), buffer);
    Song {
        ptr: buffer,
        len: num_samples,
    }
}

pub struct Player {
    ptr: IPlayer,
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe { wsc_free(self.ptr) };
    }
}

pub unsafe fn play(factory: unsafe extern "C" fn(DeviceId) -> Device, data: &[u8]) -> Player {
    Player {
        ptr: wsc_play(factory, data.as_ptr()),
    }
}

pub fn length(data: &[u8]) -> Duration {
    let offset = size_of::<i32>() * 2;
    Duration::from_secs_f64(f64::from_le_bytes(
        data[offset..offset + size_of::<f64>()].try_into().unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static SONG_BLOB: &'static [u8] = &[
        0x8c, 0x00, 0x00, 0x00, 0x44, 0xac, 0x00, 0x00, 0x25, 0x49, 0x92, 0x24, 0x49, 0x92, 0x24,
        0x40, 0x01, 0x00, 0x00, 0x00, 0x01, 0xac, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x31, 0xdf, 0x7f, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f,
        0x00, 0x00, 0x00, 0x00, 0x69, 0xb4, 0xe7, 0x3c, 0x00, 0x00, 0x00, 0x3f, 0x0a, 0xd7, 0x23,
        0x3c, 0x00, 0x00, 0x00, 0x00, 0x69, 0xb4, 0xe7, 0x3c, 0x00, 0x00, 0x80, 0x3f, 0x0a, 0xd7,
        0x23, 0x3c, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x69, 0xb4, 0xe7, 0x3c, 0x00, 0x00, 0x00, 0x3f, 0x0a, 0xd7, 0x23,
        0x3c, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c,
        0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00,
        0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24,
        0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64,
        0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00,
        0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00,
        0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0x8a, 0x73, 0x02, 0x00, 0x3c, 0x64, 0xea,
        0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc,
        0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00,
        0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24,
        0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea,
        0x24, 0x00, 0x00, 0x3c, 0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0xea, 0x24, 0x00, 0x00, 0x3c,
        0x64, 0xea, 0x24, 0x00, 0x00, 0xbc, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0xc5,
        0x8f, 0x61, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0x8f, 0x61, 0x3f, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x01,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    unsafe extern "C" fn factory(id: DeviceId) -> Device {
        match id {
            DeviceId::Slaughter => device::slaughter(),
            _ => panic!(),
        }
    }

    #[test]
    fn can_render() {
        let result = unsafe { render(factory, &SONG_BLOB) };
        assert_eq!(907200, result.len);
    }

    #[test]
    fn can_read_length() {
        assert_eq!(10.285714286, length(SONG_BLOB).as_secs_f64());
    }
}
