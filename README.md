# Facebook Data Parser

A Rust CLI tool for searching and analyzing Facebook data exports.

## Features

- **Unified Search** - Search across all Facebook activity types with date range filtering
- **Activity Types** - Messages, posts, comments, reactions, events, groups
- **Attachment Filtering** - Find messages with photos, videos, files, or stickers
- **Path Display** - Show file paths for media attachments with emoji indicators

## Quick Start

```bash
# Build
cargo build --release

# Search all activity in date range
cargo run -- search --earliest 2021-11-25 --latest 2023-03-13

# Search specific types
cargo run -- search --types messages,comments --earliest 2022-01-01

# Find messages with attachments
cargo run -- search --types messages --has-attachments --show-paths
```

## Setup

1. Download your Facebook data export (JSON format)
2. Extract to project directory as `data/`
3. Expected structure:
   ```
   data/
   ├── your_activity_across_facebook/
   │   ├── messages/
   │   ├── posts/
   │   ├── comments_and_reactions/
   │   ├── events/
   │   └── groups/
   ├── connections/
   └── personal_information/
   ```

## Search Options

| Option | Description | Example |
|--------|-------------|---------|
| `--earliest` | Start date (YYYY-MM-DD) | `--earliest 2021-01-01` |
| `--latest` | End date (YYYY-MM-DD) | `--latest 2023-12-31` |
| `--types` | Filter by type(s) | `--types messages,posts` |
| `--has-attachments` | Messages with media only | `--has-attachments` |
| `--show-paths` | Show file paths | `--show-paths` |

### Supported Types
- `messages` - Direct messages and group chats
- `posts` - Your posts, check-ins, photos, videos
- `comments` - Comments on posts
- `reactions` - Likes and reactions
- `events` - Events you created or attended
- `groups` - Groups you admin

## Example Output

```
# Basic search
[2017-10-11 01:08:21] messages - Message from User1: Hi.
[2017-10-11 01:09:29] messages - Message from User2: sounds like a plan...

# With --show-paths
[2017-10-11 04:17:13] messages - Message from User2 (photo)
  📷 your_activity_across_facebook/messages/inbox/user/photos/image.jpg
[2017-10-11 04:20:59] messages - Message from User1 (sticker)
  🎨 your_activity_across_facebook/messages/stickers_used/sticker.png
```

## Legacy Commands

For backward compatibility, message reorganization commands are available:

```bash
# Reorganize images by date into output/{username}/{YYYY/MM}/
cargo run -- activity messages reorg-images

# Reorganize videos by date
cargo run -- activity messages reorg-videos

# Interactive message search (legacy)
cargo run -- activity messages search-messages
```

## Development

```bash
cargo build          # Build debug
cargo test           # Run tests
cargo clippy         # Lint
cargo fmt            # Format
```
