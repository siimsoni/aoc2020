pub struct Passports {
    buffer: Vec<u8>,
}

impl Passports {
    pub fn new(buffer: Vec<u8>) -> Self {
        Passports { buffer }
    }

    pub fn iter(&self) -> PassportCollectionIterator {
        PassportCollectionIterator {
            passports: &self,
            i: 0,
            start: 0,
        }
    }
}

pub struct PassportCollectionIterator<'a> {
    passports: &'a Passports,
    i: usize,
    start: usize,
}

impl<'a> Iterator for PassportCollectionIterator<'a> {
    type Item = Passport<'a>;
    fn next(&mut self) -> Option<Passport<'a>> {
        let buffer: &'a [u8];
        self.start = self.i;
        for c in &self.passports.buffer[self.i..] {
            if *c == b'\n' {
                buffer = &self.passports.buffer[self.start..self.i];
                self.i += 1;
                return Some(Passport { buffer });
            }
            self.i += 1;
        }
        if self.i > self.start {
            return Some(Passport {
                buffer: &self.passports.buffer[self.start..self.i],
            });
        }
        None
    }
}

#[derive(Debug)]
pub struct Passport<'a> {
    buffer: &'a [u8],
}

pub struct PassportIterator<'a> {
    passport: &'a Passport<'a>,
    i: usize,
    start: usize,
}

impl Passport<'_> {
    pub fn iter(&self) -> PassportIterator {
        PassportIterator {
            passport: self,
            i: 0,
            start: 0,
        }
    }
}

impl<'a> Iterator for PassportIterator<'a> {
    type Item = (&'a [u8], &'a [u8]);
    fn next(&mut self) -> Option<(&'a [u8], &'a [u8])> {
        if self.i > self.passport.buffer.len() {
            return None;
        }
        self.start = self.i;
        let mut key = None;
        let mut value = None;

        for c in &self.passport.buffer[self.i..] {
            if *c == b':' {
                key = Some(&self.passport.buffer[self.start..self.i]);
                break;
            }
            self.i += 1;
        }
        self.i += 1;
        if self.i > self.passport.buffer.len() {
            return None;
        }
        self.start = self.i;
        for c in &self.passport.buffer[self.start..] {
            if *c == b' ' {
                value = Some(&self.passport.buffer[self.start..self.i]);
                break;
            }
            self.i += 1;
        }
        if value.is_none() && self.i > self.start {
            value = Some(&self.passport.buffer[self.start..self.i]);
        }
        self.i += 1;
        key.and_then(|k| value.map(|v| (k, v)))
    }
}
