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
