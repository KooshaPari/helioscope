use uuid::Uuid;

pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

pub fn short_id(id: &Uuid, namespace: &str, len: usize) -> String {
    if len <= namespace.len() + 1 {
        return format!("{}_{:x}", namespace, id.as_u128());
    }

    let digits = (len - namespace.len() - 1).min(32);
    let compact = id.as_simple().to_string();
    format!("{}_{}", namespace, &compact[..digits])
}
