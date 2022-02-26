use serde::Deserialize;
#[derive(Deserialize)]
pub struct ArgsMsgOnly {
    pub msg: Option<String>,
}
impl ArgsMsgOnly {
    pub fn msg(&self) -> String {
        match &self.msg {
            Some(msg) => msg.to_string(),
            None => "".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct ArgsForNode {
    pub msg: Option<String>,
    pub page: Option<u32>,
}
impl ArgsForNode {
    pub fn msg(&self) -> String {
        match &self.msg {
            Some(msg) => msg.to_string(),
            None => "".to_string(),
        }
    }
    pub fn page(&self) -> u32 {
        match self.page {
            Some(u) => u,
            None => 0,
        }
    }
}
