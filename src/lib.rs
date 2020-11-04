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

/// SDO Position
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SdoPos(u16);

impl From<u16> for SdoPos {
    fn from(pos: u16) -> Self {
        Self(pos)
    }
}

impl From<SdoPos> for u16 {
    fn from(pos: SdoPos) -> Self {
        pos.0
    }
}

/// PDO Position
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PdoPos(u8);

impl From<u8> for PdoPos {
    fn from(pos: u8) -> Self {
        Self(pos)
    }
}

impl From<PdoPos> for u8 {
    fn from(pos: PdoPos) -> Self {
        pos.0
    }
}

/// PDO Entry Position
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PdoEntryPos(u8);

impl From<u8> for PdoEntryPos {
    fn from(pos: u8) -> Self {
        Self(pos)
    }
}

impl From<PdoEntryPos> for u8 {
    fn from(pos: PdoEntryPos) -> Self {
        pos.0
    }
}

/// SDO Index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SdoIdx {
    pub idx: Idx,
    pub sub_idx: SubIdx,
}

/// SDO Meta Information
#[derive(Debug, Clone, PartialEq)]
pub struct SdoInfo {
    pub pos: SdoPos, // TODO: do we need this info here?
    pub idx: Idx,
    pub max_sub_idx: SubIdx,
    pub object_code: Option<u8>,
    pub name: String,
}

/// SDO Entry Information
#[derive(Debug, Clone, PartialEq)]
pub struct SdoEntryInfo {
    pub data_type: DataType,
    pub bit_len: u16,
    pub access: SdoEntryAccess,
    pub description: String,
}

/// SDO Entry Address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SdoEntryAddr {
    ByPos(SdoPos, SubIdx),
    ByIdx(SdoIdx),
}

/// SDO Entry Access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SdoEntryAccess {
    pub pre_op: Access,
    pub safe_op: Access,
    pub op: Access,
}

/// PDO Index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PdoIdx(u16);

impl From<u16> for PdoIdx {
    fn from(idx: u16) -> Self {
        Self(idx)
    }
}

/// PDO Meta Information
#[derive(Debug, Clone, PartialEq)]
pub struct PdoInfo {
    pub sm: SmIdx,
    pub pos: PdoPos,
    pub idx: Idx,
    pub entry_count: u8,
    pub name: String,
}

/// PDO Entry Meta Information
#[derive(Debug, Clone, PartialEq)]
pub struct PdoEntryInfo {
    pub pos: PdoEntryPos,
    pub entry_idx: PdoEntryIdx,
    pub bit_len: u8,
    pub name: String,
}

impl From<PdoIdx> for u16 {
    fn from(idx: PdoIdx) -> Self {
        idx.0
    }
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
    ReadOnly,
    /// Read write
    ReadWrite,
    /// Write only
    WriteOnly,
    /// Unknown access
    Unknown,
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

/// ESM states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlState {
    Boot = 0x3,
    Init = 0x1,
    PreOp = 0x2,
    SafeOp = 0x4,
    Op = 0x8,
}

/// ESM states
#[derive(Debug)]
pub struct InvalidAlStateError;

impl TryFrom<u8> for AlState {
    type Error = InvalidAlStateError;
    fn try_from(st: u8) -> Result<Self, Self::Error> {
        match st {
            1 => Ok(AlState::Init),
            2 => Ok(AlState::PreOp),
            3 => Ok(AlState::Boot),
            4 => Ok(AlState::SafeOp),
            8 => Ok(AlState::Op),
            _ => Err(InvalidAlStateError),
        }
    }
}

impl From<AlState> for u8 {
    fn from(st: AlState) -> Self {
        st as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_al_state() {
        assert_eq!(u8::from(AlState::Init), 1);
        assert_eq!(u8::from(AlState::PreOp), 2);
        assert_eq!(u8::from(AlState::Boot), 3);
        assert_eq!(u8::from(AlState::SafeOp), 4);
        assert_eq!(u8::from(AlState::Op), 8);
    }

    #[test]
    fn try_al_state_from_u8() {
        assert_eq!(AlState::try_from(1).unwrap(), AlState::Init);
        assert_eq!(AlState::try_from(2).unwrap(), AlState::PreOp);
        assert_eq!(AlState::try_from(3).unwrap(), AlState::Boot);
        assert_eq!(AlState::try_from(4).unwrap(), AlState::SafeOp);
        assert_eq!(AlState::try_from(8).unwrap(), AlState::Op);
        assert!(AlState::try_from(0).is_err());
        assert!(AlState::try_from(5).is_err());
        assert!(AlState::try_from(6).is_err());
        assert!(AlState::try_from(7).is_err());
    }
}
