//! BharatOS libaep — model loader and GGUF support
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct ModelRegistry {
    pub models: [Option<Model>; 32],
    pub model_count: usize,
    pub model_paths: [u8; 256],
}

pub struct Model {
    pub id: u64,
    pub ty: ModelType,
    pub name: [u8; 64],
    pub path: [u8; 256],
    pub size_bytes: u64,
    pub memory_size_bytes: u64,
    pub context_size: u32,
    pub embedding_dim: u32,
    pub hidden_dim: u32,
    pub heads: u16,
    pub kv_heads: u16,
    pub layers: u16,
    pub vocab_size: u32,
    pub quantization: Quantization,
    pub flags: AepFlags,
    pub offloaded: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModelType {
    LLM,
    Embedding,
    Vision,
    Audio,
    Translator,
    Code,
    SpeechToText,
    TextToSpeech,
    OCR,
    Segmentation,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Quantization {
    None, // F16/F32
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q8_1,
    IQ4_NL,
    IQ3_S,
}

impl Quantization {
    pub fn bits_per_weight(&self) -> u8 {
        match self {
            Self::None => 16,
            Self::Q4_0 => 4,
            Self::Q4_1 => 4,
            Self::Q5_0 => 5,
            Self::Q5_1 => 5,
            Self::Q8_0 => 8,
            Self::IQ4_NL => 4,
            _ => 8,
        }
    }
}

impl ModelRegistry {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn add(&mut self, model: Model) -> usize {
        let idx = self.model_count;
        if idx < 32 {
            self.models[idx] = Some(model);
            self.model_count += 1;
        }
        idx
    }

    pub fn get(&self, id: u64) -> Option<&Model> {
        self.models.iter().find_map(|m| m.as_ref().filter(|mm| mm.id == id))
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Model> {
        self.models.iter().find_map(|m| {
            m.as_ref().filter(|mm| {
                let n = unsafe { core::str::from_utf8_unchecked(&mm.name) };
                n == name
            })
        })
    }

    pub fn count(&self) -> usize {
        self.model_count
    }

    pub fn list_models(&self) -> Vec<&Model> {
        self.models.iter().filter_map(|m| m.as_ref()).collect()
    }
}

static mut MODEL_REGISTRY: Option<ModelRegistry> = None;

pub fn registry<'a>() -> &'a mut ModelRegistry {
    unsafe {
        MODEL_REGISTRY.get_or_insert(ModelRegistry::new());
        MODEL_REGISTRY.as_mut().unwrap()
    }
}
