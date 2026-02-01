# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **PhotoVault** - an AI-powered photo gallery cloud service built with Leptos (Rust/WASM) for the frontend and PostgreSQL for data storage. The project is designed as a cross-platform application using Tauri + Leptos, with Exif 3.0 compliant photo metadata handling.

**Service Vision**: "모든 순간을 지능적으로 보관하고 아름답게 공유하는 개인 사진 클라우드"

For detailed service planning and business model, see: `docs/SERVICE_PLANNING.md`

## Development Commands

### Prerequisites Installation
```bash
# Install trunk (WASM bundler for Rust)
cargo install trunk

# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# Add necessary Rust components
rustup component add rust-src
rustup target add wasm32-unknown-unknown
```

### Development
```bash
# Run the development server (Web only)
cd ui && trunk serve

# Run Desktop App (Tauri)
cd app && cargo tauri dev

# Build for production (Web)
cd ui && trunk build --release

# Build for production (Desktop)
cd app && cargo tauri build

# Check code
cargo check --workspace

# Format code
cargo fmt --all

# Run clippy lints
cargo clippy --workspace --all-targets
```

### Database Setup
```bash
# PostgreSQL 설치 (macOS)
brew install postgresql postgis

# Create database
createdb photo_gallery

# Enable extensions
psql photo_gallery -c "CREATE EXTENSION postgis; CREATE EXTENSION pg_trgm; CREATE EXTENSION uuid-ossp;"

# Run migrations
psql photo_gallery < db/migrations/20260130123000_create_photo_gallery_schema.sql
```

## Architecture

### Project Structure
```
leptos_toy/
├── ui/                    # Leptos 프론트엔드 애플리케이션 (Workspace Member)
│   ├── src/
│   │   ├── domain/       # Clean Architecture: Entities
│   │   ├── infrastructure/ # Repositories & API Clients
│   │   ├── presentation/ # ViewModels, Pages, Components
│   │   └── main.rs
│   ├── style/
│   ├── dist/             # Web 빌드 출력
│   └── Trunk.toml
├── app/                   # Tauri 데스크톱 애플리케이션 루트
│   └── src-tauri/        # Tauri Rust Core (Workspace Member)
│       ├── src/          # Tauri 엔트리 포인트
│       ├── tauri.conf.json # Tauri 설정 (ui/dist 참조)
│       └── Cargo.toml
├── db/                   # 데이터베이스 마이그레이션
├── docs/                 # 프로젝트 문서
└── Cargo.toml           # Cargo Workspace 설정
```

### Frontend Architecture (Leptos)
- **Pattern**: MVVM (Model-View-ViewModel) with Clean Architecture
- **State Management**: `create_resource` for async data, `Signal` for local state
- **Styling**: TailwindCSS v4.1.13
- **Optimization**: `IntersectionObserver` (Infinite Scroll), Lazy Loading, Async Decoding

### Desktop Architecture (Tauri 2.0)
- **Core**: Rust (app/src-tauri)
- **Frontend**: Shared with `ui` project (dist folder)
- **Communication**: Tauri Commands (Invokes) & Events
- **Database**: PostgreSQL with PostGIS, pg_trgm extensions
- **Schema Design**:
  - Hybrid approach: 정규화된 테이블 + JSONB for Exif metadata
  - PostGIS for GPS-based location queries
  - Full Exif 3.0 compliance
- **Key Tables**:
  - `users`: 사용자 관리 및 스토리지 할당
  - `photos`: 사진 메타데이터 및 Exif 데이터 (JSONB)
  - `albums`: 앨범 및 컬렉션
  - `tags`: 태그 시스템 (AI 태그 지원)
  - `shares`: 공유 권한 관리
  - `people`: 얼굴 인식 데이터
  - `processing_queue`: 비동기 이미지 처리 작업

## Key Features (PhotoVault)

### Core Functionalities
- **Smart Storage**: AI-powered automatic classification and organization
- **Privacy First**: End-to-end encryption options, secure cloud storage
- **Beautiful Sharing**: Intuitive sharing experience with customizable permissions
- **Cross-Platform**: Consistent experience across web, mobile, and desktop (Tauri)

### Photo Management
- **Exif 3.0 Support**: Complete metadata extraction and storage
- **Deduplication**: SHA-256 hash-based duplicate detection
- **Storage Management**: Freemium model (10GB free, 1TB pro, 5TB family)
- **Image Processing**: Thumbnail generation, AI tagging, face detection
- **RAW Support**: Professional photography format support (Pro tier)

### AI-Powered Features
- **Auto-Tagging**: Object, scene, and activity recognition
- **Face Recognition**: Automatic person grouping with privacy controls
- **Color Analysis**: Dominant color palette extraction
- **Quality Assessment**: Automatic blur and exposure evaluation
- **Smart Albums**: Condition-based auto-updating collections

### Search & Discovery
- **Natural Language Search**: "작년 여름 바다에서 찍은 사진"
- **Location-based**: GPS coordinates with PostGIS and reverse geocoding
- **Camera/Lens**: Search by equipment and EXIF settings
- **Similar Images**: Find photos with similar composition or colors
- **Complex Filters**: Combine date, location, people, and camera settings

### Social & Collaboration
- **Shared Albums**: Collaborative collections with family/friends
- **Link Sharing**: Password-protected, expiring links
- **Comments & Reactions**: Photo-level interaction
- **Stories**: Automatic chronological storytelling
- **Social Export**: Direct sharing to major social platforms

## Development Roadmap

### Current Phase: MVP Development (Phase 1)
- ✅ Database schema design (PostgreSQL + PostGIS)
- ✅ Basic UI framework (Leptos + TailwindCSS)
- 🔄 Photo upload/download functionality
- 🔄 Basic metadata extraction (Exif 3.0)
- ⏳ Album creation and management
- ⏳ User authentication system

### Business Model
- **Freemium**: 10GB free, Pro ($9.99/mo for 1TB), Family ($19.99/mo for 5TB)
- **Target Users**: Photography enthusiasts, families, travelers, content creators
- **Differentiators**: Complete EXIF preservation, open-source, true cross-platform

## Recent Updates

### TailwindCSS Integration (2026-01-30)
- Added TailwindCSS v4.1.13 for styling
- Trunk automatically handles TailwindCSS compilation via `Trunk.toml` configuration
- Style input: `ui/style/input.css` with `@import "tailwindcss"`
- HTML template updated with Trunk data attributes for CSS processing

### Service Planning Documentation (2026-01-30)
- Complete service planning document added at `docs/SERVICE_PLANNING.md`
- Includes business model, roadmap, KPIs, and technical architecture
- Target: 100K MAU within first year, 5% free-to-pro conversion

### UI Layout Improvement (2026-02-01)
- Implemented `MainLayout` with fixed title bar and independent content scrolling
- Applied `h-dvh` for better mobile viewport handling
- Refactored `GalleryPage` to use the new layout structure

### Photo Preview & History API (2026-02-01)
- Implemented `PhotoPreviewModal` for full-screen photo details without page navigation
- Integrated Browser History API (`pushState`, `popstate`) to handle "Back" button for closing the modal
- Updated `GalleryViewModel` to manage `selected_photo` state and history synchronization
- Enhanced `PhotoCard` with click-to-preview and event propagation handling
- Added navigation (Previous/Next) support in `PhotoPreviewModal` with arrow buttons and keyboard shortcuts

## Important Notes

- The project is configured for Tauri integration but currently runs as a standalone web app
- Database uses PostgreSQL with PostGIS for optimal photo gallery performance
- JSONB storage for Exif data provides flexibility for various Exif versions
- Trunk handles both WASM compilation and TailwindCSS processing automatically
- The `index.html` includes Trunk-specific data attributes (`data-trunk`) for asset processing
- All database operations should consider the trigger functions for automatic updates (storage usage, view counts, timestamps)