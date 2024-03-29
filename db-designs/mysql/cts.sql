-- SQL script to setup additional constraints on database tables --

-- USER_ROLE
ALTER TABLE
    USER_ROLE
MODIFY COLUMN
    ID TINYINT UNSIGNED AUTO_INCREMENT,
AUTO_INCREMENT = 3;

-- USER
ALTER TABLE
    USER
MODIFY COLUMN
    ID SMALLINT UNSIGNED AUTO_INCREMENT,
AUTO_INCREMENT = 1;

ALTER TABLE
    USER
ADD CONSTRAINT
    USER_FK_USER_ROLE_ID FOREIGN KEY (USER_ROLE) REFERENCES USER_ROLE(ID);

-- USER_PROJECT
ALTER TABLE
    USER_PROJECT
ADD CONSTRAINT
    USER_PROJECT_FK_USER_ID FOREIGN KEY (USER_ID) REFERENCES USER(ID);

