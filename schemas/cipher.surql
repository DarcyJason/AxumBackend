DEFINE TABLE OVERWRITE cipher SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON cipher TYPE record<user>;
DEFINE FIELD OVERWRITE cipher_type ON cipher TYPE string;
DEFINE FIELD OVERWRITE folder ON cipher TYPE option<record<folder>>;
DEFINE FIELD OVERWRITE organization ON cipher TYPE option<record<organization>>;
DEFINE FIELD OVERWRITE data ON cipher TYPE string;
DEFINE FIELD OVERWRITE remarks ON cipher TYPE option<string>;
DEFINE FIELD OVERWRITE additional_fields ON cipher TYPE option<array<object>>;
DEFINE FIELD OVERWRITE created_at ON cipher TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON cipher TYPE datetime;
