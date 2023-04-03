use std::ffi::c_void;
use std::fmt;
use std::mem;

use error_stack::{Context, Report, Result};

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

#[derive(Debug)]
pub struct ElevationError;

impl fmt::Display for ElevationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error occurred in privilege check")
    }
}

impl Context for ElevationError {}

pub fn is_elevated() -> Result<bool, ElevationError> {
    let mut is_elevated = false;
    let mut h_token = HANDLE::default();

    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token).as_bool() {
            let mut elevation = TOKEN_ELEVATION::default();
            let elevation_ptr: *mut TOKEN_ELEVATION = &mut elevation;

            // Calculate the size of the buffer
            let buffer_max = mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut len = 0;

            // Check for elevation
            let token_info = GetTokenInformation(
                h_token,
                TokenElevation,
                Some(elevation_ptr as *mut c_void),
                buffer_max,
                &mut len,
            )
            .as_bool();

            // Ensure buffer is not overflowed
            // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation
            // Read `[out] ReturnLength` section
            if len > buffer_max {
                return Err(
                    Report::new(ElevationError).attach_printable("Privilege buffer overflow")
                );
            }

            // Check if token information was retrieved
            if token_info {
                // Return bool elevation status
                is_elevated = elevation.TokenIsElevated == 1;
            } else {
                // Handle error
                return Err(
                    Report::new(ElevationError).attach_printable("Failed to get token information")
                );
            }
        }
    }

    Ok(is_elevated)
}
