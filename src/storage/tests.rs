use super::*;
use tempfile::NamedTempFile;

#[test]
fn test_next_id() {
    let tasks = vec![];
    assert_eq!(next_id(&tasks), 1);

    let tasks = vec![
        Task { id: 1, text: "a".to_string(), done: false, due: None, priority: Priority::Normal, tags: vec![] },
        Task { id: 3, text: "b".to_string(), done: true, due: None, priority: Priority::Normal, tags: vec![] },
    ];
    assert_eq!(next_id(&tasks), 4);
}

#[test]
fn test_save_and_load_tasks() -> Result<()> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap();

    let tasks = vec![
        Task { id: 1, text: "Task 1".to_string(), done: false, due: None, priority: Priority::Normal, tags: vec![] },
        Task { id: 2, text: "Task 2".to_string(), done: true, due: None, priority: Priority::Normal, tags: vec![] },
    ];

    save_tasks(path, &tasks)?;

    let loaded_tasks = load_tasks(path)?;
    assert_eq!(loaded_tasks.len(), 2);
    assert_eq!(loaded_tasks[0].text, "Task 1");
    assert_eq!(loaded_tasks[1].done, true);

    Ok(())
}

#[test]
fn test_load_non_existent_file() -> Result<()> {
    let tasks = load_tasks("/path/to/non/existent/file.json")?;
    assert!(tasks.is_empty());
    Ok(())
}
