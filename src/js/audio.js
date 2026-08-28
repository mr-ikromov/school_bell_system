const FILES = {
  bell:   'assets/sounds/bell.mp3',
  anthem: 'assets/sounds/anthem.mp3',
  alarm:  'assets/sounds/alarm.mp3',
};
const LOOPS = { bell: false, anthem: false, alarm: true };

export function setBellFile(path) {
  FILES.bell = path;
  P.bell.noFile = false;
}

let volume = 0.8;
let onChange = () => {};
export const onStateChange = fn => { onChange = fn; };

const P = {
  bell:   { el: null, synth: null, playing: false, noFile: false },
  anthem: { el: null, synth: null, playing: false, noFile: false },
  alarm:  { el: null, synth: null, playing: false, noFile: false },
};
export const anyPlaying = () => Object.keys(P).some(k => P[k].playing);

function mark(kind, on) {
  if (P[kind].playing === on) return;
  P[kind].playing = on;
  onChange(kind, on);
}

let ctx, master;
function ac() {
  if (!ctx) {
    ctx = new (window.AudioContext || window.webkitAudioContext)();
    master = ctx.createGain();
    master.gain.value = volume;
    master.connect(ctx.destination);
  }
  if (ctx.state === 'suspended') ctx.resume();
  return ctx;
}

export function setVolume(v) {
  volume = Math.max(0, Math.min(1, v));
  if (master) master.gain.setTargetAtTime(volume, ac().currentTime, .05);
  Object.values(P).forEach(p => { if (p.el) p.el.volume = volume; });
}

function strike(t0, freq, dur, gain) {
  const c = ac();
  const g = c.createGain();
  g.gain.setValueAtTime(0, t0);
  g.gain.linearRampToValueAtTime(gain, t0 + .006);
  g.gain.exponentialRampToValueAtTime(.0001, t0 + dur);
  g.connect(master);
  const list = [];
  [[1, 1], [2.01, .42], [2.98, .26], [4.17, .14], [5.43, .08]].forEach(([m, a]) => {
    const o = c.createOscillator();
    o.type = 'sine';
    o.frequency.setValueAtTime(freq * m, t0);
    const og = c.createGain();
    og.gain.value = a;
    o.connect(og).connect(g);
    o.start(t0); o.stop(t0 + dur);
    list.push(o);
  });
  return list;
}

function tone(t0, freq, dur, gain) {
  const c = ac();
  const o = c.createOscillator();
  const g = c.createGain();
  o.type = 'triangle';
  o.frequency.setValueAtTime(freq, t0);
  g.gain.setValueAtTime(0, t0);
  g.gain.linearRampToValueAtTime(gain, t0 + .03);
  g.gain.setTargetAtTime(0, t0 + dur * .6, dur * .25);
  o.connect(g).connect(master);
  o.start(t0); o.stop(t0 + dur + .3);
  return [o];
}

function synthBell() {
  const t = ac().currentTime + .05, nodes = [];
  for (let i = 0; i < 3; i++) nodes.push(...strike(t + i * .85, 660, 2.4, .5));
  return { nodes, ms: 3 * 850 + 1600 };
}

function synthAnthem() {
  const t = ac().currentTime + .05, nodes = [];
  const seq = [[392, .5], [392, .28], [523, .6], [494, .32], [440, .32],
               [392, .55], [330, .38], [349, .32], [392, .95]];
  let at = t, total = 0;
  seq.forEach(([f, d]) => { nodes.push(...tone(at, f, d, .26)); at += d; total += d; });
  return { nodes, ms: total * 1000 + 400 };
}

function synthAlarm() {
  const c = ac();
  const o = c.createOscillator(), g = c.createGain();
  const lfo = c.createOscillator(), lfoG = c.createGain();
  o.type = 'sawtooth'; o.frequency.value = 620;
  lfo.type = 'sine'; lfo.frequency.value = 1.4; lfoG.gain.value = 300;
  lfo.connect(lfoG).connect(o.frequency);
  g.gain.value = .0001;
  g.gain.setTargetAtTime(.34, c.currentTime, .12);
  o.connect(g).connect(master);
  o.start(); lfo.start();
  return { nodes: [o, lfo], ms: 0 };
}

const SYNTH = { bell: synthBell, anthem: synthAnthem, alarm: synthAlarm };

function start(kind) {
  const p = P[kind];
  if (p.playing) return;
  mark(kind, true);

  if (p.noFile) return startSynth(kind);

  const el = new Audio(FILES[kind]);
  el.volume = volume;
  el.loop = LOOPS[kind];
  el.onended = () => { if (!el.loop) stop(kind); };

  const fallback = () => {
    if (p.el !== el) return;
    p.el = null; p.noFile = true;
    if (p.playing) startSynth(kind);
  };
  el.onerror = fallback;

  p.el = el;
  el.play().catch(fallback);
}

function startSynth(kind) {
  const p = P[kind];
  if (p.synth) return;
  const run = () => {
    const s = SYNTH[kind]();
    p.synth = s;
    if (s.ms) {
      s.timer = setTimeout(() => {
        if (!p.playing) return;
        LOOPS[kind] ? run() : stop(kind);
      }, s.ms);
    }
  };
  run();
  mark(kind, true);
}

export function stop(kind) {
  const p = P[kind];
  if (p.el) { p.el.pause(); p.el.src = ''; p.el = null; }
  if (p.synth) {
    clearTimeout(p.synth.timer);
    p.synth.nodes.forEach(n => { try { n.stop(); } catch {} });
    p.synth = null;
  }
  mark(kind, false);
}

export function toggle(kind) {
  P[kind].playing ? stop(kind) : start(kind);
  return P[kind].playing;
}

export function ringBell() { stop('bell'); start('bell'); }
