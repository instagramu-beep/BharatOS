//! BharatOS Calculator
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct CalcFlags: u32 {
        const DEGREE = 1 << 0;
        const RADIAN = 1 << 1;
        const GRADIAN = 1 << 2;
        const SCIENTIFIC = 1 << 3;
        const PROGRAMMER = 1 << 4;
        const STORED = 1 << 5;
        const MEMORY = 1 << 6;
        const ERROR = 1 << 7;
        const HISTORY = 1 << 8;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CalcMode { Standard, Scientific, Programmer, Currency, Unit }

#[derive(Clone, Copy, PartialEq)]
pub enum AngleUnit { Degree, Radian, Gradian }

#[derive(Clone, Copy, PartialEq)]
pub enum NumberBase { Binary, Octal, Decimal, Hexadecimal }

#[repr(C)]
pub struct CalcState {
    pub flags: CalcFlags,
    pub mode: CalcMode,
    pub angle_unit: AngleUnit,
    pub number_base: NumberBase,
    pub current: Decimal,
    pub previous: Decimal,
    pub operator: Option<Operator>,
    pub memory: Decimal,
    pub history: Vec<CalcEntry>,
    pub max_history: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Decimal {
    pub value: f64,
    pub precision: u8,
    pub is_integer: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Operator { Add, Sub, Mul, Div, Mod, Pow, Sqrt, Sin, Cos, Tan, Log, Ln, Factorial, And, Or, Xor, Shl, Shr, Not }

#[derive(Clone)]
pub struct CalcEntry {
    pub expression: String,
    pub result: Decimal,
    pub timestamp: u128,
}

impl CalcState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(mode: CalcMode) -> Self {
        let mut state = Self::new();
        state.mode = mode;
        state.flags.insert(CalcFlags::DEGREE);
        state.max_history = 100;
        state.clear();
        state
    }

    pub fn input_digit(&mut self, digit: u8) {
        if self.operator.is_some() {
            self.previous = self.current;
            self.current = Decimal { value: digit as f64, precision: 0, is_integer: true };
            self.operator = None;
        } else {
            self.current.value = self.current.value * 10.0 + digit as f64;
        }
    }

    pub fn input_decimal(&mut self) {
        self.current.precision = 2;
    }

    pub fn input_operator(&mut self, op: Operator) {
        if self.operator.is_some() && !self.previous.value.is_nan() {
            self.calculate();
        }
        self.previous = self.current;
        self.operator = Some(op);
        self.current = Decimal::default();
    }

    pub fn calculate(&mut self) {
        if let Some(op) = self.operator {
            let a = self.previous.value;
            let b = self.current.value;
            let result = match op {
                Operator::Add => a + b,
                Operator::Sub => a - b,
                Operator::Mul => a * b,
                Operator::Div => a / (if b == 0.0 { 1.0 } else { b }),
                Operator::Pow => a.powf(b),
                Operator::Sqrt => a.sqrt(),
                _ => b,
            };
            self.current = Decimal { value: result, precision: 2, is_integer: false };
            self.operator = None;
        }
    }

    pub fn clear(&mut self) {
        self.current = Decimal::default();
        self.previous = Decimal::default();
        self.operator = None;
        self.flags.remove(CalcFlags::ERROR);
    }

    pub fn clear_entry(&mut self) {
        self.current = Decimal::default();
    }

    pub fn toggle_sign(&mut self) {
        self.current.value = -self.current.value;
    }

    pub fn percent(&mut self) {
        self.current.value /= 100.0;
    }

    pub fn factorial(&mut self) {
        let n = self.current.value as u64;
        let mut result = 1u64;
        for i in 2..=n { result *= i; }
        self.current.value = result as f64;
    }

    pub fn sin(&mut self) {
        let angle = self.current.value;
        let rad = match self.angle_unit {
            AngleUnit::Degree => angle * core::f64::consts::PI / 180.0,
            AngleUnit::Radian => angle,
            AngleUnit::Gradian => angle * core::f64::consts::PI / 200.0,
        };
        self.current.value = rad.sin();
    }

    pub fn cos(&mut self) {
        let angle = self.current.value;
        let rad = match self.angle_unit {
            AngleUnit::Degree => angle * core::f64::consts::PI / 180.0,
            AngleUnit::Radian => angle,
            AngleUnit::Gradian => angle * core::f64::consts::PI / 200.0,
        };
        self.current.value = rad.cos();
    }

    pub fn tan(&mut self) {
        let angle = self.current.value;
        let rad = match self.angle_unit {
            AngleUnit::Degree => angle * core::f64::consts::PI / 180.0,
            AngleUnit::Radian => angle,
            AngleUnit::Gradian => angle * core::f64::consts::PI / 200.0,
        };
        self.current.value = rad.tan();
    }

    pub fn sqrt(&mut self) {
        self.current.value = self.current.value.sqrt();
    }

    pub fn log10(&mut self) {
        self.current.value = self.current.value.log10();
    }

    pub fn ln(&mut self) {
        self.current.value = self.current.value.ln();
    }

    pub fn memory_store(&mut self) {
        self.memory = self.current;
    }

    pub fn memory_recall(&mut self) {
        self.current = self.memory;
    }

    pub fn memory_add(&mut self) {
        self.memory.value += self.current.value;
    }

    pub fn memory_sub(&mut self) {
        self.memory.value -= self.current.value;
    }

    pub fn memory_clear(&mut self) {
        self.memory = Decimal::default();
    }
}
