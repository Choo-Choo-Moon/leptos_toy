-- Photo Gallery Database Schema with Exif 3.0 Support
-- Database: PostgreSQL with PostGIS extension

-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";
CREATE EXTENSION IF NOT EXISTS "pg_trgm"; -- For fuzzy text search

-- =====================================================
-- 핵심 사용자 관리
-- =====================================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    display_name VARCHAR(255),
    avatar_url TEXT,
    storage_used_bytes BIGINT DEFAULT 0,
    storage_limit_bytes BIGINT DEFAULT 10737418240, -- 10GB 기본값
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    INDEX idx_users_email (email),
    INDEX idx_users_username (username)
);

-- =====================================================
-- 사진 저장 및 메타데이터
-- =====================================================

CREATE TABLE photos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- 파일 정보
    original_filename VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_hash VARCHAR(64) NOT NULL, -- SHA-256 해시 (중복 제거용)
    storage_path TEXT NOT NULL,
    thumbnail_path TEXT,
    preview_path TEXT,

    -- 기본 EXIF 데이터 (자주 조회되는 필드, JSONB에서 추출)
    taken_at TIMESTAMPTZ,
    camera_make VARCHAR(100),
    camera_model VARCHAR(100),
    lens_model VARCHAR(200),
    focal_length_mm NUMERIC(6,2),
    aperture_value NUMERIC(4,2),
    shutter_speed_seconds NUMERIC(10,8),
    iso_value INTEGER,
    flash_used BOOLEAN,

    -- 이미지 크기
    width_pixels INTEGER NOT NULL,
    height_pixels INTEGER NOT NULL,
    orientation INTEGER DEFAULT 1, -- EXIF 방향 (1-8)

    -- GPS 위치 (PostGIS 사용)
    location GEOGRAPHY(POINT, 4326),
    location_name VARCHAR(500), -- 역지오코딩된 위치명
    altitude_meters NUMERIC(8,2),

    -- 전체 EXIF 데이터 (Exif 3.0 준수)
    exif_data JSONB NOT NULL DEFAULT '{}',

    -- AI/ML 생성 데이터
    ai_tags JSONB DEFAULT '[]',
    color_palette JSONB DEFAULT '[]',
    dominant_colors VARCHAR(7)[], -- 16진수 색상 배열

    -- 메타데이터
    is_public BOOLEAN DEFAULT false,
    view_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_user_file_hash UNIQUE(user_id, file_hash),
    INDEX idx_photos_user_id (user_id),
    INDEX idx_photos_taken_at (taken_at),
    INDEX idx_photos_camera (camera_make, camera_model),
    INDEX idx_photos_location (location),
    INDEX idx_photos_file_hash (file_hash),
    INDEX idx_photos_exif_data_gin (exif_data) USING GIN
);

-- =====================================================
-- 앨범 및 컬렉션
-- =====================================================

CREATE TABLE albums (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    cover_photo_id UUID REFERENCES photos(id) ON DELETE SET NULL,
    is_public BOOLEAN DEFAULT false,
    share_token VARCHAR(32) UNIQUE, -- 링크 공유용
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_albums_user_id (user_id),
    INDEX idx_albums_share_token (share_token)
);

CREATE TABLE album_photos (
    album_id UUID NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    added_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (album_id, photo_id),
    INDEX idx_album_photos_position (album_id, position)
);

-- =====================================================
-- 태그 및 키워드
-- =====================================================

CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) UNIQUE NOT NULL,
    category VARCHAR(50), -- 'person', 'place', 'object', 'event' 등
    created_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_tags_name (name),
    INDEX idx_tags_category (category)
);

CREATE TABLE photo_tags (
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    confidence NUMERIC(3,2), -- AI 생성 태그용 신뢰도 (0.00 ~ 1.00)
    added_by UUID REFERENCES users(id) ON DELETE SET NULL,
    added_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (photo_id, tag_id),
    INDEX idx_photo_tags_tag_id (tag_id)
);

-- =====================================================
-- 공유 및 권한
-- =====================================================

CREATE TABLE shares (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    resource_type VARCHAR(20) NOT NULL CHECK (resource_type IN ('photo', 'album')),
    resource_id UUID NOT NULL,
    shared_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    shared_with UUID REFERENCES users(id) ON DELETE CASCADE, -- 공개 링크는 NULL
    permission VARCHAR(20) NOT NULL CHECK (permission IN ('view', 'download', 'edit')),
    share_token VARCHAR(32) UNIQUE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_shares_resource (resource_type, resource_id),
    INDEX idx_shares_shared_with (shared_with),
    INDEX idx_shares_token (share_token)
);

-- =====================================================
-- 사용자 활동 및 분석
-- =====================================================

CREATE TABLE photo_views (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    viewer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    viewer_ip INET,
    viewed_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_photo_views_photo_id (photo_id),
    INDEX idx_photo_views_viewer_id (viewer_id),
    INDEX idx_photo_views_viewed_at (viewed_at)
);

CREATE TABLE favorites (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (user_id, photo_id),
    INDEX idx_favorites_photo_id (photo_id)
);

-- =====================================================
-- 댓글 및 소셜 기능
-- =====================================================

CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    INDEX idx_comments_photo_id (photo_id),
    INDEX idx_comments_user_id (user_id),
    INDEX idx_comments_parent (parent_comment_id)
);

-- =====================================================
-- 얼굴 인식 및 인물
-- =====================================================

CREATE TABLE people (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    face_encoding BYTEA, -- 얼굴 인식 벡터
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_people_user_id (user_id)
);

CREATE TABLE photo_faces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    person_id UUID REFERENCES people(id) ON DELETE SET NULL,
    face_rectangle JSONB NOT NULL, -- {x, y, width, height}
    confidence NUMERIC(3,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_photo_faces_photo_id (photo_id),
    INDEX idx_photo_faces_person_id (person_id)
);

-- =====================================================
-- 처리 대기열
-- =====================================================

CREATE TABLE processing_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
    task_type VARCHAR(50) NOT NULL, -- 'thumbnail', 'exif', 'ai_tags', 'face_detection'
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    priority INTEGER DEFAULT 5,
    retry_count INTEGER DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    INDEX idx_processing_queue_status (status, priority, created_at),
    INDEX idx_processing_queue_photo_id (photo_id)
);

-- =====================================================
-- 감사 로그
-- =====================================================

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    resource_type VARCHAR(20),
    resource_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    INDEX idx_audit_logs_user_id (user_id),
    INDEX idx_audit_logs_resource (resource_type, resource_id),
    INDEX idx_audit_logs_created_at (created_at)
);

-- =====================================================
-- 함수 및 트리거
-- =====================================================

-- updated_at 타임스탬프 업데이트
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_photos_updated_at BEFORE UPDATE ON photos
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_albums_updated_at BEFORE UPDATE ON albums
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 사진 추가/삭제 시 사용자 스토리지 업데이트
CREATE OR REPLACE FUNCTION update_user_storage()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users
        SET storage_used_bytes = storage_used_bytes + NEW.file_size_bytes
        WHERE id = NEW.user_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users
        SET storage_used_bytes = storage_used_bytes - OLD.file_size_bytes
        WHERE id = OLD.user_id;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_user_storage_on_photo_change
    AFTER INSERT OR DELETE ON photos
    FOR EACH ROW EXECUTE FUNCTION update_user_storage();

-- 사진 조회수 업데이트
CREATE OR REPLACE FUNCTION increment_photo_view_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE photos
    SET view_count = view_count + 1
    WHERE id = NEW.photo_id;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER increment_view_count_on_view
    AFTER INSERT ON photo_views
    FOR EACH ROW EXECUTE FUNCTION increment_photo_view_count();

-- =====================================================
-- 일반 쿼리용 인덱스
-- =====================================================

-- 지리공간 쿼리
CREATE INDEX idx_photos_location_spatial ON photos USING GIST (location);

-- 날짜 범위 쿼리
CREATE INDEX idx_photos_date_range ON photos (user_id, taken_at DESC);

-- 설명 및 태그 전문 검색
CREATE INDEX idx_albums_title_trgm ON albums USING GIN (title gin_trgm_ops);
CREATE INDEX idx_tags_name_trgm ON tags USING GIN (name gin_trgm_ops);

-- EXIF 검색 최적화
CREATE INDEX idx_photos_exif_camera_settings ON photos ((exif_data->>'ExposureProgram'), (exif_data->>'MeteringMode'));
CREATE INDEX idx_photos_exif_lens ON photos ((exif_data->>'LensModel'));
CREATE INDEX idx_photos_exif_software ON photos ((exif_data->>'Software'));

-- =====================================================
-- 파티셔닝 (대규모용)
-- =====================================================

-- 연도별 photos 테이블 파티셔닝 (필요시 주석 해제)
-- CREATE TABLE photos_2024 PARTITION OF photos
--     FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
-- CREATE TABLE photos_2025 PARTITION OF photos
--     FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');

-- =====================================================
-- 초기 데이터
-- =====================================================

-- 기본 태그 카테고리 삽입
INSERT INTO tags (name, category) VALUES
    ('landscape', 'scene'),
    ('portrait', 'scene'),
    ('nature', 'scene'),
    ('architecture', 'scene'),
    ('street', 'scene'),
    ('macro', 'technique'),
    ('black-white', 'style'),
    ('sunset', 'time'),
    ('night', 'time')
ON CONFLICT (name) DO NOTHING;

-- =====================================================
-- 코멘트
-- =====================================================

COMMENT ON TABLE photos IS 'Exif 3.0 메타데이터를 지원하는 메인 사진 저장 테이블';
COMMENT ON COLUMN photos.exif_data IS 'JSONB로 저장된 완전한 Exif 3.0 메타데이터';
COMMENT ON COLUMN photos.location IS 'PostGIS geography 타입을 사용한 GPS 좌표';
COMMENT ON COLUMN photos.file_hash IS '중복 제거를 위한 SHA-256 해시';
COMMENT ON COLUMN photos.orientation IS '올바른 표시를 위한 EXIF 방향 플래그 (1-8)';
COMMENT ON COLUMN photos.storage_used_bytes IS '사용자가 사용 중인 스토리지 용량 (바이트)';
COMMENT ON COLUMN photos.storage_limit_bytes IS '사용자 스토리지 제한 (바이트)';

COMMENT ON TABLE albums IS '사진 앨범 및 컬렉션 관리 테이블';
COMMENT ON COLUMN albums.share_token IS '링크를 통한 공유를 위한 고유 토큰';

COMMENT ON TABLE tags IS '사진 분류를 위한 태그 테이블';
COMMENT ON COLUMN photo_tags.confidence IS 'AI가 생성한 태그의 신뢰도 점수 (0.00-1.00)';

COMMENT ON TABLE shares IS '사진 및 앨범 공유 권한 관리';
COMMENT ON COLUMN shares.shared_with IS '특정 사용자와 공유 (NULL이면 공개 링크)';

COMMENT ON TABLE people IS '얼굴 인식을 통한 인물 관리';
COMMENT ON COLUMN people.face_encoding IS '얼굴 인식을 위한 특징 벡터';

COMMENT ON TABLE processing_queue IS '비동기 이미지 처리 작업 대기열';
COMMENT ON TABLE audit_logs IS '사용자 활동 및 시스템 이벤트 감사 로그';