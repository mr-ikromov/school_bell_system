use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sink {
    pub id: String,
    pub name: String,
    pub bluetooth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BtDevice {
    pub id: String,
    pub name: String,
    pub usable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeakerRoute {
    ByDevice(String),

    MoveAfterOpen,

    None,
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn is_bluetooth(name: &str) -> bool {
    let low = name.to_ascii_lowercase();
    low.starts_with("bluez_")
        || low.contains("bluez_output")
        || low.contains(".a2dp")
        || low.contains("bluetooth")
}

#[cfg(target_os = "linux")]
mod imp {
    use std::process::Command;

    use super::Sink;

    fn pactl(args: &[&str]) -> Option<String> {
        let out = Command::new("pactl").args(args).output().ok()?;
        if !out.status.success() {
            log::warn!("pactl {args:?} muvaffaqiyatsiz");
            return None;
        }
        String::from_utf8(out.stdout).ok()
    }

    fn parse_sinks(out: &str) -> Vec<Sink> {
        let mut all = Vec::new();
        let mut id: Option<String> = None;

        for line in out.lines() {
            let l = line.trim();
            if let Some(v) = l.strip_prefix("Name: ") {
                if let Some(prev) = id.take() {
                    all.push(Sink {
                        bluetooth: super::is_bluetooth(&prev),
                        name: prev.clone(),
                        id: prev,
                    });
                }
                id = Some(v.trim().to_string());
            } else if let Some(v) = l.strip_prefix("Description: ") {
                if let Some(prev) = id.take() {
                    let d = v.trim();
                    all.push(Sink {
                        bluetooth: super::is_bluetooth(&prev),
                        name: if d.is_empty() {
                            prev.clone()
                        } else {
                            d.to_string()
                        },
                        id: prev,
                    });
                }
            }
        }
        if let Some(prev) = id {
            all.push(Sink {
                bluetooth: super::is_bluetooth(&prev),
                name: prev.clone(),
                id: prev,
            });
        }
        all
    }

    fn sink_number(name: &str) -> Option<String> {
        pactl(&["list", "short", "sinks"])?
            .lines()
            .find(|l| l.split('\t').nth(1) == Some(name))
            .and_then(|l| l.split('\t').next())
            .map(str::to_string)
    }

    fn move_input(idx: u32, sink: &str) -> bool {
        pactl(&["move-sink-input", &idx.to_string(), sink]).is_some()
    }

    pub fn sinks() -> Vec<Sink> {
        pactl(&["list", "sinks"])
            .map(|s| parse_sinks(&s))
            .unwrap_or_default()
    }

    pub fn default_sink() -> Option<Sink> {
        let name = pactl(&["get-default-sink"])?.trim().to_string();
        sinks().into_iter().find(|s| s.id == name)
    }

    pub fn set_default_sink(id: &str) -> bool {
        pactl(&["set-default-sink", id]).is_some()
    }

    pub fn move_foreign_off(from_id: &str, to_id: &str, marker: &str) -> usize {
        let (Some(num), Some(raw)) = (sink_number(from_id), pactl(&["list", "sink-inputs"])) else {
            return 0;
        };
        super::foreign_on_sink(&super::parse_inputs(&raw), &num, marker)
            .into_iter()
            .filter(|idx| move_input(*idx, to_id))
            .count()
    }

    pub fn move_everything_to(to_id: &str) -> usize {
        let Some(raw) = pactl(&["list", "sink-inputs"]) else {
            return 0;
        };
        super::parse_inputs(&raw)
            .into_iter()
            .filter(|i| move_input(i.idx, to_id))
            .count()
    }

    pub fn move_all_ours(speaker_id: &str, marker: &str) -> usize {
        let Some(raw) = pactl(&["list", "sink-inputs"]) else {
            return 0;
        };
        super::ours(&super::parse_inputs(&raw), marker)
            .into_iter()
            .filter(|idx| move_input(*idx, speaker_id))
            .count()
    }

    pub fn bluetooth_cards() -> Vec<(String, String)> {
        let Some(raw) = pactl(&["list", "cards"]) else {
            return Vec::new();
        };
        super::parse_cards(&raw)
    }

    pub fn set_sink_volume(sink: &str, percent: u32) {
        let v = format!("{percent}%");
        if pactl(&["set-sink-volume", sink, &v]).is_none() {
            log::warn!("kolonka ovozini {v} ga qo'yib bo'lmadi");
        }
    }

    pub fn available() -> bool {
        Command::new("pactl")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn route_by_device_name() -> bool {
        false
    }

    pub fn subscribe() -> Option<std::process::Child> {
        Command::new("pactl")
            .arg("subscribe")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()
    }
}

#[cfg(target_os = "windows")]
mod imp {
    use std::sync::Once;

    use windows::core::{Interface, GUID, HRESULT, PCWSTR};
    use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
    use windows::Win32::Foundation::PROPERTYKEY;
    use windows::Win32::Media::Audio::{
        eCommunications, eConsole, eMultimedia, eRender, IMMDevice, IMMDeviceEnumerator,
        MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    };
    use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
    };
    use windows::Win32::System::Variant::VT_LPWSTR;

    use super::Sink;

    const PKEY_DEVICE_ENUMERATOR_NAME: PROPERTYKEY = PROPERTYKEY {
        fmtid: GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
        pid: 24,
    };

    fn com_init() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        });
    }

    fn enumerator() -> Option<IMMDeviceEnumerator> {
        com_init();
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok() }
    }

    unsafe fn prop_string(pv: &PROPVARIANT) -> Option<String> {
        unsafe {
            let inner = &pv.Anonymous.Anonymous;
            if inner.vt != VT_LPWSTR {
                return None;
            }
            let p = inner.Anonymous.pwszVal;
            if p.is_null() {
                return None;
            }
            p.to_string().ok()
        }
    }

    unsafe fn prop(
        store: &windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore,
        key: &PROPERTYKEY,
    ) -> Option<String> {
        unsafe { prop_string(&store.GetValue(key).ok()?) }
    }

    fn to_sink(dev: &IMMDevice) -> Option<Sink> {
        unsafe {
            let id = dev.GetId().ok()?.to_string().ok()?;
            let store = dev.OpenPropertyStore(STGM_READ).ok()?;
            let name = prop(&store, &PKEY_Device_FriendlyName)?;
            let bus = prop(&store, &PKEY_DEVICE_ENUMERATOR_NAME).unwrap_or_default();

            Some(Sink {
                bluetooth: bus.to_ascii_uppercase().contains("BTH") || super::is_bluetooth(&name),
                id,
                name,
            })
        }
    }

    pub fn sinks() -> Vec<Sink> {
        let Some(e) = enumerator() else {
            return Vec::new();
        };
        unsafe {
            let Ok(coll) = e.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE) else {
                return Vec::new();
            };
            (0..coll.GetCount().unwrap_or(0))
                .filter_map(|i| coll.Item(i).ok())
                .filter_map(|d| to_sink(&d))
                .collect()
        }
    }

    pub fn default_sink() -> Option<Sink> {
        let e = enumerator()?;
        unsafe { to_sink(&e.GetDefaultAudioEndpoint(eRender, eConsole).ok()?) }
    }

    const CLSID_POLICY_CONFIG: GUID = GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

    const IID_POLICY_CONFIG: GUID = GUID::from_u128(0xf8679f50_850a_41cf_9c72_430f290290c8);

    const IID_POLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x568b9108_44bf_40b4_9006_86afe5b5a620);

    #[repr(C)]
    struct PolicyConfigVtbl {
        query_interface: usize,
        add_ref: usize,
        release: unsafe extern "system" fn(*mut PolicyConfig) -> u32,
        get_mix_format: usize,
        get_device_format: usize,
        reset_device_format: usize,
        set_device_format: usize,
        get_processing_period: usize,
        set_processing_period: usize,
        get_share_mode: usize,
        set_share_mode: usize,
        get_property_value: usize,
        set_property_value: usize,
        set_default_endpoint: unsafe extern "system" fn(*mut PolicyConfig, PCWSTR, i32) -> HRESULT,
        set_endpoint_visibility: usize,
    }

    #[repr(C)]
    struct PolicyConfig {
        vtbl: *const PolicyConfigVtbl,
    }

    fn open_policy_config() -> Option<*mut PolicyConfig> {
        com_init();
        let unk: windows::core::IUnknown =
            unsafe { CoCreateInstance(&CLSID_POLICY_CONFIG, None, CLSCTX_ALL) }.ok()?;

        for iid in [IID_POLICY_CONFIG, IID_POLICY_CONFIG_VISTA] {
            let mut raw: *mut core::ffi::c_void = std::ptr::null_mut();
            let hr = unsafe { unk.query(&iid, &mut raw) };
            if hr.is_ok() && !raw.is_null() {
                return Some(raw as *mut PolicyConfig);
            }
        }
        None
    }

    pub fn set_default_sink(id: &str) -> bool {
        let wide: Vec<u16> = id.encode_utf16().chain(std::iter::once(0)).collect();

        let Some(pc) = open_policy_config() else {
            log::error!(
                "IPolicyConfig ochilmadi — standart chiqish qurilmasi almashtirilmaydi. \
                 Kompyuterning boshqa ovozlari kolonkaga borishi mumkin."
            );
            return false;
        };

        unsafe {
            let vtbl = &*(*pc).vtbl;
            let mut ok = true;
            for role in [eConsole, eMultimedia, eCommunications] {
                if (vtbl.set_default_endpoint)(pc, PCWSTR(wide.as_ptr()), role.0).is_err() {
                    ok = false;
                }
            }
            (vtbl.release)(pc);
            ok
        }
    }

    pub fn move_foreign_off(_from_id: &str, _to_id: &str, _marker: &str) -> usize {
        0
    }

    pub fn move_all_ours(_spk: &str, _marker: &str) -> usize {
        0
    }

    pub fn move_everything_to(_to_id: &str) -> usize {
        0
    }

    pub fn bluetooth_cards() -> Vec<(String, String)> {
        Vec::new()
    }

    pub fn set_sink_volume(_sink: &str, _percent: u32) {}

    pub fn available() -> bool {
        enumerator().is_some()
    }

    pub fn route_by_device_name() -> bool {
        true
    }

    pub fn subscribe() -> Option<std::process::Child> {
        None
    }
}

#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals, non_snake_case)]
mod imp {
    use super::Sink;
    use std::ffi::c_void;
    type OSStatus = i32;
    type AudioObjectID = u32;
    type CFStringRef = *const c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Addr {
        selector: u32,
        scope: u32,
        element: u32,
    }

    #[repr(C)]
    struct AudioBuffer {
        channels: u32,
        byte_size: u32,
        data: *mut c_void,
    }
    #[repr(C)]
    struct AudioBufferList {
        count: u32,
        buffers: [AudioBuffer; 1],
    }

    const fn fourcc(s: &[u8; 4]) -> u32 {
        ((s[0] as u32) << 24) | ((s[1] as u32) << 16) | ((s[2] as u32) << 8) | (s[3] as u32)
    }

    const SYSTEM_OBJECT: AudioObjectID = 1;
    const DEVICES: u32 = fourcc(b"dev#");
    const DEFAULT_OUTPUT: u32 = fourcc(b"dOut");
    const SYSTEM_OUTPUT: u32 = fourcc(b"sOut");
    const NAME: u32 = fourcc(b"name");
    const UID: u32 = fourcc(b"uid ");
    const TRANSPORT: u32 = fourcc(b"tran");
    const STREAM_CONFIG: u32 = fourcc(b"slay");
    const VOLUME: u32 = fourcc(b"volm");
    const SCOPE_GLOBAL: u32 = fourcc(b"glob");
    const SCOPE_OUTPUT: u32 = fourcc(b"outp");
    const ELEMENT_MAIN: u32 = 0;
    const TRANSPORT_BT: u32 = fourcc(b"blue");
    const TRANSPORT_BT_LE: u32 = fourcc(b"blea");
    const UTF8: u32 = 0x0800_0100;

    #[link(name = "CoreAudio", kind = "framework")]
    extern "C" {
        fn AudioObjectGetPropertyDataSize(
            id: AudioObjectID,
            a: *const Addr,
            qsz: u32,
            q: *const c_void,
            out: *mut u32,
        ) -> OSStatus;
        fn AudioObjectGetPropertyData(
            id: AudioObjectID,
            a: *const Addr,
            qsz: u32,
            q: *const c_void,
            sz: *mut u32,
            data: *mut c_void,
        ) -> OSStatus;
        fn AudioObjectSetPropertyData(
            id: AudioObjectID,
            a: *const Addr,
            qsz: u32,
            q: *const c_void,
            sz: u32,
            data: *const c_void,
        ) -> OSStatus;
        fn AudioObjectHasProperty(id: AudioObjectID, a: *const Addr) -> u8;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringGetCString(s: CFStringRef, buf: *mut u8, len: isize, enc: u32) -> u8;
        fn CFRelease(o: *const c_void);
    }

    fn addr(selector: u32, scope: u32) -> Addr {
        Addr {
            selector,
            scope,
            element: ELEMENT_MAIN,
        }
    }

    fn cfstring_prop(id: AudioObjectID, selector: u32, scope: u32) -> Option<String> {
        let a = addr(selector, scope);
        let mut cf: CFStringRef = std::ptr::null();
        let mut sz = std::mem::size_of::<CFStringRef>() as u32;
        let st = unsafe {
            AudioObjectGetPropertyData(
                id,
                &a,
                0,
                std::ptr::null(),
                &mut sz,
                &mut cf as *mut _ as *mut c_void,
            )
        };
        if st != 0 || cf.is_null() {
            return None;
        }
        let mut buf = [0u8; 512];
        let ok = unsafe { CFStringGetCString(cf, buf.as_mut_ptr(), buf.len() as isize, UTF8) };
        unsafe { CFRelease(cf) };
        if ok == 0 {
            return None;
        }
        let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        Some(String::from_utf8_lossy(&buf[..end]).into_owned())
    }

    fn u32_prop(id: AudioObjectID, selector: u32, scope: u32) -> Option<u32> {
        let a = addr(selector, scope);
        let mut v = 0u32;
        let mut sz = 4u32;
        let st = unsafe {
            AudioObjectGetPropertyData(
                id,
                &a,
                0,
                std::ptr::null(),
                &mut sz,
                &mut v as *mut _ as *mut c_void,
            )
        };
        (st == 0).then_some(v)
    }

    fn has_output(id: AudioObjectID) -> bool {
        let a = addr(STREAM_CONFIG, SCOPE_OUTPUT);
        let mut sz = 0u32;
        if unsafe { AudioObjectGetPropertyDataSize(id, &a, 0, std::ptr::null(), &mut sz) } != 0 {
            return false;
        }
        if sz == 0 {
            return false;
        }
        let mut buf = vec![0u8; sz as usize];
        if unsafe {
            AudioObjectGetPropertyData(
                id,
                &a,
                0,
                std::ptr::null(),
                &mut sz,
                buf.as_mut_ptr() as *mut c_void,
            )
        } != 0
        {
            return false;
        }
        let list = buf.as_ptr() as *const AudioBufferList;
        let n = unsafe { (*list).count } as usize;
        let first = unsafe { std::ptr::addr_of!((*list).buffers) } as *const AudioBuffer;
        (0..n).any(|i| unsafe { (*first.add(i)).channels } > 0)
    }

    fn device_ids() -> Vec<AudioObjectID> {
        let a = addr(DEVICES, SCOPE_GLOBAL);
        let mut sz = 0u32;
        if unsafe {
            AudioObjectGetPropertyDataSize(SYSTEM_OBJECT, &a, 0, std::ptr::null(), &mut sz)
        } != 0
        {
            return Vec::new();
        }
        let n = sz as usize / std::mem::size_of::<AudioObjectID>();
        let mut v = vec![0u32; n];
        if unsafe {
            AudioObjectGetPropertyData(
                SYSTEM_OBJECT,
                &a,
                0,
                std::ptr::null(),
                &mut sz,
                v.as_mut_ptr() as *mut c_void,
            )
        } != 0
        {
            return Vec::new();
        }
        v
    }

    fn is_bt(id: AudioObjectID) -> bool {
        matches!(u32_prop(id, TRANSPORT, SCOPE_GLOBAL), Some(t) if t == TRANSPORT_BT || t == TRANSPORT_BT_LE)
    }

    fn to_sink(id: AudioObjectID) -> Option<Sink> {
        if !has_output(id) {
            return None;
        }
        Some(Sink {
            id: cfstring_prop(id, UID, SCOPE_GLOBAL)?,
            name: cfstring_prop(id, NAME, SCOPE_OUTPUT)?,
            bluetooth: is_bt(id),
        })
    }

    pub fn sinks() -> Vec<Sink> {
        device_ids().into_iter().filter_map(to_sink).collect()
    }

    fn id_by_uid(uid: &str) -> Option<AudioObjectID> {
        device_ids()
            .into_iter()
            .find(|&d| cfstring_prop(d, UID, SCOPE_GLOBAL).as_deref() == Some(uid))
    }

    pub fn default_sink() -> Option<Sink> {
        let a = addr(DEFAULT_OUTPUT, SCOPE_GLOBAL);
        let mut id: AudioObjectID = 0;
        let mut sz = 4u32;
        let st = unsafe {
            AudioObjectGetPropertyData(
                SYSTEM_OBJECT,
                &a,
                0,
                std::ptr::null(),
                &mut sz,
                &mut id as *mut _ as *mut c_void,
            )
        };
        if st != 0 || id == 0 {
            return None;
        }
        to_sink(id)
    }

    pub fn set_default_sink(uid: &str) -> bool {
        let Some(id) = id_by_uid(uid) else {
            return false;
        };

        [DEFAULT_OUTPUT, SYSTEM_OUTPUT].iter().all(|&sel| {
            let a = addr(sel, SCOPE_GLOBAL);
            0 == unsafe {
                AudioObjectSetPropertyData(
                    SYSTEM_OBJECT,
                    &a,
                    0,
                    std::ptr::null(),
                    4,
                    &id as *const _ as *const c_void,
                )
            }
        })
    }

    pub fn bluetooth_cards() -> Vec<(String, String)> {
        sinks()
            .into_iter()
            .filter(|s| s.bluetooth)
            .map(|s| (s.id, s.name))
            .collect()
    }

    pub fn set_sink_volume(uid: &str, percent: u32) {
        let Some(id) = id_by_uid(uid) else { return };
        let v = (percent as f32 / 100.0).clamp(0.0, 1.0);

        for element in [0u32, 1, 2] {
            let a = Addr {
                selector: VOLUME,
                scope: SCOPE_OUTPUT,
                element,
            };
            if 0 == unsafe { AudioObjectHasProperty(id, &a) } {
                continue;
            }
            unsafe {
                AudioObjectSetPropertyData(
                    id,
                    &a,
                    0,
                    std::ptr::null(),
                    4,
                    &v as *const _ as *const c_void,
                );
            }
            if element == 0 {
                return;
            }
        }
    }

    pub fn move_foreign_off(_from: &str, _to: &str, _marker: &str) -> usize {
        0
    }
    pub fn move_all_ours(_spk: &str, _marker: &str) -> usize {
        0
    }
    pub fn move_everything_to(_to_id: &str) -> usize {
        0
    }

    pub fn available() -> bool {
        true
    }

    pub fn route_by_device_name() -> bool {
        true
    }

    pub fn subscribe() -> Option<std::process::Child> {
        None
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
mod imp {
    use super::Sink;

    pub fn sinks() -> Vec<Sink> {
        Vec::new()
    }
    pub fn default_sink() -> Option<Sink> {
        None
    }
    pub fn set_default_sink(_id: &str) -> bool {
        false
    }
    pub fn move_foreign_off(_from: &str, _to: &str, _marker: &str) -> usize {
        0
    }
    pub fn move_all_ours(_spk: &str, _marker: &str) -> usize {
        0
    }
    pub fn move_everything_to(_to_id: &str) -> usize {
        0
    }
    pub fn bluetooth_cards() -> Vec<(String, String)> {
        Vec::new()
    }
    pub fn set_sink_volume(_sink: &str, _percent: u32) {}
    pub fn available() -> bool {
        false
    }
    pub fn route_by_device_name() -> bool {
        true
    }
    pub fn subscribe() -> Option<std::process::Child> {
        None
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    idx: u32,
    sink: String,

    tag: String,
}

#[cfg(target_os = "linux")]
fn parse_inputs(out: &str) -> Vec<Input> {
    let mut all = Vec::new();
    let mut cur: Option<Input> = None;

    for line in out.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("Sink Input #") {
            if let Some(i) = cur.take() {
                all.push(i);
            }
            cur = rest.trim().parse().ok().map(|idx| Input {
                idx,
                sink: String::new(),
                tag: String::new(),
            });
        } else if let Some(c) = cur.as_mut() {
            if let Some(rest) = t.strip_prefix("Sink: ") {
                c.sink = rest.trim().to_string();
            } else if let Some(rest) = t.strip_prefix("application.name = ") {
                c.tag.push_str(rest.trim().trim_matches('"'));
                c.tag.push(' ');
            } else if let Some(rest) = t.strip_prefix("node.name = ") {
                c.tag.push_str(rest.trim().trim_matches('"'));
                c.tag.push(' ');
            }
        }
    }
    if let Some(i) = cur {
        all.push(i);
    }
    all
}

#[cfg(target_os = "linux")]
fn foreign_on_sink(inputs: &[Input], sink_num: &str, marker: &str) -> Vec<u32> {
    inputs
        .iter()
        .filter(|i| i.sink == sink_num && !i.tag.contains(marker))
        .map(|i| i.idx)
        .collect()
}

#[cfg(target_os = "linux")]
fn ours(inputs: &[Input], marker: &str) -> Vec<u32> {
    inputs
        .iter()
        .filter(|i| i.tag.contains(marker))
        .map(|i| i.idx)
        .collect()
}

#[cfg(target_os = "linux")]
fn parse_cards(out: &str) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let mut mac: Option<String> = None;
    let mut name = String::new();

    for line in out.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("Name: bluez_card.") {
            if let Some(m) = mac.take() {
                res.push((
                    m,
                    if name.is_empty() {
                        "Bluetooth".into()
                    } else {
                        name.clone()
                    },
                ));
            }
            mac = Some(rest.trim().to_string());
            name.clear();
        } else if t.starts_with("Name: ") {
            if let Some(m) = mac.take() {
                res.push((
                    m,
                    if name.is_empty() {
                        "Bluetooth".into()
                    } else {
                        name.clone()
                    },
                ));
            }
            name.clear();
        } else if let Some(rest) = t.strip_prefix("device.description = ") {
            if mac.is_some() {
                name = rest.trim().trim_matches('"').to_string();
            }
        }
    }
    if let Some(m) = mac {
        res.push((
            m,
            if name.is_empty() {
                "Bluetooth".into()
            } else {
                name
            },
        ));
    }
    res
}

pub fn own_marker() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "bell".into())
}

pub fn available() -> bool {
    imp::available()
}

pub fn sinks() -> Vec<Sink> {
    imp::sinks()
}

static PREFERRED: RwLock<Option<String>> = RwLock::new(None);

pub fn set_preferred_speaker(id: Option<String>) {
    if let Ok(mut g) = PREFERRED.write() {
        *g = id;
    }
}

pub fn bluetooth_sinks() -> Vec<Sink> {
    sinks().into_iter().filter(|s| s.bluetooth).collect()
}

pub fn bluetooth_devices() -> Vec<BtDevice> {
    let usable = bluetooth_sinks();
    let mut out: Vec<BtDevice> = usable
        .iter()
        .map(|s| BtDevice {
            id: s.id.clone(),
            name: s.name.clone(),
            usable: true,
        })
        .collect();

    for (mac, name) in imp::bluetooth_cards() {
        if usable.iter().any(|s| s.id.contains(&mac)) {
            continue;
        }
        out.push(BtDevice {
            id: format!("card:{mac}"),
            name,
            usable: false,
        });
    }
    out
}

pub fn bluetooth_sink() -> Option<Sink> {
    let all = bluetooth_sinks();
    if all.is_empty() {
        return None;
    }

    if let Ok(g) = PREFERRED.read() {
        if let Some(want) = g.as_deref() {
            if let Some(s) = all.iter().find(|s| s.id == want || s.name == want) {
                return Some(s.clone());
            }
        }
    }

    if all.len() == 1 {
        return all.into_iter().next();
    }

    log::warn!(
        "{} ta Bluetooth ovoz qurilmasi ulangan: {}. '{}' tanlandi — \
         agar bu ovoz kuchaytirgich bo'lmasa, sozlamada aniq ko'rsatish kerak.",
        all.len(),
        all.iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        all[0].name
    );
    all.into_iter().next()
}

pub fn internal_sink() -> Option<Sink> {
    sinks().into_iter().find(|s| !s.bluetooth)
}

pub fn speaker_route() -> SpeakerRoute {
    match bluetooth_sink() {
        None => SpeakerRoute::None,
        Some(s) if imp::route_by_device_name() => SpeakerRoute::ByDevice(s.name),
        Some(_) => SpeakerRoute::MoveAfterOpen,
    }
}

pub fn keep_others_off_speaker(marker: &str) -> (bool, usize) {
    let Some(bt) = bluetooth_sink() else {
        return (false, 0);
    };
    let Some(internal) = internal_sink() else {
        log::warn!("ichki chiqish qurilmasi topilmadi — marshrut o'zgartirilmadi");
        return (false, 0);
    };

    let mut switched = false;
    if imp::default_sink().map(|d| d.id) == Some(bt.id.clone()) {
        switched = imp::set_default_sink(&internal.id);
        if switched {
            log::info!(
                "standart chiqish '{}' dan '{}' ga qaytarildi — kompyuterning \
                 boshqa ovozlari endi maktabga eshitilmaydi",
                bt.name,
                internal.name
            );
        }
    }

    let moved = imp::move_foreign_off(&bt.id, &internal.id, marker);
    if moved > 0 {
        log::info!("{moved} ta begona ovoz oqimi kolonkadan ichki dinamikka ko'chirildi");
    }

    (switched, moved)
}

pub fn release_to_speaker() -> bool {
    let Some(bt) = bluetooth_sink() else {
        return false;
    };
    let switched = imp::set_default_sink(&bt.id);
    let moved = imp::move_everything_to(&bt.id);
    log::info!(
        "tizim o'chirildi — marshrut himoyasi bekor qilindi: standart chiqish '{}' \
         (almashtirildi={switched}, ko'chirilgan oqim={moved})",
        bt.name
    );
    switched
}

pub fn send_our_streams_to_speaker(marker: &str) -> usize {
    let Some(bt) = bluetooth_sink() else {
        return 0;
    };
    imp::move_all_ours(&bt.id, marker)
}

pub fn set_speaker_volume_max() {
    if let Some(bt) = bluetooth_sink() {
        imp::set_sink_volume(&bt.id, 100);
    }
}

pub fn subscribe() -> Option<std::process::Child> {
    imp::subscribe()
}

pub fn event_matters(line: &str) -> bool {
    line.contains("on server") || line.contains("on sink #") || line.contains("on sink-input #")
}
