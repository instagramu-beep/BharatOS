//! BharatOS core shader formats for the 3D compositor and URP-compatible apps
//! Provides WGSL/Sark shader parsing, SPIR-V lowering, SHIR transpilation,
//! and a minimal runtime compiler.
#![no_std]

extern crate alloc;
use alloc::string::String;

pub mod ast;
pub mod token;
pub mod parser;
pub mod lower;
pub mod runtime;

/// VShader as bytecode or AST (universal intermediate)
pub struct VShader {
    pub source: String,
    pub stage: ShaderStage,
    pub bytecode: Option<Box<[u8]>>,
    pub uniforms: [VUniform; 16],
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
    RayGeneration,
    RayClosestHit,
    RayMiss,
}

#[derive(Clone, Default)]
pub struct VUniform {
    pub name: [u8; 32],
    pub loc: u32,
    pub size: u32,
    pub ty: UniformType,
}
#[derive(Clone, Copy, Default)]
pub enum UniformType { F32, F32Vec4, Mat4x4f, Sampler, ImageBuffer, AccelerationStructure }

impl UniformType { pub fn size(&self) -> u32 { match self { Self::F32 => 4, Self::F32Vec4 => 16, Self::Mat4x4f => 64, _ => 0 } } }

pub mod SarkShader {
    use alloc::string::String;
    
    /// Minimal Sark shader (pseudo-IR) → SPIR-V compiler for Bharat units
    pub struct SarkPass {
        pub code: String,
        pub capabilities: u64, // bitmask of feature flags
    }
    
    impl SarkPass {
        pub fn from_src(src: &str) -> Self {
            Self { code: String::from(src), capabilities: 0 }
        }
        
        pub fn compile(self) -> crate::Result<Box<[u8]>> {
            // Lower Sark AST → SPIR-V binary
            crate::shader::lower::lower_sark_to_spirv(&self.code, self.capabilities)
        }
    }
}
