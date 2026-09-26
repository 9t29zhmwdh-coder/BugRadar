//! The dashboard builds WatchSource objects in TypeScript. These are the exact
//! shapes WatchPathList.tsx sends; the camel-case names it used before were
//! rejected by serde, so no source could be added from the interface.

use br_core::{WatchSource, WatchSourceKind};

fn source_with(kind: &str) -> String {
    format!(
        r#"{{"id":"s1","label":"demo","kind":{kind},"parser_id":"auto","enabled":true,"created_at":"2026-09-26T17:00:00.000Z"}}"#
    )
}

#[test]
fn file_path_from_the_dashboard_is_accepted() {
    let source: WatchSource =
        serde_json::from_str(&source_with(r#"{"file_path":{"path":"/var/log/app.log"}}"#)).unwrap();
    assert_eq!(source.kind, WatchSourceKind::FilePath { path: "/var/log/app.log".into() });
}

#[test]
fn docker_container_from_the_dashboard_is_accepted() {
    let source: WatchSource = serde_json::from_str(&source_with(
        r#"{"docker_container":{"container_id":"api","container_name":"api"}}"#,
    ))
    .unwrap();
    assert_eq!(
        source.kind,
        WatchSourceKind::DockerContainer { container_id: "api".into(), container_name: "api".into() }
    );
}

#[test]
fn all_containers_from_the_dashboard_is_accepted() {
    let source: WatchSource = serde_json::from_str(&source_with(r#""docker_all_containers""#)).unwrap();
    assert_eq!(source.kind, WatchSourceKind::DockerAllContainers);
}

#[test]
fn the_old_camel_case_shape_is_what_failed() {
    let old = source_with(r#"{"FilePath":{"path":"/var/log/app.log"}}"#);
    assert!(serde_json::from_str::<WatchSource>(&old).is_err());
}
