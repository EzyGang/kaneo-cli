# kaneo

[![Checks](https://github.com/EzyGang/kaneo-cli/actions/workflows/checks.yml/badge.svg)](https://github.com/EzyGang/kaneo-cli/actions/workflows/checks.yml)
[![Latest Release](https://img.shields.io/github/v/release/EzyGang/kaneo-cli?color=6366f1)](https://github.com/EzyGang/kaneo-cli/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A minimalist CLI for [Kaneo](https://kaneo.app) project management. Manage projects, tasks, columns, labels, and comments from your terminal.

## Installation

### Prebuilt binaries

**Linux / macOS**

```bash
curl -fsSL https://raw.githubusercontent.com/EzyGang/kaneo-cli/main/install.sh | bash
```

**Uninstall**
```bash
curl -fsSL https://raw.githubusercontent.com/EzyGang/kaneo-cli/main/install.sh | bash -s -- --uninstall
```

**Windows (PowerShell)**

```powershell
irm https://raw.githubusercontent.com/EzyGang/kaneo-cli/main/install.ps1 | iex
```

**Uninstall** (PowerShell)
```powershell
Remove-Item "$env:USERPROFILE\.local\bin\kaneo.exe" -Force
```

### Cargo (from source)

```bash
cargo install --git https://github.com/EzyGang/kaneo-cli kaneo
```

### Build from source

```bash
git clone https://github.com/EzyGang/kaneo-cli.git
cd kaneo-cli
cargo build --release
```

## Quick start

```bash
# Authenticate
kaneo login <your-api-key> --instance your-instance.example.com

# Pin your default workspace
kaneo set <workspace-id>
```

## Configuration

API keys are AES-256-GCM encrypted and stored at `~/.config/.kaneo-conf.json`.

Workspace and project IDs can be persisted per directory in `.kaneo-conf.json`.

**Resolution order (highest priority first):**

1. CLI flags (`-w`, `-p`)
2. Environment variables (`KANEO_WORKSPACE`, `KANEO_PROJECT`)
3. Local `.kaneo-conf.json` (walked up from current directory)
4. Global `~/.config/.kaneo-conf.json`

## Global flags

| Flag | Env | Description |
|------|-----|-----------|
| `-w`, `--workspace` | `KANEO_WORKSPACE` | Override workspace ID |
| `-p`, `--project` | `KANEO_PROJECT` | Override project ID |

## Commands

### `kaneo login <api-key>`

Store an API key to authenticate with Kaneo.

| Arg | Description |
|-----|-----------|
| `--instance` | Hostname (default: `cloud.kaneo.app`) |

### `kaneo logout`

Remove stored credentials.

### `kaneo set`

Pin workspace and optionally project in the current directory. Uses the same `-w`/`-p` flags as every other command.

| Arg | Description |
|-----|-----------|
| `-w`, `--workspace` | Workspace ID to pin |
| `-p`, `--project` | Project ID to pin |
| `--global` | Write to global config instead of current directory |

### `kaneo unset`

Remove pinned workspace/project IDs.

| Arg | Description |
|-----|-----------|
| `--workspace` | Remove only the workspace pin |
| `--project` | Remove only the project pin |
| `--global` | Remove from global config instead of current directory |

---

### `kaneo project` (alias `proj`)

#### `project list` (alias `ls`)

| Arg | Description |
|-----|-----------|
| `--workspace-id` | Workspace ID |
| `--include-archived` | Show archived projects |

#### `project get <id>`

#### `project create`

| Arg | Description |
|-----|-----------|
| `--name` | Project name (required) |
| `--workspace-id` | Workspace ID |
| `--slug` | URL slug |
| `--icon` | Icon name |
| `--description` | Project description |

#### `project update <id>`

| Arg | Description |
|-----|-----------|
| `--name` | New name |
| `--icon` | New icon |
| `--slug` | New slug |
| `--description` | New description |
| `--is-public` | Set to `true` or `false` |

#### `project archive <id>`

#### `project unarchive <id>`

#### `project delete <id>` (alias `rm`)

Requires `--force` to confirm.

---

### `kaneo task`

#### `task list` (alias `ls`)

| Arg | Description |
|-----|-----------|
| `--project-id` | Project ID |
| `--status` | Filter by status |
| `--priority` | Filter by priority |
| `--sort` | Sort field (default: `createdAt`) |
| `--limit` | Max results (default: `20`) |

#### `task get <id>`

#### `task create`

| Arg | Description |
|-----|-----------|
| `--title` | Task title (required) |
| `--project-id` | Project ID |
| `--description` | Description |
| `--priority` | `low`, `medium`, `high`, `urgent` |
| `--status` | Initial status |
| `--due-date` | Due date |
| `--start-date` | Start date |
| `--assignee` | User ID to assign |

#### `task update <id>`

| Arg | Description |
|-----|-----------|
| `--title` | New title |
| `--description` | New description |
| `--priority` | New priority |
| `--status` | New status |
| `--due-date` | New due date |
| `--start-date` | New start date |
| `--assignee` | New assignee user ID |

#### `task status <id> <status>`

Quick status change.

#### `task priority <id> <priority>`

Quick priority change.

#### `task assign <id>`

Assign or unassign a task.

| Arg | Description |
|-----|-----------|
| `--user-id` | User ID (omit to unassign) |

#### `task move <id> <project-id>`

Move a task to another project.

#### `task delete <id>` (alias `rm`)

Requires `--force` to confirm.

#### `task comment`

| Subcommand | Description |
|------------|-----------|
| `list <task-id>` | List comments on a task |
| `add <task-id> <content>` | Add a comment |
| `update <id> <content>` | Update a comment |
| `delete <id>` (alias `rm`) | Delete a comment |

#### `task label`

| Subcommand | Description |
|------------|-----------|
| `list <task-id>` | List labels on a task |
| `attach <task-id> <label-id>` | Attach a label |
| `detach <task-id> <label-id>` | Detach a label |

---

### `kaneo column` (alias `col`)

#### `column list` (alias `ls`)

| Arg | Description |
|-----|-----------|
| `--project-id` | Project ID |

#### `column create`

| Arg | Description |
|-----|-----------|
| `--name` | Column name (required) |
| `--project-id` | Project ID |
| `--icon` | Icon name |
| `--color` | Hex color |
| `--is-final` | Mark as final column |

#### `column update <id>`

| Arg | Description |
|-----|-----------|
| `--name` | New name |
| `--icon` | New icon |
| `--color` | New color |
| `--is-final` | Set to `true` or `false` |

#### `column delete <id>` (alias `rm`)

Requires `--force` to confirm.

#### `column reorder`

| Arg | Description |
|-----|-----------|
| `--project-id` | Project ID (required) |
| `--order` | Comma-separated column IDs in desired order |

---

### `kaneo label`

#### `label list` (alias `ls`)

| Arg | Description |
|-----|-----------|
| `--workspace-id` | Workspace ID |

#### `label get <id>`

#### `label create`

| Arg | Description |
|-----|-----------|
| `--name` | Label name (required) |
| `--workspace-id` | Workspace ID |
| `--color` | Hex color (default: `#6366f1`) |

#### `label update <id>`

| Arg | Description |
|-----|-----------|
| `--name` | New name |
| `--color` | New color |

#### `label delete <id>` (alias `rm`)

Requires `--force` to confirm.

---

### `kaneo search <query>`

Search across tasks, projects, and comments.

| Arg | Description |
|-----|-----------|
| `--type` | `all`, `tasks`, `projects`, or `comments` |
| `--project-id` | Restrict to a project |
| `--limit` | Max results (default: `10`) |

---

### `kaneo upgrade`

Download and install the latest version from GitHub releases.

| Arg | Description |
|-----|-----------|
| `--force` | Reinstall even if on latest |
| `--version` | Install a specific version (e.g., `v0.2.0`) |

### `kaneo install-skill`

Write a `SKILL.md` for AI agents (opencode, claude, codex).

| Arg | Description |
|-----|-----------|
| `--agent` | `opencode`, `claude`, or `codex` |
| `--scope` | `global` or `local` |

## Environment variables

| Variable | Description |
|----------|-----------|
| `KANEO_API_KEY` | API key (plaintext, fallback for scripted use) |
| `KANEO_INSTANCE` | Instance hostname |
| `KANEO_WORKSPACE` | Default workspace ID |
| `KANEO_PROJECT` | Default project ID |

## License

MIT
