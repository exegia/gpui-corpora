#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_update_status_serialization() {
    #[derive(serde::Serialize)]
    struct StatusVariants {
        idle: UpdateStatus,
        checking: UpdateStatus,
        up_to_date: UpdateStatus,
        available: UpdateStatus,
        downloading: UpdateStatus,
        downloaded: UpdateStatus,
        ready_to_install: UpdateStatus,
        error: UpdateStatus,
    }

    let variants = StatusVariants {
        idle: UpdateStatus::Idle,
        checking: UpdateStatus::Checking,
        up_to_date: UpdateStatus::UpToDate,
        available: UpdateStatus::Available {
            version: "2.0.0".to_string(),
            notes: "Bug fixes".to_string(),
        },
        downloading: UpdateStatus::Downloading { progress: 42 },
        downloaded: UpdateStatus::Downloaded {
            version: "2.0.0".to_string(),
            path: "/tmp/update.tar.gz".to_string(),
        },
        ready_to_install: UpdateStatus::ReadyToInstall,
        error: UpdateStatus::Error("network timeout".to_string()),
    };

    insta::assert_yaml_snapshot!(
            &variants,
    @r#"
        idle: Idle
        checking: Checking
        up_to_date: UpToDate
        available:
          Available:
            version: 2.0.0
            notes: Bug fixes
        downloading:
          Downloading:
            progress: 42
        downloaded:
          Downloaded:
            version: 2.0.0
            path: /tmp/update.tar.gz
        ready_to_install: ReadyToInstall
        error:
          Error: network timeout
        "#
        );
}
