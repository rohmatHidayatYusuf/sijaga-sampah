-- ========================================================================
-- SIJAGA SAMPAH - PostgreSQL Database Schema Migration
-- Version: 0001_initial_schema.sql
-- Based on PRD SIJAGA SAMPAH (Rust + PostgreSQL)
-- ========================================================================

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 1. Roles table
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(40) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 2. Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(150) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(30),
    password_hash TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 3. User roles mapping table
CREATE TABLE IF NOT EXISTS user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, role_id)
);

-- 4. Regions table
CREATE TABLE IF NOT EXISTS regions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES regions(id) ON DELETE RESTRICT,
    code VARCHAR(50) UNIQUE,
    name VARCHAR(150) NOT NULL,
    region_type VARCHAR(40) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- 5. Waste categories table
CREATE TABLE IF NOT EXISTS waste_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(120) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- 6. Reports table
CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(32) NOT NULL UNIQUE,
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    category_id UUID NOT NULL REFERENCES waste_categories(id) ON DELETE RESTRICT,
    region_id UUID REFERENCES regions(id) ON DELETE SET NULL,
    title VARCHAR(160) NOT NULL,
    description TEXT NOT NULL,
    address_text TEXT,
    latitude NUMERIC(9,6),
    longitude NUMERIC(9,6),
    status VARCHAR(30) NOT NULL DEFAULT 'submitted'
      CHECK (status IN ('submitted','under_review','verified','assigned',
        'in_progress','resolved','closed','rejected','reopened')),
    priority VARCHAR(20) NOT NULL DEFAULT 'normal'
      CHECK (priority IN ('low','normal','high','urgent')),
    rejection_reason TEXT,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180),
    CHECK (latitude IS NOT NULL OR longitude IS NOT NULL OR address_text IS NOT NULL)
);

-- 7. Report attachments table
CREATE TABLE IF NOT EXISTS report_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    object_key TEXT NOT NULL UNIQUE,
    original_filename VARCHAR(255),
    content_type VARCHAR(100) NOT NULL,
    file_size_bytes BIGINT NOT NULL CHECK (file_size_bytes > 0),
    attachment_type VARCHAR(30) NOT NULL
      CHECK (attachment_type IN ('report_photo','completion_evidence')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 8. Report status history table
CREATE TABLE IF NOT EXISTS report_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    from_status VARCHAR(30),
    to_status VARCHAR(30) NOT NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 9. Assignments table
CREATE TABLE IF NOT EXISTS assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    assignee_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    assigned_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    instructions TEXT,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    accepted_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- 10. Collection schedules table
CREATE TABLE IF NOT EXISTS collection_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID NOT NULL REFERENCES regions(id) ON DELETE RESTRICT,
    responsible_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(150) NOT NULL,
    scheduled_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'planned'
      CHECK (status IN ('planned','in_progress','completed','cancelled')),
    notes TEXT,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 11. Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(160) NOT NULL,
    body TEXT NOT NULL,
    target_type VARCHAR(40),
    target_id UUID,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 12. Audit logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(80) NOT NULL,
    entity_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 13. Optional waste banks (P2)
CREATE TABLE IF NOT EXISTS waste_banks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID REFERENCES regions(id) ON DELETE SET NULL,
    name VARCHAR(160) NOT NULL,
    address TEXT,
    contact_phone VARCHAR(30),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 14. Waste bank transactions (P2)
CREATE TABLE IF NOT EXISTS waste_bank_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    waste_bank_id UUID NOT NULL REFERENCES waste_banks(id) ON DELETE RESTRICT,
    customer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    waste_type VARCHAR(100) NOT NULL,
    weight_kg NUMERIC(10,3) NOT NULL CHECK (weight_kg > 0),
    unit_price NUMERIC(14,2) CHECK (unit_price >= 0),
    recorded_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    notes TEXT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_reports_status_submitted
    ON reports(status, submitted_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_reporter_submitted
    ON reports(reporter_id, submitted_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_region_submitted
    ON reports(region_id, submitted_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_category 
    ON reports(category_id);
CREATE INDEX IF NOT EXISTS idx_history_report_created
    ON report_status_history(report_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_assignments_assignee_active
    ON assignments(assignee_id, is_active);
CREATE INDEX IF NOT EXISTS idx_notifications_user_created
    ON notifications(user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_entity
    ON audit_logs(entity_type, entity_id, created_at DESC);
