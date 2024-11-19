//! Useful conversion types.

/// Representation of scalar numeric type information
/// as runtime parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DType {
    F32,
    F64,
    C32,
    C64,
    U8,
    U32,
    U64,
    I8,
    I32,
    I64,
}

/// Return runtime numeric type information.
pub trait DTypeIdentifier {
    fn dtype() -> DType;
}

impl DTypeIdentifier for f32 {
    fn dtype() -> DType {
        DType::F32
    }
}

impl DTypeIdentifier for f64 {
    fn dtype() -> DType {
        DType::F64
    }
}

impl DTypeIdentifier for num::complex::Complex<f32> {
    fn dtype() -> DType {
        DType::C32
    }
}

impl DTypeIdentifier for num::complex::Complex<f64> {
    fn dtype() -> DType {
        DType::C64
    }
}

impl DTypeIdentifier for u8 {
    fn dtype() -> DType {
        DType::U8
    }
}

impl DTypeIdentifier for u32 {
    fn dtype() -> DType {
        DType::U32
    }
}

impl DTypeIdentifier for u64 {
    fn dtype() -> DType {
        DType::U64
    }
}

impl DTypeIdentifier for i8 {
    fn dtype() -> DType {
        DType::I8
    }
}

impl DTypeIdentifier for i32 {
    fn dtype() -> DType {
        DType::I32
    }
}

impl DTypeIdentifier for i64 {
    fn dtype() -> DType {
        DType::I64
    }
}