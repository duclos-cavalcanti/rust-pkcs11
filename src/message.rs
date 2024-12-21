#[derive(Debug, Clone)]
pub enum MessageFlag {
    Ack,
    Req,
    End,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub flag: MessageFlag,
    pub data: String,
}

impl Message {
    pub fn new(id: i32, flag: MessageFlag, data: &str) -> Self {
        Self {
            id,
            flag,
            data: data.to_string(),
        }
    }
}
