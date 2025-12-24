DEFINE TABLE OVERWRITE card SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON card TYPE record<user>;
DEFINE FIELD OVERWRITE project_name ON card TYPE string;
DEFINE FIELD OVERWRITE project_owner ON card TYPE record<user | group>;
DEFINE FIELD OVERWRITE folder ON card TYPE option<array<record<folder>>>;
DEFINE FIELD OVERWRITE owner_name ON card TYPE option<string>;
DEFINE FIELD OVERWRITE card_number ON card TYPE option<string>;
DEFINE FIELD OVERWRITE card_brand ON card TYPE option<string>;
DEFINE FIELD OVERWRITE expired_month ON card TYPE option<array<string>>;
DEFINE FIELD OVERWRITE expired_year ON card TYPE option<number>;
DEFINE FIELD OVERWRITE safe_code ON card TYPE option<string>;
DEFINE FIELD OVERWRITE remarks ON card TYPE option<string>;
DEFINE FIELD OVERWRITE additional_fields ON card TYPE option<array<object>>;
DEFINE FIELD OVERWRITE created_at ON card TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON card TYPE datetime;
