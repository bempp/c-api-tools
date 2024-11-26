//! Useful conversion types.

/// Representation of scalar numeric type information
/// as runtime parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DType {
    /// Float 32
    F32,
    /// Float 64
    F64,
    /// Complex 32
    C32,
    /// Complex 64
    C64,
    /// Unsigned int 8
    U8,
    /// Unsigned int 32
    U32,
    /// Unsigned int 64
    U64,
    /// Int 8
    I8,
    /// Int 32
    I32,
    /// Int 64
    I64,
    /// Undefined type
    Undefined,
}

impl DType {
    /// Return true of type is real.
    pub fn is_real(&self) -> bool {
        matches!(
            self,
            DType::F32
                | DType::F64
                | DType::U8
                | DType::U32
                | DType::U64
                | DType::I8
                | DType::I32
                | DType::I64,
        )
    }

    /// Return true if type is complex.
    pub fn is_complex(&self) -> bool {
        matches!(self, DType::C32 | DType::C64)
    }

    /// Return the associated real type.
    pub fn real_type(&self) -> DType {
        match self {
            DType::C32 => DType::F32,
            DType::C64 => DType::F64,
            _ => *self,
        }
    }

    /// Return the associated complex type.
    ///
    /// If there is no associated complex type `DType::Undefined` is returned.
    pub fn complex_type(&self) -> DType {
        match self {
            DType::F32 => DType::C32,
            DType::F64 => DType::C64,
            _ => DType::Undefined,
        }
    }
}

/// Runtime numeric type information.
pub trait DTypeIdentifier {
    /// Return runtime numeric type information.
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
