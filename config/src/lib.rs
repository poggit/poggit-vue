use std::path::PathBuf;
use std::{env, fs, io};

use byte_unit::Byte;
use getset::{CopyGetters, Getters};
use lazy_static::lazy_static;
use serde::Deserialize;

fn load() -> io::Result<(PubConfig, PrivConfig)> {
    Ok((public_config, private_config))
}

pub fn load_public() -> io::Result<PubConfig> {
    let config = toml::from_slice(&fs::read("../default-public-config.toml")?)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Box::new(err)))?;
    Ok(config)
}

pub fn load_private() -> io::Result<PrivConfig> {
    let config = toml::from_slice(&fs::read("../default-public-config.toml")?)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Box::new(err)))?;
    Ok(config)
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct PubConfig {
    domains: PubDomains,
    github: PubGitHub,
    debug: PubDebug,
    global_quotas: PubGlobalQuotas,
    default_quotas: PubDefaultQuotas,
    didscord: PubDiscord,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct PubDomains {
    backend: Domain,
    frontend: Domain,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct Domain {
    #[getset(get_copy = "pub")]
    secure: bool,
    #[getset(get = "pub")]
    host: String,
    #[getset(get_copy = "pub")]
    port: u16,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PubGitHub {
    #[getset(get_copy = "pub")]
    app_id: u32,
    #[getset(get = "pub")]
    app_client_id: String,
    #[getset(get = "pub")]
    app_slug: String,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PubDebug {
    #[getset(get_copy = "pub")]
    enabled: bool,
    #[getset(get_copy = "pub")]
    whitelist_internal_ips: bool,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PubGlobalQuotas {
    #[getset(get_copy = "pub")]
    max_review_length: u32,
    #[getset(get_copy = "pub")]
    max_version_length: u32,
    #[getset(get_copy = "pub")]
    max_keyword_count: u32,
    #[getset(get_copy = "pub")]
    max_keyword_length: u32,
    #[getset(get_copy = "pub")]
    min_synopsis_length: u32,
    #[getset(get_copy = "pub")]
    max_synopsis_length: u32,
    #[getset(get_copy = "pub")]
    max_license_length: u32,
    #[getset(get_copy = "pub")]
    min_changelog_length: u32,
    #[getset(get_copy = "pub")]
    max_changelog_length: u32,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PubDefaultQuotas {
    #[getset(get_copy = "pub")]
    max_weekly_builds: u32,
    #[getset(get_copy = "pub")]
    max_weekly_projects: u32,
    #[serde(with = "serde_str")]
    #[getset(get_copy = "pub")]
    max_phar_size: Byte,
    #[serde(with = "serde_str")]
    #[getset(get_copy = "pub")]
    max_zipball_size: Byte,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub struct PubDiscord {
    #[getset(get = "pub")]
    invite: String,
}

#[derive(Debug, Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct PrivConfig {
    database: PrivDatabase,
    github: PrivGitHub,
    discord: PrivDiscord,
    raw: PrivRaw,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PrivDatabase {
    #[getset(get = "pub")]
    host: String,
    #[getset(get_copy = "pub")]
    port: u16,
    #[getset(get = "pub")]
    user: String,
    #[getset(get = "pub")]
    password: String,
    #[getset(get = "pub")]
    database: String,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PrivGitHub {
    #[getset(get = "pub")]
    app_client_secret: String,
    #[getset(get = "pub")]
    apiaccess_token: String,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PrivDiscord {
    #[getset(get = "pub")]
    new_builds: String,
    #[getset(get = "pub")]
    new_release: String,
    #[getset(get = "pub")]
    staff_audit: String,
    #[getset(get = "pub")]
    abuse_audit: String,
    #[getset(get = "pub")]
    registrations: String,
    #[getset(get = "pub")]
    errors: String,
}

#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
pub struct PrivDiscord {
    #[serde(with = "relative_path")]
    data_dir: PathBuf,
}

mod relative_path {
    use std::path::{Path, PathBuf};

    use serde::Deserializer;

    pub(super) fn deserialize<'de, D>(d: D) -> Result<PathBuf, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(d).map(|value| Path::new("..").join(value))
    }
}
