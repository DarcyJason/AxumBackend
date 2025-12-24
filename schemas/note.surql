DEFINE TABLE OVERWRITE note SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON note TYPE record<user>;
DEFINE FIELD OVERWRITE project_name ON note TYPE string;
DEFINE FIELD OVERWRITE project_owner ON note TYPE record<user | group>;
DEFINE FIELD OVERWRITE folder ON note TYPE option<array<record<folder>>>;
DEFINE FIELD OVERWRITE remarks ON note TYPE option<string>;
DEFINE FIELD OVERWRITE additional_fields ON note TYPE option<array<object>>;
DEFINE FIELD OVERWRITE created_at ON note TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON note TYPE datetime;
