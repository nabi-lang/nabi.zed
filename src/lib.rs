//! Zed extension for Nabi: starts `nabils`, the Nabi language server.

use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

const SERVER: &str = "nabils";

struct Nabi;

impl zed::Extension for Nabi {
    fn new() -> Self {
        Nabi
    }

    fn language_server_command(
        &mut self,
        _id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // lsp.nabils.binary in settings wins over PATH.
        let binary = LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.binary);
        let (path, args, env) = match binary {
            Some(b) => (b.path, b.arguments, b.env),
            None => (None, None, None),
        };

        let command = match path {
            Some(p) => p,
            None => worktree.which(SERVER).ok_or_else(|| {
                format!(
                    "{SERVER} not found on PATH. Install the Nabi toolchain \
                     (https://nabi-lang.org/download) so {SERVER} is on PATH, \
                     or set lsp.{SERVER}.binary.path in Zed settings."
                )
            })?,
        };

        // Settings env overrides the login shell's.
        let env = env.unwrap_or_default();
        let mut env_vars = worktree.shell_env();
        env_vars.retain(|(k, _)| !env.contains_key(k));
        env_vars.extend(env);

        Ok(zed::Command {
            command,
            args: args.unwrap_or_default(),
            env: env_vars,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.settings))
    }
}

zed::register_extension!(Nabi);
