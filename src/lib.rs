use firestorm::profile_fn;
use std::convert::TryInto;

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

pub struct ValueOutOfRange(u32);

macro_rules! impl_simple {
    ($T:ty, $MAX:expr) => {
        impl Simple16 for $T {
            fn compress(values: &[Self]) -> Result<(u32, usize), ValueOutOfRange> {
                for i in 0..($MAX as u32) {
                    let mut value = i << 28;
                    let count = COUNTS[i as usize].min(values.len());

                    let mut bits = 0;
                    let mut j = 0;
                    while j < count {
                        if values[j] as u32 >= (1 << (&BITS[i as usize])[j]) {
                            break;
                        }
                        value |= ((values[j] as u32) << bits);
                        bits += (&BITS[i as usize])[j];
                        j += 1;
                    }
                    if j == count {
                        return Ok((value, j));
                    }
                }
                // TODO: This is not necessarily the value that is out of range.
                Err(ValueOutOfRange(values[0] as u32))
            }

            fn consume(values: &[Self]) -> Result<usize, ValueOutOfRange> {
                for i in 0..($MAX as usize) {
                    let count = COUNTS[i].min(values.len());

                    let mut j = 0;
                    while j < count {
                        if values[j] as u32 >= (1u32 << (&BITS[i])[j]) {
                            break;
                        }
                        j += 1;
                    }
                    if j == count {
                        return Ok(j);
                    }
                }
                // TODO: This is not necessarily the value that is out of range.
                Err(ValueOutOfRange(values[0] as u32))
            }
        }
    };
}

// TODO: (Performance) Express that some of these are infallible
impl_simple!(u32, 16);
impl_simple!(u16, 16);
impl_simple!(u8, 14);

pub trait Simple16: Sized {
    fn consume(values: &[Self]) -> Result<usize, ValueOutOfRange>;
    fn compress(values: &[Self]) -> Result<(u32, usize), ValueOutOfRange>;
}

pub fn calculate_size<T: Simple16>(mut values: &[T]) -> Result<usize, ValueOutOfRange> {
    profile_fn!(calculate_size);
    let mut size = 0;
    while values.len() > 0 {
        let advanced = T::consume(values)?;
        values = &values[advanced..];
        size += 4;
    }

    Ok(size)
}

pub fn compress<T: Simple16>(mut values: &[T], out: &mut Vec<u8>) -> Result<(), ValueOutOfRange> {
    profile_fn!(compress);
    while values.len() > 0 {
        let (next, advanced) = T::compress(values)?;
        values = &values[advanced..];
        out.extend_from_slice(&next.to_le_bytes());
    }

    Ok(())
}

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
        let out: Vec<_> = out.into_iter().map(|o| o.try_into().unwrap_or_else(|_| panic!("round trip failed"))).collect();

        assert_eq!(data, &out[..data.len()]);
    }

    #[test]
    fn t1() {
        let i = &[1u32, 5, 18, 99, 2023, 289981, 223389999];
        let i = &[1u16, 5, 18, 99, 2023, u16::MAX];
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
        assert!(compress(&[std::u32::MAX], &mut Vec::new()).is_err())
    }
}
