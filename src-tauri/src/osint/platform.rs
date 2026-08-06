pub trait Platform {

    type Profile;

    fn name() -> &'static str;

    fn search(
        username: &str,
    ) -> Result<Self::Profile, String>;
}