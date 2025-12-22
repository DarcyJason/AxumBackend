DEFINE TABLE OVERWRITE user SCHEMAFULL;

DEFINE FIELD OVERWRITE name ON user TYPE string;
DEFINE FIELD OVERWRITE email ON user TYPE string;
DEFINE FIELD OVERWRITE password_hashed ON user TYPE string;
DEFINE FIELD OVERWRITE role ON user TYPE string;
DEFINE FIELD OVERWRITE is_verified ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE is_banned ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE created_at ON user TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON user TYPE datetime DEFAULT time::now();

DEFINE INDEX OVERWRITE idx_user_email ON user FIELDS email UNIQUE;
DEFINE INDEX OVERWRITE idx_user_role ON user FIELDS role;
DEFINE INDEX OVERWRITE idx_user_is_banned ON user FIELDS is_banned;
DEFINE INDEX OVERWRITE idx_user_created_at ON user FIELDS created_at;
