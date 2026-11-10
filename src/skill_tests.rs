use std::ffi::OsString;
use std::path::Path;

use crate::cli::args::{InstallSkillArgs, Runner};
use crate::skill::{runners, skills_add_args};

fn args() -> InstallSkillArgs {
    InstallSkillArgs {
        global: false,
        agents: Vec::new(),
        yes: false,
        copy: false,
        runner: Runner::Auto,
        extra: Vec::new(),
    }
}

fn strings(args: &[OsString]) -> Vec<String> {
    args.iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn bare_install_only_names_the_source() {
    let out = skills_add_args(Path::new("src-dir"), &args());
    assert_eq!(strings(&out), ["skills@latest", "add", "src-dir"]);
}

#[test]
fn flags_agents_and_extra_options_are_forwarded_in_order() {
    let options = InstallSkillArgs {
        global: true,
        agents: vec!["claude-code".to_owned(), "codex".to_owned()],
        yes: true,
        copy: true,
        extra: vec!["--json".to_owned()],
        ..args()
    };
    let out = skills_add_args(Path::new("src-dir"), &options);
    assert_eq!(
        strings(&out),
        [
            "skills@latest",
            "add",
            "src-dir",
            "--global",
            "--yes",
            "--copy",
            "--agent",
            "claude-code",
            "codex",
            "--json"
        ]
    );
}

#[test]
fn auto_tries_npx_then_pnpm_then_bun() {
    assert_eq!(
        runners(Runner::Auto),
        [Runner::Npx, Runner::Pnpm, Runner::Bun]
    );
    assert_eq!(runners(Runner::Bun), [Runner::Bun]);
    assert_eq!(runners(Runner::Pnpm), [Runner::Pnpm]);
}

#[test]
fn embedded_skill_matches_the_skills_folder() {
    let on_disk = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/skills/kaneo/SKILL.md"
    ))
    .unwrap_or_default();
    assert_eq!(crate::skill::SKILL, on_disk);
    assert!(crate::skill::SKILL.starts_with("---\nname: kaneo\n"));
}
