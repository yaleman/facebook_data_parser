# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool for parsing and searching Facebook data exports. The tool processes JSON files from Facebook data dumps and provides:
- Unified search across all activity types (messages, posts, comments, reactions, events, groups)
- Date-range filtering with `--earliest` and `--latest` options
- Activity type filtering with `--types` option
- Message attachment filtering and path display
- Legacy message reorganization (reorg images/videos by date)

## Build and Test Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the binary
cargo run -- <subcommand>

# Run tests
cargo test

# Run a specific test
cargo test test_messagefileparser

# Check code (lint + clippy)
cargo clippy

# Format code
cargo fmt
```

## Architecture

### Data Model

The application expects Facebook data export in a specific folder structure:
- `data/` - Base directory containing all Facebook export data
- `data/your_activity_across_facebook/messages/` - Message data organized by conversation

### Core Components

**src/lib.rs** - CLI command structure and folder validation
- Two main commands: `search` (unified search) and `activity` (legacy message operations)
- `folder_checks()` validates expected Facebook export folder structure exists
- `Skippable` trait for checking if data folders are present

**src/search.rs** - Unified search functionality
- `SearchOptions` - Configures date range, activity types, attachment filters
- Searches across all activity types in parallel
- Filters results by date range and activity type
- Special handling for message attachments (photos/videos/files/stickers)

**src/activity/common.rs** - Activity trait system
- `ActivityItem` trait - Common interface for all activity types (timestamp, description, type)
- `ActivityType` enum - All supported types (Messages, Posts, Comments, Reactions, Events, Groups, Friends)
- `SearchResult` - Unified result wrapper with formatted display

**src/activity/{type}.rs** - Type-specific parsers
- Each module implements `ActivityItem` trait for its type
- Uses serde with strict `deny_unknown_fields` for complete data coverage
- Handles type-specific schemas (comments_v2, your_events_v2, etc.)
- Posts: `your_posts__check_ins__photos_and_videos_*.json`
- Comments: `comments.json` with `comments_v2` array
- Reactions: `likes_and_reactions_*.json` files
- Events: `your_events.json` with `your_events_v2` array
- Groups: `your_groups.json` with `groups_admined_v2` array
- Messages: `message_*.json` files with full message history

### Message File Schema

Message JSON files must match specific patterns:
- Filename: `message_[\d]+\.json`
- Contains: participants, messages array, title, thread_path, magic_words, etc.
- Messages include: sender, timestamps, content, media (photos/videos/files), reactions, shares

## Command Examples

### Search Command
```bash
# Search all activity between dates
cargo run -- search --earliest 2021-11-25 --latest 2023-03-13

# Search specific activity types
cargo run -- search --types messages,comments,posts

# Search messages with attachments only
cargo run -- search --types messages --has-attachments

# Show file paths for attachments
cargo run -- search --types messages --show-paths

# Combined filters
cargo run -- search --earliest 2017-10-11 --latest 2017-10-12 \
  --types messages --has-attachments --show-paths
```

### Legacy Activity Commands
```bash
# Reorganize images by date
cargo run -- activity messages reorg-images

# Search messages interactively
cargo run -- activity messages search-messages
```

## Development Notes

- All dates use chrono with UTC timezone (format: YYYY-MM-DD)
- Search results sorted chronologically
- Message attachments detected: photos, videos, files, stickers
- Attachment paths displayed with emoji indicators (📷🎥📄🎨)
- UTF-8 safe truncation for long content (uses char boundaries)
- Error handling uses custom `MagicError` enum with Generic and Skippable variants
- Interactive folder selection uses dialoguer with fuzzy matching
