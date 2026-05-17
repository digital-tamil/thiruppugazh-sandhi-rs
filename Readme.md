

# 🦚 Thiruppugazh Sandhi Corpus (thiruppugazh-sandhi-rs)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-ODbL-blue.svg)](https://opendatacommons.org/licenses/odbl/)
[![Tamil](https://img.shields.io/badge/language-Tamil-red.svg)](#தமிழ்-விளக்கம்)

A high-performance parallel corpus of 1,300+ **Thiruppugazh** (திருப்புகழ்) songs, mapping the original rhythmic verses (Sandhi/Joined) to their word-split versions (Padam Pirithal/பதம் பிரித்தது).

> "முருகா முருகா வருவாய் முத்தமிழ் இன்பம் தருவாய்"
> <br />
> "O Muruga, come and grant us the bliss of Mutthamil (Literature, Music, and Drama)."

## 📖 The Problem & Mission

Classical Tamil poetry, especially the works of **Arunagirinathar** (15th Century), utilizes a complex rhythmic system called **Chandam**. To maintain this rhythm, words are joined together according to strict grammar rules (Sandhi).

**The Challenge:** Modern readers and Natural Language Processing (NLP) models find it difficult to parse these joined words. 
**The Solution:** This project provides a structured JSON dataset that pairs the joined text with the split text, enabling:
1. **AI Training:** Teaching machine learning models to "auto-split" ancient Tamil text.
2. **Linguistic Research:** Analyzing the transformation rules of the Tamil language over 600 years.
3. **Accessibility:** Building tools that make ancient literature readable for modern speakers.

---

## 🛠️ Technical Implementation (Rust)

The dataset was generated using a custom scraper built in **Rust** to ensure memory safety and high-performance Unicode handling.

### Key Features:
- **Parallel Processing:** Utilizes the `rayon` crate to process 1,300+ pages in parallel threads.
- **Strict Unicode Handling:** Uses `unicode-segmentation` to correctly identify Tamil Grapheme Clusters (e.g., treating `கொ` as one letter, not two bytes).
- **Polite Scraping:** Implements a staggered thread-sleep (1012ms) to respect the source server's bandwidth limits.

### How to Build & Run:
```bash
# Clone the repository
git clone https://github.com/digital-tamil/thiruppugazh-sandhi-rs.git

# Run the scraper
cargo run --release
```

---

## 📊 Dataset Schema

The data is stored in `data/thiruppugazh_final.json`. Each entry follows this structure:

```json
{
  "id": 18,
  "url": "https://mayuragiri.com/thiruppugal18",
  "rhythm": "தனத்தனந் தந்தன தனத்தனந் தந்தன...",
  "verses": [
    {
      "original": "பொருப்புறுங் கொங்கையர் பொருட்கவர்ந் தொன்றிய",
      "split": "பொருப்புறும் கொங்கையர் பொருள் கவர்ந்து ஒன்றிய"
    },
    {
      "original": "பிணக்கிடுஞ் சண்டிகள் – வஞ்சமாதர்",
      "split": "பிணக்கு இடும் சண்டிகள் – வஞ்ச மாதர்"
    }
  ]
}
```

---

## 🤝 Attribution & Credit

**Data Source:** The word-split (Padam Pirithal) data is sourced from [Mayuragiri.com](https://mayuragiri.com). 

We express our deepest gratitude to the scholars and editors at Mayuragiri for their meticulous work in splitting these verses. This project aims to preserve and digitize their effort for the next generation of Tamil developers.

---

## தமிழ் விளக்கம்

இந்தத் திட்டம் அருணகிரிநாதரின் **திருப்புகழ்** பாடல்களைக் கணினி மொழியியல் (NLP) ஆய்வுகளுக்காகத் தொகுக்கும் ஒரு முயற்சியாகும். 

**முக்கிய அம்சம்:** திருப்புகழ் பாடல்கள் சந்தத்திற்காகச் சொற்கள் புணர்ந்து (Joined) இருக்கும். இந்தப் புராஜெக்ட், அந்தச் சொற்களைப் பதம் பிரித்து (Split) ஒரு தரவுத்தளமாக (Dataset) வழங்குகிறது. இதன் மூலம்:
- தமிழ் மொழிக்கான செயற்கை நுண்ணறிவு (AI) கருவிகளை உருவாக்க முடியும்.
- பழைய இலக்கியங்களை எளிமையாக வாசிக்க உதவும் மென்பொருள்களை உருவாக்கலாம்.

இது **Rust** மென்பொருள் மொழியைக் கொண்டு மிக வேகமாகவும், துல்லியமாகவும் வடிவமைக்கப்பட்டுள்ளது.

---

## 🤝 Connect with the Developer
Building the future of Tamil AI. Let's collaborate!

| Platform | Profile |
| :--- | :--- |
| **Personal Developer** | [![Instagram](https://img.shields.io/badge/Instagram-E4405F?logo=instagram&logoColor=white)](https://www.instagram.com/sanjaiyan_dev/) |
| **Tamil AI Community** | [![Instagram](https://img.shields.io/badge/Instagram-E4405F?logo=instagram&logoColor=white)](https://www.instagram.com/tamil.ai.llm/) |

---


## 📜 License


The code is licensed under the **MIT License**. 
The dataset is provided under the **Open Data Commons Open Database License (ODbL)**. You are free to share and adapt the data, provided you give credit to the source and maintain the same license.
<br/>
**Designed with ❤️ for the தமிழ் Language.**

---

