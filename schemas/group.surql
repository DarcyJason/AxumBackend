DEFINE TABLE OVERWRITE group SCHEMAFULL;

DEFINE FIELD OVERWRITE name ON group TYPE string;
DEFINE FIELD OVERWRITE users ON group TYPE array<record<user>>;
DEFINE FIELD OVERWRITE created_at ON group TYPE datetime;
DEFINE FIELD OVERWRITE updated_at ON group TYPE datetime;
