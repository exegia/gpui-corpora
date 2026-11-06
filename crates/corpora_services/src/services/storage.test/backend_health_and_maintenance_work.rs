#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn backend_health_and_maintenance_work() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("app.db");
    init_db(&db_path).expect("init db");
    let backend = SqliteStorage::new(db_path);
    block_on(backend.health_check()).expect("health check");
    block_on(backend.maintenance()).expect("maintenance");
    assert_eq!(
        block_on(backend.schema_version()).expect("schema version"),
        3
    );
}
