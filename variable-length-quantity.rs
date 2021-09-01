#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const SHIFT: u8 = 7;
const FLAG: u8 = 0x80;
const MAX: u64 = 0xFFFF_FFFF;

fn get_bases(val: u64) -> (u8, u64) {
    let mut shift: u8 = 28;
    let mut base: u64 = 0x7_F000_0000;
    loop {
        if (val & base) == 0 {
            shift -= SHIFT;
            base >>= SHIFT;
        } else {
            break (shift, base);
        }
    }
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for val in values {
        if *val == 0x00 {
            res.push(0x00);
            continue;
        }
        let mut val: u64 = *val as u64;
        let (mut shift, mut base) = get_bases(val);
        loop {
            let el = (val & base) >> shift;
            if shift == 0 {
                res.push(el as u8);
                break;
            }
            res.push(el as u8 | FLAG);
            val &= !base;
            shift -= SHIFT;
            base >>= SHIFT;
        }
    }
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res: Vec<u32> = Vec::new();
    let mut temp: u64 = 0;
    let mut incomplete = false;
    for byte in bytes {
        let val: u8 = !FLAG & *byte;
        temp += val as u64;
        incomplete = *byte > val;
        if incomplete {
            temp <<= SHIFT;
            if temp > MAX {
                return Err(Error::Overflow);
            }
        } else {
            res.push(temp as u32);
            temp = 0;
        }
    }
    if incomplete {
        Err(Error::IncompleteNumber)
    } else {
        Ok(res)
    }
}
