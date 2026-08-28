const T = () => window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke ?? null;
const isTauri = () => !!T();

export const hasBackend = isTauri;

export async function on(event, handler) {
  const ev = window.__TAURI__?.event;
  if (!ev?.listen) return () => {};
  return ev.listen(event, e => handler(e.payload));
}

async function call(cmd, args) { return T()(cmd, args); }

const LS = {
  get(k, def) { try { return JSON.parse(localStorage.getItem(k)) ?? def; } catch { return def; } },
  set(k, v)   { localStorage.setItem(k, JSON.stringify(v)); },
};

export async function loadSchedule() {
  if (isTauri()) return call('schedule_list');
  return LS.get('bell.schedule', null);
}
export async function saveSchedule(list) {
  if (isTauri()) return call('schedule_save', { items: list });
  LS.set('bell.schedule', list);
}

export async function loadSettings() {
  if (isTauri()) return call('settings_get');
  return LS.get('bell.settings', null);
}
export async function saveSettings(s) {
  if (isTauri()) return call('settings_set', { settings: s });
  LS.set('bell.settings', s);
}

export async function win(action) {
  if (action === 'close') {
    if (!isTauri()) return false;
    await call('app_quit');
    return true;
  }
  if (action === 'tray') {
    if (!isTauri()) return false;
    await call('app_tray');
    return true;
  }
  const w = window.__TAURI__?.window?.getCurrentWindow?.()
         ?? window.__TAURI__?.window?.appWindow;
  if (!w) return false;
  if (action === 'minimize') await w.minimize();
  return true;
}

export async function setVolume(v) {
  if (!isTauri()) return;
  await call('volume_set', { volume: v });
}

export async function pickSoundFile(kind) {
  if (isTauri()) return call('sound_pick', { kind });
  return null;
}

export async function speakerList() {
  if (isTauri()) return call('speaker_list');
  return [];
}

export async function soundToggle(kind) {
  if (isTauri()) return call('sound_toggle', { kind });
  return null;
}

export async function btStatus() {
  if (isTauri()) return call('bt_status');
  return null;
}
