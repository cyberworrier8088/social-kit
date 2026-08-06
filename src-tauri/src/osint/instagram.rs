use super::types::InstagramProfile;

pub fn search_instagram(
    username: &str,
) -> Result<InstagramProfile, String> {

    Err(format!(
        "Instagram prfile lookup is not implemented yet for {}", username
    ))
}