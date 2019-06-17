extern crate winapi;
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{SendMessageW};

fn main() {
    let hwnd_broadcast: HWND = 0xffff as HWND;
    let wm_syscommand = 0x0112;
    let sc_monitorpower = 0xf170;
    let power_off = 0x0002;

    unsafe {
        SendMessageW(hwnd_broadcast, wm_syscommand, sc_monitorpower, power_off);
    }
}
