use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

// use bluefile::parse_data::ParseData;
use bluefile::types::CommonHeader;
use deku::DekuContainerRead;

const COMMON_HEADER_OFFSET: usize = 0; // in bytes
const COMMON_HEADER_SIZE: usize = 256; // in bytes

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/pulse_cx.tmp");
    let mut file = File::open(&d).unwrap();
    let mut header_data: Vec<u8> = vec![0_u8; COMMON_HEADER_SIZE];

    match file.seek(SeekFrom::Start(COMMON_HEADER_OFFSET as u64)) {
        Ok(x) => x,
        // Err(_) => Err(BlueError::HeaderSeekError),
        Err(_) => return,
    };

    let n = match file.read(&mut header_data) {
        Ok(x) => x,
        // Err(_) => return Err(BlueError::FileReadError),
        Err(_) => return,
    };
    if n < COMMON_HEADER_SIZE {
        return;
    }

    let (_residual, value) =
        CommonHeader::from_bytes((&header_data.as_ref(), 0)).expect("Something went wrong");
    println!("{:#?}", value);
    // let header = read_header(&file).unwrap();
    // TODO: switch statements for parsing data according to data type
    // let parsed_data = <Complex<f32>>::parse_data(&file, &header).unwrap();
    // println!("Data type {:?}", header.data_type.to_string());
    // println!("Data length: {:?}", parsed_data.len());
    // println!("Data endianness: {:?}", header.data_endianness);
}
