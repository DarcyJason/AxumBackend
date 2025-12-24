DEFINE TABLE OVERWRITE ssh SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON ssh TYPE record<user>;
DEFINE FIELD OVERWRITE project_name ON ssh TYPE string;
DEFINE FIELD OVERWRITE project_owner ON ssh TYPE record<user | group>;
DEFINE FIELD OVERWRITE folder ON ssh TYPE option<array<record<folder>>>;
DEFINE FIELD OVERWRITE secret_key ON ssh TYPE option<string>;
DEFINE FIELD OVERWRITE public_key ON ssh TYPE option<string>;
DEFINE FIELD OVERWRITE fingerprint ON ssh TYPE option<string>;
DEFINE FIELD OVERWRITE remarks ON ssh TYPE option<string>;
DEFINE FIELD OVERWRITE addtional_fields ON ssh TYPE option<array<object>>;
DEFINE FIELD OVERWRITE created_at ON ssh TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON ssh TYPE datetime;
