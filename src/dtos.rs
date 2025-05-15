//data transfer objects

use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use std::{option, string};
use validator::{Validate, ValidationError};

use crate::models::{RecieveFileDetails, SendFileDetails, User};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(min = 7, message = "Email is required"),
        email(message = "Email is required")
    )]
    pub email: String,

    #[validate(
        // length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters")
    )]
    pub password: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        must_match(other = "password", message = "message do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequireQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub public_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto { 
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            public_key: user.public_key.to_owned(),
            created_at: user.created_at.unwrap(), 
            updated_at: user.updated_at.unwrap()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
    pub file_id: String,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}


impl UserSendFileDto {
    pub fn filter_send_user_file(file_data: &SendFileDetails) -> Self {
        UserSendFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    pub fn filter_send_user_files(user: &[SendFileDetails]) -> Vec<UserSendFileDto> {
        user.iter().map(UserSendFileDto::filter_send_user_file).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFileListResonseDto {
    pub status: String,
    pub files: Vec<UserSendFileDto>,
    pub result: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecieveFileDto {
    pub file_id: String,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}