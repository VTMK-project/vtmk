pub use uuid::Uuid;

pub(crate) fn uuid_generator() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}
