DEFINE TABLE OVERWRITE vault SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON vault TYPE record<user>;
DEFINE FIELD OVERWRITE account_projects ON vault TYPE option<array<record<account>>>;
DEFINE FIELD OVERWRITE card_projects ON vault TYPE option<array<record<card>>>;
DEFINE FIELD OVERWRITE id_projects ON vault TYPE option<array<record<id>>>;
DEFINE FIELD OVERWRITE note_projects ON vault TYPE option<array<record<note>>>;
DEFINE FIELD OVERWRITE ssh_projects ON vault TYPE option<array<record<ssh>>>;
