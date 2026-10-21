use zed_extension_api as zed;

struct LapExtension;

impl zed::Extension for LapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = worktree
            .which("lapzt")
            .ok_or_else(|| "lapzt not found; run `cargo install --path .`".to_string())?;
        Ok(zed::Command {
            command,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(LapExtension);
