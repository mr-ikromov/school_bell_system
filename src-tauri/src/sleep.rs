use std::sync::Mutex;

static HOLD: Mutex<Option<Hold>> = Mutex::new(None);

pub fn keep_awake(on: bool) {
    let mut hold = HOLD.lock().unwrap_or_else(|e| e.into_inner());
    if on == hold.is_some() {
        return;
    }
    if on {
        match Hold::new() {
            Some(h) => {
                *hold = Some(h);
                log::info!("uyqu bloklandi — qo'ng'iroqlar o'z vaqtida chalinadi");
            }
            None => {
                log::warn!("uyquni bloklab bo'lmadi — kompyuter uxlab qolsa signal chalinmaydi")
            }
        }
    } else {
        *hold = None;
        log::info!("uyqu bloki olib tashlandi");
    }
}

#[cfg(target_os = "linux")]
struct Hold(std::process::Child);

#[cfg(target_os = "linux")]
impl Hold {
    fn new() -> Option<Self> {
        std::process::Command::new("systemd-inhibit")
            .args([
                "--what=idle:sleep",
                "--who=Maktab qo'ng'irog'i",
                "--why=Jadval bo'yicha signal chalinadi",
                "--mode=block",
                "cat",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()
            .filter(|c| c.stdin.is_some())
            .map(Self)
    }
}

#[cfg(target_os = "linux")]
impl Drop for Hold {
    fn drop(&mut self) {
        drop(self.0.stdin.take());
        let _ = self.0.wait();
    }
}

#[cfg(target_os = "windows")]
struct Hold(std::sync::mpsc::Sender<()>);

#[cfg(target_os = "windows")]
impl Hold {
    fn new() -> Option<Self> {
        use windows::Win32::System::Power::{
            SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED,
        };
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        std::thread::Builder::new()
            .name("bell-uyqu".into())
            .spawn(move || {
                unsafe { SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED) };

                let _ = rx.recv();
                unsafe { SetThreadExecutionState(ES_CONTINUOUS) };
            })
            .ok()?;
        Some(Self(tx))
    }
}

#[cfg(target_os = "windows")]
impl Drop for Hold {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

#[cfg(target_os = "macos")]
struct Hold(u32);

#[cfg(target_os = "macos")]
impl Hold {
    fn new() -> Option<Self> {
        use std::ffi::c_void;

        type CFStringRef = *const c_void;
        const K_ON: u32 = 255;
        const UTF8: u32 = 0x0800_0100;

        #[link(name = "IOKit", kind = "framework")]
        extern "C" {
            fn IOPMAssertionCreateWithName(
                kind: CFStringRef,
                level: u32,
                name: CFStringRef,
                id: *mut u32,
            ) -> i32;
        }
        #[link(name = "CoreFoundation", kind = "framework")]
        extern "C" {
            fn CFStringCreateWithCString(
                alloc: *const c_void,
                s: *const u8,
                enc: u32,
            ) -> CFStringRef;
            fn CFRelease(o: *const c_void);
        }

        let mk = |s: &str| -> CFStringRef {
            let c = std::ffi::CString::new(s).unwrap_or_default();
            unsafe { CFStringCreateWithCString(std::ptr::null(), c.as_ptr() as *const u8, UTF8) }
        };

        let kind = mk("PreventUserIdleSystemSleep");
        let name = mk("Maktab qo'ng'irog'i: jadval bo'yicha signal");
        if kind.is_null() || name.is_null() {
            return None;
        }
        let mut id = 0u32;
        let st = unsafe { IOPMAssertionCreateWithName(kind, K_ON, name, &mut id) };
        unsafe {
            CFRelease(kind);
            CFRelease(name);
        }
        (st == 0).then_some(Self(id))
    }
}

#[cfg(target_os = "macos")]
impl Drop for Hold {
    fn drop(&mut self) {
        #[link(name = "IOKit", kind = "framework")]
        extern "C" {
            fn IOPMAssertionRelease(id: u32) -> i32;
        }
        unsafe { IOPMAssertionRelease(self.0) };
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
struct Hold;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
impl Hold {
    fn new() -> Option<Self> {
        None
    }
}
