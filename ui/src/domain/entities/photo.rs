use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Photo {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub is_liked: bool,
}

impl Photo {
    pub fn new_mock(id: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            url: format!("https://picsum.photos/id/{}/400/400", id % 1000), // 유효한 이미지 ID 범위 내로 순환
            title: format!("Photo #{}", id),
            created_at: Utc::now(),
            is_liked: false,
        }
    }
}
