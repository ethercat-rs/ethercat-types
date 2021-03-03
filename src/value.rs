/// EtherCAT Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// BIT
    Bool(bool),

    /// BYTE
    Byte(u8),

    /// SINT
    I8(i8),
    /// INT
    I16(i16),
    /// DINT
    I32(i32),
    /// LINT
    I64(i64),

    /// USINT
    U8(u8),
    /// UINT
    U16(u16),
    /// UDINT
    U32(u32),
    /// ULINT
    U64(u64),

    /// REAL
    F32(f32),
    /// LREAL
    F64(f64),

    /// STRING(n) a.k.a. visiable string
    String(String),

    /// ARRAY of BYTE a.k.a. Octet String
    U8Array(Vec<u8>),

    /// ARRAY of UINT a.k.a. Unicode String
    U16Array(Vec<u16>),

    // TODO:
    //I24
    //I40
    //I48
    //I56

    // TODO:
    //U24
    //U40
    //U48
    //U56
    /// BIT 1
    Bit1(bool),
    /// BIT 2
    Bit2(bool),
    /// BIT 3
    Bit3(bool),
    /// BIT 4
    Bit4(bool),
    /// BIT 5
    Bit5(bool),
    /// BIT 6
    Bit6(bool),
    /// BIT 7
    Bit7(bool),
    /// BIT 8
    Bit8(bool),

    // TODO:
    // /// Time of Day
    // TimeOfDay,
    // /// Time Difference
    // TimeDifference,

    // TODO:
    // /// Domain
    // Domain,
    Raw(Vec<u8>),
}
