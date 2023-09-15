#[non_exhaustive]
struct type;

impl type {
    // List of encodings from type to the code.
    pub const NULL:u8          = 0x40;    // fixed/0, the null value
    pub const BOOLEAN:u8       = 0x56;    // fixed/1, boolean with the octet 0x00 being false and octet 0x01 being true
    pub const BOOLEAN_TRUE:u8  = 0x41;    // fixed/0, the boolean value true
    pub const BOOLEAN_FALSE:u8 = 0x42;    // fixed/0, the boolean value false
    pub const UTYPE:u8         = 0x50;    // fixed/1, 8-bit unsigned integer
    pub const USHORT:u8        = 0x60;    // fixed/2, 16-bit unsigned integer in network byte order
    pub const UINT:u8          = 0x70;    // fixed/4, 32-bit unsigned integer in network byte order
    pub const SMALL_UNINT:u8   = 0x52;    // fixed/1, unsigned integer value in the range 0 to 255 inclusive
    pub const UINT0:u8         = 0x43;    // fixed/0, the uint value 0
    pub const ULONG:u8         = 0x80;    // fixed/8, 64-bit unsigned integer in network byte order
    pub const SMALL_ULONG:u8   = 0x53;    // fixed/1, unsigned long value in the range 0 to 255 inclusive
    pub const ULONG0:u8        = 0x44;    // fixed/0, the ulong value 0
    pub const BYTE:u8          = 0x51;    // fixed/1, 8-bit two’s-complement integer
    pub const SHORT:u8         = 0x61;    // fixed/2, 16-bit two’s-complement integer in network byte order
    pub const INT:u8           = 0x71;    // fixed/4, 32-bit two’s-complement integer in network byte order
    pub const SMALL_INT:u8     = 0x54;    // fixed/1, signed integer value in the range -128 to 127 inclusive
    pub const LONG:u8          = 0x81;    // fixed/8, 64-bit two’s-complement integer in network byte order
    pub const SMALL_LONG:u8    = 0x55;    // fixed/1, signed long value in the range -128 to 127 inclusive
    pub const FLOAT:u8         = 0x72;    // fixed/4, IEEE 754-2008 binary32
    pub const DOUBLE:u8        = 0x82;    // fixed/8, IEEE 754-2008 binary64
    pub const DECIMAL32:u8     = 0x74;    // fixed/4, IEEE 754-2008 decimal32 using the Binary Integer Decimal encoding
    pub const DECIMAL64:u8     = 0x84;    // fixed/8, IEEE 754-2008 decimal64 using the Binary Integer Decimal encoding
    pub const DECIMAL128:u8    = 0x94;    // fixed/16, IEEE 754-2008 decimal128 using the Binary Integer Decimal encoding
    pub const CHAR:u8          = 0x73;    // fixed/4, a UTF-32BE encoded unicode character
    pub const TIMESTAMP:u8     = 0x83;    // fixed/8, 64-bit signed integer representing milliseconds since the unix epoch
    pub const UUID:u8          = 0x98;    // fixed/16, UUID as defined in section 4.1.2 of RFC-4122
    pub const BINARY_VBIN8:u8  = 0xa0;    // variable/1, up to 2^8 - 1 octets of binary data
    pub const BINARY_VBIN32:u8 = 0xb0;    // variable/4, up to 2^32 - 1 octets of binary data
    pub const STR8_UTF8:u8     = 0xa1;    // variable/1, up to 2^8 - 1 octets worth of UTF-8 unicode (with no byte order mark)
    pub const STR32_UTF8:u8    = 0xb1;    // variable/4, up to 2^32 - 1 octets worth of UTF-8 unicode (with no byte order mark)
    pub const SYMBOL8:u8       = 0xa3;    // variable/1, up to 2^8 - 1 seven bit ASCII characters representing a symbolic value
    pub const SYMBOL32:u8      = 0xb3;    // variable/4, up to 2^32 - 1 seven bit ASCII characters representing a symbolic value
    pub const LIST0:u8         = 0x45;    // fixed/0, the empty list (i.e. the list with no elements)
    pub const LIST8:u8         = 0xc0;    // compound/1, up to 2^8 - 1 list elements with total size less than 2^8 octets
    pub const LIST32:u8        = 0xd0;    // compound/4, up to 2^32 - 1 list elements with total size less than 2^32 octets
    pub const MAP8:u8          = 0xc1;    // compound/1, up to 2^8 - 1 octets of encoded map data
    pub const MAP32:u8         = 0xd1;    // compound/4, up to 2^32 - 1 octets of encoded map data
    pub const ARRAY8:u8        = 0xe0;    // array/1, up to 2^8 - 1 array elements with total size less than 2^8 octets
    pub const ARRAY32:u8       = 0xf0;    // array/4, up to 2^32 - 1 array elements with total size less than 2^32 octets
}
