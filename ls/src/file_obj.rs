use windows::Win32::Foundation::{FILETIME, SYSTEMTIME, WIN32_ERROR, INVALID_HANDLE_VALUE, GetLastError, HANDLE};
use windows::Win32::Storage::FileSystem::{
    WIN32_FIND_DATAW,
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM,
    FindFirstFileW, FindNextFileW
};
use windows::Win32::System::Time::{FileTimeToSystemTime};
use std::io::{Error, ErrorKind};
use windows::core::PCWSTR;

pub struct FileObj {
    data: WIN32_FIND_DATAW
}

impl FileObj {
    pub fn name(&self) -> String {
        self.data.cFileName
            .iter()
            .map(|&v| (v & 0xFF) as u8)
            .take_while(|&c| c != 0)
            .map(|c| c as char)
            .collect()
    }

    pub fn size(&self) -> u32 {
        self.data.nFileSizeLow
    }

    pub fn attributes(&self) -> String {
        let mut attr:String = String::from("--------");
        if self.data.dwFileAttributes == FILE_ATTRIBUTE_HIDDEN.0 {
            attr.replace_range(6..7, "h");
        }
        if self.data.dwFileAttributes == FILE_ATTRIBUTE_SYSTEM.0 {
            attr.replace_range(5..6, "s");
        }
        if self.data.dwFileAttributes == FILE_ATTRIBUTE_DIRECTORY.0 {
            attr.replace_range(3..4, "d");
        }
        attr
    }

    pub fn creation_time(&self) -> String {
        time(self.data.ftCreationTime)
    }

    pub fn modify_time(&self) -> String {
        time(self.data.ftLastWriteTime)
    }

    pub fn access_time(&self) -> String {
        time(self.data.ftLastAccessTime)
    }
}

pub fn find_all_file(path: String) -> Result<Vec<FileObj>, WIN32_ERROR> {
    let mut data: WIN32_FIND_DATAW = Default::default();
    let mut file_objects = Vec::<FileObj>::new();
    unsafe {
        let file = FindFirstFileW(
            PCWSTR(path.encode_utf16().collect::<Vec<u16>>().as_ptr()),
            &mut data
        );
        let file = match file {
            Ok(file) => file,
            Err(file) => return Err(GetLastError())
        };
        file_objects.push(FileObj{data});
        loop {
            if FindNextFileW(HANDLE(file.0), &mut data).0 == 0 {
                break;
            }
            file_objects.push(FileObj{data});
        }
    }
    Ok(file_objects)
}

fn time(file_time: FILETIME) -> String {
    let mut sys_time: SYSTEMTIME = Default::default();
    unsafe {
        if FileTimeToSystemTime(&file_time, &mut sys_time).0 == 0 {
            return String::from("");
        }
    }
    format!("{}:{}:{} {}/{}/{}",
        sys_time.wHour, sys_time.wMinute, sys_time.wSecond,
        sys_time.wMonth, sys_time.wDay, sys_time.wYear
    )
}
