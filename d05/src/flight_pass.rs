pub struct FlightPassCollection {
    ids: Vec<u16>,
}

impl FlightPassCollection {
    pub fn new(buffer: Vec<u16>) -> Self {
        FlightPassCollection { ids: buffer }
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
        let id: &'a u16;
        if self.i == self.collection.ids.len() {
            return None;
        }
        id = &self.collection.ids[self.i];
        self.i += 1;
        Some(FlightPass { id })
    }
}

#[derive(Debug)]
pub struct FlightPass<'a> {
    pub id: &'a u16,
}

impl<'a> FlightPass<'a> {
    pub fn get_id(&self) -> u16 {
        *self.id
    }
}
