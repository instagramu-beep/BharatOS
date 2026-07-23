//! BharatOS PDF Reader — GPU-accelerated PDF rendering with text selection
#![no_std]

use crate::prelude::*;
use libsurface::*;

#[repr(C)]
pub struct PdfDocument {
    pub pages: [PdfPage; 4096],
    pub page_count: u32,
    pub title: [u8; 256],
    pub author: [u8; 128],
    pub subject: [u8; 128],
    pub creator: [u8; 128],
    pub producer: [u8; 128],
    pub creation_date: u128,
    pub mod_date: u128,
    pub version: [u8; 16],
    pub encryption: PdfEncryption,
    pub permissions: PdfPermissions,
    pub metadata: BTreeMap<u32, Vec<u8>>,
    pub font_table: [PdfFont; 256],
    pub font_count: u16,
    pub color_space: ColorSpace,
    pub default_resolution: u16,
    pub actual_resolution: u16,
    pub linearized: bool,
    pub tagged: bool,
    pub optimized: bool,
}

#[repr(C)]
pub struct PdfPage {
    pub width: f32,
    pub height: f32,
    pub rotation: u8,
    pub media_box: [f32; 4],
    pub crop_box: [f32; 4],
    pub bleed_box: [f32; 4],
    pub trim_box: [f32; 4],
    pub art_box: [f32; 4],
    pub resources: PdfResources,
    pub content: [u8; 65536],
    pub content_len: u32,
    pub thumb: Option<GpuTexture>,
    pub annotations: [PdfAnnotation; 128],
    pub annotation_count: u16,
    pub form_fields: [PdfFormField; 64],
    pub form_count: u16,
    pub links: [PdfLink; 256],
    pub link_count: u16,
}

#[repr(C)]
pub struct PdfFont {
    pub name: [u8; 64],
    pub ty: PdfFontType,
    pub encoding: PdfEncoding,
    pub embedded: bool,
    pub subset: bool,
    pub data: Option<&'static [u8]>,
    pub cmap: Option<&'static [u8]>,
}

#[repr(u8)]
pub enum PdfFontType { Type1, TrueType, Type3, CIDFont, CIDType0, CIDType2 }

#[repr(u8)]
pub enum PdfEncoding { Standard, MacRoman, MacExpert, WinAnsi, IdentityH, IdentityV, Custom }

#[repr(C)]
pub struct PdfResources {
    pub fonts: [u16; 64],
    pub xobjects: [u16; 64],
    pub color_spaces: [u16; 16],
    pub patterns: [u16; 16],
    pub properties: [u16; 16],
    pub ext_gstate: [u16; 16],
    pub shadings: [u16; 16],
    pub font_count: u8,
    pub xobject_count: u8,
}

#[repr(C)]
pub struct PdfAnnotation {
    pub rect: [f32; 4],
    pub ty: PdfAnnotationType,
    pub contents: [u8; 512],
    pub color: [f32; 4],
    pub flags: u32,
    pub border: [f32; 4],
    pub appearance: Option<GpuTexture>,
}

#[repr(u8)]
pub enum PdfAnnotationType { Text, Link, FreeText, Line, Square, Circle, Polygon, PolyLine, Highlight, Underline, Squiggly, StrikeOut, Stamp, Caret, Ink, Popup, FileAttachment, Sound, Movie, Widget, Screen, PrinterMark, TrapNet, Watermark, _3D }

#[repr(C)]
pub struct PdfLink {
    pub rect: [f32; 4],
    pub kind: PdfLinkKind,
    pub dest: Ptr512,
    pub dest_page: u32,
    pub uri: [u8; 256],
    pub action: [u8; 64],
}

#[repr(u8)]
pub enum PdfLinkKind { Internal, External, Action, Named }

#[repr(C)]
pub struct PdfFormField {
    pub rect: [f32; 4],
    pub ty: PdfFormType,
    pub name: [u8; 64],
    pub value: [u8; 256],
    pub default_value: [u8; 256],
    pub flags: u32,
    pub options: [u8; 128],
}

#[repr(u8)]
pub enum PdfFormType { Text, Button, CheckBox, RadioButton, ComboBox, ListBox, Signature, Tree }

#[repr(C)]
pub struct PdfEncryption {
    pub encrypted: bool,
    pub algorithm: PdfEncAlgorithm,
    pub key_length: u16,
    pub permissions: PdfPermissions,
    pub encrypted_metadata: bool,
}

#[repr(u8)]
pub enum PdfEncAlgorithm { None, RC4V1, RC4V2, AESV2, AESV3 }

bitflags::bitflags! {
    pub struct PdfPermissions: u32 {
        const PRINT         = 1 << 0;
        const MODIFY        = 1 << 1;
        const COPY          = 1 << 2;
        const ANNOTATE      = 1 << 3;
        const FILL_FORMS    = 1 << 4;
        const EXTRACT       = 1 << 5;
        const ASSEMBLE      = 1 << 6;
        const PRINT_HIGH    = 1 << 7;
    }
}

#[derive(Clone, Copy)]
pub enum ColorSpace { DeviceGray, DeviceRGB, DeviceCMYK, CalibratedRGB, CalibratedGray, ICCBased, Indexed, Pattern, Separation, DeviceN, CIEBased }

pub struct PdfRenderer {
    pub ctx: PaintContext,
    pub current_page: Option<u32>,
    pub zoom: f32,
    pub rotation: u8,
    pub rendering_intent: RenderingIntent,
    pub color_conversion: ColorConvert,
    pub cache: BTreeMap<u32, GpuTexture>,
    pub tiles: [PdfTile; 64],
    pub tile_count: u8,
}

#[derive(Clone, Copy)]
pub enum RenderingIntent { Auto, Colorimetric, Saturation, Perceptual, AbsoluteColorimetric }

pub enum ColorConvert { No, sRGB, DisplayP3, AdobeRGB }

#[derive(Clone, Copy)]
pub struct PdfTile {
    pub page: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
    pub texture: Option<GpuTexture>,
    pub dirty: bool,
}

struct _Ptr512([u8; 64]);
struct _GpuTexture([u8; 64]);
struct _PaintContext([u8; 64]);
