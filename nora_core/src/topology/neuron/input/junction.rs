use uuid::Uuid;

#[derive(Debug)]
pub struct JunctionAffer {
    id: Uuid,
}
impl JunctionAffer {
    pub fn id(&self) -> Uuid {
        self.id
    }
}
