pub trait ToClash {
    fn to_clash(&self) -> String;
}

pub trait ToV2ray {
    fn to_v2ray(&self) -> String;
}

pub trait ToSub: ToClash + ToV2ray {}
