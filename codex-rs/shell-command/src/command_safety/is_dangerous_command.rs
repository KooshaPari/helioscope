use crate::bash::parse_shell_lc_plain_commands;
use crate::bash::shell_lc_script_has_complex_constructs;
use std::path::Path;
#[cfg(windows)]
#[path = "windows_dangerous_commands.rs"]
mod windows_dangerous_commands;

pub fn command_might_be_dangerous(command: &[String]) -> bool {
    #[cfg(windows)]
    {
        if windows_dangerous_commands::is_dangerous_command_windows(command) {
            return true;
        }
    }

    if is_dangerous_to_call_with_exec(command) {
        return true;
    }

    // Support `bash -lc "<script>"` where the any part of the script might contain a dangerous command.
    if let Some(all_commands) = parse_shell_lc_plain_commands(command)
        && all_commands
            .iter()
            .any(|cmd| is_dangerous_to_call_with_exec(cmd))
    {
        return true;
    }

    // When a bash -lc script contains redirections, command substitutions, or
    // other complex constructs, we cannot reliably parse its inner commands.
    // Treat it as potentially dangerous and require approval.
    if shell_lc_script_has_complex_constructs(command) {
        return true;
    }

    false
}

fn is_git_global_option_with_value(arg: &str) -> bool {
    matches!(
        arg,
        "-C" | "-c"
            | "--config-env"
            | "--exec-path"
            | "--git-dir"
            | "--namespace"
            | "--super-prefix"
            | "--work-tree"
    )
}

fn is_git_global_option_with_inline_value(arg: &str) -> bool {
    matches!(
        arg,
        s if s.starts_with("--config-env=")
            || s.starts_with("--exec-path=")
            || s.starts_with("--git-dir=")
            || s.starts_with("--namespace=")
            || s.starts_with("--super-prefix=")
            || s.starts_with("--work-tree=")
    ) || ((arg.starts_with("-C") || arg.starts_with("-c")) && arg.len() > 2)
}

pub(crate) fn executable_name_lookup_key(raw: &str) -> Option<String> {
    #[cfg(windows)]
    {
        Path::new(raw)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| {
                let name = name.to_ascii_lowercase();
                for suffix in [".exe", ".cmd", ".bat", ".com"] {
                    if let Some(stripped) = name.strip_suffix(suffix) {
                        return stripped.to_string();
                    }
                }
                name
            })
    }

    #[cfg(not(windows))]
    {
        Path::new(raw)
            .file_name()
            .and_then(|name| name.to_str())
            .map(std::borrow::ToOwned::to_owned)
    }
}

/// Find the first matching git subcommand, skipping known global options that
/// may appear before it (e.g., `-C`, `-c`, `--git-dir`).
///
/// Shared with `is_safe_command` to avoid git-global-option bypasses.
pub(crate) fn find_git_subcommand<'a>(
    command: &'a [String],
    subcommands: &[&str],
) -> Option<(usize, &'a str)> {
    let cmd0 = command.first().map(String::as_str)?;
    if executable_name_lookup_key(cmd0).as_deref() != Some("git") {
        return None;
    }

    let mut skip_next = false;
    for (idx, arg) in command.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }

        let arg = arg.as_str();

        if is_git_global_option_with_inline_value(arg) {
            continue;
        }

        if is_git_global_option_with_value(arg) {
            skip_next = true;
            continue;
        }

        if arg == "--" || arg.starts_with('-') {
            continue;
        }

        if subcommands.contains(&arg) {
            return Some((idx, arg));
        }

        // In git, the first non-option token is the subcommand. If it isn't
        // one of the subcommands we're looking for, we must stop scanning to
        // avoid misclassifying later positional args (e.g., branch names).
        return None;
    }

    None
}

fn is_dangerous_to_call_with_exec(command: &[String]) -> bool {
    let cmd0 = command.first().map(String::as_str);

    match cmd0 {
        Some("rm") => matches!(command.get(1).map(String::as_str), Some("-f" | "-rf")),

        // for sudo <cmd> simply do the check for <cmd>
        Some("sudo") => is_dangerous_to_call_with_exec(&command[1..]),

        // Destructive disk operations
        Some("dd") => true,
        Some("mkfs") | Some("mkfs.ext4") | Some("mkfs.xfs") | Some("mkfs.vfat") | Some("mkfs.btrfs") => true,
        Some("wipefs") | Some("blkdiscard") => true,

        // Dangerous permission/ownership changes
        Some("chmod") => {
            if let Some(mode) = command.get(1).map(String::as_str) {
                if mode == "-R" || mode.starts_with('-') && mode.contains('R') {
                    return true;
                }
                if mode == "777" || mode == "0777" {
                    return true;
                }
            }
            false
        }
        Some("chown") => {
            command.get(1).map(String::as_str) == Some("-R")
                || command.get(1).map(String::as_str).is_some_and(|a| a.starts_with('-') && a.contains('R'))
        }

        // Network tools commonly used for exfiltration or attack
        Some("nc") | Some("ncat") | Some("netcat") => true,
        Some("nmap") => true,

        // Download-and-execute patterns are handled at the caller level,
        // but flag curl/wget when piped to a shell as dangerous.
        Some("curl") | Some("wget") => {
            let has_pipe_or_eval = command.iter().any(|arg| {
                arg.contains("|") || arg.contains("bash") || arg.contains("sh ") || arg.contains("python")
            });
            has_pipe_or_eval
        }

        // Process manipulation
        Some("kill") => {
            command.get(1).map(String::as_str) == Some("-9")
                || command.get(1).map(String::as_str) == Some("-SIGKILL")
        }

        // ── anything else ─────────────────────────────────────────────────
        _ => false,
    }
}
                if mode == "777" || mode == "0777" {
                    return true;
                }
            }
            false
        }
        Some("chown") => {
            command.get(1).map(String::as_str) == Some("-R")
                || command.get(1).map(String::as_str).is_some_and(|a| a.starts_with('-') && a.contains('R'))
        }

        // Network tools commonly used for exfiltration or attack
        Some("nc") | Some("ncat") | Some("netcat") => true,
        Some("nmap") => true,

        // Download-and-execute patterns are handled at the caller level,
        // but flag curl/wget when piped to a shell as dangerous.
        Some("curl") | Some("wget") => {
            let has_pipe_or_eval = command.iter().any(|arg| {
                arg.contains("|") || arg.contains("bash") || arg.contains("sh ") || arg.contains("python")
            });
            has_pipe_or_eval
        }

        // Process manipulation
        Some("kill") => {
            command.get(1).map(String::as_str) == Some("-9")
                || command.get(1).map(String::as_str) == Some("-SIGKILL")
        }

        // ── anything else ─────────────────────────────────────────────────
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_str(items: &[&str]) -> Vec<String> {
        items.iter().map(std::string::ToString::to_string).collect()
    }

    #[test]
    fn rm_rf_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["rm", "-rf", "/"])));
    }

    #[test]
    fn rm_f_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["rm", "-f", "/"])));
    }

    #[test]
    fn dd_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["dd", "if=/dev/zero", "of=/dev/sda"])));
    }

    #[test]
    fn mkfs_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["mkfs.ext4", "/dev/sda1"])));
        assert!(command_might_be_dangerous(&vec_str(&["mkfs", "-t", "ext4", "/dev/sda1"])));
    }

    #[test]
    fn chmod_recursive_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "-R", "777", "/"])));
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "-R", "755", "/tmp"])));
    }

    #[test]
    fn chmod_777_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "777", "/tmp"])));
    }

    #[test]
    fn chown_recursive_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chown", "-R", "nobody:nobody", "/"])));
    }

    #[test]
    fn nc_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["nc", "-lvp", "4444"])));
        assert!(command_might_be_dangerous(&vec_str(&["netcat", "evil.com", "4444"])));
    }

    #[test]
    fn nmap_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["nmap", "-sS", "192.168.1.0/24"])));
    }

    #[test]
    fn sudo_recursive_check() {
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "rm", "-rf", "/"])));
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "dd", "if=/dev/zero", "of=/dev/sda"])));
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "chmod", "-R", "777", "/"])));
    }

    #[test]
    fn kill_sigkill_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["kill", "-9", "1"])));
        assert!(command_might_be_dangerous(&vec_str(&["kill", "-SIGKILL", "1"])));
    }
}

    #[test]
    fn rm_f_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["rm", "-f", "/"])));
    }

    #[test]
    fn dd_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["dd", "if=/dev/zero", "of=/dev/sda"])));
    }

    #[test]
    fn mkfs_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["mkfs.ext4", "/dev/sda1"])));
        assert!(command_might_be_dangerous(&vec_str(&["mkfs", "-t", "ext4", "/dev/sda1"])));
    }

    #[test]
    fn chmod_recursive_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "-R", "777", "/"])));
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "-R", "755", "/tmp"])));
    }

    #[test]
    fn chmod_777_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chmod", "777", "/tmp"])));
    }

    #[test]
    fn chown_recursive_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["chown", "-R", "nobody:nobody", "/"])));
    }

    #[test]
    fn nc_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["nc", "-lvp", "4444"])));
        assert!(command_might_be_dangerous(&vec_str(&["netcat", "evil.com", "4444"])));
    }

    #[test]
    fn nmap_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["nmap", "-sS", "192.168.1.0/24"])));
    }

    #[test]
    fn sudo_recursive_check() {
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "rm", "-rf", "/"])));
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "dd", "if=/dev/zero", "of=/dev/sda"])));
        assert!(command_might_be_dangerous(&vec_str(&["sudo", "chmod", "-R", "777", "/"])));
    }

    #[test]
    fn kill_sigkill_is_dangerous() {
        assert!(command_might_be_dangerous(&vec_str(&["kill", "-9", "1"])));
        assert!(command_might_be_dangerous(&vec_str(&["kill", "-SIGKILL", "1"])));
    }
}
