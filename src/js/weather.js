import { $, pad } from './util.js';
import { t, dayShort } from './i18n.js';

const DEFAULT_PLACE = { name: 'Toshkent', lat: 41.311, lon: 69.240 };

const I = {
  clear: '<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="4.2"/><path d="M12 2.4v2.4M12 19.2v2.4M2.4 12h2.4M19.2 12h2.4M5.2 5.2l1.7 1.7M17.1 17.1l1.7 1.7M18.8 5.2l-1.7 1.7M6.9 17.1l-1.7 1.7"/></svg>',
  night: '<svg viewBox="0 0 24 24"><path d="M20 14.5A8.5 8.5 0 0 1 9.5 4a8.5 8.5 0 1 0 10.5 10.5z"/></svg>',
  part:  '<svg viewBox="0 0 24 24"><circle cx="8.6" cy="7.6" r="3"/><path d="M8.6 1.9v1.4M2.9 7.6h1.4M4.6 3.6l1 1M12.6 3.6l-1 1"/><path d="M10.2 20h7.2a3.5 3.5 0 0 0 .4-7 4.9 4.9 0 0 0-9.4 1 2.9 2.9 0 0 0 1.8 6z"/></svg>',
  cloud: '<svg viewBox="0 0 24 24"><path d="M7.2 18.5h9.6a4.2 4.2 0 0 0 .6-8.3 5.9 5.9 0 0 0-11.3 1.3 3.5 3.5 0 0 0 1.1 7z"/></svg>',
  rain:  '<svg viewBox="0 0 24 24"><path d="M7.2 15h9.6a4.2 4.2 0 0 0 .6-8.3 5.9 5.9 0 0 0-11.3 1.3 3.5 3.5 0 0 0 1.1 7z"/><path d="M9 18l-.9 2.4M13 18l-.9 2.4M17 18l-.9 2.4"/></svg>',
  snow:  '<svg viewBox="0 0 24 24"><path d="M7.2 15h9.6a4.2 4.2 0 0 0 .6-8.3 5.9 5.9 0 0 0-11.3 1.3 3.5 3.5 0 0 0 1.1 7z"/><path d="M9.2 18.6v.1M13 18v.1M16.8 18.6v.1M11 21v.1M15 21v.1"/></svg>',
  fog:   '<svg viewBox="0 0 24 24"><path d="M6.5 13h11a4 4 0 0 0 .4-8 5.7 5.7 0 0 0-10.9 1.2A3.4 3.4 0 0 0 6.5 13z"/><path d="M4.5 16.5h15M7 20h10"/></svg>',
  storm: '<svg viewBox="0 0 24 24"><path d="M7.2 14h9.6a4.2 4.2 0 0 0 .6-8.3 5.9 5.9 0 0 0-11.3 1.3 3.5 3.5 0 0 0 1.1 7z"/><path d="M13.2 14.6l-3.4 4.4h3.2l-1.6 3.6"/></svg>',
};

const WMO = {
  0: ['Ochiq', 'clear'], 1: ['Asosan ochiq', 'clear'], 2: ['Qisman bulutli', 'part'],
  3: ['Bulutli', 'cloud'], 45: ['Tuman', 'fog'], 48: ['Qirovli tuman', 'fog'],
  51: ['Mayda yomg\'ir', 'rain'], 53: ['Yomg\'ir', 'rain'], 55: ['Kuchli yomg\'ir', 'rain'],
  56: ['Muzli yomg\'ir', 'rain'], 57: ['Muzli yomg\'ir', 'rain'],
  61: ['Yengil yomg\'ir', 'rain'], 63: ['Yomg\'ir', 'rain'], 65: ['Kuchli yomg\'ir', 'rain'],
  66: ['Muzli yomg\'ir', 'rain'], 67: ['Muzli yomg\'ir', 'rain'],
  71: ['Yengil qor', 'snow'], 73: ['Qor', 'snow'], 75: ['Kuchli qor', 'snow'],
  77: ['Qor donalari', 'snow'], 80: ['Jala', 'rain'], 81: ['Jala', 'rain'],
  82: ['Kuchli jala', 'rain'], 85: ['Qor jalasi', 'snow'], 86: ['Qor jalasi', 'snow'],
  95: ['Momaqaldiroq', 'storm'], 96: ['Do\'lli momaqaldiroq', 'storm'],
  99: ['Do\'lli momaqaldiroq', 'storm'],
};

const TINT = { clear: 'var(--gold)', night: 'var(--violet-3)', part: 'var(--gold)',
               cloud: 'var(--ink-2)', rain: '#7fc4ff', snow: '#cfe8ff',
               fog: 'var(--ink-3)', storm: '#ff9d5c' };

function look(code, isDay = 1) {
  const [txt, k0] = WMO[code] || ['—', 'cloud'];
  const k = (!isDay && (k0 === 'clear' || k0 === 'part')) ? 'night' : k0;
  return { txt, icon: I[k], tint: TINT[k] };
}

const place = { ...DEFAULT_PLACE };
let timer = null, last = null;
let lastFetch = 0;
const MIN_GAP = 5 * 60000;

function paintNow(c, vis) {
  const set = (id, v) => $(id).textContent = v;
  if (!c) return ['#wxFeels','#wxWind','#wxVis','#wxHum','#wxPres'].forEach(i => set(i, '—'));

  set('#wxFeels', Math.round(c.apparent_temperature) + '°');
  set('#wxWind',  Math.round(c.wind_speed_10m) + ' km/s');
  set('#wxVis',   vis == null ? '—' : (vis / 1000).toFixed(vis < 10000 ? 1 : 0) + ' km');
  set('#wxHum',   Math.round(c.relative_humidity_2m) + '%');

  set('#wxPres',  Math.round(c.pressure_msl * 0.750062) + ' mm');
}

function paintHours(rows) {
  const box = $('#wxHours');
  if (!rows?.length) { box.innerHTML = `<div class="wx__none">${t('wx.none')}</div>`; return; }

  box.innerHTML = rows.map((r, i) => {
    const l = look(r.code, r.isDay);
    return `<div class="wxh${i === 0 ? ' is-now' : ''}">
              <div class="wxh__t">${i === 0 ? t('wx.now') : pad(r.h) + ':00'}</div>
              <div class="wxh__i" style="color:${l.tint}">${l.icon}</div>
              <div class="wxh__v tnum">${Math.round(r.temp)}°</div>
            </div>`;
  }).join('');
}

function smooth(pts) {
  let d = `M${pts[0].x.toFixed(1)},${pts[0].y.toFixed(1)}`;
  for (let i = 0; i < pts.length - 1; i++) {
    const p0 = pts[i - 1] || pts[i], p1 = pts[i], p2 = pts[i + 1], p3 = pts[i + 2] || p2;
    const c1x = p1.x + (p2.x - p0.x) / 6, c1y = p1.y + (p2.y - p0.y) / 6;
    const c2x = p2.x - (p3.x - p1.x) / 6, c2y = p2.y - (p3.y - p1.y) / 6;
    d += ` C${c1x.toFixed(1)},${c1y.toFixed(1)} ${c2x.toFixed(1)},${c2y.toFixed(1)} ${p2.x.toFixed(1)},${p2.y.toFixed(1)}`;
  }
  return d;
}

function paintChart(days) {
  const box = $('#wxChart');
  if (!days?.length) { box.innerHTML = ''; return; }

  const w = box.clientWidth || 612;
  const h = box.clientHeight || 92;
  const padT = 28, padB = 14;
  const vals = days.map(d => d.max);
  const min = Math.min(...vals), max = Math.max(...vals);
  const span = (max - min) || 1;

  const pts = vals.map((v, i) => ({
    x: (i + .5) * (w / vals.length),
    y: padT + (1 - (v - min) / span) * (h - padT - padB),
  }));

  box.innerHTML = `
    <svg width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">
      <defs>
        <linearGradient id="wxFill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%"   stop-color="var(--violet-2)" stop-opacity=".22"/>
          <stop offset="100%" stop-color="var(--violet-2)" stop-opacity="0"/>
        </linearGradient>
      </defs>
      <path class="wxc__area" d="${smooth(pts)} L${pts.at(-1).x.toFixed(1)},${h} L${pts[0].x.toFixed(1)},${h} Z" fill="url(#wxFill)" stroke="none"/>
      <path class="wxc__line" d="${smooth(pts)}"/>
      ${pts.map((p, i) => `
        <circle class="wxc__dot${i === 0 ? ' is-now' : ''}" cx="${p.x.toFixed(1)}" cy="${p.y.toFixed(1)}" r="3.2"/>
        <text class="wxc__lbl" x="${p.x.toFixed(1)}" y="${(p.y - 13).toFixed(1)}">${Math.round(vals[i])}°</text>`).join('')}

    </svg>`;

}

function paintDays(days) {
  const box = $('#wxDays');
  if (!days?.length) { box.innerHTML = ''; return; }

  box.innerHTML = days.map((d, i) => {
    const l = look(d.code, 1);
    return `<div class="wxd${i === 0 ? ' is-now' : ''}">
              <div class="wxd__d">${i === 0 ? t('wx.today') : dayShort(d.iso - 1)}</div>
              <div class="wxd__n tnum">${d.date}</div>
              <div class="wxd__i" style="color:${l.tint}">${l.icon}</div>
              <div class="wxd__t tnum"><b>${Math.round(d.max)}°</b><span>${Math.round(d.min)}°</span></div>
            </div>`;
  }).join('');
}

async function refresh({ force = false } = {}) {

  if (!force && last && Date.now() - lastFetch < MIN_GAP) return;

  const u = new URL('https://api.open-meteo.com/v1/forecast');
  u.search = new URLSearchParams({
    latitude: place.lat, longitude: place.lon,
    current: 'temperature_2m,apparent_temperature,relative_humidity_2m,' +
             'pressure_msl,wind_speed_10m,weather_code,is_day',
    hourly: 'temperature_2m,weather_code,visibility,is_day',
    daily:  'weather_code,temperature_2m_max,temperature_2m_min',
    forecast_days: 7, timezone: 'auto',
  });

  try {
    const r = await fetch(u, { signal: AbortSignal.timeout?.(9000) });
    if (!r.ok) throw new Error(r.status);
    const j = await r.json();

    const key = j.current.time.slice(0, 13) + ':00';
    let i0 = j.hourly.time.indexOf(key);
    if (i0 < 0) i0 = new Date().getHours();

    const hours = [];
    for (let i = i0; i < Math.min(i0 + 12, j.hourly.time.length); i++) {
      hours.push({
        h: +j.hourly.time[i].slice(11, 13),
        temp: j.hourly.temperature_2m[i],
        code: j.hourly.weather_code[i],
        isDay: j.hourly.is_day[i],
      });
    }

    const days = j.daily.time.map((t, i) => {
      const d = new Date(t + 'T00:00');
      return {
        iso: d.getDay() === 0 ? 7 : d.getDay(),
        date: `${d.getDate()}/${d.getMonth() + 1}`,
        code: j.daily.weather_code[i],
        max: j.daily.temperature_2m_max[i],
        min: j.daily.temperature_2m_min[i],
      };
    });

    last = { c: j.current, vis: j.hourly.visibility[i0] ?? null, hours, days };
    lastFetch = Date.now();
  } catch {

    if (!last) { paintNow(null); return; }
  }

  paintNow(last?.c, last?.vis);
  paintHours(last?.hours);
  paintDays(last?.days);

  paintChart(last?.days);
}

export function applyLang() {
  if (!last) return;
  paintHours(last.hours);
  paintDays(last.days);
}

export function startAutoRefresh(minutes = 15) {

  const box = $('#wxChart');
  if (window.ResizeObserver && box) {
    new ResizeObserver(() => last && paintChart(last.days)).observe(box);
  }
  document.addEventListener('visibilitychange', () => { if (!document.hidden) refresh(); });

  refresh({ force: true });
  clearInterval(timer);
  timer = setInterval(() => refresh({ force: true }), minutes * 60000);
}
