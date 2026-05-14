use crate::cli::{InstallSkillAgent, InstallSkillScope};

const SKILL_CONTENT: &str = r#"---
name: kaneo
description: Skill for the Kaneo CLI — a minimalist, open-source project management and task tracker. Covers authentication, workspaces, projects, tasks, columns, labels, comments, search, and all supported commands for both Kaneo Cloud and self-hosted instances.
---

# Kaneo CLI

## Overview

`kaneo` is a CLI for [Kaneo](https://kaneo.app/) project management.

## Installation

```bash
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/EzyGang/kaneo-cli/main/install.sh | sh

 irm https://raw.githubusercontent.com/EzyGang/kaneo-cli/main/install.ps1 | iex
```

## Authentication

```
kaneo login <your-api-key> [--instance <hostname>]
```

- Default instance: `cloud.kaneo.app`
- API key is encrypted locally (AES-256-GCM)
- Config stored at `~/.config/.kaneo-conf.json`

## Output Modes

All output is plain text designed to be both human and machine readable. Errors include a `Hint:` line explaining how to fix the issue.

## Global Flags

| Flag | Env | Description |
|------|-----|-------------|
| `-w, --workspace <id>` | `KANEO_WORKSPACE` | Override workspace ID |
| `-p, --project <id>` | `KANEO_PROJECT` | Override project ID |

## Context Resolution Order

1. CLI flags (`-w`, `-p`)
2. Environment variables (`KANEO_WORKSPACE`, `KANEO_PROJECT`, `KANEO_INSTANCE`)
3. `.kaneo-conf.json` walk-up from CWD to `$HOME`
4. `~/.config/.kaneo-conf.json` global config

Authentication is only through `kaneo login`

## Commands

### Authentication

```
kaneo login <api-key>                    Authenticate with an API key
  [--instance <hostname>]                Instance (default: cloud.kaneo.app)

kaneo logout                            Remove stored credentials
```

### Local Context

```
kaneo set -w <workspace-id>            Write local .kaneo-conf.json
  [-p <project-id>] [--global]

kaneo unset                              Remove pinned config
  [--workspace] [--project] [--global]
```

### Projects (`kaneo project` or `kaneo proj`)

```
kaneo {project|proj} list                List all projects in a workspace
  [--workspace-id <id>]
  [--include-archived]

kaneo {project|proj} get <id>            Get project details

kaneo {project|proj} create              Create a new project
  --name <name>
  --workspace-id <id>
  [--slug <slug>]
  [--icon <emoji>]
  [--description <text>]

kaneo {project|proj} update <id>         Update project fields
  [--name <name>] [--slug <slug>]
  [--icon <emoji>] [--description <text>]
  [--is-public]

kaneo {project|proj} [rm] delete <id>    Delete project
  [--force]

kaneo {project|proj} archive <id>        Archive project

kaneo {project|proj} unarchive <id>      Unarchive project
```

### Tasks (`kaneo task`, no shortcut)

```
kaneo task [ls] list                     List tasks
  [--project-id <id>]
  [--status <status>]
  [--priority <priority>]
  [--limit <n>]

kaneo task get <id>                      Get task details

kaneo task create                        Create a new task
  --title <title>
  --project-id <id>
  [--description <text>]
  [--priority urgent|high|medium|low|none]
  [--status <status>]
  [--due-date <date>]
  [--start-date <date>]
  [--assignee <user-id>]

kaneo task update <id>                   Update task fields
  [--title <title>] [--description <text>]
  [--priority <p>] [--status <s>]
  [--due-date <date>] [--start-date <date>]
  [--assignee <user-id>]

kaneo task [rm] delete <id>              Delete task
  [--force]

kaneo task status <id> <status>          Set task status

kaneo task priority <id> <priority>      Set task priority

kaneo task assign <id> [--user-id <id>]  Assign/unassign task

kaneo task move <id> <project-id>        Move task to another project
```

### Task Comments (`kaneo task comment`)

```
kaneo task comment [ls] list <task-id>   List comments on a task

kaneo task comment add <task-id> <text>  Add a comment to a task

kaneo task comment update <id> <text>    Update a comment

kaneo task comment [rm] delete <id>      Delete a comment
```

### Task Labels (`kaneo task label`)

```
kaneo task label [ls] list <task-id>     List labels on a task

kaneo task label attach <task-id> <label-id>   Attach label to task

kaneo task label detach <task-id> <label-id>   Detach label from task
```

### Columns (`kaneo column` or `kaneo col`)

```
kaneo {column|col} [ls] list             List columns
  [--project-id <id>]

kaneo {column|col} create                Create a column
  --name <name>
  --project-id <id>
  [--icon <emoji>] [--color <hex>]
  [--is-final]

kaneo {column|col} update <id>           Update a column
  [--name <name>] [--icon <emoji>]
  [--color <hex>] [--is-final]

kaneo {column|col} [rm] delete <id>      Delete a column
  [--force]

kaneo {column|col} reorder               Reorder columns
  --project-id <id>
  --order id1,id2,id3
```

### Labels (`kaneo label`)

```
kaneo label [ls] list              List workspace labels
  [--workspace-id <id>]

kaneo label get <id>               Get label details

kaneo label create                 Create a label
  --name <name>
  --workspace-id <id>
  [--color <hex>]

kaneo label update <id>            Update a label
  [--name <name>] [--color <hex>]

kaneo label [rm] delete <id>       Delete a label
  [--force]
```

### Search (`kaneo search`)

```
kaneo search <query>                 Search across workspaces
  [--type all|tasks|projects|comments]
  [--project-id <id>]
  [--limit <n>]
```

### Self-Update

```
kaneo upgrade                             Upgrade to latest version
  [--force] [--version <vX.Y.Z>]
```

### Agent Skill Installation

```
kaneo install-skill                     Install skill for agent
  --agent opencode|claude|codex
  --scope global|local
```

## Common Patterns for Agents

1. **List before creating**: Use `list` commands to find IDs
2. **Error handling**: Errors include a `Hint:` line explaining how to fix
3. **Deletion safety**: `delete` commands require `--force` to confirm
4. **No interactivity**: All commands are non-interactive, pure I/O
"#;

pub fn run(agent: InstallSkillAgent, scope: InstallSkillScope) -> anyhow::Result<()> {
    let global = matches!(scope, InstallSkillScope::Global);
    let target_dir = match (agent, global) {
        (InstallSkillAgent::Opencode, true) | (InstallSkillAgent::Codex, true) => {
            let home = dirs::home_dir().unwrap_or_default();
            home.join(".config")
                .join("opencode")
                .join("skills")
                .join("kaneo")
        }
        (InstallSkillAgent::Opencode, false) | (InstallSkillAgent::Codex, false) => {
            let cwd = std::env::current_dir()?;
            cwd.join(".agents").join("skills").join("kaneo")
        }
        (InstallSkillAgent::Claude, true) => {
            let home = dirs::home_dir().unwrap_or_default();
            home.join(".claude").join("skills").join("kaneo")
        }
        (InstallSkillAgent::Claude, false) => {
            let cwd = std::env::current_dir()?;
            cwd.join(".claude").join("skills").join("kaneo")
        }
    };

    std::fs::create_dir_all(&target_dir)?;
    let file_path = target_dir.join("SKILL.md");
    std::fs::write(&file_path, SKILL_CONTENT)?;

    let file_path_str = file_path.display();
    eprintln!("  ✓ Installed skill to {file_path_str}");

    Ok(())
}
