---
name: kaneo
description: Use the `kaneo` CLI to manage Kaneo project management data (Kaneo Cloud or self-hosted, server 2.49+) — workspaces, projects, tasks by ticket ID (KAN-12), columns, labels, comments, relations, links and pull requests, custom fields, recurring tasks, bulk changes, time tracking, notifications, workflow rules, and search.
---

# Kaneo CLI

`kaneo` is a non-interactive CLI for [Kaneo](https://kaneo.app/). Output is plain text: data on stdout, confirmations/warnings on stderr. List rows end with the entity ID. Errors print `Error: <action>: <status>: <message> (<CODE>)` plus a `Hint:` line, and exit with code 1.

## Setup

```bash
kaneo login <api-key> [--instance <hostname>]   # default instance: cloud.kaneo.app; verifies the key
kaneo whoami                                     # check who you are
kaneo workspace list                             # find workspace IDs (and your role)
kaneo set -w <workspace-id> [--project <project-id>] [--global]
```

Context resolution (highest first): `-w`/`-p` flags → `KANEO_WORKSPACE`/`KANEO_PROJECT` → `.kaneo-conf.json` walked up from the current directory → `~/.config/.kaneo-conf.json`. `KANEO_API_KEY` and `KANEO_INSTANCE` override stored credentials.

## Conventions

- `<task>` accepts a **ticket ID** (`KAN-12`: project slug + number, resolved in the active workspace) or a raw task ID. Prefer ticket IDs.
- Task `status` is a **column slug** (`to-do`, `in-progress`, `done`, …). Run `kaneo column list` to see slugs; new tasks default to `planned`.
- Priorities: `no-priority`, `low`, `medium`, `high`, `urgent`.
- `<user>` arguments accept a user ID or `me`.
- Destructive commands need `--force`; without it they print a note and do nothing.
- Dates are ISO (`2026-11-10` or full RFC 3339). `none` clears `--due-date`/`--start-date` on `task update`.
- Rate limits are retried automatically; on `RATE_LIMITED` wait and retry later.

## Workspaces, projects, columns, labels

```
kaneo workspace list | get [<id>] | members [--project-id <id>] | activity
kaneo project list [--include-archived] | get <id>
kaneo project create --name <n> [--slug <KEY>] [--icon <Lucide>] [--description <md>]
kaneo project update <id> [--name] [--slug] [--icon] [--description] [--is-public true|false]
kaneo project archive|unarchive <id>
kaneo project move <id> --to <workspace-id> --force
kaneo project reorder --order <id,id,...>
kaneo project delete <id> --force

kaneo column list [--project-id <id>]                # name (slug) id [final]
kaneo column create --name <n> [--icon] [--color <hex>] [--is-final]
kaneo column update <id> [--name] [--icon] [--color] [--is-final true|false]
kaneo column reorder --order <id,id,...>
kaneo column delete <id> --force                      # column must be empty

kaneo label list | get <id>
kaneo label create --name <n> [--color <hex>]
kaneo label update <id> [--name] [--color]
kaneo label delete <id> --force
```

## Tasks

```
kaneo task list [--project-id <id>] [--status <slug>] [--priority <p>] [--assignee <user>]
                [--due-before <date>] [--due-after <date>]
                [--sort-by created-at|priority|due-date|position|title|number] [--order asc|desc]
                [--limit <n> (default 50) | --all] [--include-done]
kaneo task mine [--workspace-id <id>] [--count]       # open tasks assigned to you
kaneo task get <task>
kaneo task create --title <t> [--project-id] [--description <md>] [--priority <p>]
                  [--status <slug>] [--due-date] [--start-date] [--assignee <user>]
                  [--repeat daily|weekly|monthly|yearly [--every N] [--on mon,thu] [--time-zone <IANA>]]
kaneo task update <task> [--title] [--description] [--priority] [--status <slug>]
                  [--due-date <date|none>] [--start-date <date|none>] [--assignee <user>]
kaneo task status <task> <slug>
kaneo task priority <task> <p>
kaneo task assign <task> [--user-id <user>]           # omit --user-id to unassign
kaneo task move <task> <project-id> [--status <slug>]
kaneo task repeat <task> <frequency> [--every N] [--on mon,thu] [--time-zone <IANA>] | --stop
kaneo task duplicate <task> [--title <t>]
kaneo task bulk <task>... (--status <slug> | --priority <p> | --assignee <user> | --unassign
                | --due <date|none> | --add-label <label-id> | --remove-label <label-id> | --delete --force)
kaneo task export [--project-id] [-o <file>]
kaneo task import <file|-> [--project-id] [--force]   # dry run without --force
kaneo task delete <task> --force
```

`task list` hides tasks in final columns and archived tasks unless `--include-done`; the footer shows `<shown> shown, <total> matching`.

### Task sub-resources

```
kaneo task comment list <task> | add <task> <text> | update <comment-id> <text> | delete <comment-id>
kaneo task label list <task>                          # task label IDs
kaneo task label attach <task> <workspace-label-id>
kaneo task label detach <task-label-id>
kaneo task relation list <task>
kaneo task relation create <source-task> <target-task> subtask|blocks|related
kaneo task relation delete <relation-id>
kaneo task link list <task>
kaneo task link add <task> <url> [--title <t>] [--pr [--state <state>]]
kaneo task link update <task> <link-id> [--title] [--state]
kaneo task link remove <task> <link-id> --force
kaneo task field <task>                               # show custom field values
kaneo task field <task> <field-name-or-id> <value>    # multiselect: comma-separated
kaneo task activity <task> [--limit N]
```

## Custom fields and workflow rules

```
kaneo field list [--project-id] [--all]
kaneo field create --name <n> --type text|number|date|dropdown|boolean|multiselect
                   [--required] [--default <v>] [--option <o>]...
kaneo field update <id> [--name] [--type] [--required true|false] [--default <v> | --clear-default]
                   [--option <o>]... [--hide-option <o>]...   # option lists replace the current ones
kaneo field reorder --order <id,id,...>
kaneo field delete <id> --force

kaneo workflow list [--project-id]
kaneo workflow set <integration> branch_push|pr_opened|pr_merged|issue_opened|issue_closed|issue_reopened <column-id-or-slug>
kaneo workflow delete <rule-id> --force
```

## Time tracking

```
kaneo time start <task> [-m <note>]
kaneo time stop <task>
kaneo time log <task> <duration> [--date today|yesterday|YYYY-MM-DD] [-m <note>]   # 1h30m, 45m, 1.5h, 90
kaneo time list <task>
kaneo time edit <entry-id> [--duration <d>] [--start <rfc3339>] [-m <note>]
```

## Notifications and search

```
kaneo notification list [--unread]                    # alias: kaneo inbox
kaneo notification read <id>... | unread <id> | read-all | delete <id>
kaneo notification clear [--read-only] --force

kaneo search <query> [--type all|tasks|projects|workspaces|comments|activities] [--project-id <id>] [--limit <n>]
```

## Maintenance

```
kaneo upgrade [--force] [--version <vX.Y.Z>]
kaneo skill                                           # print this skill
kaneo install-skill [-g] [-a <agent>...] [-y] [--copy] [--runner auto|npx|pnpm|bun]
```

## Patterns for agents

1. Resolve context first: `kaneo whoami`, `kaneo workspace list`, `kaneo project list`, `kaneo column list`.
2. Address tasks by ticket ID from `task list` / `task mine` / `search` output (first column).
3. Use `task status <task> <slug>` to move tasks; use `task bulk` for many tasks at once.
4. Read the `Hint:` line on errors: `NOT_FOUND` → list IDs again; `MISSING_PERMISSION` → the key's role lacks rights; `VALIDATION_ERROR` lists each invalid field.
5. Never pass `--force` unless the user asked for the destructive action.
