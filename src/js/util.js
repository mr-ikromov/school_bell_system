export const $  = (s, r = document) => r.querySelector(s);
export const $$ = (s, r = document) => [...r.querySelectorAll(s)];

export const pad = n => String(n).padStart(2, '0');
export const uid = () => Date.now().toString(36) + Math.random().toString(36).slice(2, 7);

export const isoDay = d => (d.getDay() === 0 ? 7 : d.getDay());

let toastTimer;
export function toast(msg, type = 'ok') {
  const el = $('#toast');
  el.textContent = msg;
  el.dataset.t = type;
  el.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.remove('show'), 2200);
}
