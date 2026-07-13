use serde::Deserialize;
use validator::Validate;

// structs
#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeFormData {
    #[validate(length(min = 1, message="Name cannot be empty!"))]
    pub name: String,
    
    #[validate(email(message="Email cannot be empty!"))]
    pub email: String,
}