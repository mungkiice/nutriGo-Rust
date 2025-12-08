/// Common test utilities and fixtures
use serde::Serialize;

/// Helper to create a sample user request for testing
pub fn create_sample_user_request(email: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "email": email,
        "name": name
    })
}

/// Helper to create multiple sample users
pub fn create_sample_users(count: usize) -> Vec<serde_json::Value> {
    (1..=count)
        .map(|i| {
            serde_json::json!({
                "email": format!("user{}@example.com", i),
                "name": format!("User {}", i)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_user_request_has_required_fields() {
        let user = create_sample_user_request("test@example.com", "Test User");
        assert_eq!(user["email"], "test@example.com");
        assert_eq!(user["name"], "Test User");
    }

    #[test]
    fn create_sample_users_generates_correct_count() {
        let users = create_sample_users(5);
        assert_eq!(users.len(), 5);
    }

    #[test]
    fn sample_users_have_unique_emails() {
        let users = create_sample_users(3);
        let emails: Vec<_> = users
            .iter()
            .map(|u| u["email"].as_str().unwrap())
            .collect();
        
        assert_eq!(emails.len(), emails.iter().collect::<std::collections::HashSet<_>>().len());
    }
}
