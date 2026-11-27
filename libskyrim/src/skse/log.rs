//!
//! @file log.rs
//! @author Andrew Spaulding (Kasplat)
//! @brief Implements a logging API that writes into the SKSE log folder using
//!        the exported plugin declaration metadata.
//!

use alloc::format;
use core::ffi::CStr;
use core::fmt::{Arguments, Write};
use core::str::FromStr;

use core_util::{Later, RacyCell, StringBuffer, WideStr, WideStringBuffer};
use cstd::io::File;
use windows_sys::Win32::Foundation::{MAX_PATH, S_OK};
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::UI::Shell::{FOLDERID_Documents, SHGetKnownFolderPath};
use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;

use windows_sys::Win32::Foundation::{FILETIME, SYSTEMTIME};
use windows_sys::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime;
use windows_sys::Win32::System::Time::{FileTimeToSystemTime, SystemTimeToTzSpecificLocalTime};

#[doc(hidden)]
pub use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_ICONWARNING};

use crate::SKSEPlugin_Version;

#[doc(hidden)]
pub enum LogType {
    File,
    Window(u32),
    Both(u32),
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

const BUF_SIZE: usize = 8192;
static LOG_FILE: Later<RacyCell<File>> = Later::new();
static LOG_LEVEL: RacyCell<LogLevel> = RacyCell::new(LogLevel::Info);

impl LogType {
    fn log(&self, msg: &CStr) -> Result<(), ()> {
        let win_res = match self {
            Self::Window(ico) | Self::Both(ico) => {
                let res = unsafe {
                    MessageBoxA(
                        core::ptr::null_mut(),
                        msg.as_ptr().cast(),
                        SKSEPlugin_Version.get_name_ptr().cast(),
                        *ico,
                    )
                };
                if res == 0 { Err(()) } else { Ok(()) }
            }
            _ => Ok(()),
        };

        let log_res = match self {
            Self::File | Self::Both(_) => {
                if LOG_FILE.is_init()
                    && unsafe { (*LOG_FILE.get()).write(msg.to_bytes()).is_ok() }
                    && unsafe { (*LOG_FILE.get()).flush().is_ok() }
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Ok(()),
        };

        win_res.and(log_res)
    }
}

impl LogLevel {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL ERROR",
        }
    }
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" | "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            _ => Err(()),
        }
    }
}

#[inline(always)]
pub fn set_level(level: LogLevel) {
    unsafe {
        *LOG_LEVEL.get() = level;
    }
}

#[inline(always)]
pub fn level() -> LogLevel {
    unsafe { *LOG_LEVEL.get() }
}

#[inline(always)]
pub fn enabled(level: LogLevel) -> bool {
    matches!(level, LogLevel::Fatal) || (level as u8) >= (crate::skse::log::level() as u8)
}

pub fn set_level_from_str(level_name: &str) -> Result<LogLevel, ()> {
    let parsed = LogLevel::from_str(level_name)?;
    set_level(parsed);
    Ok(parsed)
}

pub fn set_level_from_ini(
    ini: &crate::ini::Ini,
    section: &str,
    field: &str,
) -> Result<LogLevel, ()> {
    let level_name = ini.get(section, field).ok_or(())?;
    set_level_from_str(level_name)
}

pub(crate) fn open() {
    let mut buf: WideStringBuffer<BUF_SIZE> = WideStringBuffer::new();

    unsafe {
        assert!(BUF_SIZE > MAX_PATH as usize);
        let mut path: windows_sys::core::PWSTR = core::ptr::null_mut();

        assert!(
            SHGetKnownFolderPath(&FOLDERID_Documents, 0, core::ptr::null_mut(), &mut path) == S_OK
        );
        buf.write_w_str(WideStr::from_ptr(path)).unwrap();
        CoTaskMemFree(path.cast());
    }

    buf.write_fmt(format_args!(
        "\\My Games\\{}\\SKSE\\{}.log",
        crate::runtime::current_version().save_folder(),
        unsafe {
            CStr::from_ptr(SKSEPlugin_Version.get_name_ptr())
                .to_str()
                .unwrap()
        }
    ))
    .unwrap();

    let mode_storage = core_util::create_utf16_string::<4>("w+b");
    let mode = core_util::WideStr::from_slice(&mode_storage);
    LOG_FILE.init(RacyCell::new(File::wopen(buf.as_w_str(), mode).unwrap()));

    unsafe {
        (*LOG_FILE.get()).write(&cstd::io::UTF8_BOM).unwrap();
    }
}

#[doc(hidden)]
fn write_with_level(
    log_type: LogType,
    level: LogLevel,
    file: &str,
    line: u32,
    args: Arguments<'_>,
) {
    if !enabled(level) {
        return;
    }

    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);

    let mut ft: FILETIME = unsafe { core::mem::zeroed() };
    unsafe { GetSystemTimePreciseAsFileTime(&mut ft) };

    let combined = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
    let microseconds = (combined / 10) % 1_000_000;

    let mut st_utc: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { FileTimeToSystemTime(&ft, &mut st_utc) };

    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { SystemTimeToTzSpecificLocalTime(core::ptr::null(), &st_utc, &mut st) };

    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe {
        CStr::from_ptr(SKSEPlugin_Version.get_name_ptr())
            .to_str()
            .unwrap_or("Unknown")
    };

    buf.write_fmt(format_args!(
        "[{}] [{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}] [{}] {:<25} ",
        plugin_name,
        st.wYear,
        st.wMonth,
        st.wDay,
        st.wHour,
        st.wMinute,
        st.wSecond,
        microseconds,
        level.as_str(),
        loc_str
    ))
    .unwrap();

    buf.write_fmt(args).unwrap();
    buf.write_str("\n").unwrap();

    log_type.log(buf.as_c_str()).unwrap();
}

#[doc(hidden)]
pub fn write(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    write_with_level(log_type, LogLevel::Info, file, line, args);
}

#[doc(hidden)]
pub fn debug(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    write_with_level(log_type, LogLevel::Debug, file, line, args);
}

#[doc(hidden)]
pub fn warning(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    write_with_level(log_type, LogLevel::Warning, file, line, args);
}

#[doc(hidden)]
pub fn error(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    write_with_level(log_type, LogLevel::Error, file, line, args);
}

#[doc(hidden)]
pub fn fatal(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);
    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe {
        CStr::from_ptr(SKSEPlugin_Version.get_name_ptr())
            .to_str()
            .unwrap_or("Unknown")
    };

    if buf
        .write_fmt(format_args!(
            "[{}] [FATAL ERROR] {:<25} ",
            plugin_name, loc_str
        ))
        .is_err()
        || buf.write_fmt(args).is_err()
        || buf.write_str("\n").is_err()
    {
        let _ = log_type.log(core_util::cstr!(
            "The plugin encountered an unknown fatal error.\n"
        ));
    } else {
        let _ = log_type.log(buf.as_c_str());
    }
}

#[track_caller]
pub fn fatal_runtime(args: Arguments<'_>) -> ! {
    crate::skse::crash::raise_logged_runtime_error(args)
}

#[macro_export]
macro_rules! skse_message {
    ( $($arg:tt)* ) => {{
        $crate::skse::log::write(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! skse_warning {
    ( window, $($arg:tt)* ) => {{
        $crate::skse::log::warning(
            $crate::skse::log::LogType::Window($crate::skse::log::MB_ICONWARNING),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( log, $($arg:tt)* ) => {{
        $crate::skse::log::warning(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( $($arg:tt)* ) => {{
        $crate::skse::log::warning(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! skse_debug {
    ( $($arg:tt)* ) => {{
        $crate::skse::log::debug(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! skse_error {
    ( window, $($arg:tt)* ) => {{
        $crate::skse::log::error(
            $crate::skse::log::LogType::Window($crate::skse::log::MB_ICONERROR),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( log, $($arg:tt)* ) => {{
        $crate::skse::log::error(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( $($arg:tt)* ) => {{
        $crate::skse::log::error(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! skse_fatal {
    ( window, $($arg:tt)* ) => {{
        $crate::skse::log::fatal(
            $crate::skse::log::LogType::Window($crate::skse::log::MB_ICONERROR),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( log, $($arg:tt)* ) => {{
        $crate::skse::log::fatal(
            $crate::skse::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
    ( $($arg:tt)* ) => {{
        $crate::skse::log::fatal(
            $crate::skse::log::LogType::Both($crate::skse::log::MB_ICONERROR),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    }};
}
