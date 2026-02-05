use crate::Endianness;
use crate::Error;

// pub fn from_bytes_chunks<T>(v: &[u8], endianness: Endianness) -> Result<T, Error>
// where
//     T: num_traits::FromBytes,
//     for<'a> &'a [u8]: TryInto<&'a T::Bytes>,
//     std::result::Result<T, Error>: FromIterator<T>,
// {
//     v.chunks_exact(std::mem::size_of::<T>())
//         .map(TryInto::<&T::Bytes>::try_into)
//         .map(|x| x.unwrap_or_else(|_| panic!("could not convert slice to array reference!")))
//         .map(|x| {
//             if endianness == Endianness::Little {
//                 <T>::from_le_bytes(x)
//             } else {
//                 <T>::from_be_bytes(x)
//             }
//         })
//         .collect()
// }

pub trait FromBytes<T> {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<T, Error>;
}

impl FromBytes<i8> for i8 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<i8, Error> {
        let b: [u8; 1] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<i8>::from_le_bytes(b))
        } else {
            Ok(<i8>::from_be_bytes(b))
        }
    }
}

impl FromBytes<i16> for i16 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<i16, Error> {
        let b: [u8; 2] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<i16>::from_le_bytes(b))
        } else {
            Ok(<i16>::from_be_bytes(b))
        }
    }
}

impl FromBytes<i32> for i32 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<i32, Error> {
        let b: [u8; 4] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<i32>::from_le_bytes(b))
        } else {
            Ok(<i32>::from_be_bytes(b))
        }
    }
}

impl FromBytes<i64> for i64 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<i64, Error> {
        let b: [u8; 8] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<i64>::from_le_bytes(b))
        } else {
            Ok(<i64>::from_be_bytes(b))
        }
    }
}

impl FromBytes<f32> for f32 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<f32, Error> {
        let b: [u8; 4] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<f32>::from_le_bytes(b))
        } else {
            Ok(<f32>::from_be_bytes(b))
        }
    }
}

impl FromBytes<f64> for f64 {
    fn from_bytes(v: &[u8], endianness: Endianness) -> Result<f64, Error> {
        let b: [u8; 8] = match v.try_into() {
            Ok(x) => x,
            Err(_) => return Err(Error::ByteConversionError),
        };

        if endianness == Endianness::Little {
            Ok(<f64>::from_le_bytes(b))
        } else {
            Ok(<f64>::from_be_bytes(b))
        }
    }
}
