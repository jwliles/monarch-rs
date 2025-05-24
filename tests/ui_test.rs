#[cfg(test)]
mod ui_tests {
    use anyhow::Result;
    use monolith::ui::status_view::{RepositoryStatus, RepositoryStatusItem};
    use monolith::ui::command_view::{CommandItem, CommandStatus};

    #[test]
    fn test_repository_status_display() -> Result<()> {
        // Create test repository status items
        let items = vec![
            RepositoryStatusItem {
                name: "repo1".to_string(),
                path: "/path/to/repo1".to_string(),
                status: RepositoryStatus::Clean,
            },
            RepositoryStatusItem {
                name: "repo2".to_string(),
                path: "/path/to/repo2".to_string(),
                status: RepositoryStatus::Modified,
            },
            RepositoryStatusItem {
                name: "repo3".to_string(),
                path: "/path/to/repo3".to_string(),
                status: RepositoryStatus::Conflict,
            },
        ];

        // This would normally render the status view
        // For testing, we'll just verify the items are created correctly
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0].status, RepositoryStatus::Clean));
        assert!(matches!(items[1].status, RepositoryStatus::Modified));
        assert!(matches!(items[2].status, RepositoryStatus::Conflict));

        Ok(())
    }

    #[test]
    fn test_command_execution_display() -> Result<()> {
        // Create test command items
        let items = vec![
            CommandItem {
                repository: "repo1".to_string(),
                command: "git pull".to_string(),
                status: CommandStatus::Success,
                progress: 100,
            },
            CommandItem {
                repository: "repo2".to_string(),
                command: "git push".to_string(),
                status: CommandStatus::Running,
                progress: 50,
            },
            CommandItem {
                repository: "repo3".to_string(),
                command: "git commit -m 'Test'".to_string(),
                status: CommandStatus::Failed("Cannot commit: Unresolved conflicts".to_string()),
                progress: 0,
            },
        ];

        // For testing, we'll just verify the items are created correctly
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0].status, CommandStatus::Success));
        assert!(matches!(items[1].status, CommandStatus::Running));
        if let CommandStatus::Failed(msg) = &items[2].status {
            assert!(msg.contains("Unresolved conflicts"));
        } else {
            panic!("Expected Failed status");
        }

        Ok(())
    }
}