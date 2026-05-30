#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::fs;
use std::path::{Path, PathBuf};

use zed_extension_api::{self as zed, Result};

const BINARY_NAME: &str = "dhall-lsp-server";

struct DhallExtension;

impl zed::Extension for DhallExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        let command = find_or_download_proxy(language_server_id, worktree)?;
        Ok(zed::Command {
            command,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(zed::serde_json::json!({
            "vscode-dhall-lsp-server": {
            }
        })))
    }
}

fn find_or_download_proxy(
    language_server_id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
) -> Result<String> {
    if let Some(dhall_lsp) = worktree.which(BINARY_NAME) {
        return Ok(dhall_lsp);
    }

    Err(format!(
        "Failed to find {BINARY_NAME} in the system PATH."
    ))
    // let (platform, arch) = zed::current_platform();
    // let arch_str = match arch {
    //     zed::Architecture::Aarch64 => "aarch64",
    //     zed::Architecture::X8664 => "x86_64",
    //     zed::Architecture::X86 => {
    //         return Err("32-bit x86 architecture is not supported".into());
    //     }
    // };
    // let platform_str = match platform {
    //     zed::Os::Windows => "windows",
    //     zed::Os::Mac => "darwin",
    //     zed::Os::Linux => "linux",
    // };
    // let (download_ext, bin_ext, archive_type) = if matches!(platform, zed::Os::Windows) {
    //     (".exe", ".exe", zed::DownloadedFileType::Zip)
    // } else {
    //     (".tar.bz2", "", zed::DownloadedFileType::Bzip2Tar)
    // };
    // let release_ending = format!("{arch_str}-{platform_str}{download_ext}");
    // let bin_name = format!("{BINARY_NAME}{bin_ext}");

    // let cwd =
    //     std::env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;

    // zed::set_language_server_installation_status(
    //     language_server_id,
    //     &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    // );
    // let release = match zed::latest_github_release(
    //     "dhall-lang/dhall-haskell",
    //     zed::GithubReleaseOptions {
    //         require_assets: true,
    //         pre_release: false,
    //     },
    // ) {
    //     Ok(release) => release,
    //     Err(release_err) => {
    //         if let Some(binary) = latest_cached_binary(&cwd, &bin_name) {
    //             return Ok(binary.to_string_lossy().into_owned());
    //         }
    //         return Err(format!(
    //             "failed to fetch latest Dhall LSP release ({release_err}) and no cached version is available"
    //         ));
    //     }
    // };

    // let version_dir = cwd.join(format!("{BINARY_NAME}-{}", release.version));
    // let binary_path = version_dir.join(&bin_name);
    // if binary_path.exists() {
    //     return Ok(binary_path.to_string_lossy().into_owned());
    // }

    // let asset = release
    //     .assets
    //     .iter()
    //     .find(|a| a.name.starts_with(BINARY_NAME) && a.name.ends_with(&release_ending))
    //     .ok_or_else(|| {
    //         format!(
    //             "Failed to find a suitable Dhall LSP release asset for platform {platform_str} and architecture {arch_str}"
    //         )
    //     })?;

    // fs::create_dir_all(&version_dir)
    //     .map_err(|e| format!("Failed to create version directory: {e}"))?;

    // zed::set_language_server_installation_status(
    //     language_server_id,
    //     &zed::LanguageServerInstallationStatus::Downloading,
    // );

    // zed::download_file(
    //     &asset.download_url,
    //     &version_dir.to_string_lossy(),
    //     archive_type,
    // )
    // .map_err(|e| format!("Failed to download Dhall LSP release asset: {e}"))?;

    // let binary_path_str = binary_path.to_string_lossy().into_owned();
    // zed::make_file_executable(&binary_path_str)
    //     .map_err(|e| format!("Failed to make Dhall LSP binary executable: {e}"))?;

    // Ok(binary_path_str)
}

fn latest_cached_binary(cwd: &Path, bin_name: &str) -> Option<PathBuf> {
    let prefix = format!("{BINARY_NAME}-");
    fs::read_dir(cwd)
        .ok()?
        .filter_map(std::result::Result::ok)
        .filter(|entry| {
            entry.path().is_dir() && entry.file_name().to_string_lossy().starts_with(&prefix)
        })
        .filter_map(|entry| {
            let binary = entry.path().join(bin_name);
            let mtime = entry.metadata().ok()?.modified().ok()?;
            binary.exists().then_some((mtime, binary))
        })
        .max_by_key(|(mtime, _)| *mtime)
        .map(|(_, binary)| binary)
}

zed::register_extension!(DhallExtension);
