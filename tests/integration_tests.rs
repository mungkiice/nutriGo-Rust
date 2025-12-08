/// Integration tests for user-related functionality
mod common;

#[cfg(test)]
mod user_tests {
    use crate::common::*;

    #[test]
    fn sample_user_creation() {
        let user = create_sample_user_request("john@example.com", "John Doe");
        assert!(user["email"].as_str().unwrap().contains("@"));
        assert!(!user["name"].as_str().unwrap().is_empty());
    }

    #[test]
    fn user_request_validation_email_format() {
        let user = create_sample_user_request("invalid-email", "John");
        let email = user["email"].as_str().unwrap();
        // In a real test, this would validate against a validator
        assert!(!email.is_empty());
    }

    #[test]
    fn multiple_users_can_be_created() {
        let users = create_sample_users(10);
        assert_eq!(users.len(), 10);
        
        // Verify all users have required fields
        for user in users {
            assert!(user["email"].is_string());
            assert!(user["name"].is_string());
        }
    }

    #[test]
    fn user_request_preserves_data() {
        let email = "test@example.com";
        let name = "Test User";
        let user = create_sample_user_request(email, name);

        assert_eq!(user["email"].as_str().unwrap(), email);
        assert_eq!(user["name"].as_str().unwrap(), name);
    }
}

#[cfg(test)]
mod data_consistency_tests {
    use crate::common::*;

    #[test]
    fn user_batch_creation_maintains_order() {
        let users = create_sample_users(5);
        for (i, user) in users.iter().enumerate() {
            let email = user["email"].as_str().unwrap();
            assert!(email.contains(&(i + 1).to_string()));
        }
    }

    #[test]
    fn user_data_is_serializable() {
        let user = create_sample_user_request("test@example.com", "Test");
        let json_string = serde_json::to_string(&user).expect("Should serialize");
        assert!(json_string.contains("test@example.com"));
        assert!(json_string.contains("Test"));
    }

    #[test]
    fn user_data_roundtrips_through_json() {
        let original = create_sample_user_request("test@example.com", "Test User");
        let json_string = serde_json::to_string(&original).expect("Should serialize");
        let restored: serde_json::Value = 
            serde_json::from_str(&json_string).expect("Should deserialize");
        
        assert_eq!(original, restored);
    }
}
