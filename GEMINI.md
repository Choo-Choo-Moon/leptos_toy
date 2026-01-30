# GEMINI.md

This file serves as the primary entry point and context guide for Gemini when working on the **PhotoVault** project.

## 🚨 Critical Instructions

**Before performing any code modification or architectural decision, you MUST reference the following documents:**

### 1. Project Overview & Commands (`CLAUDE.md`)
*   **Location**: `./CLAUDE.md`
*   **Purpose**: Builds, testing, running the server, database setup, and high-level architecture.
*   **Mandate**: Use the commands listed here for all development tasks. Do not assume standard cargo commands without checking here first (e.g., using `trunk` for the frontend).

### 2. Coding Standards & Architecture (`docs/DEVELOPMENT_GUIDELINES.md`)
*   **Location**: `./docs/DEVELOPMENT_GUIDELINES.md`
*   **Purpose**: **Clean Architecture** rules, Naming Conventions, File Structure, and Code Quality standards.
*   **Mandate**:
    *   Strictly adhere to the **30-line limit** for functions.
    *   Follow the folder structure: `domain` -> `application` -> `presentation` -> `infrastructure`.
    *   Use the specified naming conventions (snake_case mod/fn, PascalCase structs).
    *   Apply Leptos-specific patterns (Signals, Resources) as defined.

### 3. Service Specifications (`docs/SERVICE_PLANNING.md`)
*   **Location**: `./docs/SERVICE_PLANNING.md`
*   **Purpose**: Business logic, feature requirements, user personas, and roadmap.
*   **Mandate**: Align all feature implementations with the "Core Features" and "User Personas" defined here.

## Quick Context Summary

*   **Project**: PhotoVault (AI-powered Photo Gallery)
*   **Tech Stack**:
    *   **Frontend**: Leptos (Rust/WASM) + TailwindCSS v4
    *   **Build**: Trunk
    *   **Database**: PostgreSQL + PostGIS + pg_trgm
    *   **Storage**: JSONB for Exif metadata
*   **Current Phase**: MVP Development (Focus on UI skeleton and Metadata extraction)

---
*Always verify existing files before creating new ones to avoid duplication and maintain consistency with the guidelines linked above.*
