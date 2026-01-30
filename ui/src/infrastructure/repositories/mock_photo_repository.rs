use crate::domain::entities::photo::Photo;
use gloo_timers::future::TimeoutFuture;

pub async fn fetch_mock_photos(page: usize, limit: usize) -> Vec<Photo> {
    // 네트워크 지연 시뮬레이션
    TimeoutFuture::new(800).await;

    let start = (page - 1) * limit;
    (start..start + limit)
        .map(Photo::new_mock)
        .collect()
}
