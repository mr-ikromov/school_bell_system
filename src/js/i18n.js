export const LANGS = [
  { code: 'uz', name: "O'zbekcha", flag: '🇺🇿' },
  { code: 'en', name: 'English',   flag: '🇬🇧' },
  { code: 'ru', name: 'Русский',   flag: '🇷🇺' },
];

const DICT = {
  uz: {
    'schedule':         'Jadval',
    'schedule.empty':   "Jadval bo'sh.<br>Pastdagi tugma orqali birinchi qo'ng'iroqni qo'shing.",
    'schedule.add':     "Yangi qo'ng'iroq qo'shish",
    'schedule.name':    'nom kiriting',
    'next':             'Keyingi signal',
    'left':             'Qolgan vaqt',
    'editor.title':     'Vaqt va kunlar',
    'wx.feels':         'Daraja',
    'wx.wind':          'Shamol',
    'wx.vis':           "Ko'rinish",
    'wx.hum':           'Namlik',
    'wx.pres':          'Bosim',
    'wx.now':           'Hozir',
    'wx.today':         'Bugun',
    'wx.none':          "ma'lumot yo'q",
    'bell':             "Qo'ng'iroq",
    'bell.swap':        'Boshqa mp3 tanlash',
    'anthem':           'Madhiya',
    'anthem.sub':       "to'liq yangraydi",
    'alarm':            'Xavf signali',
    'alarm.sub':        'takrorlanadi',
    'bt.off':           'Ulanmagan',
    'bt.muted':         'Ovoz uzilgan',
    'bt.on':            'Ulangan',
    'bt.unusable':      'ovoz qabul qilmaydi',
    'settings':         'Sozlamalar',
    'settings.speaker': 'Bluetooth',
    'settings.lang':    'Til',
    'settings.power':   'Tizim',
    'win.close':        "Tizimni o'chirib chiqish",
    'win.min':          'Kichiklashtirish',
    'win.tray':         'Trayga yashirish',
    'lock.title':       'Qulf holati',
    'close':            'Yopish',
    'settings.volume':  'Ovoz',
    'power.on':         'Yoniq',
    'power.off':        "O'chiq",
    'power.ok':         'Tizim yoqildi',
    'power.warn':       "Tizim o'chirildi — signallar chalinmaydi",
    'lock.hint':        'Tahrirlash uchun qulfni oching',
    'lock.close':       'Qulflash',
    'day.on':           'yoqildi',
    'day.off':          "o'chirildi",
    'deleted':          "o'chirildi",
    'speaker.chosen':   'Ovoz kuchaytirgich tanlandi',
    'time.busy':        "Band vaqt o'tkazib yuborildi",
    'no.free.time':     "Bo'sh vaqt qolmadi",
    'need.day':         'Kamida bitta kun qolishi kerak',
    'clock.jump':       'Soat sakradi — {n} ta signal o‘tkazib yuborildi',
    'dup.removed':      '{n} ta takroriy vaqt olib tashlandi',
    'err.bell-only':    "Faqat qo'ng'iroq ovozi almashtiriladi",
    'err.unknown-kind': "Noma'lum tovush turi",
    'err.system-off':   "Tizim o'chirilgan",
    'err.unsupported-format': "Qo'llab-quvvatlanmaydigan format",
    'err.undecodable':  "Fayl ochilmadi — ovoz fayli buzuq bo'lishi mumkin",
    'err.too-short':    'Fayl juda qisqa yoki bo\'sh',
  },

  en: {
    'schedule':         'Schedule',
    'schedule.empty':   'Schedule is empty.<br>Use the button below to add the first bell.',
    'schedule.add':     'Add new bell',
    'schedule.name':    'enter a name',
    'next':             'Next bell',
    'left':             'Time left',
    'editor.title':     'Time and days',
    'wx.feels':         'Degrees',
    'wx.wind':          'Wind',
    'wx.vis':           'Visibility',
    'wx.hum':           'Humidity',
    'wx.pres':          'Pressure',
    'wx.now':           'Now',
    'wx.today':         'Today',
    'wx.none':          'no data',
    'bell':             'Bell',
    'bell.swap':        'Choose another mp3',
    'anthem':           'Anthem',
    'anthem.sub':       'plays in full',
    'alarm':            'Alarm',
    'alarm.sub':        'repeats',
    'bt.off':           'Not connected',
    'bt.muted':         'Audio cut',
    'bt.on':            'Connected',
    'bt.unusable':      'cannot receive audio',
    'settings':         'Settings',
    'settings.speaker': 'Bluetooth',
    'settings.lang':    'Language',
    'settings.power':   'System',
    'win.close':        'Turn off and quit',
    'win.min':          'Minimise',
    'win.tray':         'Hide to tray',
    'lock.title':       'Lock state',
    'close':            'Close',
    'settings.volume':  'Volume',
    'power.on':         'On',
    'power.off':        'Off',
    'power.ok':         'System on',
    'power.warn':       'System off — bells will not ring',
    'lock.hint':        'Unlock to edit',
    'lock.close':       'Lock',
    'day.on':           'enabled',
    'day.off':          'disabled',
    'deleted':          'deleted',
    'speaker.chosen':   'Speaker selected',
    'time.busy':        'Occupied time skipped',
    'no.free.time':     'No free time left',
    'need.day':         'At least one day must remain',
    'clock.jump':       'Clock jumped — {n} bells were skipped',
    'dup.removed':      '{n} duplicate times removed',
    'err.bell-only':    'Only the bell sound can be changed',
    'err.unknown-kind': 'Unknown sound type',
    'err.system-off':   'System is off',
    'err.unsupported-format': 'Unsupported format',
    'err.undecodable':  'Cannot read the file — it may be corrupted',
    'err.too-short':    'The file is too short or empty',
  },

  ru: {
    'schedule':         'Расписание',
    'schedule.empty':   'Расписание пусто.<br>Добавьте первый звонок кнопкой ниже.',
    'schedule.add':     'Добавить звонок',
    'schedule.name':    'введите название',
    'next':             'Следующий звонок',
    'left':             'Осталось',
    'editor.title':     'Время и дни',
    'wx.feels':         'Градус',
    'wx.wind':          'Ветер',
    'wx.vis':           'Видимость',
    'wx.hum':           'Влажность',
    'wx.pres':          'Давление',
    'wx.now':           'Сейчас',
    'wx.today':         'Сегодня',
    'wx.none':          'нет данных',
    'bell':             'Звонок',
    'bell.swap':        'Выбрать другой mp3',
    'anthem':           'Гимн',
    'anthem.sub':       'звучит полностью',
    'alarm':            'Тревога',
    'alarm.sub':        'повторяется',
    'bt.off':           'Не подключено',
    'bt.muted':         'Звук отключён',
    'bt.on':            'Подключено',
    'bt.unusable':      'не принимает звук',
    'settings':         'Настройки',
    'settings.speaker': 'Bluetooth',
    'settings.lang':    'Язык',
    'settings.power':   'Система',
    'win.close':        'Выключить и выйти',
    'win.min':          'Свернуть',
    'win.tray':         'Скрыть в трей',
    'lock.title':       'Состояние замка',
    'close':            'Закрыть',
    'settings.volume':  'Звук',
    'power.on':         'Вкл',
    'power.off':        'Выкл',
    'power.ok':         'Система включена',
    'power.warn':       'Система выключена — звонки не прозвучат',
    'lock.hint':        'Откройте замок для редактирования',
    'lock.close':       'Заблокировать',
    'day.on':           'включён',
    'day.off':          'выключен',
    'deleted':          'удалён',
    'speaker.chosen':   'Усилитель выбран',
    'time.busy':        'Занятое время пропущено',
    'no.free.time':     'Свободного времени не осталось',
    'need.day':         'Должен остаться хотя бы один день',
    'clock.jump':       'Часы сдвинулись — пропущено звонков: {n}',
    'dup.removed':      'Удалено повторяющихся времён: {n}',
    'err.bell-only':    'Менять можно только звук звонка',
    'err.unknown-kind': 'Неизвестный тип звука',
    'err.system-off':   'Система выключена',
    'err.unsupported-format': 'Неподдерживаемый формат',
    'err.undecodable':  'Файл не открылся — возможно, он повреждён',
    'err.too-short':    'Файл слишком короткий или пустой',
  },
};

const DAYS = {
  uz: { short: ['Du','Se','Ch','Pa','Ju','Sh','Ya'],
        full:  ['Dushanba','Seshanba','Chorshanba','Payshanba','Juma','Shanba','Yakshanba'] },
  en: { short: ['Mo','Tu','We','Th','Fr','Sa','Su'],
        full:  ['Monday','Tuesday','Wednesday','Thursday','Friday','Saturday','Sunday'] },
  ru: { short: ['Пн','Вт','Ср','Чт','Пт','Сб','Вс'],
        full:  ['Понедельник','Вторник','Среда','Четверг','Пятница','Суббота','Воскресенье'] },
};

const MONTHS = {
  uz: ['yanvar','fevral','mart','aprel','may','iyun','iyul','avgust','sentabr','oktabr','noyabr','dekabr'],
  en: ['January','February','March','April','May','June','July','August','September','October','November','December'],
  ru: ['января','февраля','марта','апреля','мая','июня','июля','августа','сентября','октября','ноября','декабря'],
};

const UNITS = {
  uz: { h: 'soat', m: 'min', lt1: 'bir daqiqadan kam' },
  en: { h: 'h',    m: 'min', lt1: 'less than a minute' },
  ru: { h: 'ч',    m: 'мин', lt1: 'меньше минуты' },
};

let lang = 'uz';

export function setLang(code) {
  lang = DICT[code] ? code : 'uz';
  document.documentElement.lang = lang;
}
export const getLang = () => lang;

export function t(key, vars) {
  let s = DICT[lang][key] ?? DICT.uz[key] ?? key;
  if (vars) for (const [k, v] of Object.entries(vars)) s = s.replaceAll(`{${k}}`, v);
  return s;
}

export const tErr = code => t(`err.${code}`);

export const dayShort = i => DAYS[lang].short[i];
export const dayFull  = i => DAYS[lang].full[i];

export function dateText(d) {
  const day = d.getDate(), mon = MONTHS[lang][d.getMonth()];
  return lang === 'uz' ? `${day}-${mon}` : `${day} ${mon}`;
}

export function humanLeft(sec) {
  if (sec < 0) return '—';
  const u = UNITS[lang];
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  if (h > 0) return `${h} ${u.h} ${String(m).padStart(2, '0')} ${u.m}`;
  if (m > 0) return `${m} ${u.m}`;
  return u.lt1;
}
