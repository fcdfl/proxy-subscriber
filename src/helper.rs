use uuid::Uuid;

/// 生成新的UUID
pub fn uuid() -> String {
    Uuid::new_v4().to_simple().to_string()
}
