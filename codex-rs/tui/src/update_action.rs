/// Update action the CLI should perform after the TUI exits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateAction {
    /// Update via `npm install -g @cometix/codex`.
    NpmGlobalLatest,
    /// Update via `bun install -g @cometix/codex`.
    BunGlobalLatest,
}

impl UpdateAction {
    /// Returns the list of command-line arguments for invoking the update.
    pub fn command_args(self) -> (&'static str, &'static [&'static str]) {
        match self {
            UpdateAction::NpmGlobalLatest => ("npm", &["install", "-g", "@cometix/codex"]),
            UpdateAction::BunGlobalLatest => ("bun", &["install", "-g", "@cometix/codex"]),
        }
    }

    /// Returns string representation of the command-line arguments for invoking the update.
    pub fn command_str(self) -> String {
        let (command, args) = self.command_args();
        shlex::try_join(std::iter::once(command).chain(args.iter().copied()))
            .unwrap_or_else(|_| format!("{command} {}", args.join(" ")))
    }
}

#[cfg(not(debug_assertions))]
pub(crate) fn get_update_action() -> Option<UpdateAction> {
    let managed_by_npm = std::env::var_os("CODEX_MANAGED_BY_NPM").is_some();
    let managed_by_bun = std::env::var_os("CODEX_MANAGED_BY_BUN").is_some();

    detect_update_action(managed_by_npm, managed_by_bun)
}

#[cfg(any(not(debug_assertions), test))]
fn detect_update_action(managed_by_npm: bool, managed_by_bun: bool) -> Option<UpdateAction> {
    if managed_by_npm {
        Some(UpdateAction::NpmGlobalLatest)
    } else if managed_by_bun {
        Some(UpdateAction::BunGlobalLatest)
    } else {
        // Default to npm if no specific manager is detected
        Some(UpdateAction::NpmGlobalLatest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_update_action_without_env_mutation() {
        // Default to npm when no manager is detected
        assert_eq!(
            detect_update_action(false, false),
            Some(UpdateAction::NpmGlobalLatest)
        );
        // npm managed
        assert_eq!(
            detect_update_action(true, false),
            Some(UpdateAction::NpmGlobalLatest)
        );
        // bun managed
        assert_eq!(
            detect_update_action(false, true),
            Some(UpdateAction::BunGlobalLatest)
        );
        // npm takes precedence over bun
        assert_eq!(
            detect_update_action(true, true),
            Some(UpdateAction::NpmGlobalLatest)
        );
    }
}
