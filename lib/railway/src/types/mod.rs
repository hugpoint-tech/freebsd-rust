pub mod buffer;
pub mod enums;
pub mod object;
pub mod events;
pub mod requests;
pub mod id_store;
pub mod mempool;
pub mod handler;

pub use events::*;
pub use enums::*;
pub use object::*;
pub use requests::*;

/// Typless object
#[derive(Clone, Copy)]
pub struct Id {
    pub id: u32,
}


#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub obj_id: u32,
    pub opcode: u16,
    pub len: u16,
}

impl MessageHeader {

    pub const MAX_MSG_SIZE: u16 = 65532;

    #[inline]
    pub fn opcode_len(opcode: u16, msg_size: u16) -> u32 {
        ((msg_size as u32) << 16) | (opcode as u32)
    }

    pub fn from_words(word1: u32, word2: u32) -> MessageHeader {
        let opcode:u16 = (word2 & 0xFFFF) as u16;
        let len:u16 = (word2 >> 16) as u16;
        MessageHeader {
            obj_id: word1,
            opcode,
            len
        }
    }

    pub fn into_bytes(self) -> [u8;8] {
        let mut result: u64 = u64::from(self.obj_id);
        result |= u64::from(self.opcode) << 32;
        result |= u64::from(self.len) << 48;
        result.to_ne_bytes()
    }

}

#[derive(Debug, Copy, Clone)]
pub struct Fixed(pub u32);

impl Fixed {
    pub fn new(value: u32) -> Fixed {
        Fixed(value)
    }
}
// pub struct Id {
//         id: u32,
//         kind: ObjectType
// }


