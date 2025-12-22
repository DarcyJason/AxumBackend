DEFINE TABLE OVERWRITE audit_log SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON audit_log TYPE option<record<user>>;
DEFINE FIELD OVERWRITE action ON audit_log TYPE string;
DEFINE FIELD OVERWRITE status ON audit_log TYPE string;
DEFINE FIELD OVERWRITE ip ON audit_log TYPE option<string>;
DEFINE FIELD OVERWRITE user_agent ON audit_log TYPE option<string>;
DEFINE FIELD OVERWRITE detail ON audit_log TYPE option<string>;
DEFINE FIELD OVERWRITE created_at ON audit_log TYPE datetime;

DEFINE INDEX OVERWRITE idx_audit_log_created_at ON audit_log FIELDS created_at;
DEFINE INDEX OVERWRITE idx_audit_log_with_user_id ON audit_log FIELDS user_id;
DEFINE INDEX OVERWRITE idx_audit_log_action ON audit_log FIELDS action;
DEFINE INDEX OVERWRITE idx_audit_log_user_action_time ON audit_log FIELDS user_id, action, created_at;
DEFINE INDEX OVERWRITE idx_audit_log_status ON audit_log FIELDS status;
