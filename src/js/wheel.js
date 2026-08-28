const REPEATS = 3;

export function createWheel(el, { min, max, value = min, onChange, enabled }) {
  const track = el.querySelector('.wheel__track');
  const N     = max - min + 1;

  const frag = document.createDocumentFragment();
  for (let r = 0; r < REPEATS; r++) {
    for (let v = min; v <= max; v++) {
      const d = document.createElement('div');
      d.className = 'wheel__item';
      d.textContent = String(v).padStart(2, '0');
      d.dataset.v = v;
      frag.appendChild(d);
    }
  }
  track.appendChild(frag);
  const items = [...track.children];

  let index  = N + (value - min);
  let marked = [];
  let recenterTimer = null;
  let itemH = 0, center = 0, boxH = 0;

  function metrics() {
    const h = el.clientHeight;
    if (!itemH || h !== boxH) {
      boxH   = h;
      itemH  = items[0].offsetHeight || 31;
      center = Math.max(0, Math.round((h / itemH - 1) / 2));
    }
    return itemH;
  }
  const H = metrics;
  const val = () => min + ((index % N) + N) % N;

  function paint(animate = true) {
    metrics();
    el.classList.toggle('no-anim', !animate);
    track.style.transform = `translate3d(0, ${-(index - center) * itemH}px, 0)`;
    if (!animate) void track.offsetHeight;

    marked.forEach(i => items[i]?.classList.remove('sel', 'near'));
    marked = [];
    for (let d = -center; d <= center; d++) {
      const i = index + d;
      if (!items[i]) continue;
      items[i].classList.add(d === 0 ? 'sel' : 'near');
      marked.push(i);
    }
    el.classList.remove('no-anim');
  }

  function scheduleRecenter() {
    clearTimeout(recenterTimer);
    recenterTimer = setTimeout(() => {
      if (index >= N && index < 2 * N) return;
      index = N + ((index % N) + N) % N;
      paint(false);
    }, 300);
  }

  function move(delta, silent = false) {
    if (!delta) return;

    if (enabled && !enabled()) return;
    index += delta;
    paint(true);
    scheduleRecenter();
    if (!silent) onChange?.(val());
  }

  function set(v, animate = true) {
    const target = N + (((v - min) % N) + N) % N;
    if (target === index) return;
    index = target;
    paint(animate);
  }

  let acc = 0;
  el.addEventListener('wheel', e => {
    e.preventDefault();

    const d = e.deltaMode === 1 ? e.deltaY * 16
            : e.deltaMode === 2 ? e.deltaY * 160
            : e.deltaY;
    if (Math.abs(d) >= 40) {
      acc = 0;
      move(Math.sign(d));
    } else {
      acc += d;
      if (Math.abs(acc) >= 16) { move(Math.sign(acc)); acc = 0; }
    }
  }, { passive: false });

  let dragY = null, dragAcc = 0, moved = false;
  el.addEventListener('pointerdown', e => {
    dragY = e.clientY; dragAcc = 0; moved = false;
    el.setPointerCapture(e.pointerId);
  });
  el.addEventListener('pointermove', e => {
    if (dragY === null) return;
    dragAcc += dragY - e.clientY;
    dragY = e.clientY;
    const steps = Math.trunc(dragAcc / (H() * 0.55));
    if (steps) { dragAcc -= steps * H() * 0.55; moved = true; move(steps); }
  });
  const endDrag = e => {
    if (dragY === null) return;
    dragY = null;
    el.releasePointerCapture?.(e.pointerId);
  };
  el.addEventListener('pointerup', endDrag);
  el.addEventListener('pointercancel', endDrag);

  el.addEventListener('click', e => {
    if (moved) return;
    const it = e.target.closest('.wheel__item');
    if (!it) return;
    move(items.indexOf(it) - index);
  });

  el.addEventListener('keydown', e => {
    const k = { ArrowUp: -1, ArrowDown: 1, PageUp: -5, PageDown: 5 }[e.key];
    if (!k) return;
    e.preventDefault();
    move(k);
  });

  paint(false);
  return { get value() { return val(); }, set };
}
