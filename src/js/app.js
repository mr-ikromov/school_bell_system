import { $, $$, pad, uid, isoDay, toast } from './util.js';
import * as i18n from './i18n.js';
import { t, dayShort, dayFull, dateText, humanLeft } from './i18n.js';
import { createWheel } from './wheel.js';
import * as be from './backend.js';
import * as wx from './weather.js';

const TRASH = '<svg viewBox="0 0 24 24"><path d="M4 7h16M9 7V5h6v2M6 7l1 13h10l1-13"/></svg>';

const WORK = [1, 2, 3, 4, 5, 6];
const DEFAULT_SETTINGS = { volume: 1.0, bellFile: 'bell.mp3', language: 'uz', enabled: true };

const S = {
  schedule: [],
  settings: { ...DEFAULT_SETTINGS },
  viewId: null,
  locked: true,
  bt: null,
  quiet: false,
};

let wheelH, wheelM;

const STAGE_W = 780, STAGE_H = 560;
function fitStage() {
  const el = document.getElementById('stage');
  if (!el) return;
  const k = Math.min(window.innerWidth / STAGE_W, window.innerHeight / STAGE_H);
  el.style.transform = 'translate(-50%,-50%)' +
    (Math.abs(k - 1) < 0.004 ? '' : ` scale(${k.toFixed(4)})`);
}

(async function init() {
  const raw  = (await be.loadSchedule()) || [];
  S.schedule = dedupe(raw);
  const dropped = raw.length - S.schedule.length;
  S.settings = { ...DEFAULT_SETTINGS, ...((await be.loadSettings()) || {}) };
  i18n.setLang(S.settings.language || 'uz');

  fitStage();
  window.addEventListener('resize', fitStage);

  warmFonts();
  buildWheels();
  buildDays();
  bindDays();
  bindTitlebar();
  bindSettings();
  applyLang();
  bindList();
  bindActions();

  view(nextBell()?.bell.id ?? sorted()[0]?.id ?? null);
  setLocked(true);
  renderAll();

  if (dropped) {
    persist();
    toast(t('dup.removed', { n: dropped }), 'warn');
  }

  bindBackendEvents();
  wx.startAutoRefresh(15);
  startClock();
})();

function buildWheels() {
  const now = new Date();
  const editable = () => !S.locked;
  wheelH = createWheel($('#wheelH'), { min: 0, max: 23, value: now.getHours(),
                                       enabled: editable, onChange: () => applyTime() });
  wheelM = createWheel($('#wheelM'), { min: 0, max: 59, value: 0,
                                       enabled: editable, onChange: () => applyTime() });
}

function buildDays() {
  $('#days').innerHTML = [0,1,2,3,4,5,6].map(i =>
    `<button class="day${i === 6 ? ' is-weekend' : ''}" data-d="${i + 1}" title="${dayFull(i)}">${dayShort(i)}</button>`
  ).join('');
}

function bindDays() {
  $('#days').addEventListener('click', e => {
    const b = e.target.closest('.day');
    if (!b || S.locked) return;
    const cur = current();
    if (!cur) return;

    const d = +b.dataset.d;
    const set = new Set(cur.days);
    set.has(d) ? set.delete(d) : set.add(d);
    if (!set.size) return toast(t('need.day'), 'warn');

    cur.days = [...set].sort();
    paintDays(); refreshRow(cur); refreshFlags(); persist();
  });
}

function paintDays() {
  const cur = current();
  const on = new Set(cur?.days || []);
  $$('#days .day').forEach(b => b.classList.toggle('on', on.has(+b.dataset.d)));
}

function applyTime() {
  if (S.quiet || S.locked) return;
  const cur = current();
  if (!cur) return;

  const prev = mins(cur);
  let t = wheelH.value * 60 + wheelM.value;
  if (t === prev) return;

  if (isTaken((t / 60) | 0, t % 60, cur.id)) {
    const fwd  = ((t - prev) % 1440 + 1440) % 1440 <= 720;
    const unit = (wheelH.value !== cur.hour && wheelM.value === cur.minute) ? 60 : 1;
    const step = fwd ? unit : -unit;

    let guard = 0;
    do { t = (t + step + 1440) % 1440; }
    while (isTaken((t / 60) | 0, t % 60, cur.id) && ++guard < 1440);
    if (guard >= 1440) return;

    S.quiet = true;
    wheelH.set((t / 60) | 0);
    wheelM.set(t % 60);
    S.quiet = false;
    toast(t('time.busy'), 'warn');
  }

  cur.hour   = (t / 60) | 0;
  cur.minute = t % 60;
  renderList(); paintTitle(); persist();
}

function bindBackendEvents() {
  be.on('bell-ring', p => toast('🔔 ' + (p.label || p.time)));

  be.on('sound-state', p => paintActionState(p.kind, p.playing));

  be.on('bt-status', setBt);

  be.on('clock-jump', p => {
    if (p.skipped > 0) {
      toast(t('clock.jump', { n: p.skipped }), 'warn');
    }
  });
}

function bindTitlebar() {
  $$('.light').forEach(b => b.addEventListener('click', () => be.win(b.dataset.win)));

  pollBt();
}

async function pollBt() {
  let st = null;
  try { st = await be.btStatus(); } catch { }
  setBt(st);
}

function setBt(st) {
  S.bt = st;
  const chip  = $('#btChip');
  const state = !st?.connected ? 'off' : st.muted ? 'muted' : 'on';

  chip.dataset.state = state;

  chip.querySelector('.bt-txt').textContent =
    state === 'off' ? t('bt.off') : (st.device || t('bt.on'));

  chip.title = state === 'off' ? t('bt.off')
             : state === 'muted' ? `${st.device} — ${t('bt.muted')}`
             : `${st.device} — ${t('bt.on')}`;
}

function warmFonts() {

  setTimeout(() => {
    const el = document.documentElement;
    const cur = el.lang;
    for (const l of i18n.LANGS) {
      el.lang = l.code;
      void document.body.offsetHeight;
    }
    el.lang = cur;
  }, 350);
}

function bindSettings() {
  $('#gearBtn').addEventListener('click', toggleSettings);

  $('#langs').addEventListener('click', e => {
    const b = e.target.closest('.seg');
    if (b) chooseLang(b.dataset.code);
  });

  $('#spkBtn').addEventListener('click', toggleSpk);
  $('#spkMenu').addEventListener('click', e => {
    const o = e.target.closest('.sel__opt');
    if (o && !o.classList.contains('off')) chooseSpeaker(o.dataset.id);
  });

  $('#power').addEventListener('click', e => {
    const b = e.target.closest('.seg');
    if (b) setPower(b.dataset.on === '1');
  });

  $('#volRng').addEventListener('input',  e => setVolume(+e.target.value));
  $('#volRng').addEventListener('change', () => be.saveSettings(S.settings));

  $('#setClose').addEventListener('click', closeSettings);

  $('#settings').addEventListener('pointerdown', e => {
    if (e.target === $('#settings')) return closeSettings();

    if (!e.target.closest('.sel')) closeSpk();
  });
  document.addEventListener('keydown', e => {
    if (e.key !== 'Escape') return;

    if (!$('#spkMenu').hidden) return closeSpk();
    closeSettings();
  });

  buildLangs();
  buildPower();
  const v = Math.round((S.settings.volume ?? 1) * 100);
  $('#volRng').value = v;
  put($('#volVal'), v + '%');
}

function closeSettings() {
  closeSpk();
  $('#settings').hidden = true;
  $('#gearBtn').classList.remove('open');
}

async function toggleSettings() {
  const box = $('#settings');
  if (!box.hidden) return closeSettings();
  await buildSpeakers();
  box.hidden = false;
  $('#gearBtn').classList.add('open');
}

function buildLangs() {
  $('#langs').innerHTML = i18n.LANGS.map(l => `
    <button class="seg${l.code === i18n.getLang() ? ' on' : ''}" data-code="${l.code}">
      <span class="lang__flag">${l.flag}</span><span>${l.code.toUpperCase()}</span>
    </button>`).join('');
}

async function chooseLang(code) {
  if (code === i18n.getLang()) return;
  i18n.setLang(code);
  S.settings.language = code;
  await be.saveSettings(S.settings);
  applyLang();
}

function applyLang() {
  $$('[data-i18n]').forEach(el => { el.innerHTML = t(el.dataset.i18n); });
  $$('[data-i18n-title]').forEach(el => {
    const s = t(el.dataset.i18nTitle);
    el.title = s;
    el.setAttribute('aria-label', s);
  });

  buildLangs();
  buildDays();
  buildPower();
  paintDays();
  paintTitle();
  renderList();
  wx.applyLang();
  setBt(S.bt);
  if (!$('#settings').hidden) buildSpeakers();

  lastDate = '';
  tick();
}

async function buildSpeakers() {
  const list = await be.speakerList();
  const menu = $('#spkMenu');
  const btn  = $('#spkBtn');

  closeSpk();

  if (!list.length) {
    btn.classList.add('is-empty');
    btn.disabled = true;
    put($('#spkTxt'), t('bt.off'));
    menu.innerHTML = '';
    return;
  }

  btn.classList.remove('is-empty');
  btn.disabled = false;
  const cur = list.find(d => d.selected) || list[0];
  put($('#spkTxt'), cur.name);

  menu.innerHTML = list.map(d => `
    <button class="sel__opt${d.selected ? ' on' : ''}${d.usable ? '' : ' off'}"
            role="option" aria-selected="${!!d.selected}" data-id="${esc(d.id)}">
      <span class="sel__dot"></span>
      <span class="sel__txt">${esc(d.name)}</span>
      ${d.usable ? '' : `<span class="sel__why">${esc(t('bt.unusable'))}</span>`}

    </button>`).join('');

}

function closeSpk() {
  $('#spkMenu').hidden = true;
  $('#spkBtn').setAttribute('aria-expanded', 'false');
}

function toggleSpk() {
  const menu = $('#spkMenu');
  menu.hidden = !menu.hidden;
  $('#spkBtn').setAttribute('aria-expanded', String(!menu.hidden));
}

async function chooseSpeaker(id) {
  if (!id) return;
  closeSpk();
  S.settings.speaker = id;
  await be.saveSettings(S.settings);
  await buildSpeakers();
  toast(t('speaker.chosen'));
}

function buildPower() {
  const on = S.settings.enabled !== false;
  $('#power').innerHTML = `
    <button class="seg${on ? ' on' : ''}" data-on="1">
      <svg viewBox="0 0 24 24"><path d="M12 3.5v7.2"/><path d="M17.6 6.6a7.5 7.5 0 1 1-11.2 0"/></svg>
      <span>${esc(t('power.on'))}</span>
    </button>
    <button class="seg${on ? '' : ' on'}" data-on="0">
      <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="8.5"/><path d="M8 8l8 8"/></svg>
      <span>${esc(t('power.off'))}</span>
    </button>`;
  document.body.classList.toggle('is-off', !on);
}

async function setPower(on) {
  if (on === (S.settings.enabled !== false)) return;
  S.settings.enabled = on;
  await be.saveSettings(S.settings);
  buildPower();
  toast(on ? t('power.ok') : t('power.warn'), on ? 'ok' : 'warn');
}

function setVolume(pct) {
  put($('#volVal'), pct + '%');
  S.settings.volume = pct / 100;
  be.setVolume(S.settings.volume);
}

const fmt    = b => `${pad(b.hour)}:${pad(b.minute)}`;
const mins   = b => b.hour * 60 + b.minute;

const isTaken = (h, m, exceptId) =>
  S.schedule.some(b => b.id !== exceptId && b.hour === h && b.minute === m);

function dedupe(list) {
  const seen = new Set();
  const out  = [];
  for (const b of [...list].sort((a, c) => mins(a) - mins(c))) {
    if (seen.has(mins(b))) continue;
    seen.add(mins(b));
    out.push(b);
  }
  return out;
}
const sorted = () => [...S.schedule].sort((a, b) => (a.hour - b.hour) || (a.minute - b.minute));
const current = () => S.schedule.find(b => b.id === S.viewId) || null;
const rowOf   = b => $(`.bell[data-id="${b.id}"]`);

function view(id) {
  S.viewId = id;
  const cur = current();

  if (cur) {
    S.quiet = true;
    wheelH.set(cur.hour);
    wheelM.set(cur.minute);
    S.quiet = false;
  }

  $$('.bell').forEach(r => r.classList.toggle('is-active', r.dataset.id === S.viewId));
  paintNameEditable();
  paintDays();
  paintTitle();
}

function paintNameEditable() {
  $$('.bell').forEach(r => {
    const on  = !S.locked && r.dataset.id === S.viewId;
    const inp = r.querySelector('.bell__name');
    r.classList.toggle('is-editable', on);
    if (!inp) return;

    inp.readOnly = !on;
    inp.tabIndex = on ? 0 : -1;

    if (!on) {
      if (document.activeElement === inp) inp.blur();

      const b = S.schedule.find(x => x.id === r.dataset.id);
      if (b && inp.value !== b.label) inp.value = b.label;
    }
  });
}

function setLocked(on) {
  S.locked = on;
  $('.panel--editor').classList.toggle('is-locked', on);

  const ed = $('.editor');
  on ? ed.setAttribute('inert', '') : ed.removeAttribute('inert');

  $('#edLockBtn').title = on ? t('lock.hint') : t('lock.close');
  paintNameEditable();
  armAutoLock(!on);
}

function select(id) {
  const wasOpen = !S.locked && S.viewId !== id;
  view(id);
  setLocked(true);
  if (wasOpen) toast('Qulflandi — tahrirlash uchun qulfni oching', 'warn');
}

function lock(msg) {
  if (S.locked) return;
  setLocked(true);
  if (msg) toast(msg);
}

function toggleLock() {
  if (!current()) return;
  if (S.locked) { setLocked(false); toast('Tahrirlash ochildi — ' + fmt(current())); }
  else          { setLocked(true);  toast('Qulflandi'); }
}

const AUTO_LOCK_MS = 60000;
let autoLockTimer = null;

function armAutoLock(on) {
  clearTimeout(autoLockTimer);
  if (!on) return;
  autoLockTimer = setTimeout(() => lock('Editor qulflandi'), AUTO_LOCK_MS);
}
const bumpAutoLock = () => { if (!S.locked) armAutoLock(true); };

function paintTitle() {
  const cur = current();
  $('#edTitle').textContent = cur
    ? `${fmt(cur)} — ${cur.label || '—'}`
    : t('editor.title');
}

function bindList() {
  $('#btnAdd').addEventListener('click', addBell);

  $('#edLockBtn').addEventListener('click', toggleLock);
  document.addEventListener('keydown', e => {
    if (e.key === 'Escape') lock('Editor qulflandi');
  });

  ['pointerdown', 'wheel', 'keydown'].forEach(ev =>
    $('.panel--editor').addEventListener(ev, bumpAutoLock, { passive: true }));

  const box = $('#bellList');

  box.addEventListener('click', e => {
    const row = e.target.closest('.bell');
    if (!row) return;
    const b = S.schedule.find(x => x.id === row.dataset.id);
    if (!b) return;

    const act = e.target.closest('[data-act]')?.dataset.act;
    if (act === 'toggle') {
      b.enabled = !b.enabled;
      refreshRow(b); refreshFlags(); persist();
      return toast(`${fmt(b)} ${b.enabled ? t('day.on') : t('day.off')}`, b.enabled ? 'ok' : 'warn');
    }
    if (act === 'del') {
      S.schedule = S.schedule.filter(x => x.id !== b.id);
      const wasViewed = S.viewId === b.id;
      if (wasViewed) S.viewId = sorted()[0]?.id ?? null;
      renderAll();
      view(S.viewId);
      if (wasViewed) setLocked(true);
      persist();
      return toast(`${fmt(b)} ${t('deleted')}`, 'danger');
    }

    if (e.target.closest('.bell__name') && b.id === S.viewId && !S.locked) return;

    select(b.id);
  });

  box.addEventListener('input', e => {
    const inp = e.target.closest('.bell__name');
    if (!inp) return;
    const b = S.schedule.find(x => x.id === inp.closest('.bell').dataset.id);
    if (!b || S.locked || b.id !== S.viewId) return;
    b.label = inp.value;
    if (b.id === S.viewId) paintTitle();
    persist();
  });
  box.addEventListener('keydown', e => {
    if (e.key === 'Enter' && e.target.closest('.bell__name')) e.target.blur();
  });
}

function addBell() {

  let t = wheelH.value * 60 + wheelM.value;
  let guard = 0;
  while (isTaken((t / 60) | 0, t % 60) && guard++ < 1440) t = (t + 5) % 1440;
  if (guard >= 1440) return toast(t('no.free.time'), 'warn');

  const h = (t / 60) | 0, m = t % 60;
  const b = { id: uid(), hour: h, minute: m, label: '',
              days: current() ? [...current().days] : [...WORK], enabled: true };
  S.schedule.push(b);
  renderAll();
  view(b.id);
  setLocked(false);

  const inp = rowOf(b)?.querySelector('.bell__name');
  inp?.scrollIntoView({ block: 'nearest' });
  inp?.focus();
  persist();
}

function bindActions() {
  ['#actBell', '#actAnthem', '#actAlarm'].forEach(sel => {
    $(sel).addEventListener('click', () => {
      const kind = $(sel).dataset.kind;

      if (S.settings.enabled === false) return toast(t('power.warn'), 'warn');

      be.soundToggle(kind);
    });
  });

  $('#actBellName').addEventListener('click', async e => {
    e.stopPropagation();

    let f;
    try {
      f = await be.pickSoundFile('bell');
    } catch (err) {

      return toast(i18n.tErr(String(err)), 'danger');
    }

    if (!f) return;

    S.settings.bellFile = f;
    setBellName(f);
    persist();
    toast(`${t('bell')}: ${f}`);
  });

  setBellName(S.settings.bellFile);
}

function setBellName(f) {
  put($('#bellFileName'), String(f || '').split(/[\\/]/).pop());
}

const playing = new Set();
function paintActionState(kind, on) {
  on ? playing.add(kind) : playing.delete(kind);
  $(`.act[data-kind="${kind}"]`)?.classList.toggle('is-playing', on);
  $('#tlLive').dataset.state = playing.size ? 'ring' : '';
}

function renderAll() { renderList(); }

function renderList() {
  const box  = $('#bellList');
  const top  = box.scrollTop;
  const list = sorted();
  const nx   = nextBell();

  $('#listCount').textContent = list.length;

  if (!list.length) {
    box.innerHTML = `<div class="list__empty">${t('schedule.empty')}</div>`;
    return;
  }

  box.innerHTML = list.map(b => `
    <div class="bell${b.id === S.viewId ? ' is-active' : ''}${b.enabled ? '' : ' is-off'}${nx && nx.bell.id === b.id ? ' is-next' : ''}" data-id="${b.id}">
      <div class="bell__time tnum">${fmt(b)}</div>
      <div class="bell__mid">
        <input class="bell__name" type="text" maxlength="28" placeholder="${t('schedule.name')}"
               value="${esc(b.label)}" spellcheck="false" />
        <div class="bell__days">${[1,2,3,4,5,6,7].map(d =>
          `<i class="${b.days.includes(d) ? 'on' : ''}"></i>`).join('')}</div>
      </div>

      <div class="bell__right">
        <span class="sw${b.enabled ? ' on' : ''}" data-act="toggle"></span>

        <span class="bell__del" data-act="del">${TRASH}</span>

      </div>

    </div>`).join('');

  box.scrollTop = top;
  paintNameEditable();
}

function refreshRow(b) {
  const row = rowOf(b);
  if (!row) return;
  row.classList.toggle('is-off', !b.enabled);
  row.querySelector('.sw').classList.toggle('on', b.enabled);
  row.querySelector('.bell__time').textContent = fmt(b);
  $$('.bell__days i', row).forEach((i, k) => i.classList.toggle('on', b.days.includes(k + 1)));
}

function refreshFlags() {
  const nx = nextBell();
  $$('.bell').forEach(r => r.classList.toggle('is-next', !!nx && nx.bell.id === r.dataset.id));
}

function nextBell(now = new Date()) {
  const nowSec = now.getHours() * 3600 + now.getMinutes() * 60 + now.getSeconds();
  let best = null;

  for (let off = 0; off < 8; off++) {
    const d = isoDayAfter(now, off);
    for (const b of S.schedule) {
      if (!b.enabled || !b.days.includes(d)) continue;
      const abs = off * 86400 + b.hour * 3600 + b.minute * 60;
      if (off === 0 && abs <= nowSec) continue;
      if (!best || abs < best.abs) best = { abs, bell: b, offDays: off };
    }
  }
  return best && { ...best, left: best.abs - nowSec };
}

const isoDayAfter = (now, off) =>
  isoDay(new Date(now.getFullYear(), now.getMonth(), now.getDate() + off));

let clockTimer = null;
function startClock() {
  const step = () => {
    if (!document.hidden) tick();
    const ms = 1000 - (Date.now() % 1000);
    clockTimer = setTimeout(step, ms + 5);
  };
  step();

  document.addEventListener('visibilitychange', () => {
    if (document.hidden) { clearTimeout(clockTimer); clockTimer = null; }
    else if (!clockTimer) step();
  });
}

function put(el, value) {
  if (el.textContent !== value) el.textContent = value;
}

let lastMin = -1;
let lastDate = '';
function tick() {
  const now = new Date();

  put($('#clockHM'), `${pad(now.getHours())}:${pad(now.getMinutes())}`);

  const key = now.toDateString();
  if (key !== lastDate) {
    lastDate = key;
    const d = isoDay(now);
    put($('#heroDay'), dayFull(d - 1));
    put($('#heroDate'), dateText(now));
  }

  const nx = nextBell(now);
  put($('#nextTime'), nx
    ? fmt(nx.bell) + (nx.offDays ? ` · ${dayShort(isoDayAfter(now, nx.offDays) - 1)}` : '')
    : '—');
  put($('#nextIn'), nx ? humanLeft(nx.left) : '—');

  if (now.getMinutes() !== lastMin) {
    lastMin = now.getMinutes();
    refreshFlags();
  }
}


let saveTimer;
function persist() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    be.saveSchedule(S.schedule);
    be.saveSettings(S.settings);
  }, 300);
}
function esc(s) {
  return String(s ?? '').replace(/[&<>"]/g, c =>
    ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
}

document.addEventListener('wheel', e => {
  if (!e.target.closest('.list__scroll')) e.preventDefault();
}, { passive: false });
document.addEventListener('contextmenu', e => e.preventDefault());
