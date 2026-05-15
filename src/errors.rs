use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaneoError {
    #[error("No API key found")]
    MissingApiKey {
        source: anyhow::Error,
        hint: &'static str,
    },

    #[error("No workspace configured")]
    MissingWorkspace {
        source: anyhow::Error,
        hint: &'static str,
    },

    #[error("No project configured")]
    MissingProject {
        source: anyhow::Error,
        hint: &'static str,
    },

    #[error("Resource not found: {message}")]
    NotFound {
        message: String,
        hint: &'static str,
        source: anyhow::Error,
    },

    #[error("{message}")]
    Api {
        message: String,
        source: anyhow::Error,
    },

    #[error("{0}")]
    Config(#[source] anyhow::Error),

    #[error("{message}")]
    Upgrade {
        message: String,
        source: anyhow::Error,
    },

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

pub fn missing_api_key() -> KaneoError {
    KaneoError::MissingApiKey {
        source: anyhow::anyhow!("no API key found in config or environment"),
        hint: "Run `kaneo login <your-api-key>` to authenticate",
    }
}

pub fn missing_workspace() -> KaneoError {
    KaneoError::MissingWorkspace {
        source: anyhow::anyhow!("no workspace ID configured"),
        hint: "Provide it with `-w <id>`, KANEO_WORKSPACE env var, or `kaneo set <workspace-id>`",
    }
}

pub fn missing_project() -> KaneoError {
    KaneoError::MissingProject {
        source: anyhow::anyhow!("no project ID configured"),
        hint: "Provide it with `-p <id>`, KANEO_PROJECT env var, or `kaneo set <workspace-id> --project <id>`",
    }
}

pub fn not_found(resource: &str, id: &str, source: anyhow::Error) -> KaneoError {
    KaneoError::NotFound {
        message: format!("{resource} \"{id}\" not found"),
        hint: "Use `list` commands to discover available IDs",
        source,
    }
}

pub fn api_error(message: String, source: anyhow::Error) -> KaneoError {
    KaneoError::Api {
        message: format!("{message}: {source}"),
        source,
    }
}

pub fn config_error(source: anyhow::Error) -> KaneoError {
    KaneoError::Config(source)
}

impl From<anyhow::Error> for KaneoError {
    fn from(err: anyhow::Error) -> Self {
        KaneoError::Config(err)
    }
}
