DEFINE TABLE OVERWRITE account SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON account TYPE record<user>;
DEFINE FIELD OVERWRITE project_name ON account TYPE string;
DEFINE FIELD OVERWRITE project_owner ON account TYPE record<user | group>;
DEFINE FIELD OVERWRITE folder ON account TYPE option<array<record<folder>>>;
DEFINE FIELD OVERWRITE username ON account TYPE option<string>;
DEFINE FIELD OVERWRITE password ON account TYPE option<string>;
DEFINE FIELD OVERWRITE web_url ON account TYPE option<string>;
DEFINE FIELD OVERWRITE remarks ON account TYPE option<string>;
DEFINE FIELD OVERWRITE additional_fields ON account TYPE option<array<object>>;
DEFINE FIELD OVERWRITE created_at ON account TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON account TYPE datetime;
