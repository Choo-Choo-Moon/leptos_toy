use crate::domain::entities::photo::Photo;
use crate::infrastructure::repositories::mock_photo_repository::fetch_mock_photos;
use leptos::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GalleryState {
    pub photos: Vec<Photo>,
    pub is_loading: bool,
    pub page: usize,
    pub has_more: bool,
}

impl Default for GalleryState {
    fn default() -> Self {
        Self {
            photos: vec![],
            is_loading: false,
            page: 1,
            has_more: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GalleryViewModel {
    pub state: ReadSignal<GalleryState>,
    set_state: WriteSignal<GalleryState>,
}

impl GalleryViewModel {
    pub fn new() -> Self {
        let (state, set_state) = signal(GalleryState::default());
        Self { state, set_state }
    }

    pub fn load_more(&self) {
        let state = self.state.get_untracked();
        if state.is_loading || !state.has_more {
            return;
        }

        let set_state = self.set_state;
        set_state.update(|s| s.is_loading = true);

        let current_page = state.page;
        
        leptos::task::spawn_local(async move {
            let limit = 20;
            let new_photos = fetch_mock_photos(current_page, limit).await;
            
            set_state.update(|s| {
                if new_photos.is_empty() {
                    s.has_more = false;
                } else {
                    s.photos.extend(new_photos);
                    s.page += 1;
                }
                s.is_loading = false;
            });
        });
    }

    pub fn toggle_like(&self, photo_id: Uuid) {
        self.set_state.update(|s| {
            if let Some(photo) = s.photos.iter_mut().find(|p| p.id == photo_id) {
                photo.is_liked = !photo.is_liked;
            }
        });
    }
}
