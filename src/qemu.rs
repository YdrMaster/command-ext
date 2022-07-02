use super::ext;
use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
};

ext!(def; Qemu);

lazy_static::lazy_static! {
    static ref SEARCH_DIRS: Mutex<HashSet<PathBuf>> =
        if cfg!(target_os = "windows") {
            Mutex::new(HashSet::from_iter([PathBuf::from(r"C:\Program Files\qemu")]))
        } else {
            Mutex::new(HashSet::new())
        };
}

impl Qemu {
    pub fn search_at(path: impl AsRef<Path>) {
        SEARCH_DIRS
            .lock()
            .unwrap()
            .insert(path.as_ref().to_path_buf());
    }

    #[inline]
    fn find(name: impl AsRef<OsStr>) -> Self {
        Self(Command::new(Self::find_qemu(OsString::from_iter([
            OsStr::new("qemu-"),
            name.as_ref(),
        ]))))
    }

    #[inline]
    pub fn system(arch: impl AsRef<OsStr>) -> Self {
        Self::find(OsString::from_iter([OsStr::new("system-"), arch.as_ref()]))
    }

    #[inline]
    pub fn img() -> Self {
        Self::find("img")
    }

    fn find_qemu(mut name: OsString) -> OsString {
        #[cfg(target_os = "windows")]
        name.push(OsStr::new(".exe"));
        SEARCH_DIRS
            .lock()
            .unwrap()
            .iter()
            .map(|dir| dir.join(&name))
            .find(|path| path.is_file())
            .map_or(name, |p| p.into_os_string())
    }
}
