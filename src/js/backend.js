const invoke = (cmd, args) => window.__TAURI__.core.invoke(cmd, args);

export const on = (event, handler) =>
  window.__TAURI__.event.listen(event, e => handler(e.payload));

export const loadSchedule  = ()   => invoke('schedule_list');
export const saveSchedule  = list => invoke('schedule_save', { items: list });
export const loadSettings  = ()   => invoke('settings_get');
export const saveSettings  = s    => invoke('settings_set', { settings: s });
export const setVolume     = v    => invoke('volume_set', { volume: v });
export const pickSoundFile = kind => invoke('sound_pick', { kind });
export const speakerList   = ()   => invoke('speaker_list');
export const soundToggle   = kind => invoke('sound_toggle', { kind });
export const btStatus      = ()   => invoke('bt_status');

export function win(action) {
  if (action === 'close') return invoke('app_quit');
  if (action === 'tray')  return invoke('app_tray');
  return window.__TAURI__.window.getCurrentWindow().minimize();
}
