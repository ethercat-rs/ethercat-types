#[macro_use]
extern crate num_derive;

use std::{convert::TryFrom, num::TryFromIntError};

/// EtherCAT Slave Position
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SlavePos(u16);

impl From<u16> for SlavePos {
    fn from(pos: u16) -> Self {
        Self(pos)
    }
}

impl From<SlavePos> for u16 {
    fn from(pos: SlavePos) -> Self {
        pos.0
    }
}

/// Object Directory Index
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Idx(u16);

impl From<u16> for Idx {
    fn from(idx: u16) -> Self {
        Self(idx)
    }
}

impl From<Idx> for u16 {
    fn from(idx: Idx) -> Self {
        idx.0
    }
}

/// Object Directory Sub-index
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SubIdx(u8);

impl From<u8> for SubIdx {
    fn from(sub: u8) -> Self {
        Self(sub)
    }
}

impl From<SubIdx> for u8 {
    fn from(sub: SubIdx) -> Self {
        sub.0
    }
}

/// SDO Index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SdoIdx {
    pub idx: Idx,
    pub sub_idx: SubIdx,
}

/// PDO Entry Index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PdoEntryIdx {
    pub idx: Idx,
    pub sub_idx: SubIdx,
}

/// Domain Index
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DomainIdx(usize);

impl From<usize> for DomainIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl From<DomainIdx> for usize {
    fn from(idx: DomainIdx) -> Self {
        idx.0
    }
}

impl TryFrom<DomainIdx> for u32 {
    type Error = TryFromIntError;
    fn try_from(idx: DomainIdx) -> Result<Self, Self::Error> {
        u32::try_from(idx.0)
    }
}

impl TryFrom<DomainIdx> for u64 {
    type Error = TryFromIntError;
    fn try_from(idx: DomainIdx) -> Result<Self, Self::Error> {
        u64::try_from(idx.0)
    }
}

/// Sync Master Index
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SmIdx(u8);

impl From<u8> for SmIdx {
    fn from(idx: u8) -> Self {
        Self(idx)
    }
}

impl From<SmIdx> for u8 {
    fn from(idx: SmIdx) -> Self {
        idx.0
    }
}

/// Data Access Type
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Access {
    /// Read only
    Ro,
    /// Read write
    Rw,
}

/// Data Type
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromPrimitive)]
pub enum DataType {
    /// BIT
    Bool = 0x001,
    /// BYTE
    Byte = 0x01E,

    /// SINT
    I8 = 0x0002,
    /// INT
    I16 = 0x0003,
    /// DINT
    I32 = 0x0004,
    /// LINT
    I64 = 0x0015,

    /// USINT
    U8 = 0x0005,
    /// UINT
    U16 = 0x0006,
    /// UDINT
    U32 = 0x0007,
    /// ULINT
    U64 = 0x001B,

    /// REAL
    F32 = 0x0008,
    /// LREAL
    F64 = 0x0011,

    /// STRING(n) a.k.a. visiable string
    String = 0x0009,

    /// ARRAY of BYTE a.k.a. Octet String
    U8Array = 0x000A,

    /// ARRAY of UINT a.k.a. Unicode String
    U16Array = 0x000B,

    I24 = 0x0010,
    I40 = 0x0012,
    I48 = 0x0013,
    I56 = 0x0014,

    U24 = 0x0016,
    U40 = 0x0018,
    U48 = 0x0019,
    U56 = 0x001A,

    Raw = 0xFFFF,
}

/// Offset of a PDO entry in the domain image.
#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Offset {
    pub byte: usize,
    pub bit: u32,
}
