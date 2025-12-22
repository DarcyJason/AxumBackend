DEFINE TABLE OVERWRITE device SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON device TYPE record<user>;
DEFINE FIELD OVERWRITE device_name ON device TYPE string;
DEFINE FIELD OVERWRITE device_type ON device TYPE string;
DEFINE FIELD OVERWRITE raw_user_agent ON device TYPE string;
DEFINE FIELD OVERWRITE first_login_at ON device TYPE datetime;
DEFINE FIELD OVERWRITE last_login_at ON device TYPE datetime;
DEFINE FIELD OVERWRITE ip ON device TYPE string;
DEFINE FIELD OVERWRITE is_trusted ON device TYPE bool;

DEFINE INDEX OVERWRITE idx_device_with_user_id ON device FIELDS user_id;
DEFINE INDEX OVERWRITE idx_device_name_with_user_id ON device FIELDS user_id, device_name UNIQUE;
DEFINE INDEX OVERWRITE idx_device_type_with_user_id ON device FIELDS user_id, device_type;
DEFINE INDEX OVERWRITE idx_device_trusted_with_user_id ON device FIELDS user_id, is_trusted;
DEFINE INDEX OVERWRITE idx_device_user_last_login_at ON device FIELDS user_id, last_login_at;
