DEFINE TABLE OVERWRITE user SCHEMAFULL;

REMOVE EVENT ensure_single_system_owner ON TABLE user;

DEFINE EVENT ensure_single_system_owner
ON TABLE user
WHEN $event = "CREATE" AND $input.role = "SystemOwner"
THEN {
    LET $existing = SELECT count() AS total FROM user WHERE role = "SystemOwner";
    IF $existing[0].total > 0 {
        THROW "ONLY one SystemOwner is allowed";
    };
};
