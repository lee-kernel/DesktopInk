use std::{ffi::c_void, ptr};

pub const CURRENT: &str = env!("CARGO_PKG_VERSION");
pub const RELEASES: &str = "https://github.com/lee-kernel/DesktopInk/releases/latest";
const TAG_PREFIX: &str = "https://github.com/lee-kernel/DesktopInk/releases/tag/";
type Handle = *mut c_void;

#[link(name = "winhttp")]
extern "system" {
    fn WinHttpOpen(
        agent: *const u16,
        access: u32,
        proxy: *const u16,
        bypass: *const u16,
        flags: u32,
    ) -> Handle;
    fn WinHttpConnect(session: Handle, host: *const u16, port: u16, reserved: u32) -> Handle;
    fn WinHttpOpenRequest(
        connection: Handle,
        verb: *const u16,
        path: *const u16,
        version: *const u16,
        referer: *const u16,
        accept: *const *const u16,
        flags: u32,
    ) -> Handle;
    fn WinHttpSetTimeouts(
        handle: Handle,
        resolve: i32,
        connect: i32,
        send: i32,
        receive: i32,
    ) -> i32;
    fn WinHttpSetOption(handle: Handle, option: u32, buffer: *const c_void, length: u32) -> i32;
    fn WinHttpSendRequest(
        request: Handle,
        headers: *const u16,
        length: u32,
        data: Handle,
        data_length: u32,
        total: u32,
        context: usize,
    ) -> i32;
    fn WinHttpReceiveResponse(request: Handle, reserved: Handle) -> i32;
    fn WinHttpQueryHeaders(
        request: Handle,
        info: u32,
        name: *const u16,
        buffer: Handle,
        length: *mut u32,
        index: *mut u32,
    ) -> i32;
    fn WinHttpCloseHandle(handle: Handle) -> i32;
}

struct Internet(Handle);
impl Internet {
    fn new(handle: Handle) -> Result<Self, String> {
        if handle.is_null() {
            Err(network_error())
        } else {
            Ok(Self(handle))
        }
    }
}
impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            WinHttpCloseHandle(self.0);
        }
    }
}
fn network_error() -> String {
    format!(
        "无法连接 GitHub，请检查网络或代理后重试。\nWindows 错误：{}",
        std::io::Error::last_os_error()
    )
}
fn checked(ok: i32) -> Result<(), String> {
    if ok == 0 {
        Err(network_error())
    } else {
        Ok(())
    }
}

// Stable release tags must be vMAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH.
fn version(tag: &str) -> Option<[u64; 3]> {
    let mut parts = tag.strip_prefix('v').unwrap_or(tag).split('.');
    let mut result = [0; 3];
    for part in &mut result {
        let value = parts.next()?;
        if value.is_empty() || !value.bytes().all(|c| c.is_ascii_digit()) {
            return None;
        }
        *part = value.parse().ok()?;
    }
    if parts.next().is_some() {
        None
    } else {
        Some(result)
    }
}

#[derive(Debug, PartialEq)]
pub struct Release {
    pub tag: String,
    pub url: String,
    pub newer: bool,
}
fn release_from_location(location: &str) -> Result<Release, String> {
    let tag = location
        .strip_prefix(TAG_PREFIX)
        .ok_or("GitHub 返回了无法识别的发布地址，请稍后重试。")?;
    let latest = version(tag).ok_or("最新发布的版本号无法识别，请到 GitHub 发布页查看。")?;
    Ok(Release {
        tag: tag.into(),
        url: location.into(),
        newer: latest > version(CURRENT).expect("package version must be stable"),
    })
}

pub fn check() -> Result<Release, String> {
    unsafe {
        let session = Internet::new(WinHttpOpen(
            super::wide(&format!("DesktopInk/{CURRENT}")).as_ptr(),
            4,
            ptr::null(),
            ptr::null(),
            0,
        ))?;
        checked(WinHttpSetTimeouts(session.0, 10000, 10000, 10000, 10000))?;
        let connection = Internet::new(WinHttpConnect(
            session.0,
            super::wide("github.com").as_ptr(),
            443,
            0,
        ))?;
        let request = Internet::new(WinHttpOpenRequest(
            connection.0,
            super::wide("HEAD").as_ptr(),
            super::wide("/lee-kernel/DesktopInk/releases/latest").as_ptr(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            0x00800000,
        ))?;
        // Read the latest-release redirect itself rather than downloading release HTML.
        let disable_redirects: u32 = 2;
        checked(WinHttpSetOption(
            request.0,
            63,
            &disable_redirects as *const _ as *const c_void,
            4,
        ))?;
        checked(WinHttpSendRequest(
            request.0,
            ptr::null(),
            0,
            ptr::null_mut(),
            0,
            0,
            0,
        ))?;
        checked(WinHttpReceiveResponse(request.0, ptr::null_mut()))?;
        let mut status: u32 = 0;
        let mut bytes = 4;
        checked(WinHttpQueryHeaders(
            request.0,
            19 | 0x20000000,
            ptr::null(),
            &mut status as *mut _ as Handle,
            &mut bytes,
            ptr::null_mut(),
        ))?;
        match status {
            301 | 302 | 303 | 307 | 308 => {}
            404 => return Err("尚未找到可用的正式发布版本。".into()),
            403 | 429 => return Err("GitHub 暂时限制了访问，请稍后重试。".into()),
            _ => {
                return Err(format!(
                    "检查更新失败：GitHub 返回 HTTP {status}，请稍后重试。"
                ))
            }
        }
        let mut location = [0u16; 2048];
        bytes = std::mem::size_of_val(&location) as u32;
        checked(WinHttpQueryHeaders(
            request.0,
            33,
            ptr::null(),
            location.as_mut_ptr() as Handle,
            &mut bytes,
            ptr::null_mut(),
        ))?;
        let location = String::from_utf16(&location[..bytes as usize / 2])
            .map_err(|_| "GitHub 返回的发布地址无效。")?;
        release_from_location(&location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_numeric_versions() {
        assert!(version("v0.1.10") > version("0.1.9"));
        assert!(version("1.0.0") > version("0.99.99"));
        assert_eq!(version("v0.1.5"), version("0.1.5"));
        for tag in [
            "",
            "1.2",
            "1.2.3.4",
            "1.2.3-beta",
            "1.2.-3",
            "1.2.18446744073709551616",
        ] {
            assert!(version(tag).is_none(), "{tag}");
        }
    }
    #[test]
    fn validate_release_destination_and_result() {
        let current = release_from_location(&format!("{TAG_PREFIX}v{CURRENT}")).unwrap();
        assert!(!current.newer);
        assert!(
            !release_from_location(&format!("{TAG_PREFIX}v0.0.1"))
                .unwrap()
                .newer
        );
        assert!(
            release_from_location(&format!("{TAG_PREFIX}v99.0.0"))
                .unwrap()
                .newer
        );
        for url in [
            "https://example.com/v99.0.0",
            "https://github.com/lee-kernel/DesktopInk/releases/tag/v1.2.3/extra",
            "https://github.com/lee-kernel/DesktopInk/releases/tag/v1.2.3-beta",
        ] {
            assert!(release_from_location(url).is_err());
        }
    }
}
