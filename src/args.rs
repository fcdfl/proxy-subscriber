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
// https://xxxxx/s/4182d807326c4fceaa87eaaeb56b7d9a?c=v2ray&cf=ct&co=1
#[derive(Deserialize, Debug)]
pub struct ArgsForSub {
    /// 客户端
    pub c: String,
    /// 优选IP的线路
    pub cf: String,
    /// 是否只获取优选IP的节点
    pub co: u8,
}
impl ArgsForSub {
    pub fn co(&self) -> bool {
        self.co == 1
    }
    pub fn cfs(&self) -> Option<Vec<String>> {
        let cf = self.cf.clone();
        let cf = cf.trim_matches(',').trim();
        if cf.is_empty() {
            return None;
        }

        let mut cfs = Vec::new();
        if !cf.contains(",") {
            cfs.push(cf.to_string());
            return Some(cfs);
        }

        cf.split(',')
            .filter(|s| !s.trim().is_empty())
            .for_each(|s| cfs.push(s.trim().to_string()));
        Some(cfs)
    }
}
