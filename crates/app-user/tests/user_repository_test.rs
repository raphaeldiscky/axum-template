use app_testing::postgres::{PostgresConfig, PostgresContainer};
use app_user::repository::{PgUserRepository, UserRepository};

async fn setup() -> (PostgresContainer, PgUserRepository) {
    let migration_dir = format!("{}/migrations/*.sql", env!("CARGO_MANIFEST_DIR"));
    let config = PostgresConfig::new("user_test")
        .with_migrations(vec![migration_dir])
        .with_cleanup_tables(vec!["users".to_string()]);

    let container = PostgresContainer::start(config)
        .await
        .expect("failed to start postgres container");

    let repo = PgUserRepository::new(container.pool().clone());
    (container, repo)
}

#[tokio::test]
async fn create_and_find_by_id() {
    let (_pg, repo) = setup().await;

    let user = repo
        .create("Alice", "alice@example.com")
        .await
        .expect("create failed");
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");

    let found = repo.find_by_id(user.id).await.expect("find_by_id failed");
    assert!(found.is_some());
    assert_eq!(found.expect("user should exist").id, user.id);
}

#[tokio::test]
async fn find_by_email() {
    let (_pg, repo) = setup().await;

    repo.create("Bob", "bob@example.com")
        .await
        .expect("create failed");

    let found = repo
        .find_by_email("bob@example.com")
        .await
        .expect("find_by_email failed");
    assert!(found.is_some());
    assert_eq!(found.expect("user should exist").name, "Bob");

    let not_found = repo
        .find_by_email("nobody@example.com")
        .await
        .expect("find_by_email failed");
    assert!(not_found.is_none());
}

#[tokio::test]
async fn find_all() {
    let (_pg, repo) = setup().await;

    repo.create("User1", "user1@example.com")
        .await
        .expect("create failed");
    repo.create("User2", "user2@example.com")
        .await
        .expect("create failed");

    let users = repo.find_all(10, 0).await.expect("find_all failed");
    assert_eq!(users.len(), 2);

    let count = repo.count_all().await.expect("count_all failed");
    assert_eq!(count, 2);

    // Pagination: limit 1, offset 1 returns second user only.
    let page2 = repo.find_all(1, 1).await.expect("find_all page2 failed");
    assert_eq!(page2.len(), 1);
}

#[tokio::test]
async fn update_user() {
    let (_pg, repo) = setup().await;

    let user = repo
        .create("Charlie", "charlie@example.com")
        .await
        .expect("create failed");

    let updated = repo
        .update(user.id, Some("Charles"), None)
        .await
        .expect("update failed");
    let updated = updated.expect("user should exist after update");
    assert_eq!(updated.name, "Charles");
    assert_eq!(updated.email, "charlie@example.com");
}

#[tokio::test]
async fn delete_user() {
    let (_pg, repo) = setup().await;

    let user = repo
        .create("Dave", "dave@example.com")
        .await
        .expect("create failed");

    let deleted = repo.delete(user.id).await.expect("delete failed");
    assert!(deleted);

    let found = repo.find_by_id(user.id).await.expect("find_by_id failed");
    assert!(found.is_none());

    // Deleting again returns false.
    let deleted_again = repo.delete(user.id).await.expect("delete again failed");
    assert!(!deleted_again);
}

#[tokio::test]
async fn duplicate_email_fails() {
    let (_pg, repo) = setup().await;

    repo.create("Eve", "eve@example.com")
        .await
        .expect("create failed");

    let result = repo.create("Evil Eve", "eve@example.com").await;
    assert!(result.is_err());
}
