DEFINE TABLE OVERWRITE folder SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON folder TYPE record<user>;
DEFINE FIELD OVERWRITE name ON folder TYPE string;
DEFINE FIELD OVERWRITE created_at ON folder TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON folder TYPE datetime;
