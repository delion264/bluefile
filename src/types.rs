/*
 * Offset	Name	    Size	Type	    Description
 * 0	    version	    4	    char[4]	    Header version
 * 4	    head_rep	4	    char[4]	    Header representation
 * 8	    data_rep	4	    char[4]	    Data representation
 * 12	    detached	4	    int_4	    Detached header
 * 16	    protected	4	    int_4	    Protected from overwrite
 * 20	    pipe	    4	    int_4	    Pipe mode (N/A)
 * 24	    ext_start	4	    int_4	    Extended header start, in 512-byte blocks
 * 28	    ext_size	4	    int_4	    Extended header size in bytes
 * 32	    data_start	8	    real_8	    Data start in bytes
 * 40	    data_size	8	    real_8	    Data size in bytes
 * 48	    type	    4	    int_4	    File type code
 * 52	    format	    2	    char[2]	    Data format code
 * 54	    flagmask	2	    int_2	    16-bit flagmask (1=flagbit)
 * 56	    timecode	8	    real_8	    Time code field
 * 64	    inlet	    2	    int_2	    Inlet owner
 * 66	    outlets	    2	    int_2	    Number of outlets
 * 68	    outmask	    4	    int_4	    Outlet async mask
 * 72	    pipeloc	    4	    int_4	    Pipe location
 * 76	    pipesize	4	    int_4	    Pipe size in bytes
 * 80	    in_byte	    8	    real_8	    Next input byte
 * 88	    out_byte	8	    real_8	    Next out byte (cumulative)
 * 96	    outbytes	64	    real_8[8]	Next out byte (each outlet)
 * 160	    keylength	4	    int_4	    Length of keyword string
 * 164	    keywords	92	    char[92]	User defined keyword string
 * 256	    Adjunct	    256	    char[256]	Type-specific adjunct union (See below for 1000 and 2000 type bluefiles)
 */

// use rkyv::{deserialize, Archive, Deserialize, Serialize};
use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "little")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub struct DataType {
    #[deku(map = "|b: u8| -> Result<char, DekuError> { Ok(b as char) }")]
    // #[deku(writer = "|c: char| -> Result<u8, DekuError> { Ok(c as u8) }")]
    rank: char,
    #[deku(map = "|b: u8| -> Result<char, DekuError> { Ok(b as char) }")]
    // #[deku(writer = "|c: char| -> Result<u8, DekuError> { Ok(c as u8) }")]
    format: char,
}

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "little")]
pub struct CommonHeader {
    #[deku(
        count = "4",
        map = "|bytes: Vec<u8>| -> Result<Vec<char>, DekuError> {
            Ok(bytes.iter().map(|&b| b as char).collect())
        }"
    )]
    pub is_blue: Vec<char>, // 0 - 4 [char]

    #[deku(
        count = "4",
        map = "|bytes: Vec<u8>| -> Result<Vec<char>, DekuError> {
            Ok(bytes.iter().map(|&b| b as char).collect())
        }"
    )]
    pub header_endianness: Vec<char>, // 4- 8 [char]

    #[deku(
        // 1. Read exactly 4 bytes from the stream
        count = "4",
        // 2. Map the Vec<u8> result into a Vec<char>
        map = "|bytes: Vec<u8>| -> Result<Vec<char>, DekuError> {
            Ok(bytes.iter().map(|&b| b as char).collect())
        }"
    )]
    pub data_endianness: Vec<char>, // 8 - 12 [char]

    pub detached: i32,   // 12 - 16
    pub protected: i32,  // 16 - 20
    pub pipe: i32,       // 20 - 24
    pub ext_start: i32,  // 24 - 28
    pub ext_size: i32,   // 28 - 32
    pub data_start: f64, // 32 - 40
    pub data_size: f64,  // 40 - 48
    pub type_code: i32,  // 48 - 52
    pub data_type: DataType,
    pub flagmask: i16,  // 54 - 56
    pub timecode: i64,  // 56 - 64
    pub inlet: i16,     // 64 - 66
    pub outlets: i16,   // 66 - 68
    pub outmask: i32,   // 68 - 72
    pub pipeloc: i32,   // 72 - 76
    pub pipesize: i32,  // 76 - 80
    pub in_byte: i64,   // 80 - 88
    pub out_byte: i64,  // 88 - 96
    pub out_bytes: i64, // 96 - 160

    // TODO: Implement variable count based on size of extended header
    #[deku(count = "3")]
    pub keywords: Vec<ExtHeaderKeyword>,
    // adjunct: [u8; 256],     // 256 - 512 [char]
    // data: Vec<u8>,
}

/// TODO: Verify the accuracy of this struct layout
#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "little")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub struct ExtHeaderKeyword {
    pub length: usize,

    #[deku(map = "|field: i16| -> Result<usize, DekuError> { Ok(field as usize) }")]
    pub keyword_hdr_length: usize,
    pub tag_length: usize,

    #[deku(map = "|b: u8| -> Result<char, DekuError> { Ok(b as char) }")]
    pub format: char,

    #[deku(
        count = "tag_length",
        map = "|bytes: Vec<u8>| -> Result<Vec<char>, DekuError> {
            Ok(bytes.iter().map(|&b| b as char).collect())
        }"
    )]
    pub tag: Vec<char>,

    #[deku(ctx = "*count as usize, deku::ctx::Endian::Little")]
    #[deku(count = "length - keyword_hdr_length")]
    pub value: Vec<u8>,
}

pub struct Type1000AdjunctHeader {
    pub xstart: f64,
    pub xdelta: f64,
    pub xunits: i32,
}

pub struct Type2000AdjunctHeader {
    pub xstart: f64, // f64
    pub xdelta: f64, // f64
    pub xunits: i32,
    pub subsize: i32,
    pub ystart: f64, // f64
    pub ydelta: f64, // f64
    pub yunits: i32,
}
