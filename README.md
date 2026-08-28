<div align="center">

<br>

# 🔔 Maktab Qo‘ng‘irog‘i

### Maktab signallarini boshqaruvchi ish stoli dasturi

Jadval bo‘yicha o‘zi chaladi · Kuchaytirgichga faqat o‘z ovozini yuboradi · Kun bo‘yi ishlaydi

<br>

![Linux](https://img.shields.io/badge/Linux-✓-2ea44f?style=for-the-badge&logo=linux&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-✓-2ea44f?style=for-the-badge&logo=windows&logoColor=white)
![macOS](https://img.shields.io/badge/macOS-✓-2ea44f?style=for-the-badge&logo=apple&logoColor=white)

![Rust](https://img.shields.io/badge/Rust-Tauri_v2-DEA584?style=for-the-badge&logo=rust&logoColor=black)
![SQLite](https://img.shields.io/badge/Baza-24_KB-003B57?style=for-the-badge&logo=sqlite&logoColor=white)
![CPU](https://img.shields.io/badge/Protsessor-0,5%25-38e2a8?style=for-the-badge)

<br>

</div>

---

<div align="center">

## 🎯 Bir qarashda

<br>

</div>

<table width="100%">
<tr>
<td width="25%" align="center">

### ⏰

**Jadval**

Istalgancha vaqt.
Har biriga hafta kunlari
alohida belgilanadi.

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

### 🔊

**Aniq manzil**

Signal kuchaytirgichga,
kompyuter ovozi esa
o‘z dinamigida qoladi.

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

### 🌙

**To‘xtamaydi**

Uyquni bloklaydi,
qulflangan ekranda ham
o‘z vaqtida chalinadi.

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

### 🪶

**Yengil**

Bo‘sh turganda 0,5 %
protsessor, diskka
umuman yozmaydi.

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
</tr>
</table>

---

<div align="center">

## 🔊 Ovoz qayerga boradi

Kuchaytirgich kompyuterga Bluetooth orqali ulanadi.
Dastur ovoz oqimlarini shunday taqsimlaydi:

<br>

```mermaid
%%{init: {'flowchart': {'wrappingWidth': 600}}}%%
flowchart LR
    T["⠀💬 Telegram⠀⠀"] --> IC(["🔈 Ichki dinamik"])
    Y["⠀▶️ YouTube⠀⠀"] --> IC
    S["🖥️ Tizim signali⠀"] --> IC
    IC --> AD["👤 Administrator"]

    B["⠀🔔 Qo‘ng‘iroq⠀"] --> KU(("🔊 Kuchaytirgich"))
    M["⠀🎵 Madhiya⠀⠀"] --> KU
    X["🚨 Xavf signali⠀"] --> KU
    KU --> MK["🏫 Butun maktab"]

    style IC fill:#16213e,stroke:#60b2ff,stroke-width:2px,color:#fff
    style AD fill:#16213e,stroke:#60b2ff,color:#fff
    style KU fill:#14281f,stroke:#38e2a8,stroke-width:3px,color:#fff
    style MK fill:#14281f,stroke:#38e2a8,stroke-width:2px,color:#fff
```

<br>

Bluetooth ulanishi hech qachon uzilmaydi — dastur unga tegmaydi,
u operatsion tizimning o‘z ishi.

<br>

</div>

---

<div align="center">

## ⚙️ Signal qanday chalinadi

<br>

```mermaid
%%{init: {'flowchart': {'wrappingWidth': 600}}}%%
flowchart TD
    A(["⏱️ Har 0,5 soniyada"]) --> B{"Vaqt keldimi?"}
    B -->|yo‘q| A
    B -->|ha| C{"Bugun shu kunmi?"}
    C -->|yo‘q| A
    C -->|ha| D["⠀⠀⠀⠀⠀⠀🎵 Ovoz<br/>ochiladi⠀⠀⠀⠀⠀⠀⠀"]
    D --> E["➡️ Kuchaytirgichga<br/>yo‘naltiriladi"]
    E --> F["⠀⠀⠀🔊 Ovoz to‘liq<br/>ochiladi⠀⠀⠀⠀"]
    F --> G(["✅ Signal yangraydi"])
    G --> H["⠀⏹️ Oqim yopiladi<br/>kuchaytirgich jim⠀⠀"]
    H --> A

    style A fill:#1a1535,stroke:#7c4dff,stroke-width:2px,color:#fff
    style D fill:#14281f,stroke:#38e2a8,color:#fff
    style G fill:#14281f,stroke:#38e2a8,stroke-width:3px,color:#fff
    style H fill:#2a2015,stroke:#ffc44d,color:#fff
```

<br>

</div>

---

<div align="center">

## 📆 Maktab kuni

Jadvalga qo‘yilgan vaqtlar kun davomida shunday taqsimlanadi

<br>

```mermaid
gantt
    title Namuna jadval
    dateFormat HH:mm
    axisFormat %H:%M

    section 1-smena
    1-dars      :done, 08:00, 45m
    2-dars      :done, 08:55, 45m
    3-dars      :done, 09:50, 45m
    Tanaffus    :active, 10:35, 20m
    4-dars      :done, 10:55, 45m

    section 2-smena
    5-dars      :done, 13:00, 45m
    6-dars      :done, 13:55, 45m
    7-dars      :done, 14:50, 45m
```

<br>

Har bir dars boshi va oxiri — alohida qo‘ng‘iroq.
Hafta kunlari har biriga alohida belgilanadi.

<br>

</div>

---

<div align="center">

## 🖥️ Interfeys

<br>

</div>

<table width="100%">
<tr>
<td width="50%" align="center">

### 📋 Jadval

Chap tomonda qo‘ng‘iroqlar ro‘yxati.
Keyingi signalgacha qolgan vaqt doim ko‘rinib turadi.

Har bir qatorni yoqib-o‘chirish mumkin —
o‘chirilgan qo‘ng‘iroq jadvalda qoladi, lekin chalinmaydi.

<img src=".github/assets/spacer.png" width="800" height="1">
</td>
<td width="50%" align="center">

### 🕐 Vaqt tanlash

Soat va daqiqa sichqoncha g‘ildiragi bilan aylantiriladi.

Alohida oyna ochilmaydi —
o‘zgarish darhol jadvalda ko‘rinadi.

<img src=".github/assets/spacer.png" width="800" height="1">
</td>
</tr>
<tr>
<td width="50%" align="center">

### 🔒 Qulf

Tahrirlash qismi qulflangan turadi.
Administrator qulfni ochib vaqtni o‘zgartiradi.

Boshqa qatorga o‘tilsa eskisi yana qulflanadi.

</td>
<td width="50%" align="center">

### 🎛️ Uchta tugma

**Qo‘ng‘iroq** — o‘z mp3 faylingiz
**Madhiya** — to‘liq yangraydi
**Xavf signali** — takrorlanadi

Bir vaqtda faqat bittasi chalinadi.

</td>
</tr>
</table>

<div align="center">

<br>

Yuqori o‘ng burchakda kuchaytirgich holati, yonida sozlamalar tugmasi.
Sozlamalarda til, kuchaytirgich, ovoz balandligi va tizim tugmasi joylashgan.

<br>

</div>

---

<div align="center">

# 🧭 Turli vaziyatlarda

Har bir holat uchun dastur nima qilishini ko‘ring

<br>

</div>

---

<div align="center">

## 🕐 Kompyuter soati adashib qolsa

Dastur ikkita soatni solishtirib turadi: **monoton soat** — u hech qachon
orqaga qaytmaydi, va **devor soati** — foydalanuvchi ko‘radigan vaqt.
Ular orasidagi farq katta bo‘lsa, demak soat sakragan.

<br>

</div>

<table width="100%">
<tr>
<th width="50%" align="center">Vaziyat<img src=".github/assets/spacer.png" width="800" height="1"></th>
<th width="50%" align="center">Dastur nima qiladi<img src=".github/assets/spacer.png" width="800" height="1"></th>
</tr>
<tr>
<td align="center">Kompyuter 05:00 dan 11:00 gacha uxladi</td>
<td align="center">Oradagi signallar <b>belgilanadi</b>, chalinmaydi</td>
</tr>
<tr>
<td align="center">Signal vaqtidan 90 soniya o‘tdi</td>
<td align="center">Signal <b>baribir chalinadi</b></td>
</tr>
<tr>
<td align="center">Signal vaqtidan 3 daqiqa o‘tdi</td>
<td align="center"><b>Chalinmaydi</b> — kech qolgan</td>
</tr>
<tr>
<td align="center">Soat orqaga sakradi</td>
<td align="center">Hech narsa qilinmaydi</td>
</tr>
<tr>
<td align="center">Batareya o‘lgan, soat internetdan to‘g‘rilandi</td>
<td align="center">O‘tganlari belgilanadi, keyingilari normal</td>
</tr>
<tr>
<td align="center">Kompyuter 28 kun o‘chiq turgan</td>
<td align="center">Hisob 7 kun bilan cheklanadi</td>
</tr>
</table>

<div align="center">

<br>

Maktabda 3 soat kechikkan qo‘ng‘iroq darsni chalkashtiradi —
shuning uchun o‘tib ketgan signal ataylab chalinmaydi.

<br>

</div>

---

<div align="center">

## 😴 Kompyuter uyquga ketsa

Uxlagan kompyuterda vaqt hisoblagichi ham to‘xtaydi.
Shuning uchun tizim yoqilgan bo‘lsa dastur uyquni bloklaydi.

<br>

</div>

<table width="100%">
<tr>
<th width="33%" align="center">Tizim<img src=".github/assets/spacer.png" width="528" height="1"></th>
<th width="34%" align="center">Usul<img src=".github/assets/spacer.png" width="544" height="1"></th>
<th width="33%" align="center">Administrator huquqi<img src=".github/assets/spacer.png" width="528" height="1"></th>
</tr>
<tr>
<td align="center">🐧 &nbsp; Linux</td>
<td align="center"><code>systemd-inhibit</code></td>
<td align="center">kerak emas</td>
</tr>
<tr>
<td align="center">🪟 &nbsp; Windows</td>
<td align="center"><code>SetThreadExecutionState</code></td>
<td align="center">kerak emas</td>
</tr>
<tr>
<td align="center">🍏 &nbsp; macOS</td>
<td align="center">IOKit quvvat tasdiqnomasi</td>
<td align="center">kerak emas</td>
</tr>
</table>

<div align="center">

<br>

Blok tizim o‘chirilganda o‘zi olib tashlanadi.

<br>

</div>

---

<div align="center">

## 🔐 Ekran qulflangan bo‘lsa

<br>

Qulf faqat ekranga tegishli — dastur ishlashda davom etadi.

Sinovdan o‘tkazilgan: seans qulflangan holatda qo‘ng‘iroq
**0 soniya kechikish bilan** chalindi va kuchaytirgichga bordi.

<br>

</div>

---

<div align="center">

## 🔴 Tizim o‘chirilsa

Sozlamalardagi tugma butun tizimni to‘xtatadi

<br>

```mermaid
%%{init: {'flowchart': {'wrappingWidth': 600}}}%%
flowchart LR
    O(["🔴 Tizim o‘chirildi"]) --> A["⠀⠀⠀⠀⠀⠀⠀⠀⠀Jadval<br/>to‘xtaydi⠀⠀⠀⠀⠀⠀⠀⠀⠀"]
    O --> B["Kompyuter yoqilganda<br/>dastur ishga tushmaydi"]
    O --> C["⠀⠀⠀⠀⠀⠀⠀Uyqu bloki<br/>olinadi⠀⠀⠀⠀⠀⠀⠀⠀"]
    O --> D["⠀Kompyuter odatdagi<br/>ovoz holatiga qaytadi⠀"]
    O --> E["⠀⠀⠀⠀⠀Jadval bazada<br/>saqlanib qoladi⠀⠀⠀⠀⠀"]

    style O fill:#3a1520,stroke:#ff5470,stroke-width:3px,color:#fff
    style E fill:#14281f,stroke:#38e2a8,color:#fff
```

<br>

Qayta yoqilganda hammasi tiklanadi — jadval o‘z joyida turadi.

<br>

</div>

---

<div align="center">

## 🔁 Dastur qayta ishga tushsa

<br>

Kompyuter kechqurun o‘chirilib, ertasi kuni yoqilsa —
dastur toza holatda boshlanadi va bugungi o‘tib ketgan
signallarni belgilab qo‘yadi.

Ya‘ni bir qo‘ng‘iroq **ikki marta chalinmaydi**,
o‘tib ketgani esa kech chalinmaydi.

<br>

</div>

---

<div align="center">

## 🌐 Uch tilda

<br>

</div>

<table width="100%">
<tr>
<td width="33%" align="center">

# 🇺🇿

**O‘zbek**

standart til

<img src=".github/assets/spacer.png" width="528" height="1">
</td>
<td width="34%" align="center">

# 🇬🇧

**English**

to‘liq tarjima

<img src=".github/assets/spacer.png" width="544" height="1">
</td>
<td width="33%" align="center">

# 🇷🇺

**Русский**

to‘liq tarjima

<img src=".github/assets/spacer.png" width="528" height="1">
</td>
</tr>
</table>

<div align="center">

<br>

Interfeys ham, dastur xabarlari ham tarjima qilingan.
Tanlangan til saqlanadi va keyingi ochilishda tiklanadi.

<br>

</div>

---

<div align="center">

## 📥 O‘rnatish

Tayyor fayllar **[Releases](../../releases)** sahifasida

<br>

</div>

<table width="100%">
<tr>
<th width="50%" align="center">Tizim<img src=".github/assets/spacer.png" width="800" height="1"></th>
<th width="50%" align="center">Fayl<img src=".github/assets/spacer.png" width="800" height="1"></th>
</tr>
<tr><td align="center">🪟 &nbsp; <b>Windows</b></td><td align="center"><code>.msi</code> yoki <code>.exe</code></td></tr>
<tr><td align="center">🍏 &nbsp; <b>macOS</b> — Intel va Apple Silicon</td><td align="center"><code>.dmg</code> — universal</td></tr>
<tr><td align="center">🐧 &nbsp; <b>Debian · Ubuntu</b></td><td align="center"><code>.deb</code></td></tr>
<tr><td align="center">🐧 &nbsp; <b>Fedora · openSUSE</b></td><td align="center"><code>.rpm</code></td></tr>
<tr><td align="center">🐧 &nbsp; <b>Boshqa Linux</b></td><td align="center"><code>.AppImage</code></td></tr>
</table>

<div align="center">

<br>

### Birinchi ishga tushirish

<br>

</div>

<table width="100%">
<tr>
<td width="25%" align="center">

# 1️⃣

Kuchaytirgichni kompyuterga
Bluetooth orqali ulang

*operatsion tizim orqali*

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

# 2️⃣

Dasturni oching va
⚙ sozlamalardan
kuchaytirgichni tanlang

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

# 3️⃣

Qo‘ng‘iroq tugmasidagi
fayl nomini bosib
o‘z mp3‘ingizni yuklang

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
<td width="25%" align="center">

# 4️⃣

Jadvalga vaqtlarni qo‘shing
va hafta kunlarini
belgilang

<img src=".github/assets/spacer.png" width="400" height="1">
</td>
</tr>
</table>

<div align="center">

<br>

Shundan keyin kompyuter yoqilganda dastur **o‘zi ishga tushadi**.
Administrator huquqi talab qilinmaydi.

<br>

</div>

---

<div align="center">

## 📊 Resurs sarfi

Dastur maktab kompyuterida kun bo‘yi ishlaydi — har bir ko‘rsatkich o‘lchandi

<br>

</div>

<table width="100%">
<tr>
<th width="50%" align="center">Ko‘rsatkich<img src=".github/assets/spacer.png" width="800" height="1"></th>
<th width="50%" align="center">Qiymat<img src=".github/assets/spacer.png" width="800" height="1"></th>
</tr>
<tr><td align="center">Protsessor, bo‘sh turganda</td><td align="center"><b>0,5 %</b></td></tr>
<tr><td align="center">Xotira</td><td align="center"><b>~250 MB</b></td></tr>
<tr><td align="center">Diskka yozish, bo‘sh turganda</td><td align="center"><b>0 bayt</b></td></tr>
<tr><td align="center">Ma‘lumotlar bazasi</td><td align="center"><b>24 KB</b></td></tr>
<tr><td align="center">Bir kunlik protsessor vaqti</td><td align="center"><b>~8 daqiqa</b></td></tr>
</table>

<div align="center">

<br>

### Bir kunda protsessor nima bilan band

<br>

</div>

<table width="100%">
<tr>
<th width="34%" align="center">Holat<img src=".github/assets/spacer.png" width="544" height="1"></th>
<th width="33%" align="center">Vaqt<img src=".github/assets/spacer.png" width="528" height="1"></th>
<th width="33%" align="center">Ulush<img src=".github/assets/spacer.png" width="528" height="1"></th>
</tr>
<tr>
<td align="center">Bo‘sh turish</td>
<td align="center">432 soniya</td>
<td align="center"><code>████████░░░░░░░░</code> &nbsp; 0,60 %</td>
</tr>
<tr>
<td align="center">14 ta qo‘ng‘iroq</td>
<td align="center">83 soniya</td>
<td align="center"><code>██░░░░░░░░░░░░░░</code> &nbsp; 0,12 %</td>
</tr>
<tr>
<td align="center"><b>Protsessor bo‘sh</b></td>
<td align="center"><b>71 485 soniya</b></td>
<td align="center"><b>99,30 %</b></td>
</tr>
</table>

<div align="center">

<br>

Jami 515 soniya — 20 soatlik kunning **0,7 %** i.
Qolgan vaqtda protsessor butunlay bo‘sh turadi.

<br>

</div>

---

<div align="center">

## 💾 Baza kundan-kunga o‘smaydi

Chalingan qo‘ng‘iroqlar bazaga yozilmaydi.
«Chalindi» belgisi xotirada turadi va har kuni yarim tunda tozalanadi.

<br>

Ketma-ket uchta qo‘ng‘iroq chaldirib tekshirildi:

<br>

</div>

<table width="100%">
<tr>
<th width="34%" align="center"><img src=".github/assets/spacer.png" width="544" height="1"></th>
<th width="33%" align="center">Chalishdan avval<img src=".github/assets/spacer.png" width="528" height="1"></th>
<th width="33%" align="center">Uch marta chalgach<img src=".github/assets/spacer.png" width="528" height="1"></th>
</tr>
<tr>
<td align="center">Baza hajmi</td>
<td align="center">24 576 bayt</td>
<td align="center"><b>24 576 bayt</b></td>
</tr>
<tr>
<td align="center">Barmoq izi (<code>sha256</code>)</td>
<td align="center"><code>31a77a2711bc99306175</code></td>
<td align="center"><b><code>31a77a2711bc99306175</code></b></td>
</tr>
</table>

<div align="center">

<br>

Barmoq izi bir xil — birorta bayt ham o‘zgarmadi.

Jadval tahrirlanganda qatorlar **joyida yangilanadi**: mavjud qo‘ng‘iroq
vaqti o‘zgarsa yangi yozuv yaratilmaydi. 730 kunlik foydalanish
modellanganda baza o‘sha 24 KB da qoldi.

Jurnal fayllari umuman yozilmaydi.

<br>

</div>

---

<div align="center">

## 💻 Platformalar

Jadval, ovoz, interfeys, avtostart va uyqu bloki uch tizimda bir xil ishlaydi.
Ichki usul esa har birida o‘zicha:

<br>

</div>

<table width="100%">
<tr>
<th width="34%" align="center">Tizim<img src=".github/assets/spacer.png" width="544" height="1"></th>
<th width="33%" align="center">Ovoz marshrutlash<img src=".github/assets/spacer.png" width="528" height="1"></th>
<th width="33%" align="center">Uyquni bloklash<img src=".github/assets/spacer.png" width="528" height="1"></th>
</tr>
<tr>
<td align="center">🐧 &nbsp; <b>Linux</b></td>
<td align="center">PipeWire / PulseAudio</td>
<td align="center"><code>systemd-inhibit</code></td>
</tr>
<tr>
<td align="center">🪟 &nbsp; <b>Windows</b></td>
<td align="center"><code>IPolicyConfig</code> + WASAPI</td>
<td align="center"><code>SetThreadExecutionState</code></td>
</tr>
<tr>
<td align="center">🍏 &nbsp; <b>macOS</b></td>
<td align="center">CoreAudio</td>
<td align="center">IOKit</td>
</tr>
</table>

<div align="center">

<br>

Linux‘da ovoz oqimi bir qurilmadan boshqasiga ko‘chiriladi.
Windows va macOS‘da signal to‘g‘ridan-to‘g‘ri kuchaytirgichda ochiladi,
tizimning standart chiqishi esa ichki dinamikda qoladi.

**Natija uchalasida bir xil.**

<br>

Linux‘da to‘liq sinovdan o‘tgan. Windows va macOS kodi har bir
o‘zgarishda o‘z tizimlarida tekshiriladi, lekin haqiqiy uskunada
hali ishlatilmagan.

<br><br>

**Rust · Tauri v2 · SQLite · rodio**

<sub>Node.js ishlatilmaydi</sub>

<br>

</div>
