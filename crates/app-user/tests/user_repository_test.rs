#![allow(clippy::unwrap_used)]

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

    let user = repo.create("Alice", "alice@example.com").await.unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");

    let found = repo.find_by_id(user.id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test]
async fn find_by_email() {
    let (_pg, repo) = setup().await;

    repo.create("Bob", "bob@example.com").await.unwrap();

    let found = repo.find_by_email("bob@example.com").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Bob");

    let not_found = repo.find_by_email("nobody@example.com").await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn find_all() {
    let (_pg, repo) = setup().await;

    repo.create("User1", "user1@example.com").await.unwrap();
    repo.create("User2", "user2@example.com").await.unwrap();

    let users = repo.find_all().await.unwrap();
    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn update_user() {
    let (_pg, repo) = setup().await;

    let user = repo.create("Charlie", "charlie@example.com").await.unwrap();

    let updated = repo.update(user.id, Some("Charles"), None).await.unwrap();
    assert!(updated.is_some());

    let updated = updated.unwrap();
    assert_eq!(updated.name, "Charles");
    assert_eq!(updated.email, "charlie@example.com");
}

#[tokio::test]
async fn delete_user() {
    let (_pg, repo) = setup().await;

    let user = repo.create("Dave", "dave@example.com").await.unwrap();

    let deleted = repo.delete(user.id).await.unwrap();
    assert!(deleted);

    let found = repo.find_by_id(user.id).await.unwrap();
    assert!(found.is_none());

    // Deleting again returns false.
    let deleted_again = repo.delete(user.id).await.unwrap();
    assert!(!deleted_again);
}

#[tokio::test]
async fn duplicate_email_fails() {
    let (_pg, repo) = setup().await;

    repo.create("Eve", "eve@example.com").await.unwrap();

    let result = repo.create("Evil Eve", "eve@example.com").await;
    assert!(result.is_err());
}
