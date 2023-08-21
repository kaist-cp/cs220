#[cfg(test)]
mod test {
    use crate::assignments::assignment03::parse_shell::*;

    #[test]
    fn test_shell() {
        assert_eq!(
            parse_shell_command("cat file"),
            vec!["cat".to_string(), "file".to_string()]
        );
        assert_eq!(
            parse_shell_command("ls 'VirtualBox VMs'"),
            vec!["ls".to_string(), "VirtualBox VMs".to_string()]
        );
        assert_eq!(
            parse_shell_command("ls VirtualBox' 'VMs"),
            vec!["ls".to_string(), "VirtualBox VMs".to_string()]
        );
        assert_eq!(
            parse_shell_command("echo once upon a midnight dreary"),
            vec![
                "echo".to_string(),
                "once".to_string(),
                "upon".to_string(),
                "a".to_string(),
                "midnight".to_string(),
                "dreary".to_string(),
            ]
        );
        assert_eq!(
            parse_shell_command("echo 'once upon a midnight dreary'"),
            vec![
                "echo".to_string(),
                "once upon a midnight dreary".to_string(),
            ]
        );
    }
}
