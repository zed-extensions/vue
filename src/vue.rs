use std::{env, fs};

use zed::lsp::{Completion, CompletionKind};
use zed::CodeLabelSpan;
use zed_extension_api::serde_json::json;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, Result};

const SERVER_PATH: &str = "node_modules/@vue/language-server/bin/vue-language-server.js";
const PACKAGE_NAME: &str = "@vue/language-server";

const TYPESCRIPT_PACKAGE_NAME: &str = "typescript";
// TypeScript 7+ is the native (Go) compiler and no longer ships the JS API
// (`ts.server`, `createLanguageService`, ...) that `@vue/language-server`
// needs, so the bundled copy is pinned to the last JS-based release.
const BUNDLED_TYPESCRIPT_VERSION: &str = "6.0.3";
const TS_PLUGIN_PACKAGE_NAME: &str = "@vue/typescript-plugin";

struct VueExtension {
    did_find_server: bool,
}

fn extension_install_dir() -> String {
    env::current_dir().unwrap().to_string_lossy().to_string()
}

impl VueExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            self.install_typescript_if_needed()?;
            self.sync_ts_plugin();
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.install_typescript_if_needed()?;
        self.sync_ts_plugin();
        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }

    /// Ensures the bundled TypeScript next to the language server is the pinned,
    /// JS-API-compatible version.
    ///
    /// The language server always resolves `typescript` relative to its own
    /// installation directory (it is never given the worktree's copy), and
    /// installing `@vue/language-server` also pulls in `typescript@latest` as a
    /// peer dependency, so this must run even when the project has its own
    /// TypeScript to replace an incompatible bundled copy.
    fn install_typescript_if_needed(&mut self) -> Result<()> {
        let installed_typescript_version =
            zed::npm_package_installed_version(TYPESCRIPT_PACKAGE_NAME)?;

        if installed_typescript_version.as_deref() != Some(BUNDLED_TYPESCRIPT_VERSION) {
            println!("installing {TYPESCRIPT_PACKAGE_NAME}@{BUNDLED_TYPESCRIPT_VERSION}");
            zed::npm_install_package(TYPESCRIPT_PACKAGE_NAME, BUNDLED_TYPESCRIPT_VERSION)?;
        } else {
            println!("typescript already installed");
        }

        Ok(())
    }

    fn sync_ts_plugin(&mut self) {
        let server_version = match zed::npm_package_installed_version(PACKAGE_NAME) {
            Ok(Some(version)) => version,
            Ok(None) => {
                println!("warning: could not determine installed {PACKAGE_NAME} version; skipping ts-plugin sync");
                return;
            }
            Err(err) => {
                println!("warning: could not query {PACKAGE_NAME} version: {err}; skipping ts-plugin sync");
                return;
            }
        };
        if let Err(err) = self.install_ts_plugin_if_needed(&server_version) {
            println!("warning: failed to sync {TS_PLUGIN_PACKAGE_NAME}@{server_version}: {err}");
        }
    }

    fn install_ts_plugin_if_needed(&mut self, target_version: &str) -> Result<()> {
        let installed_plugin_version = zed::npm_package_installed_version(TS_PLUGIN_PACKAGE_NAME)?;
        if installed_plugin_version.as_deref() != Some(target_version) {
            println!("installing {TS_PLUGIN_PACKAGE_NAME}@{target_version}");
            zed::npm_install_package(TS_PLUGIN_PACKAGE_NAME, target_version)?;
        } else {
            println!("ts-plugin already installed");
        }
        Ok(())
    }
}

impl zed::Extension for VueExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree("vue", worktree)
            .ok()
            .and_then(|settings| settings.initialization_options)
            .unwrap_or_else(|| json!({}));
        Ok(Some(initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("vue", worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .unwrap_or_else(|| {
                json!({
                    "vue.inlayHints.inlineHandlerLeading": true,
                    "vue.inlayHints.missingProps": true,
                    "vue.inlayHints.optionsWrapper": true,
                    "vue.inlayHints.vBindShorthand": true,
                })
            });

        Ok(Some(settings))
    }

    fn language_server_additional_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        target_language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match target_language_server_id.as_ref() {
            "typescript-language-server" => Ok(Some(serde_json::json!({
                "plugins": [{
                    "name": "@vue/typescript-plugin",
                    "location": extension_install_dir(),
                    "languages": ["typescript", "vue.js"],
                }],
            }))),
            _ => Ok(None),
        }
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        target_language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match target_language_server_id.as_ref() {
            "vtsls" => Ok(Some(serde_json::json!({
                "vtsls": {
                    "tsserver": {
                        "globalPlugins": [{
                            "name": "@vue/typescript-plugin",
                            "location": extension_install_dir(),
                            "enableForWorkspaceTypeScriptVersions": true,
                            "languages": ["vue.js"],
                            "configNamespace": "typescript"
                        }]
                    }
                },
            }))),
            _ => Ok(None),
        }
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Interface => "type",
            CompletionKind::Constructor => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Function | CompletionKind::Method => "function",
            CompletionKind::Property | CompletionKind::Field => "tag",
            CompletionKind::Variable => "type",
            CompletionKind::Keyword => "keyword",
            CompletionKind::Value => "tag",
            _ => return None,
        };

        let len = completion.label.len();
        let name_span = CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));

        Some(zed::CodeLabel {
            code: Default::default(),
            spans: if let Some(detail) = completion.detail {
                vec![
                    name_span,
                    CodeLabelSpan::literal(" ", None),
                    CodeLabelSpan::literal(detail, None),
                ]
            } else {
                vec![name_span]
            },
            filter_range: (0..len).into(),
        })
    }
}

zed::register_extension!(VueExtension);
