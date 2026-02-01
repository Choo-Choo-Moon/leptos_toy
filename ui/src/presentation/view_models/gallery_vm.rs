use crate::domain::entities::photo::Photo;
use crate::infrastructure::repositories::mock_photo_repository::fetch_mock_photos;
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct GalleryState {
    pub photos: Vec<Photo>,
    pub is_loading: bool,
    pub page: usize,
    pub has_more: bool,
    pub selected_photo: Option<Photo>,
}

impl Default for GalleryState {
    fn default() -> Self {
        Self {
            photos: vec![],
            is_loading: false,
            page: 1,
            has_more: true,
            selected_photo: None,
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

    pub fn select_photo(&self, photo: Photo) {
        self.set_state.update(|s| s.selected_photo = Some(photo));

        // 브라우저 히스토리에 상태 추가
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                let _ = history.push_state_with_url(&JsValue::NULL, "", Some("#preview"));
            }
        }
    }

    // 뒤로가기 이벤트(PopState) 발생 시 상태 동기화
    pub fn sync_on_popstate(&self) {
        // 해시가 없으면 모달 닫기
        if let Some(window) = web_sys::window() {
            let location = window.location();
            if let Ok(hash) = location.hash() {
                if hash.is_empty() {
                    self.set_state.update(|s| s.selected_photo = None);
                }
            }
        }
    }

    // 사용자가 닫기 버튼을 눌렀을 때
    pub fn close_preview(&self) {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                // history.back()을 호출하면 popstate 이벤트가 발생하여 sync_on_popstate가 호출됨
                let _ = history.back();
            }
        }
    }
}
