DEFINE TABLE OVERWRITE organization SCHEMAFULL;

DEFINE FIELD OVERWRITE name ON organization TYPE string;
DEFINE FIELD OVERWRITE members ON organization TYPE array<record<user>>;
