#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    use super::super::backend::SqliteStorage;

    let path = db_path(cx);
    let backend = Arc::new(SqliteStorage::new(path.clone()));
    let mut snapshot = StorageSnapshot {
        db_path: path.display().to_string(),
        ..StorageSnapshot::default()
    };

    match init_db(&path) {
        Ok(schema_version) => {
            snapshot.available = true;
            snapshot.schema_version = schema_version;
            snapshot.last_migration_result =
                Some(format!("schema version {} ready", schema_version));
            tracing::info!(
                target: "corpora::storage",
                db_path = %snapshot.db_path,
                schema_version,
                "storage initialized"
            );
        }
        Err(err) => {
            let error = err.to_string();
            snapshot.last_error = Some(error.clone());
            snapshot.last_migration_result = Some("migration failed".to_string());
            tracing::error!(
                target: "corpora::storage",
                db_path = %snapshot.db_path,
                error = %error,
                "storage initialization failed"
            );
        }
    }

    if snapshot.available {
        // The health check runs as a spawned task and must re-derive the
        // capability from the patched snapshot: this synchronous call sees
        // `healthy: false`, and the registry has no observers to refresh it.
        let backend_for_health = Arc::clone(&backend);
        let bg = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            let result = bg
                .spawn(async move { backend_for_health.health_check().await })
                .await;
            let _ = cx.update(|cx| {
                let updated = cx.update_global::<StorageSnapshot, _>(|snap, _cx| {
                    record_health_result(snap, result);
                    snap.clone()
                });
                set_capabilities(true, &updated, cx);
            });
        })
        .detach();
    }

    set_capabilities(true, &snapshot, cx);

    cx.set_global(snapshot);
    cx.set_global(StorageRuntime { backend });
}
