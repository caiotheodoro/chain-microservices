pub struct MessageService;

impl MessageService {
    pub const ERROR_REDIS_SETUP: &'static str = "Error intanciating Redis.";
    pub const ERROR_CREATING_USER: &'static str = "Error creating user.";
    pub const USER_ALREADY_EXISTS: &'static str = "User already exists!";
    pub const ERROR_GENERATING_TOKEN: &'static str = "Error generating token for the user,";
    pub const APP_KEY_MISSING: &'static str =
        "ACCESS_KEY is missing from the environment variables.";
    pub const INVALID_EMAIL_OR_PASSWORD: &'static str = "Invalid email or password.";
    pub const INVALID_USERNAME_OR_PASSWORD: &'static str = "Invalid username or password.";
    pub const UNAUTHENTICATED: &'static str = "Unauthenticated.";
    pub const NO_ACCESS_TOKEN_SPECIFIED: &'static str = "No access token specified.";
    pub const INVALID_TOKEN: &'static str = "Invalid token.";
    pub const ERROR_GETTING_USER: &'static str = "Error getting user.";
    pub const ERROR_GETTING_USERS: &'static str = "Error getting users.";
    pub const INVALID_REFRESH_TOKEN: &'static str = "Invalid refresh token.";
}
