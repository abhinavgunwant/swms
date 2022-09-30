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
    ID, LOGIN_ID, EMAIL, PASSWORD, NAME, USER_ROLE, CREATED_ON, LAST_LOGIN_ON,
    CREATED_BY, MODIFIED_ON, MODIFIED_BY
)
VALUES
-- (0, 'unknown', '', 'Unknown User', 0, '2022-07-30 09:26:14', NULL),
(
    0, 'admin', 'admin@localhost',
'$argon2id$v=19$m=16384,t=2,p=1$Hbs85Unihs0YdlwxkTb1KQ$Cs2MWaGX6pHGAGHEGU2hwV4djkpBJMrTwSM51rDOjvid2WdYC0YElOEMR92TfwZb3GogZH5RXV+KwECjSX8nBw',
'Administrator', 1, NOW(), NULL, 0, NOW(), 0);
