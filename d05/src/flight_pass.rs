pub struct FlightPassCollection {
    buffer: Vec<bool>,
}

impl FlightPassCollection {
    pub fn new(buffer: Vec<bool>) -> Self {
        FlightPassCollection { buffer }
    }

    pub fn iter(&self) -> FlightPassCollectionIterator {
        FlightPassCollectionIterator {
            collection: &self,
            i: 0,
        }
    }
}

pub struct FlightPassCollectionIterator<'a> {
    collection: &'a FlightPassCollection,
    i: usize,
}

impl<'a> Iterator for FlightPassCollectionIterator<'a> {
    type Item = FlightPass<'a>;
    fn next(&mut self) -> Option<FlightPass<'a>> {
        let buffer: &'a [bool];
        if self.i == self.collection.buffer.len() {
            return None;
        }
        buffer = &self.collection.buffer[self.i..(self.i + 10)];
        self.i += 10;
        Some(FlightPass { buffer })
    }
}

#[derive(Debug)]
pub struct FlightPass<'a> {
    pub buffer: &'a [bool],
}

impl<'a> FlightPass<'a> {
    pub fn get_row(&self) -> u8 {
        let mut s = 0;
        let mut width = 128;
        for c in &self.buffer[0..7] {
            width /= 2;
            if *c {
                s += width;
            }
        }
        s
    }

    pub fn get_col(&self) -> u8 {
        let mut s = 0;
        let mut width = 8;
        for c in &self.buffer[7..10] {
            width /= 2;
            if *c {
                s += width;
            }
        }
        s
    }

    pub fn get_id(&self) -> u16 {
        (self.get_row() as u16) * 8 + (self.get_col() as u16)
    }
}
