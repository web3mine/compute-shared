use std::collections::HashMap;

pub type CID = HashMap<String, String>;

pub fn str2cid(str: String) -> CID {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("/".to_string(), str);
    m
}

pub fn cid2str(cid: CID) -> Option<String> {
    let msg = cid.get("/");
    msg.cloned()
}

pub fn cidr2str(cid: &CID) -> Option<&String> {
    cid.get("/")
}
