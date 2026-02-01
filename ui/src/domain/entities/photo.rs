use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Photo {
    pub id: Uuid,
    pub url: String,
    pub preview_url: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub is_liked: bool,
    pub width: u32,
    pub height: u32,
}

impl Photo {
    pub fn new_mock(id: usize) -> Self {
        let image_id = id % 1000;
        
        // Mocking varied aspect ratios based on ID
        let (width, height) = match image_id % 3 {
            0 => (1600, 900),   // 16:9 Landscape
            1 => (800, 1200),   // 2:3 Portrait
            _ => (1200, 1200),  // 1:1 Square
        };

        Self {
            id: Uuid::new_v4(),
            url: format!("https://picsum.photos/id/{}/400/400", image_id), // Thumbnail (Square crop)
            preview_url: format!("https://picsum.photos/id/{}/{}/{}", image_id, width, height), // Preview (Original ratio)
            title: format!("Photo #{}", id),
            created_at: Utc::now(),
            is_liked: false,
            width,
            height,
        }
    }
}
