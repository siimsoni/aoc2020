mod passports;

use passports::Passports;
use std::io::BufRead;
use std::str;

pub fn parse<R>(mut reader: R) -> Passports
where
    R: BufRead,
{
    let mut lines: Vec<u8> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    let mut pad = false;

    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        match n {
            0 => break,
            1 => {
                lines.append(&mut buf);
                pad = false;
            }
            _n => {
                if pad {
                    lines.push(b' ');
                }
                if buf[buf.len() - 1] == b'\n' {
                    buf.pop();
                }
                lines.append(&mut buf);
                pad = true;
            }
        }
    }

    Passports::new(lines)
}

pub fn p1_solve(passports: &Passports) -> usize {
    let mut i = 0;
    let mut fields = 0;
    for passport in passports.iter() {
        for (key, _value) in passport.iter() {
            match key {
                b"byr" => fields |= 1,
                b"iyr" => fields |= 2,
                b"eyr" => fields |= 4,
                b"hgt" => fields |= 8,
                b"hcl" => fields |= 16,
                b"ecl" => fields |= 32,
                b"pid" => fields |= 64,
                _ => (),
            }
        }
        i += (fields == 127) as usize;
        fields = 0
    }
    i
}

macro_rules! is_hex {
    ($e:expr) => {
        (is_digit!($e) || ($e >= b'a' && $e <= b'f'))
    };
}

macro_rules! is_digit {
    ($e:expr) => {
        ($e >= b'0' && $e <= b'9')
    };
}

pub fn p2_solve(passports: &Passports) -> usize {
    let mut i = 0;
    let mut fields = 0;
    let mut len;
    for passport in passports.iter() {
        for (key, value) in passport.iter() {
            match key {
                b"byr" => {
                    if let Some(byr) = str::from_utf8(value)
                        .ok()
                        .and_then(|str| str.parse::<u16>().ok())
                    {
                        if byr >= 1920 && byr <= 2002 {
                            fields |= 1;
                        }
                    }
                }
                b"iyr" => {
                    if let Some(iyr) = str::from_utf8(value)
                        .ok()
                        .and_then(|str| str.parse::<u16>().ok())
                    {
                        if iyr >= 2010 && iyr <= 2020 {
                            fields |= 2
                        }
                    }
                }
                b"eyr" => {
                    if let Some(eyr) = str::from_utf8(value)
                        .ok()
                        .and_then(|str| str.parse::<u16>().ok())
                    {
                        if eyr >= 2020 && eyr <= 2030 {
                            fields |= 4
                        }
                    }
                }
                b"hgt" => {
                    len = value.len();
                    if value.len() > 2 {
                        if let Some(hgt) = str::from_utf8(&value[0..len - 2])
                            .ok()
                            .and_then(|str| str.parse::<u8>().ok())
                        {
                            match &value[len - 2..] {
                                b"cm" => {
                                    if hgt >= 150 && hgt <= 193 {
                                        fields |= 8
                                    }
                                }
                                b"in" => {
                                    if hgt >= 59 && hgt <= 76 {
                                        fields |= 8
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                b"hcl" => {
                    len = value.len();
                    if len == 7
                        && &value[..1] == b"#"
                        && is_hex!(value[1])
                        && is_hex!(value[2])
                        && is_hex!(value[3])
                        && is_hex!(value[4])
                        && is_hex!(value[5])
                        && is_hex!(value[6])
                    {
                        fields |= 16
                    }
                }
                b"ecl" => match value {
                    b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth" => fields |= 32,
                    _ => (),
                },
                b"pid" => {
                    len = value.len();
                    if len == 9
                        && is_digit!(value[0])
                        && is_digit!(value[1])
                        && is_digit!(value[2])
                        && is_digit!(value[3])
                        && is_digit!(value[4])
                        && is_digit!(value[5])
                        && is_digit!(value[6])
                        && is_digit!(value[7])
                        && is_digit!(value[8])
                    {
                        fields |= 64;
                    }
                }
                _ => (),
            }
        }
        i += (fields == 127) as usize;
        fields = 0
    }
    i
}
