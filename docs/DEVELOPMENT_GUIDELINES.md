# 개발 가이드라인 및 코딩 규칙

## 1. 아키텍처 원칙

### 1.1 클린 아키텍처 (Clean Architecture)
```
┌─────────────────────────────────────────────────┐
│                   Presentation                   │
│              (UI Components, Views)              │
├─────────────────────────────────────────────────┤
│                   Application                    │
│              (Use Cases, Services)               │
├─────────────────────────────────────────────────┤
│                     Domain                       │
│            (Entities, Value Objects)             │
├─────────────────────────────────────────────────┤
│                 Infrastructure                   │
│          (DB, External APIs, Storage)            │
└─────────────────────────────────────────────────┘
```

### 1.2 의존성 규칙
- **내부에서 외부로의 의존성 금지**: Domain은 어떤 것에도 의존하지 않음
- **인터페이스 의존**: 구체적인 구현이 아닌 추상화에 의존
- **의존성 주입**: 외부 의존성은 DI를 통해 주입

## 2. 프로젝트 구조

### 2.1 Frontend (Leptos) 구조
```
ui/src/
├── domain/                 # 도메인 레이어
│   ├── entities/          # 핵심 비즈니스 엔티티
│   │   ├── photo.rs       # Photo 엔티티
│   │   ├── album.rs       # Album 엔티티
│   │   └── user.rs        # User 엔티티
│   └── value_objects/     # 값 객체
│       ├── exif_data.rs   # EXIF 데이터 VO
│       └── gps_location.rs # GPS 위치 VO
│
├── application/           # 애플리케이션 레이어
│   ├── use_cases/        # 유스케이스 (비즈니스 로직)
│   │   ├── upload_photo.rs
│   │   ├── create_album.rs
│   │   └── search_photos.rs
│   └── services/         # 애플리케이션 서비스
│       ├── auth_service.rs
│       └── photo_service.rs
│
├── presentation/         # 프레젠테이션 레이어
│   ├── components/      # 재사용 가능한 UI 컴포넌트
│   │   ├── common/      # 공통 컴포넌트
│   │   │   ├── button.rs
│   │   │   ├── input.rs
│   │   │   └── modal.rs
│   │   ├── photo/       # 사진 관련 컴포넌트
│   │   │   ├── photo_card.rs
│   │   │   ├── photo_grid.rs
│   │   │   └── photo_viewer.rs
│   │   └── album/       # 앨범 관련 컴포넌트
│   │       ├── album_card.rs
│   │       └── album_grid.rs
│   ├── pages/          # 페이지 컴포넌트
│   │   ├── home.rs
│   │   ├── gallery.rs
│   │   ├── album_detail.rs
│   │   └── settings.rs
│   └── layouts/        # 레이아웃 컴포넌트
│       ├── main_layout.rs
│       └── auth_layout.rs
│
└── infrastructure/      # 인프라 레이어
    ├── api/            # API 클라이언트
    │   ├── photo_api.rs
    │   └── auth_api.rs
    ├── storage/        # 로컬 스토리지
    │   └── local_storage.rs
    └── repositories/   # 리포지토리 구현
        ├── photo_repository.rs
        └── user_repository.rs
```

### 2.2 Backend 구조 (향후 구현)
```
backend/src/
├── domain/
├── application/
├── presentation/
└── infrastructure/
```

## 3. 네이밍 규칙

### 3.1 Rust 네이밍 컨벤션
```rust
// 모듈과 파일명: snake_case
mod photo_service;
mod user_repository;

// 구조체와 열거형: PascalCase
struct PhotoMetadata {
    camera_model: String,
}

enum UploadStatus {
    Pending,
    Processing,
    Completed,
}

// 함수와 변수: snake_case
fn calculate_file_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    // ...
}

// 상수: SCREAMING_SNAKE_CASE
const MAX_FILE_SIZE: usize = 100_000_000; // 100MB
const DEFAULT_THUMBNAIL_SIZE: (u32, u32) = (300, 300);

// 트레이트: PascalCase
trait PhotoRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Photo>;
}
```

### 3.2 명확한 변수명 규칙
```rust
// ❌ Bad: 모호한 이름
let d = calculate_date();
let temp = get_photo();
let flag = true;

// ✅ Good: 명확한 의도 표현
let upload_date = calculate_upload_date();
let selected_photo = get_selected_photo();
let is_public_album = true;

// ❌ Bad: 약어 남용
let pht = Photo::new();
let usr_cnt = count_users();

// ✅ Good: 읽기 쉬운 전체 단어
let photo = Photo::new();
let user_count = count_users();
```

### 3.3 파일명 규칙
```
// 컴포넌트 파일: 컴포넌트 역할을 명확히 표현
photo_upload_modal.rs   // 사진 업로드 모달
album_share_dialog.rs   // 앨범 공유 다이얼로그
exif_data_viewer.rs     // EXIF 데이터 뷰어

// 유틸리티 파일: 기능을 명확히 표현
image_processor.rs      // 이미지 처리 유틸리티
date_formatter.rs       // 날짜 포맷팅 유틸리티
file_validator.rs       // 파일 유효성 검증
```

## 4. 코드 품질 기준

### 4.1 함수 길이 제한 (30줄 규칙)
```rust
// ❌ Bad: 너무 긴 함수
fn process_photo_upload(data: Vec<u8>) -> Result<Photo> {
    // 40+ lines of code...
}

// ✅ Good: 책임별로 분리
fn process_photo_upload(data: Vec<u8>) -> Result<Photo> {
    let validated_data = validate_photo_data(&data)?;
    let metadata = extract_exif_metadata(&validated_data)?;
    let thumbnail = generate_thumbnail(&validated_data)?;
    let stored_photo = store_photo_file(validated_data)?;

    Ok(create_photo_record(stored_photo, metadata, thumbnail))
}

fn validate_photo_data(data: &[u8]) -> Result<Vec<u8>> {
    // 10-15 lines
}

fn extract_exif_metadata(data: &[u8]) -> Result<ExifData> {
    // 10-15 lines
}
```

### 4.2 코드 레이아웃
```rust
// 구조체 정의
#[derive(Debug, Clone)]
pub struct Photo {
    // 필수 필드
    pub id: Uuid,
    pub user_id: Uuid,
    pub file_hash: String,

    // 메타데이터
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: usize,

    // EXIF 데이터
    pub exif_data: Option<ExifData>,
    pub location: Option<GpsLocation>,

    // 타임스탬프
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Photo {
    // 생성자
    pub fn new(user_id: Uuid, filename: String) -> Self {
        // ...
    }

    // 공개 메소드
    pub fn update_metadata(&mut self, exif: ExifData) {
        // ...
    }

    // 비공개 헬퍼 메소드
    fn validate_file_type(&self) -> bool {
        // ...
    }
}
```

### 4.3 재사용 가능한 코드
```rust
// 제네릭과 트레이트를 활용한 재사용성
pub trait Repository<T, ID> {
    fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    fn save(&self, entity: T) -> Result<T>;
    fn delete(&self, id: ID) -> Result<()>;
}

// 재사용 가능한 Result 타입
pub type AppResult<T> = Result<T, AppError>;

// 재사용 가능한 유틸리티 함수
pub mod image_utils {
    pub fn resize_image(
        data: &[u8],
        max_width: u32,
        max_height: u32
    ) -> Result<Vec<u8>> {
        // ...
    }

    pub fn calculate_hash(data: &[u8]) -> String {
        // ...
    }
}
```

## 5. Leptos 특화 규칙

### 5.1 컴포넌트 구조
```rust
use leptos::prelude::*;

#[component]
pub fn PhotoCard(
    photo: ReadSignal<Photo>,
    #[prop(optional)] on_click: Option<Callback<Uuid>>,
) -> impl IntoView {
    // 1. 시그널과 상태 정의
    let (is_loading, set_loading) = create_signal(false);

    // 2. 이펙트와 리소스
    create_effect(move |_| {
        // 사이드 이펙트 처리
    });

    // 3. 이벤트 핸들러
    let handle_click = move |_| {
        if let Some(callback) = on_click {
            callback.call(photo.get().id);
        }
    };

    // 4. 렌더링
    view! {
        <div class="photo-card" on:click=handle_click>
            // Component JSX
        </div>
    }
}
```

### 5.2 반응성 패턴
```rust
// 파생 시그널 사용
let photo_count = create_memo(move |_| {
    photos.get().len()
});

// 리소스 패턴
let photos_resource = create_resource(
    move || search_query.get(),
    |query| async move {
        fetch_photos(query).await
    }
);
```

### 5.3 MVVM 패턴 및 상태 관리 (View-ViewModel-State)

UI의 복잡성을 관리하기 위해 Android의 MVVM 패턴을 차용하여 **View**, **ViewModel**, **State**로 책임을 분리합니다.

#### 아키텍처 구성요소
1.  **State (Model)**: 화면에 표시될 데이터의 순수 구조체입니다. UI의 "현재 상태"를 나타냅니다.
2.  **ViewModel**: State를 감싸는 `Signal`을 보유하고, 비즈니스 로직(UseCase 실행)을 담당합니다. View의 이벤트를 처리하여 State를 갱신합니다.
3.  **View**: `ViewModel`의 State를 구독하여 화면을 그립니다. 사용자 입력을 `ViewModel`의 메서드로 전달합니다.

#### 구현 예시

```rust
// 1. State: 화면 상태 정의 (Immutable)
#[derive(Clone, Debug, PartialEq)]
pub struct GalleryState {
    pub photos: Vec<Photo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

impl Default for GalleryState {
    fn default() -> Self {
        Self {
            photos: vec![],
            is_loading: false,
            error: None,
        }
    }
}

// 2. ViewModel: 로직 및 상태 관리
#[derive(Clone)]
pub struct GalleryViewModel {
    // UI가 구독할 읽기 전용 Signal
    pub state: ReadSignal<GalleryState>,
    // 내부 상태 변경용
    set_state: WriteSignal<GalleryState>,
}

impl GalleryViewModel {
    pub fn new() -> Self {
        let (state, set_state) = create_signal(GalleryState::default());
        Self { state, set_state }
    }

    // Action: 사진 목록 로드
    pub fn load_photos(&self) {
        let set_state = self.set_state;
        
        // 로딩 상태 시작
        set_state.update(|s| s.is_loading = true);
        
        spawn_local(async move {
            // 실제 앱에서는 UseCase를 주입받아 호출
            // let result = get_photos_use_case.execute().await;
            
            // 예시 로직
            let result = mock_fetch_photos().await; 
            
            set_state.update(|s| {
                s.is_loading = false;
                match result {
                    Ok(photos) => s.photos = photos,
                    Err(e) => s.error = Some(e.to_string()),
                }
            });
        });
    }
}

// 3. View: UI 렌더링
#[component]
pub fn GalleryPage() -> impl IntoView {
    // ViewModel 초기화 (Context API를 통해 상위에서 주입받는 것을 권장)
    let vm = GalleryViewModel::new();
    
    // 진입 시 데이터 로드
    create_effect(move |_| {
        vm.load_photos();
    });

    view! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">"내 갤러리"</h1>
            
            {move || {
                let state = vm.state.get();
                
                if state.is_loading {
                    view! { <div class="loader">"Loading..."</div> }.into_any()
                } else if let Some(err) = state.error {
                    view! { <div class="text-red-500">{err}</div> }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-3 gap-4">
                            {state.photos.into_iter()
                                .map(|photo| view! { <PhotoItem photo=photo /> })
                                .collect_view()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
```

#### 규칙
- **View는 로직을 모름**: `view!` 매크로 내부에서 복잡한 조건문이나 계산을 수행하지 않고, `ViewModel`이 제공한 가공된 데이터를 표시만 합니다.
- **ViewModel은 View를 모름**: `ViewModel`은 웹 DOM이나 Element에 직접 접근하지 않습니다. 오직 State만 변경합니다.
- **단방향 데이터 흐름**: `Action (View -> ViewModel)` -> `Update (ViewModel -> State)` -> `Render (State -> View)`

### 5.4 Context & Provider 패턴 (Dependency Injection)

Leptos의 Context API를 활용하여 Prop Drilling을 방지하고, ViewModel이나 Service 인스턴스를 컴포넌트 트리에 효과적으로 주입(DI)합니다.

#### 기본 원칙
1.  **Strict Typing**: Context 키로 구체적인 타입(struct)을 사용합니다.
2.  **Custom Hook**: `use_context::<T>()`를 직접 호출하기보다, `expect` 처리가 포함된 안전한 훅(`use_my_service()`)을 제공합니다.
3.  **Root vs Local**: 전역 싱글톤(AuthService 등)은 `App` 최상위에서, 특정 페이지 전용 상태(GalleryViewModel 등)는 해당 페이지 최상위에서 제공합니다.

#### 구현 예시

```rust
// 1. Context로 관리할 객체 정의
#[derive(Clone)]
pub struct GlobalState {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

// 2. Provider 컴포넌트 작성
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let (theme, set_theme) = create_signal(Theme::Light);
    
    // Context 제공
    provide_context(GlobalState { theme, set_theme });

    view! {
        {children()}
    }
}

// 3. Custom Hook (소비자)
pub fn use_theme() -> GlobalState {
    use_context::<GlobalState>()
        .expect("ThemeProvider가 상위 트리에 존재해야 합니다.")
}

// 4. 사용 (View)
#[component]
pub fn SomeChildComponent() -> impl IntoView {
    let global_state = use_theme(); // 안전하게 접근
    
    view! {
        <button on:click=move |_| global_state.set_theme.update(|t| t.toggle())>
            "Toggle Theme"
        </button>
    }
}
```

#### MVVM과 결합한 Page 레벨 Provider

페이지 단위로 ViewModel을 스코프에 가두어, 페이지를 벗어나면 상태가 정리되도록 합니다.

```rust
#[component]
pub fn GalleryPage() -> impl IntoView {
    // ViewModel 인스턴스 생성 (이 페이지와 생명주기를 같이 함)
    let vm = GalleryViewModel::new();
    
    // 하위 컴포넌트들이 vm에 접근할 수 있도록 제공
    provide_context(vm);

    view! {
        <GalleryContent />
    }
}

#[component]
fn GalleryContent() -> impl IntoView {
    // 상위 GalleryPage에서 제공한 ViewModel 주입
    let vm = use_context::<GalleryViewModel>()
        .expect("GalleryViewModel context missing");

    view! {
        // ... vm 사용 ...
    }
}
```

## 6. 메모리 효율성

### 6.1 소유권과 빌림
```rust
// ❌ Bad: 불필요한 복제
fn process_photo(photo: Photo) -> Photo {
    // photo가 이동됨
    photo
}

// ✅ Good: 참조 사용
fn process_photo(photo: &Photo) -> &Photo {
    // photo를 빌림
    photo
}

// ✅ Good: 필요한 경우만 복제
fn modify_photo(photo: &Photo) -> Photo {
    let mut new_photo = photo.clone();
    // 수정 작업
    new_photo
}
```

### 6.2 스마트 포인터 활용
```rust
use std::rc::Rc;
use std::cell::RefCell;

// 큰 데이터의 공유
type SharedPhoto = Rc<Photo>;

// 내부 가변성이 필요한 경우
type MutablePhoto = Rc<RefCell<Photo>>;

// 레이지 로딩
struct LazyPhoto {
    loader: Box<dyn Fn() -> Photo>,
    cached: Option<Photo>,
}
```

### 6.3 컬렉션 최적화
```rust
// 용량 사전 할당
let mut photos = Vec::with_capacity(expected_count);

// 이터레이터 체이닝 (중간 컬렉션 생성 방지)
let public_photos: Vec<_> = photos
    .iter()
    .filter(|p| p.is_public)
    .map(|p| p.id)
    .collect();

// String 재사용
let mut buffer = String::with_capacity(1024);
```

## 7. 에러 처리

### 7.1 Result 타입 활용
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhotoError {
    #[error("파일 크기 초과: {size} bytes (최대: {max} bytes)")]
    FileTooLarge { size: usize, max: usize },

    #[error("지원하지 않는 형식: {0}")]
    UnsupportedFormat(String),

    #[error("EXIF 데이터 추출 실패")]
    ExifExtractionFailed(#[from] exif::Error),
}

pub type PhotoResult<T> = Result<T, PhotoError>;
```

### 7.2 에러 전파
```rust
fn upload_photo(data: Vec<u8>) -> PhotoResult<Photo> {
    let validated = validate_file(data)?;
    let metadata = extract_metadata(&validated)?;
    let stored = store_file(validated)?;

    Ok(create_photo(stored, metadata))
}
```

## 8. 테스팅

### 8.1 유닛 테스트
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo_creation() {
        let photo = Photo::new(
            Uuid::new_v4(),
            "test.jpg".to_string()
        );

        assert_eq!(photo.original_filename, "test.jpg");
        assert!(photo.exif_data.is_none());
    }

    #[test]
    fn test_file_validation() {
        let invalid_data = vec![0; 10];
        let result = validate_file(invalid_data);

        assert!(result.is_err());
    }
}
```

### 8.2 통합 테스트
```rust
// tests/integration_test.rs
#[test]
async fn test_photo_upload_flow() {
    let app = spawn_app().await;
    let client = TestClient::new(app);

    let response = client
        .post("/api/photos")
        .body(test_photo_data())
        .send()
        .await;

    assert_eq!(response.status(), 201);
}
```

## 9. 문서화

### 9.1 모듈 문서화
```rust
//! # Photo Management Module
//!
//! 이 모듈은 사진 업로드, 저장, 메타데이터 처리를 담당합니다.
//!
//! ## Examples
//!
//! ```rust
//! use photo_manager::Photo;
//!
//! let photo = Photo::from_file("image.jpg")?;
//! ```

/// 사진 엔티티
///
/// EXIF 메타데이터와 함께 사진 정보를 저장합니다.
#[derive(Debug)]
pub struct Photo {
    /// 고유 식별자
    pub id: Uuid,
    // ...
}
```

## 10. CI/CD 설정

### 10.1 Pre-commit Hooks
```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: rust-fmt
        name: Rust Format
        entry: cargo fmt --all -- --check
        language: system

      - id: rust-clippy
        name: Rust Clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system

      - id: rust-test
        name: Rust Test
        entry: cargo test
        language: system
```

### 10.2 GitHub Actions
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy

      - name: Format Check
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Test
        run: cargo test --all-features
```

## 11. 성능 최적화 체크리스트

- [ ] 불필요한 clone() 제거
- [ ] Box, Rc, Arc 적절히 활용
- [ ] 큰 데이터는 참조로 전달
- [ ] 컬렉션 사전 할당 (with_capacity)
- [ ] 이터레이터 체이닝 활용
- [ ] 비동기 작업 병렬 처리
- [ ] 메모이제이션 적용
- [ ] 레이지 로딩 구현

## 12. 코드 리뷰 체크리스트

### 아키텍처
- [ ] 클린 아키텍처 원칙 준수
- [ ] 계층 간 의존성 방향 확인
- [ ] 단일 책임 원칙 준수

### 가독성
- [ ] 명확한 변수/함수명
- [ ] 함수 30줄 이내
- [ ] 적절한 주석과 문서화

### 성능
- [ ] 메모리 효율적인 코드
- [ ] 불필요한 복제 최소화
- [ ] 적절한 자료구조 선택

### 품질
- [ ] 에러 처리 완성도
- [ ] 테스트 커버리지
- [ ] 코드 재사용성

## 13. 개발 워크플로우 및 검증 (Development Workflow & Verification)

코드를 구현하거나 수정한 후에는 반드시 다음 단계를 거쳐 코드의 품질과 안정성을 검증해야 합니다.

### 13.1 단계별 검증 절차
1.  **코드 포맷팅**: `cargo fmt --all`을 실행하여 프로젝트 컨벤션에 맞게 코드를 정렬합니다.
2.  **정적 분석 (Lint)**: `cargo clippy --workspace --all-targets -- -D warnings`를 실행하여 잠재적인 버그나 비효율적인 코드를 찾아냅니다. (모든 경고를 에러로 처리)
3.  **컴파일 확인**: `cargo check --workspace`를 통해 빠르게 컴파일 오류 여부를 확인합니다.
4.  **테스트 실행**: `cargo test --workspace`를 실행하여 기존 기능의 회귀 오류가 없는지 확인하고, 새로 작성한 테스트가 통과하는지 검증합니다.

### 13.2 구현 시 주의사항
- **자동화 도구 활용**: 가능하다면 IDE의 "Format on Save" 기능을 활성화하고, 커밋 전에 위의 명령어들을 습관적으로 호출합니다.
- **경고 없는 코드**: Clippy가 발생하는 모든 경고는 무시하지 말고 수정하거나, 타당한 이유가 있는 경우에만 `#[allow(...)]`를 사용합니다.
- **테스트 동반**: 새로운 기능을 추가할 때는 해당 기능을 검증할 수 있는 유닛 테스트를 함께 작성하는 것을 원칙으로 합니다.

---

*이 가이드라인은 프로젝트의 성장과 팀의 요구에 따라 지속적으로 개선됩니다.*

**Version**: 1.0.0
**Last Updated**: 2026-01-30