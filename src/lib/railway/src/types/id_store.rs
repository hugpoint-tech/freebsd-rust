use crate::types::Object;

pub struct IdStore {
    client_ids: Vec<Object>,
    server_ids: Vec<Object>,
}

impl IdStore {
    pub fn new() -> Self {
        IdStore {
            client_ids: vec![Object::Null, Object::WlDisplay],
            server_ids: Vec::new(),
        }
    }

    pub fn get_next_id(&mut self, obj: Object) -> u32 {
        debug_assert!(self.client_ids.len() < 0xfeffffff);

        match obj {
            Object::Null => return 0,
            _ => self.client_ids.push(obj)
        }

        return self.client_ids.len() as u32;
    }

    pub fn lookup_id(&self, id: u32) -> Option<Object> {
        if id >= 1 && id <= 0xfeffffff {
            let index = id as usize - 1;
            if index < self.client_ids.len() {
                return Some(self.client_ids[index]);
            }
        } else if id >= 0xff000000 && id <= 0xffffffff {
            let index = (id - 0xff000000) as usize;
            if index < self.server_ids.len() {
                return Some(self.server_ids[index]);
            }
        }

        None
    }
}
