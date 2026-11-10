const REPO: &str = "EzyGang/kaneo-cli";
const GITHUB_API: &str = "https://api.github.com/repos";
const USER_AGENT: &str = "kaneo-cli-upgrade";

#[derive(serde::Deserialize)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
pub struct Asset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

pub fn detect_platform() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match os {
        "windows" => "win32-x64".to_owned(),
        "macos" => match arch {
            "aarch64" => "darwin-arm64".to_owned(),
            _ => "darwin-x64".to_owned(),
        },
        _ => "linux-x64".to_owned(),
    }
}

pub async fn fetch_release(tag: &str) -> anyhow::Result<ReleaseInfo> {
    fetch_release_url(&format!("{GITHUB_API}/{REPO}/releases/tags/{tag}")).await
}

pub async fn fetch_latest_release() -> anyhow::Result<ReleaseInfo> {
    fetch_release_url(&format!("{GITHUB_API}/{REPO}/releases/latest")).await
}

pub async fn download_binary(url: &str) -> anyhow::Result<Vec<u8>> {
    let resp = http_client()?.get(url).send().await?;
    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}

async fn fetch_release_url(url: &str) -> anyhow::Result<ReleaseInfo> {
    let resp = http_client()?.get(url).send().await?;
    let release = resp.json::<ReleaseInfo>().await?;
    Ok(release)
}

fn http_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder().user_agent(USER_AGENT).build()?)
}
