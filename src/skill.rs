use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use crate::cli::args::{InstallSkillArgs, Runner};
use crate::errors::KaneoError;

/// The agent skill that explains this CLI, embedded so it always matches the binary.
pub const SKILL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/skills/kaneo/SKILL.md"
));

/// Prints the embedded `SKILL.md` to stdout.
pub fn print() -> Result<(), KaneoError> {
    io::stdout().write_all(SKILL.as_bytes())?;
    Ok(())
}

/// Installs the embedded skill with the [`skills`](https://www.npmjs.com/package/skills) installer.
///
/// The skill is written to a fixed folder in the user cache first, because the installer reads a
/// local folder and records its path in `skills-lock.json`.
pub fn install(args: &InstallSkillArgs) -> Result<(), KaneoError> {
    let source = write_source()?;
    let skills_args = skills_add_args(&source, args);
    for runner in runners(args.runner) {
        for program in programs(*runner) {
            match run(*runner, program, &skills_args) {
                Ok(status) if status.success() => return Ok(()),
                Ok(status) => {
                    return Err(KaneoError::Usage(format!(
                        "The skills installer failed ({status})"
                    )));
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => return Err(error.into()),
            }
        }
    }
    Err(KaneoError::Usage(
        "install-skill needs npx (Node.js), pnpm, or bun. Install one of them, or run \
         `kaneo skill > ~/.agents/skills/kaneo/SKILL.md`"
            .to_owned(),
    ))
}

/// Builds the arguments that follow the runner, starting with `skills@latest add <source>`.
#[must_use]
pub fn skills_add_args(source: &Path, args: &InstallSkillArgs) -> Vec<OsString> {
    let mut out: Vec<OsString> = vec!["skills@latest".into(), "add".into(), source.into()];
    let flags = [
        (args.global, "--global"),
        (args.yes, "--yes"),
        (args.copy, "--copy"),
    ];
    out.extend(
        flags
            .into_iter()
            .filter(|(set, _)| *set)
            .map(|(_, flag)| flag.into()),
    );
    if !args.agents.is_empty() {
        out.push("--agent".into());
        out.extend(args.agents.iter().map(OsString::from));
    }
    out.extend(args.extra.iter().map(OsString::from));
    out
}

/// Returns the runners to try, in order.
#[must_use]
pub fn runners(runner: Runner) -> &'static [Runner] {
    match runner {
        Runner::Auto => &[Runner::Npx, Runner::Pnpm, Runner::Bun],
        Runner::Npx => &[Runner::Npx],
        Runner::Pnpm => &[Runner::Pnpm],
        Runner::Bun => &[Runner::Bun],
    }
}

/// Returns the program names of a runner. On Windows, npx and npm-installed pnpm are `.cmd` scripts.
fn programs(runner: Runner) -> &'static [&'static str] {
    match (runner, cfg!(windows)) {
        (Runner::Npx, true) => &["npx.cmd"],
        (Runner::Npx, false) => &["npx"],
        (Runner::Pnpm, true) => &["pnpm.cmd", "pnpm"],
        (Runner::Pnpm, false) => &["pnpm"],
        (Runner::Bun | Runner::Auto, _) => &["bun"],
    }
}

/// Starts the installer with the terminal of this process, so its prompts work.
fn run(runner: Runner, program: &str, args: &[OsString]) -> io::Result<ExitStatus> {
    let mut command = Command::new(program);
    match runner {
        Runner::Npx => command.arg("--yes"),
        Runner::Pnpm => command.arg("dlx"),
        Runner::Bun | Runner::Auto => command.arg("x"),
    };
    command.args(args).status()
}

/// Writes the skill to `<cache>/kaneo/skill/skills/kaneo/SKILL.md` and returns the source folder.
fn write_source() -> Result<PathBuf, KaneoError> {
    let source = dirs::cache_dir()
        .ok_or_else(|| KaneoError::Usage("Cannot find the cache folder of this user".to_owned()))?
        .join("kaneo")
        .join("skill");
    let folder = source.join("skills").join("kaneo");
    fs::create_dir_all(&folder)?;
    fs::write(folder.join("SKILL.md"), SKILL)?;
    Ok(source)
}
