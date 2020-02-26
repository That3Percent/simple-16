use std::convert::TryInto;

const BITS: [[u32; 28]; 16] = [
    [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
    [ 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0 ],
    [ 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0 ],
    [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0 ],
    [ 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 4, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 3, 4, 4, 4, 4, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 5, 5, 5, 5, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 4, 4, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 6, 6, 6, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 5, 5, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 10, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 14, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ] ];

const COUNTS: [usize; 16] = [ 28, 21, 21, 21, 14, 9, 8, 7, 6, 6, 5, 5, 4, 3, 2, 1 ];

pub struct ValueOutOfRange(u32);

#[inline(always)]
fn _compress(values: &[u32]) -> Result<(u32, usize), ValueOutOfRange> {
    for i in 0..16usize {
        let mut value = (i as u32) << 28;
        let count = COUNTS[i].min(values.len());

        let mut bits = 0;
        let mut j = 0;
        while j < count {
            if values[j] >= (1 << (&BITS[i])[j]) {
                break;
            }
            value |= values[j] << bits;
            bits += BITS[i][j];
            j += 1;
        }
        if j == count {
            return Ok((value, j));
        }
    }
    Err(ValueOutOfRange(values[0]))
}

pub fn compress(mut values: &[u32], out: &mut Vec<u8>) -> Result<(), ValueOutOfRange> {
    while values.len() > 0 {
        let (next, advanced) = _compress(values)?;
        values = &values[advanced..];
        out.extend_from_slice(&next.to_le_bytes());
    }

    Ok(())
}

pub fn decompress(bytes: &[u8], out: &mut Vec<u32>) -> Result<(), ()> {
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
    
    fn round_trip(data: &[u32]) {
        let mut bytes = Vec::new();
        compress(&data, &mut bytes).unwrap_or_else(|_| todo!());
        let mut out = Vec::new();
        decompress(&bytes, &mut out).unwrap_or_else(|_| todo!());

        assert_eq!(data, &out[..data.len()]);
    }

    #[test]
    fn t1() {
        let i = &[1, 5, 18, 99, 2023, 289981, 223389999];
        round_trip(i);
    }

    #[test]
    fn t2() {
        let i = &[1];
        round_trip(i);
    }

    #[test]
    fn too_large_is_err() {
        assert!(compress(&[std::u32::MAX], &mut Vec::new()).is_err())
    }
}
