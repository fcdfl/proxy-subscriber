use serde::Deserialize;
#[derive(Deserialize)]
pub struct ArgsMsgOnly {
    pub msg: Option<String>,
}
impl ArgsMsgOnly {
    pub fn msg(&self) -> String {
        self.msg.clone().unwrap()
    }
}
