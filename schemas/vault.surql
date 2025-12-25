DEFINE TABLE OVERWRITE vault SCHEMAFULL;

DEFINE FIELD OVERWRITE user_id ON vault TYPE record<user>;
DEFINE FIELD OVERWRITE ciphers ON vault TYPE array<record<cipher>>;
DEFINE FIELD OVERWRITE folders ON vault TYPE array<record<folder>>;
DEFINE FIELD OVERWRITE organizations ON vault TYPE array<record<organization>>;
