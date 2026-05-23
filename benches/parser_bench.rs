use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use logagg::parser::{JsonLogParser, RegexLogParser, StructuredLogParser};
use logagg::LogParser;

fn benchmark_json_parser(c: &mut Criterion) {
    let parser = JsonLogParser;
    let log_line = r#"{"timestamp":"2024-01-15T12:00:00Z","level":"ERROR","source":"app","message":"Connection timeout","request_id":"abc123","latency_ms":5000}"#;

    c.bench_function("json_parse_single_line", |b| {
        b.iter(|| parser.parse(black_box(log_line)))
    });
}

fn benchmark_regex_parser(c: &mut Criterion) {
    let parser = RegexLogParser::default();
    let lines = [
        "2024-01-15T12:00:00Z ERROR [myapp] Connection timeout after 5000ms",
        "2024-01-15 12:00:00.000 WARN  Service response slow",
        "[2024-01-15 12:00:00] [INFO] All systems operational",
        "ERROR: disk full on /dev/sda1",
    ];

    let mut group = c.benchmark_group("regex_parser");
    for (i, line) in lines.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("format", i), line, |b, l| {
            b.iter(|| parser.parse(black_box(l)))
        });
    }
    group.finish();
}

fn benchmark_structured_parser(c: &mut Criterion) {
    let parser = StructuredLogParser;
    let log_line =
        r#"ts=2024-01-15T12:00:00Z level=error source=app msg="Connection timeout" request_id=abc123 latency=5000ms"#;

    c.bench_function("structured_parse_single_line", |b| {
        b.iter(|| parser.parse(black_box(log_line)))
    });
}

fn benchmark_auto_detect(c: &mut Criterion) {
    use logagg::parser::auto_detect;

    let json_line = r#"{"level":"ERROR","message":"timeout"}"#;
    let regex_line = "2024-01-15T12:00:00Z ERROR [app] timeout";
    let struct_line = r#"level=error msg="timeout""#;

    let mut group = c.benchmark_group("auto_detect_and_parse");
    group.bench_function("json", |b| {
        b.iter(|| {
            let p = auto_detect(black_box(json_line));
            p.parse(black_box(json_line))
        })
    });
    group.bench_function("regex", |b| {
        b.iter(|| {
            let p = auto_detect(black_box(regex_line));
            p.parse(black_box(regex_line))
        })
    });
    group.bench_function("structured", |b| {
        b.iter(|| {
            let p = auto_detect(black_box(struct_line));
            p.parse(black_box(struct_line))
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_json_parser,
    benchmark_regex_parser,
    benchmark_structured_parser,
    benchmark_auto_detect,
);
criterion_main!(benches);
