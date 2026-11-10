# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [0.2.0] — Unreleased

Kaneo compatibility: **≥ 2.49.0**.

### Breaking

- `task list --sort` is replaced by `--sort-by <created-at|priority|due-date|position|title|number>` and `--order <asc|desc>`; the server ignored `sort` since the v2.26 pagination change.
- `task list --limit` now defaults to `50` (server default) and `--all` lists every task.
- `task label detach` takes the task label ID only: `kaneo task label detach <task-label-id>` (see `kaneo task label list`).
- `install-skill` uses the npm [`skills`](https://www.npmjs.com/package/skills) installer: `--agent opencode|claude|codex` and `--scope` are replaced by `-g/--global`, `-a/--agent <AGENT>...`, `-y/--yes`, `--copy`, `--runner auto|npx|pnpm|bun`, and `-- <skills options>`.
- Priorities are validated: `no-priority`, `low`, `medium`, `high`, `urgent`.

### Added

- Every task argument accepts a ticket ID such as `KAN-12` as well as a task ID.
- `kaneo whoami`; `kaneo login` verifies the API key against `GET /user/me`.
- `workspace get`, `workspace members [--project-id]`, `workspace activity`.
- `project move <id> --to <workspace-id> --force` and `project reorder --order <ids>`.
- `task mine` — open tasks assigned to you across a workspace.
- `task repeat` and `task create --repeat …` for recurring tasks (Kaneo 2.48).
- `task duplicate`, `task bulk` (atomic multi-task changes), `task export`, `task import`.
- `task link list|add|update|remove`, including pull request links (Kaneo 2.49).
- `task field` and `kaneo field list|create|update|reorder|delete` for custom fields.
- `task activity` — a task's activity feed.
- `kaneo time start|stop|log|list|edit` for time tracking.
- `kaneo notification` (alias `inbox`): list, read, unread, read-all, delete, clear.
- `kaneo workflow list|set|delete` for integration workflow rules.
- `task list`: `--assignee <id|me>`, `--due-before`, `--due-after`, `--include-done` now also covers archived tasks; follows every page of the board.
- `task move --status <slug>` places the task in a column of the destination project.
- `task update --due-date none` / `--start-date none` clear dates.
- `search --type activities`; search results are grouped by type with ticket IDs.
- `kaneo skill` prints the embedded `SKILL.md`; the skill now lives in `skills/kaneo/SKILL.md` and can be installed with `npx skills@latest add EzyGang/kaneo-cli --skill kaneo`.
- API errors show the server's stable error code, validation issues, and missing permissions, with a matching hint.
- Rate-limited requests are retried after the server's `Retry-After` delay.

### Fixed

- `search` always printed "No results" because the API returns a flat, typed result list.
- `task list` showed only the first page (20 tasks) and never hid done tasks.
- `column list` printed `-` instead of the column slug.
- `task assign` without `--user-id` sent an empty string instead of `null` to unassign.
- `label delete` now repeats the request until the server finishes removing the label from every task (HTTP 202).
- `label update` sends both name and color as the API requires.
- `project create --description` was silently dropped by the API; it is now applied with a follow-up update.
- `workspace list` uses the public `GET /workspace` endpoint and shows your role.
- Errors such as 401, 403, and 429 are no longer reported as "not found".
- The update notice suggested older releases (such as v0.1.8 while running 0.1.9); it now only appears for newer versions.

## [0.1.9]

- Added `status` (slug) to columns and `workspace list`.
