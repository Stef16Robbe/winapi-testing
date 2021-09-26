use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser;
use winapi::um::winuser::{HC_ACTION, KBDLLHOOKSTRUCT, CallNextHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_KEYUP, VK_F4, VK_LMENU, VK_LCONTROL};
use std::convert::TryFrom;

static mut HOOK_HANDLE: Option<HHOOK> = None;
static mut CTRL_DOWN: bool = false;
static mut ALT_DOWN: bool = false;
static mut F4_DOWN: bool = false;

fn main() {
    unsafe {
        let hook_id = winuser::SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        );
        HOOK_HANDLE = Some(hook_id);

        let msg: winuser::LPMSG = std::ptr::null_mut();
        while winuser::GetMessageA(msg, std::ptr::null_mut(), 0, 0) > 0 {
            winuser::TranslateMessage(msg);
            winuser::DispatchMessageA(msg);
        }

        winapi::um::winuser::UnhookWindowsHookEx(hook_id);
    }
}

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)
extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let CTRL: u32 = u32::try_from(VK_LCONTROL).unwrap();
    let ALT: u32 = u32::try_from(VK_LMENU).unwrap();
    let F4: u32 = u32::try_from(VK_F4).unwrap();

    if code < HC_ACTION {
        unsafe {
            if let Some(hook_id) = HOOK_HANDLE {
                return CallNextHookEx(hook_id, code, wparam, lparam);
            } else {
                return 0;
            }
        }
    }

    let keypress: KBDLLHOOKSTRUCT = unsafe { *(lparam as *mut KBDLLHOOKSTRUCT) };
    let is_ctrl = keypress.vkCode == CTRL;
    let is_alt = keypress.vkCode == ALT;
    let is_f4 = keypress.vkCode == F4;

    if wparam == WM_KEYDOWN as usize || wparam == WM_SYSKEYDOWN as usize {
        unsafe {
            if is_ctrl {
                CTRL_DOWN = true;
            } else if is_alt {
                ALT_DOWN = true;
            } else if is_f4 {
                F4_DOWN = true;
            }
        }
    }

    unsafe {
        if CTRL_DOWN && ALT_DOWN && F4_DOWN {
            println!("Time to kill!");
            // get foreground window
            // kill that id
            

            // prevent this keypress from being propagated
            return 1;
        }
    }
    
    // this can be neater but cba
    if wparam == WM_KEYUP as usize {
        unsafe {
            if is_ctrl {
                CTRL_DOWN = false
            }
            if is_alt {
                ALT_DOWN = false
            }
            if is_f4 {
                F4_DOWN = false
            }
        }
    }
    if wparam == WM_SYSKEYUP as usize {
        unsafe {
            if is_ctrl {
                CTRL_DOWN = false
            }
            if is_alt {
                ALT_DOWN = false
            }
            if is_f4 {
                F4_DOWN = false
            }
        }
    }

    0
}
