use {
    firestorm::{profile_fn, profile_method},
    std::{
        convert::{Infallible, TryInto},
        error::Error,
        fmt,
        hint::unreachable_unchecked,
    },
};

const BITS: [[u32; 28]; 16] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        4, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        3, 4, 4, 4, 4, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        5, 5, 5, 5, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        4, 4, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        6, 6, 6, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        5, 5, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        10, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        14, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
];

const COUNTS: [usize; 16] = [28, 21, 21, 21, 14, 9, 8, 7, 6, 6, 5, 5, 4, 3, 2, 1];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ValueOutOfRange(());

impl Error for ValueOutOfRange {}

impl fmt::Display for ValueOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Value out of range for simple16. Maximum value is 268435455"
        )
    }
}

fn pack<T: Simple16>(values: &[T]) -> (u32, usize) {
    let mut i = 0;
    'try_again: loop {
        let mut value = i << 28;
        let count = COUNTS[i as usize].min(values.len());
        let mut bits = 0;

        for j in 0..count {
            let v = values[j].as_();
            if v >= (1 << (&BITS[i as usize])[j]) {
                i += 1;
                continue 'try_again;
            }
            value |= v << bits;
            bits += (&BITS[i as usize])[j];
        }
        return (value, count);
    }
}

fn consume<T: Simple16>(values: &[T]) -> usize {
    let mut i = 0;
    'try_again: loop {
        // TODO: Get unchecked
        let count = COUNTS[i].min(values.len());

        for j in 0..count {
            if values[j].as_() >= (1u32 << (&BITS[i])[j]) {
                i += 1;
                continue 'try_again;
            }
        }
        return count;
    }
}

impl From<Infallible> for ValueOutOfRange {
    #[inline(always)]
    fn from(_: Infallible) -> Self {
        unsafe { unreachable_unchecked() }
    }
}

pub const MAX: u32 = 268435455;

/// This trait is unsafe because if check is wrong then undefined behavior can occur.
pub unsafe trait Simple16: Sized + Copy {
    fn check(data: &[Self]) -> Result<(), ValueOutOfRange>;
    fn as_(self) -> u32;
}

unsafe impl Simple16 for u32 {
    fn check(data: &[Self]) -> Result<(), ValueOutOfRange> {
        profile_method!(check);
        for &value in data {
            if value > MAX {
                return Err(ValueOutOfRange(()));
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn as_(self) -> u32 {
        self
    }
}
unsafe impl Simple16 for u64 {
    fn check(data: &[Self]) -> Result<(), ValueOutOfRange> {
        profile_method!(check);
        for &value in data {
            if value > MAX as u64 {
                return Err(ValueOutOfRange(()));
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn as_(self) -> u32 {
        self as u32
    }
}

unsafe impl Simple16 for u16 {
    #[inline(always)]
    fn check(_data: &[Self]) -> Result<(), ValueOutOfRange> {
        Ok(())
    }
    #[inline(always)]
    fn as_(self) -> u32 {
        self as u32
    }
}

unsafe impl Simple16 for u8 {
    #[inline(always)]
    fn check(_data: &[Self]) -> Result<(), ValueOutOfRange> {
        Ok(())
    }
    #[inline(always)]
    fn as_(self) -> u32 {
        self as u32
    }
}

/// Return the number of bytes that would be used to encode this data set.
pub fn calculate_size<T: Simple16>(data: &[T]) -> Result<usize, ValueOutOfRange> {
    profile_fn!(calculate_size);
    T::check(data)?;
    let size = unsafe { calculate_size_unchecked(data) };
    Ok(size)
}

pub unsafe fn calculate_size_unchecked<T: Simple16>(mut data: &[T]) -> usize {
    let mut size = 0;
    while data.len() > 0 {
        let advanced = consume(data);
        data = &data[advanced..];
        size += 4;
    }

    size
}

pub unsafe fn compress_unchecked<T: Simple16>(mut values: &[T], out: &mut Vec<u8>) {
    while values.len() > 0 {
        let (next, advanced) = pack(values);
        values = &values[advanced..];
        out.extend_from_slice(&next.to_le_bytes());
    }
}

/// Write the data set as little-endian integers in simple 16 format into an array of bytes
pub fn compress<T: Simple16>(values: &[T], out: &mut Vec<u8>) -> Result<(), ValueOutOfRange> {
    profile_fn!(compress);
    T::check(values)?;
    unsafe { compress_unchecked(values, out) }

    Ok(())
}

/// Read from a byte array into a destination of u32
pub fn decompress(bytes: &[u8], out: &mut Vec<u32>) -> Result<(), ()> {
    profile_fn!(decompress);
    if bytes.len() % 4 != 0 {
        return Err(());
    }
    let mut offset = 0;
    while offset < bytes.len() {
        let start = offset;
        offset += 4;
        let slice = &bytes[start..offset];
        let next = u32::from_le_bytes(slice.try_into().unwrap());
        let num_idx = (next >> 28) as usize;
        let num = COUNTS[num_idx];
        let mut j = 0;
        let mut bits = 0;
        while j < num {
            let value = (next >> bits) & (0xffffffff >> (32 - BITS[num_idx][j]));
            out.push(value);
            bits += BITS[num_idx][j];
            j += 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn round_trip<T: Simple16 + TryFrom<u32> + std::fmt::Debug + Eq>(data: &[T]) {
        let mut bytes = Vec::new();
        compress(&data, &mut bytes).unwrap_or_else(|_| todo!());
        assert_eq!(
            calculate_size(&data).unwrap_or_else(|_| todo!()),
            bytes.len()
        );
        let mut out = Vec::new();
        decompress(&bytes, &mut out).unwrap_or_else(|_| todo!());
        let out: Vec<_> = out
            .into_iter()
            .map(|o| o.try_into().unwrap_or_else(|_| panic!("round trip failed")))
            .collect();

        assert_eq!(data, &out[..data.len()]);
    }

    #[test]
    fn t1() {
        let i = &[1u32, 5, 18, 99, 2023, 289981, 223389999];
        round_trip(i);
        let i = &[1u16, 5, 18, 99, 2023, u16::MAX];
        round_trip(i);
        let i = &[1u8, 5, 18, 99, u8::MAX];
        round_trip(i);
    }

    #[test]
    fn t2() {
        let i = &[1u32];
        round_trip(i);
    }

    #[test]
    fn t3() {
        let i = &[0u32, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0];
        round_trip(i);
    }

    #[test]
    fn too_large_is_err() {
        assert!(compress(&[u32::MAX], &mut Vec::new()).is_err());
        assert!(compress(&[MAX + 1], &mut Vec::new()).is_err());
    }

    #[test]
    #[ignore = "Takes a while"]
    fn check_all() {
        let mut v = Vec::new();
        for i in 0..MAX {
            let data = &[i];
            if compress(&data[..], &mut v).is_err() {
                panic!("{}", i);
            }
            v.clear();
        }
    }
}
