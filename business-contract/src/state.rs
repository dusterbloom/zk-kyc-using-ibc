use cw_storage_plus::Map;

pub struct Wlmap {}

pub const WHITELIST_MAP: Map<String, bool> = Map::new("whitelist_map");

pub struct Ian {
    pub public: String,
}

pub const IAN: Map<'_, (String, u32), Ian> = Map::new("encrypted_record");
