use std::fs::File;

// use num::traits::FromBytes;
// use crate::from_bytes::FromBytes;

use num::Complex;

use crate::from_bytes::FromBytes;
use crate::read_data;
use crate::DataType;
use crate::Error;
use crate::Header;

// /// complex i8 (CB).
// /// complex i16 (CI).
// /// complex i32 (CL).
// /// complex i64 (CX).
// /// complex f32 (CF).
// /// complex f64 (CD).

// pub fn parse_data<T>(file: &File, header: &Header) -> Result<Vec<T>, Error>
// where
//     T: std::fmt::Debug + num_traits::FromBytes,
//     for<'a> &'a [u8]: TryInto<&'a T::Bytes>,
//     std::result::Result<T, Error>: FromIterator<T>,
// {
//     let data_bytes = read_data(&file, &header).unwrap();
//     let mut data: Vec<T> = Vec::new();
//     let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

//     for i in 0..data_bytes.len() - bytes_per_sample + 1 {
//         let sample_scalar: T =
//             // <f32>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
//             from_bytes_chunks::<T>(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
//         println!("{:#06x}\t({:#?})", i, sample_scalar);
//         // self.data.push(sample_scalar);
//         data.push(sample_scalar);
//     }
//     Ok(data)
// }

pub enum ParsedDataType {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub trait ParseData<T> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<T>, Error>;
}

impl ParseData<i8> for i8 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<i8>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<i8> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: i8 =
                <i8>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\t({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<i16> for i16 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<i16>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<i16> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: i16 =
                <i16>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\t({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<i32> for i32 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<i32>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<i32> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: i32 =
                <i32>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\t({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<i64> for i64 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<i64>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<i64> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: i64 =
                <i64>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\ti64({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<f32> for f32 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<f32>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<f32> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: f32 =
                <f32>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\t({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<f64> for f64 {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<f64>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<f64> = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_scalar: f64 =
                <f64>::from_bytes(&data_bytes[i..i + bytes_per_sample], header.data_endianness)?;
            println!("{:#06x}\t({:#?})", i, sample_scalar);
            // self.data.push(sample_scalar);
            data.push(sample_scalar);
        }
        Ok(data)
    }
}

impl ParseData<Complex<i8>> for Complex<i8> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<i8>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<i8>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: i8 = <i8>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: i8 = <i8>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<i8>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}

impl ParseData<Complex<i16>> for Complex<i16> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<i16>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<i16>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: i16 = <i16>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: i16 = <i16>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<i16>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}

impl ParseData<Complex<i32>> for Complex<i32> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<i32>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<i32>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: i32 = <i32>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: i32 = <i32>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<i32>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}

impl ParseData<Complex<i64>> for Complex<i64> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<i64>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<i64>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: i64 = <i64>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: i64 = <i64>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<i64>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}

impl ParseData<Complex<f32>> for Complex<f32> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<f32>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<f32>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: f32 = <f32>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: f32 = <f32>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<f32>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}

impl ParseData<Complex<f64>> for Complex<f64> {
    fn parse_data(file: &File, header: &Header) -> Result<Vec<Complex<f64>>, Error> {
        let data_bytes = read_data(&file, &header).unwrap();
        let mut data: Vec<Complex<f64>> = Vec::new();
        // self.data = Vec::new();
        let bytes_per_sample: usize = DataType::num_bytes(&header.data_type).unwrap();

        for i in 0..data_bytes.len() - bytes_per_sample + 1 {
            let sample_re: f64 = <f64>::from_bytes(
                &data_bytes[i..i + bytes_per_sample / 2],
                header.data_endianness,
            )?;
            let sample_im: f64 = <f64>::from_bytes(
                &data_bytes[i + bytes_per_sample / 2..i + bytes_per_sample],
                header.data_endianness,
            )?;
            let sample_clx = Complex::<f64>::new(sample_re, sample_im);
            println!("{:#06x}\t({:#?}, {:#?})", i, sample_clx.re, sample_clx.im);
            data.push(sample_clx);
        }
        Ok(data)
    }
}
