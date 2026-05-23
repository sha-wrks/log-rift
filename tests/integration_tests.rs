use logagg::{LogAnalyzer, LogLevel};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn test_analyze_multiple_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, r#"{{"level":"ERROR","message":"Error 1"}}"#).unwrap();
    writeln!(file1, r#"{{"level":"INFO","message":"Info 1"}}"#).unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, r#"{{"level":"ERROR","message":"Error 2"}}"#).unwrap();

    let paths = vec![
        PathBuf::from(file1.path()),
        PathBuf::from(file2.path()),
    ];

    let analyzer = LogAnalyzer::load_files(&paths).unwrap();
    let result = analyzer.filter_by_level(LogLevel::Error).analyze();

    assert_eq!(result.count, 2);
}

#[test]
fn test_filter_by_time_range() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"timestamp":"2024-01-15T10:00:00Z","level":"ERROR"}}"#).unwrap();
    writeln!(file, r#"{{"timestamp":"2024-01-15T12:00:00Z","level":"ERROR"}}"#).unwrap();
    writeln!(file, r#"{{"timestamp":"2024-01-15T14:00:00Z","level":"ERROR"}}"#).unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();

    let from = "2024-01-15T09:00:00Z".parse().unwrap();
    let to = "2024-01-15T13:00:00Z".parse().unwrap();

    let result = analyzer.filter_by_time_range(from, to).analyze();
    assert_eq!(result.count, 2);
}

#[test]
fn test_filter_by_source() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"level":"ERROR","source":"database","message":"deadlock"}}"#).unwrap();
    writeln!(file, r#"{{"level":"ERROR","source":"webserver","message":"500"}}"#).unwrap();
    writeln!(file, r#"{{"level":"ERROR","source":"database","message":"timeout"}}"#).unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.filter_by_source("database").analyze();

    assert_eq!(result.count, 2);
}

#[test]
fn test_filter_by_message_pattern() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"level":"ERROR","message":"Connection timeout"}}"#).unwrap();
    writeln!(file, r#"{{"level":"ERROR","message":"Disk full"}}"#).unwrap();
    writeln!(file, r#"{{"level":"WARN","message":"High memory: timeout risk"}}"#).unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.filter_by_message("timeout").analyze();

    assert_eq!(result.count, 2);
}

#[test]
fn test_empty_file() {
    let file = NamedTempFile::new().unwrap();
    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.analyze();
    assert_eq!(result.count, 0);
    assert_eq!(result.error_rate, 0.0);
}

#[test]
fn test_error_rate_calculation() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{"level":"INFO","message":"startup"}}"#).unwrap();
    writeln!(file, r#"{{"level":"INFO","message":"running"}}"#).unwrap();
    writeln!(file, r#"{{"level":"ERROR","message":"crash"}}"#).unwrap();
    writeln!(file, r#"{{"level":"FATAL","message":"panic"}}"#).unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.analyze();

    assert_eq!(result.count, 4);
    assert!((result.error_rate - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_structured_log_parsing() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"ts=2024-01-15T12:00:00Z level=error source=db msg="deadlock detected""#
    )
    .unwrap();
    writeln!(
        file,
        r#"ts=2024-01-15T12:01:00Z level=info source=api msg="request ok""#
    )
    .unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.filter_by_level(LogLevel::Error).analyze();

    assert_eq!(result.count, 1);
}

#[test]
fn test_regex_log_parsing() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "2024-01-15T12:00:00Z ERROR [myapp] Connection refused").unwrap();
    writeln!(file, "2024-01-15T12:00:01Z INFO  [myapp] Retrying...").unwrap();
    writeln!(file, "2024-01-15T12:00:02Z WARN  [myapp] Slow response").unwrap();

    let analyzer = LogAnalyzer::load_files(&[PathBuf::from(file.path())]).unwrap();
    let result = analyzer.analyze();

    assert_eq!(result.count, 3);
    assert_eq!(*result.level_counts.get("ERROR").unwrap_or(&0), 1);
}
