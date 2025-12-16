DEFINE TABLE OVERWRITE user SCHEMAFULL
PERMISSIONS
    FOR create
        WHERE $auth != NONE
    FOR select
        WHERE $auth.role in ["Admin", "SystemOwner"]
        OR id == $auth.id
    FOR update
        WHERE $auth.role == "SystemOwner"
        OR (
            $auth.role == "Admin"
            AND role != "SystemOwner"
            AND id == $auth.id
        )
    FOR delete
        WHERE $auth.role == "SystemOwner"
        OR ($auth.role == "Admin" AND role != "SystemOwner");

DEFINE FIELD OVERWRITE name ON user TYPE string ASSERT
    {
        IF string::len($value) > 2 && string::len($value) < 20 {
            RETURN true
        } ELSE {
            THROW "Name length must be between 3 and 20 characters"
        }
    };
DEFINE FIELD OVERWRITE email ON user TYPE string ASSERT
    {
        IF string::is::email($value) {
            RETURN true
        } ELSE {
            THROW "Invalid email format"
        }
    };
DEFINE FIELD OVERWRITE password_hashed ON user TYPE string ASSERT
    {
        IF string::len($value) > 0 {
            RETURN true
        } ELSE {
            THROW "Password hash cannot be empty"
        }
    };
DEFINE FIELD OVERWRITE role ON user TYPE string ASSERT
    {
        IF $value IN ["User", "Admin", "SystemOwner"] {
            RETURN true
        } ELSE {
            THROW "Invalid role value"
        }
    }
    PERMISSIONS FOR update WHERE $auth.role = "SystemOwner";
DEFINE FIELD OVERWRITE is_verified ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE is_banned ON user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE created_at ON user TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON user TYPE datetime DEFAULT time::now();
