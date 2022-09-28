-- DML to insert data into MySQL database --

-- Create "Administrator" user role
INSERT INTO USER_ROLE (
    ID, ROLE_NAME, CREATE_IMAGE, READ_IMAGE, MODIFY_IMAGE, DELETE_IMAGE,
    READ_RENDITIONS, MODIFY_RENDITIONS, DELETE_RENDITIONS, READ_PROJECT,
    CREATE_PROJECT, MODIFY_PROJECT, DELETE_PROJECT, READ_USER, CREATE_USER,
    MODIFY_USER, DELETE_USER, PUBLISH, PUBLISH_ALL, ACCESS_ALL_PROJECTS
)
VALUES
(
    0, 'Unknown', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE
),
(
    1, 'Administrator', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE,
    TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
);

-- Create "Administrator" and "Unknown User" users
-- The time of creation of "Unknown User" is the time (in UTC) when this
-- project was first initialized!
INSERT INTO USER (
    ID, LOGIN_ID, EMAIL, NAME, USER_ROLE, CREATED_ON, LAST_LOGIN_ON
)
VALUES
-- (0, 'unknown', '', 'Unknown User', 0, '2022-07-30 09:26:14', NULL),
(0, 'admin', 'admin@localhost', 'Administrator', 1, NOW(), NULL);
