use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl From<UserModel> for UserResponse {
    fn from(m: UserModel) -> Self {
        Self {
            id: m.id,
            email: m.email,
            name: m.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_user_request {
        use super::*;

        #[test]
        fn can_create_request() {
            let request = CreateUserRequest {
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_eq!(request.email, "test@example.com");
            assert_eq!(request.name, "Test User");
        }

        #[test]
        fn supports_valid_emails() {
            let emails = vec![
                "user@example.com",
                "first.last@example.co.uk",
                "user+tag@example.com",
            ];

            for email in emails {
                let request = CreateUserRequest {
                    email: email.to_string(),
                    name: "User".to_string(),
                };
                assert_eq!(request.email, email);
            }
        }

        #[test]
        fn supports_various_names() {
            let names = vec!["John Doe", "Jane Smith", "李明", "José García"];

            for name in names {
                let request = CreateUserRequest {
                    email: "test@example.com".to_string(),
                    name: name.to_string(),
                };
                assert_eq!(request.name, name);
            }
        }

        #[test]
        fn can_be_cloned() {
            let request1 = CreateUserRequest {
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let request2 = request1.clone();
            assert_eq!(request1, request2);
        }
    }

    mod user_model {
        use super::*;

        #[test]
        fn can_create_model() {
            let user = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_eq!(user.id, 1);
            assert_eq!(user.email, "test@example.com");
            assert_eq!(user.name, "Test User");
        }

        #[test]
        fn supports_various_ids() {
            let ids = vec![1, 100, 999, 1_000_000];

            for id in ids {
                let user = UserModel {
                    id,
                    email: "test@example.com".to_string(),
                    name: "User".to_string(),
                };
                assert_eq!(user.id, id);
            }
        }

        #[test]
        fn models_with_same_data_are_equal() {
            let user1 = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let user2 = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_eq!(user1, user2);
        }

        #[test]
        fn models_with_different_ids_are_not_equal() {
            let user1 = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let user2 = UserModel {
                id: 2,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_ne!(user1, user2);
        }

        #[test]
        fn can_be_cloned() {
            let user1 = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let user2 = user1.clone();
            assert_eq!(user1, user2);
        }
    }

    mod user_response {
        use super::*;

        #[test]
        fn can_create_response() {
            let response = UserResponse {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_eq!(response.id, 1);
            assert_eq!(response.email, "test@example.com");
            assert_eq!(response.name, "Test User");
        }

        #[test]
        fn responses_with_same_data_are_equal() {
            let response1 = UserResponse {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let response2 = UserResponse {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            assert_eq!(response1, response2);
        }

        #[test]
        fn can_be_cloned() {
            let response1 = UserResponse {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let response2 = response1.clone();
            assert_eq!(response1, response2);
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn user_model_converts_to_response() {
            let model = UserModel {
                id: 42,
                email: "user@example.com".to_string(),
                name: "John Doe".to_string(),
            };

            let response: UserResponse = model.into();

            assert_eq!(response.id, 42);
            assert_eq!(response.email, "user@example.com");
            assert_eq!(response.name, "John Doe");
        }

        #[test]
        fn conversion_preserves_all_fields() {
            let model = UserModel {
                id: 123,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let response: UserResponse = model.clone().into();
            let expected = UserResponse {
                id: model.id,
                email: model.email,
                name: model.name,
            };

            assert_eq!(response, expected);
        }

        #[test]
        fn multiple_conversions_produce_equal_results() {
            let model = UserModel {
                id: 1,
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };

            let response1: UserResponse = model.clone().into();
            let response2: UserResponse = model.into();

            assert_eq!(response1, response2);
        }
    }
}