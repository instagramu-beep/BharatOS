# BharatOS AI Translator

## Architecture

BharatOS includes a built-in translation system powered by BharatAI.

```
┌─────────────────────────────────────────┐
│              User Application           │
│  (browser, notes, documents, apps)      │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│          Translation API (libaep)       │
│  translate(), detectLanguage(), etc.    │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         BharatAI Pipeline               │
│  ┌─────────────┐  ┌──────────────┐     │
│  │  Offline LLM │  │ Cloud Bridge │     │
│  │  (NLLB, Mad│  │  (optional)  │     │
│  │   max, etc) │  │              │     │
│  └─────────────┘  └──────────────┘     │
└─────────────────────────────────────────┘
```

## Features

- **100+ languages** supported
- **Offline capable** — models stored locally on device
- **Document translation** — batch translate with formatting preserved
- **Live translation** — real-time voice/video subtitles
- **OCR + Translation** — translate text in images
- **Bharat-centric languages** — optimized for Indic languages (Hindi, Tamil, Telugu, Kannada, Malayalam, Bengali, Marathi, Gujarati, Punjabi, Odia, Assamese, Sanskrit, and more)

## Usage

```rust
use bharat_ai::translation;

// Translate text
let translated = translation::translate(
    "Hello World",
    "en",
    "hi"
)?;

// Detect language
let lang = translation::detect_language("नमस्ते")?;

// Batch translate document
let doc = translation::DocumentTranslator::new("hi", "en");
let result = doc.translate(input_stream)?;

// Live translation (voice)
let session = translation::LiveSession::new("ta", "en");
session.start().await?;
```

## Supported Languages

| Code | Language | Code | Language |
|------|----------|------|----------|
| en | English | hi | Hindi |
| ta | Tamil | te | Telugu |
| kn | Kannada | ml | Malayalam |
| bn | Bengali | mr | Marathi |
| gu | Gujarati | pa | Punjabi |
| or | Odia | as | Assamese |
| sa | Sanskrit | ne | Nepali |
| si | Sinhala | ur | Urdu |
| zh | Chinese | ja | Japanese |
| ko | Korean | fr | French |
| de | German | es | Spanish |
| ar | Arabic | pt | Portuguese |
| ru | Russian | it | Italian |
| ... and 80+ more |
