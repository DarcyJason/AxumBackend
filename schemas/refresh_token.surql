DEFINE TABLE OVERWRITE refresh_token SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON refresh_token TYPE record<user>;
DEFINE FIELD OVERWRITE token_hashed ON refresh_token TYPE string;
DEFINE FIELD OVERWRITE device ON refresh_token TYPE record<device>;
DEFINE FIELD OVERWRITE ip ON refresh_token TYPE string;
DEFINE FIELD OVERWRITE created_at ON refresh_token TYPE datetime;
DEFINE FIELD OVERWRITE expires_at ON refresh_token TYPE datetime;
DEFINE FIELD OVERWRITE is_revoked ON refresh_token TYPE bool;

DEFINE INDEX OVERWRITE idx_refresh_token_token_hashed ON refresh_token FIELDS token_hashed UNIQUE;
DEFINE INDEX OVERWRITE idx_refresh_token_with_user_id ON refresh_token FIELDS user_id;
DEFINE INDEX OVERWRITE idx_refresh_token_with_device ON refresh_token FIELDS device;
DEFINE INDEX OVERWRITE idx_refresh_token_valid_token ON refresh_token FIELDS token_hashed, is_revoked, expires_at;
DEFINE INDEX OVERWRITE idx_refresh_token_expired_token ON refresh_token FIELDS expires_at;
