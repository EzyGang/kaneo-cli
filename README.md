# kaneo

[![Checks](https://github.com/EzyGang/kaneo-cli/actions/workflows/checks.yml/badge.svg)](https://github.com/EzyGang/kaneo-cli/actions/workflows/checks.yml)
[![Latest Release](https://img.shields.io/github/v/release/EzyGang/kaneo-cli?color=6366f1)](https://github.com/EzyGang/kaneo-cli/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A minimalist CLI for [Kaneo](https://kaneo.app) project management. Manage projects, tasks, columns, labels, custom fields, time, notifications, and more from your terminal.

Requires a Kaneo server **2.49.0 or newer** (Kaneo Cloud or self-hosted). See [CHANGELOG.md](CHANGELOG.md).

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
cargo install --git https://github.com/EzyGang/kaneo-cli --locked kaneo-cli
```

### Build from source

```bash
git clone https://github.com/EzyGang/kaneo-cli.git
cd kaneo-cli
cargo build --release
```

### Agent skill

The skill in [`skills/kaneo/SKILL.md`](skills/kaneo/SKILL.md) teaches AI agents how to use this CLI. Install it with the [`skills`](https://www.npmjs.com/package/skills) installer through npm, pnpm, or bun. The repository also contains contributor skills, so `--skill kaneo` selects only the user skill.

```bash
npx skills@latest add EzyGang/kaneo-cli --skill kaneo
pnpm dlx skills@latest add EzyGang/kaneo-cli --skill kaneo
bunx skills@latest add EzyGang/kaneo-cli --skill kaneo
```

The CLI can also install the skill that matches its own version — see [`kaneo install-skill`](#kaneo-install-skill).

## Quick start

```bash
# Authenticate (the key is verified against the server)
kaneo login <your-api-key> --instance your-instance.example.com

# Find and pin your default workspace globally
kaneo workspace list
kaneo set -w <workspace-id> --global

# Pin a project for the current directory, then work with tasks by ticket ID
kaneo set -w <workspace-id> --project <project-id>
kaneo task list
kaneo task get KAN-12
```

## Configuration

API keys are stored at `~/.config/.kaneo-conf.json`.

Workspace and project IDs can be persisted per directory in `.kaneo-conf.json` or globally with `--global`.

**Resolution order (highest priority first):**

1. CLI flags (`-w`, `-p`)
2. Environment variables (`KANEO_WORKSPACE`, `KANEO_PROJECT`)
3. Local `.kaneo-conf.json` (walked up from current directory)
4. Global `~/.config/.kaneo-conf.json`

### Task references

Every `<task>` argument accepts a **ticket ID** such as `KAN-12` (project slug + task number, resolved within the active workspace) or a raw task ID.

### Errors and rate limits

Errors print the server's message and stable code (for example `NOT_FOUND`, `VALIDATION_ERROR` with each failing field, `MISSING_PERMISSION` with the missing permissions) followed by a `Hint:` line. Rate-limited requests are retried automatically after the server's `Retry-After` delay (up to 60 seconds, announced on stderr).

## Global flags

| Flag | Env | Description |
|------|-----|-----------|
| `-w`, `--workspace` | `KANEO_WORKSPACE` | Override workspace ID |
| `-p`, `--project` | `KANEO_PROJECT` | Override project ID |

## Commands

Destructive commands (`delete`, `move` between workspaces, bulk delete, …) require `--force`. List rows end with the entity ID.

### Account and context

| Command | Description |
|---------|-------------|
| `kaneo login <api-key> [--instance <host>]` | Store and verify an API key (default instance: `cloud.kaneo.app`) |
| `kaneo logout` | Remove stored credentials |
| `kaneo whoami` | Show the authenticated user and instance |
| `kaneo set [-w <id>] [--project <id>] [--global]` | Pin workspace/project in the current directory (or globally) |
| `kaneo unset [--workspace] [--project] [--global]` | Remove pins |

### Workspaces (`kaneo workspace`, alias `ws`)

| Command | Description |
|---------|-------------|
| `workspace list` | Workspaces you can read, with your role |
| `workspace get [<workspace-id>]` | Workspace details (defaults to the active workspace) |
| `workspace members [--project-id <id>]` | Members and roles; `--project-id` limits to members with project access |
| `workspace activity` | Recent task events across the workspace |

### Projects (`kaneo project`, alias `proj`)

| Command | Description |
|---------|-------------|
| `project list [--workspace-id <id>] [--include-archived]` | List projects |
| `project get <id>` | Project details |
| `project create --name <n> [--workspace-id] [--slug] [--icon] [--description]` | Create a project; the slug prefixes ticket IDs |
| `project update <id> [--name] [--icon] [--slug] [--description] [--is-public true\|false]` | Update a project |
| `project archive <id>` / `project unarchive <id>` | Archive or restore |
| `project move <id> --to <workspace-id> --force` | Move a project and its tasks to another workspace |
| `project reorder --order <id,id,...> [--workspace-id]` | Set the sidebar order |
| `project delete <id> --force` | Delete permanently |

### Tasks (`kaneo task`)

| Command | Description |
|---------|-------------|
| `task list [flags]` | List a project's tasks across every page (see below) |
| `task mine [--workspace-id <id>] [--count]` | Open tasks assigned to you across a workspace |
| `task get <task>` | Task details, recurrence, subtask progress, description |
| `task create --title <t> [--project-id] [--description] [--priority] [--status <slug>] [--due-date] [--start-date] [--assignee <id\|me>] [--repeat <freq> ...]` | Create a task (default priority `low`, status `planned`) |
| `task update <task> [--title] [--description] [--priority] [--status] [--due-date <d\|none>] [--start-date <d\|none>] [--assignee <id\|me>]` | Update fields |
| `task status <task> <slug>` | Move to a column by slug |
| `task priority <task> <no-priority\|low\|medium\|high\|urgent>` | Set priority |
| `task assign <task> [--user-id <id\|me>]` | Assign, or unassign without `--user-id` |
| `task move <task> <project-id> [--status <slug>]` | Move to another project in the same workspace |
| `task repeat <task> <daily\|weekly\|monthly\|yearly> [--every N] [--on mon,thu] [--time-zone <IANA>]` | Repeat the task after completion; `--stop` ends it |
| `task duplicate <task> [--title <t>]` | Copy a task (fields, labels, custom fields) |
| `task bulk <task>... --status\|--priority\|--assignee\|--unassign\|--due\|--add-label\|--remove-label\|--delete` | Apply one change to many tasks atomically |
| `task export [--project-id] [-o <file>]` / `task import <file\|-> [--project-id] [--force]` | Export or import tasks as JSON (import is a dry run without `--force`) |
| `task delete <task> --force` | Delete a task |

**`task list` flags**

| Flag | Description |
|------|-------------|
| `--project-id <id>` | Project (defaults to the pinned project) |
| `--status <slug>` | Only tasks in this column |
| `--priority <p>` | Only tasks with this priority |
| `--assignee <id\|me>` | Only tasks assigned to this user |
| `--due-before <date>` / `--due-after <date>` | Due date window |
| `--sort-by <created-at\|priority\|due-date\|position\|title\|number>` | Sort field |
| `--order <asc\|desc>` | Sort direction |
| `--limit <n>` | Maximum tasks to show (default `50`) |
| `--all` | Show every matching task |
| `--include-done` | Include tasks in final columns and archived tasks |

**Task sub-resources**

| Command | Description |
|---------|-------------|
| `task comment list <task>` | Comments, oldest first |
| `task comment add <task> <text>` / `update <id> <text>` / `delete <id>` | Manage comments |
| `task label list <task>` | Labels on a task (task label IDs) |
| `task label attach <task> <label-id>` | Attach a workspace label |
| `task label detach <task-label-id>` | Detach a label (ID from `task label list`) |
| `task relation list <task>` | Relations with task summaries |
| `task relation create <source> <target> <subtask\|blocks\|related>` / `delete <id>` | Manage relations |
| `task link list <task>` | External links and pull requests |
| `task link add <task> <url> [--title] [--pr --state <s>]` / `update <task> <link-id>` / `remove <task> <link-id> --force` | Manage links |
| `task field <task> [<field> <value>]` | Show custom field values, or set one by field name or ID |
| `task activity <task> [--limit N]` | Activity feed, newest first |

### Time tracking (`kaneo time`)

| Command | Description |
|---------|-------------|
| `time start <task> [-m <note>]` | Start a running entry |
| `time stop <task>` | Stop the running entry |
| `time log <task> <duration> [--date <day>] [-m <note>]` | Log a finished entry (`1h30m`, `45m`, `1.5h`, `90`) |
| `time list <task>` | Entries and total time |
| `time edit <entry-id> [--duration] [--start] [-m]` | Change an entry |

### Columns (`kaneo column`, alias `col`)

| Command | Description |
|---------|-------------|
| `column list [--project-id]` | Columns with their slugs |
| `column create --name <n> [--project-id] [--icon] [--color] [--is-final]` | Add a column |
| `column update <id> [--name] [--icon] [--color] [--is-final true\|false]` | Update a column |
| `column reorder [--project-id] --order <id,id,...>` | Set the column order |
| `column delete <id> --force` | Delete an empty column |

### Labels (`kaneo label`)

| Command | Description |
|---------|-------------|
| `label list [--workspace-id]` | Workspace labels |
| `label get <id>` | Label details |
| `label create --name <n> [--workspace-id] [--color <hex>]` | Create a label (default color `#6366f1`) |
| `label update <id> [--name] [--color]` | Update a label |
| `label delete <id> --force` | Delete a label from the workspace and every task |

### Custom fields (`kaneo field`)

| Command | Description |
|---------|-------------|
| `field list [--project-id] [--all]` | Field definitions (`--all` shows hidden options) |
| `field create --name <n> --type <text\|number\|date\|dropdown\|boolean\|multiselect> [--required] [--default] [--option <o>]...` | Create a field |
| `field update <id> [--name] [--type] [--required true\|false] [--default <v>\|--clear-default] [--option <o>]... [--hide-option <o>]...` | Update a field (option lists replace the current ones) |
| `field reorder [--project-id] --order <id,id,...>` | Set the field order |
| `field delete <id> --force` | Delete a field |

### Workflow rules (`kaneo workflow`)

| Command | Description |
|---------|-------------|
| `workflow list [--project-id]` | Rules that move tasks on integration events |
| `workflow set <integration> <event> <column-id-or-slug> [--project-id]` | Create or update a rule (events: `branch_push`, `pr_opened`, `pr_merged`, `issue_opened`, `issue_closed`, `issue_reopened`) |
| `workflow delete <id> --force` | Delete a rule |

### Notifications (`kaneo notification`, alias `inbox`)

| Command | Description |
|---------|-------------|
| `notification list [--workspace-id] [--unread]` | Your latest notifications |
| `notification read <id>...` / `unread <id>` / `read-all [--workspace-id]` | Change read state |
| `notification delete <id>` / `clear [--read-only] [--workspace-id] --force` | Remove notifications |

### Search

`kaneo search <query> [--type all|tasks|projects|workspaces|comments|activities] [--project-id <id>] [--limit <n>]` searches the active workspace and groups results by type.

### Maintenance

| Command | Description |
|---------|-------------|
| `kaneo upgrade [--force] [--version <vX.Y.Z>]` | Install the latest (or a specific) release from GitHub |
| `kaneo skill` | Print the embedded agent skill |
| `kaneo install-skill` | Install the agent skill (see below) |

### `kaneo install-skill`

Writes the skill embedded in this binary to `<user cache>/kaneo/skill/skills/kaneo/SKILL.md` and runs `npx --yes skills@latest add <that folder>` (or `pnpm dlx` / `bun x`), so the installed skill always matches the installed CLI.

```bash
kaneo install-skill                          # asks for the agents and the scope
kaneo install-skill -g -a claude-code -y     # Claude Code, for this user
kaneo install-skill -a '*' -y                # every agent, in this project
kaneo install-skill --runner bun             # use bun
```

| Option | Passed to `skills add` as | Effect |
|--------|---------------------------|--------|
| `-g`, `--global` | `--global` | Install for the current user instead of the current project |
| `-a`, `--agent <AGENT>...` | `--agent` | Agents such as `claude-code`, `codex`, `opencode`, `cursor`, or `'*'` |
| `-y`, `--yes` | `--yes` | Skip confirmations |
| `--copy` | `--copy` | Copy files instead of linking them |
| `--runner auto\|npx\|pnpm\|bun` | — | Package runner; `auto` tries npx, then pnpm, then bun |
| `-- <options>` | as written | Extra installer options |

Without Node.js or bun, save the skill yourself:

```bash
mkdir -p ~/.agents/skills/kaneo
kaneo skill > ~/.agents/skills/kaneo/SKILL.md
```

## Environment variables

| Variable | Description |
|----------|-----------|
| `KANEO_API_KEY` | API key (plaintext, fallback for scripted use) |
| `KANEO_INSTANCE` | Instance hostname |
| `KANEO_WORKSPACE` | Default workspace ID |
| `KANEO_PROJECT` | Default project ID |

## License

MIT
