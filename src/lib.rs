use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const PACKAGE_NAME: &str = "cssmodules-language-server";
// Entry point of the npm package. The server speaks LSP over stdin/stdout, so no flags
// are needed beyond pointing Node at this script.
const SERVER_SCRIPT: &str = "node_modules/cssmodules-language-server/lib/cli.js";

struct CssModulesExtension {
    // Once we have confirmed the npm package is installed we skip the (network) version
    // check on subsequent server starts within the same extension process.
    installed: bool,
}

impl CssModulesExtension {
    /// Ensure the npm package is installed (latest version) and return the path to its
    /// entry script, relative to the extension's working directory.
    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if !self.installed || installed_version.is_none() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );
            let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;

            if installed_version.as_deref() != Some(latest_version.as_str()) {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
                zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
            }
            self.installed = true;
        }

        Ok(SERVER_SCRIPT.to_string())
    }
}

impl zed::Extension for CssModulesExtension {
    fn new() -> Self {
        Self { installed: false }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // 1. Prefer a binary already on the user's $PATH (e.g. a global npm install).
        if let Some(path) = worktree.which(PACKAGE_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: worktree.shell_env(),
            });
        }

        // 2. Otherwise install the npm package and run it through Zed's bundled Node.
        let script = self.server_script_path(language_server_id)?;
        let script_path = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(&script)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![script_path],
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        // Pass through user-provided `initialization_options` (e.g. { "camelCase": "dashes" })
        // from settings under `lsp.cssmodules`. Falls back to the server's own defaults.
        let options = LspSettings::for_worktree("cssmodules", worktree)
            .ok()
            .and_then(|settings| settings.initialization_options.clone());
        Ok(options)
    }
}

zed::register_extension!(CssModulesExtension);
