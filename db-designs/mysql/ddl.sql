CREATE TABLE USER_ROLE (
    ID TINYINT UNSIGNED PRIMARY KEY,
    ROLE_NAME VARCHAR(32),
    
    CREATE_IMAGE BOOLEAN,
    READ_IMAGE BOOLEAN,
    MODIFY_IMAGE BOOLEAN,
    DELETE_IMAGE BOOLEAN,
    
    READ_RENDITIONS BOOLEAN,
    CREATE_RENDITIONS BOOLEAN,
    MODIFY_RENDITIONS BOOLEAN,
    DELETE_RENDITIONS BOOLEAN,
    
    READ_PROJECT BOOLEAN,
    CREATE_PROJECT BOOLEAN,
    MODIFY_PROJECT BOOLEAN,
    DELETE_PROJECT BOOLEAN,
    
    READ_USER BOOLEAN,
    CREATE_USER BOOLEAN,
    MODIFY_USER BOOLEAN,
    DELETE_USER BOOLEAN,
    
    PUBLISH BOOLEAN,
    
    PUBLISH_ALL BOOLEAN,
    ACCESS_ALL_PROJECTS BOOLEAN
);

CREATE TABLE USER (
    ID SMALLINT UNSIGNED PRIMARY KEY,
    LOGIN_ID VARCHAR(16) UNIQUE,
    EMAIL VARCHAR(128) UNIQUE,
    NAME VARCHAR(64) NULL,
    USER_ROLE TINYINT UNSIGNED,
    LAST_LOGIN_ON DATETIME NULL,
    CREATED_BY SMALLINT UNSIGNED DEFAULT 0,
    MODIFIED_BY SMALLINT UNSIGNED DEFAULT 0,
    CREATED_ON DATETIME DEFAULT NOW(),
    MODIFIED_ON DATETIME DEFAULT NULL,
    FOREIGN KEY (USER_ROLE) REFERENCES USER_ROLE(ID)
)

CREATE TABLE PROJECT (
    ID SMALLINT UNSIGNED PRIMARY KEY,
    NAME VARCHAR(32),
    SLUG VARCHAR(32),
    DESCRIPTION VARCHAR(256),
    CREATED_BY SMALLINT UNSIGNED DEFAULT 0,
    MODIFIED_BY SMALLINT UNSIGNED DEFAULT 0,
    CREATED_ON DATETIME DEFAULT NOW(),
    MODIFIED_ON DATETIME DEFAULT NULL
);

CREATE TABLE FOLDER (
    ID SMALLINT UNSIGNED PRIMARY KEY,
    TITLE VARCHAR(128),
    SLUG VARCHAR(128),
    PROJECT_ID SMALLINT UNSIGNED,
    DESCRIPTION VARCHAR(256) DEFAULT NULL,
    PARENT_FOLDER_ID SMALLINT UNSIGNED DEFAULT NULL,
    CREATED_BY SMALLINT UNSIGNED DEFAULT 0,
    MODIFIED_BY SMALLINT UNSIGNED DEFAULT 0,
    CREATED_ON DATETIME DEFAULT NOW(),
    MODIFIED_ON DATETIME DEFAULT NULL
);

CREATE TABLE IMAGE (
    ID INT UNSIGNED PRIMARY KEY,
    ORIGINAL_FILENAME VARCHAR(1000),
    TITLE VARCHAR(256),
    SLUG VARCHAR(256) UNIQUE,
    HEIGHT SMALLINT UNSIGNED DEFAULT 0,
    WIDTH SMALLINT UNSIGNED DEFAULT 0,
    PUBLISHED BOOLEAN DEFAULT NULL,
    PROJECT_ID SMALLINT UNSIGNED,
    FOLDER_ID SMALLINT UNSIGNED DEFAULT NULL,
    CREATED_BY SMALLINT UNSIGNED DEFAULT 0,
    MODIFIED_BY SMALLINT UNSIGNED DEFAULT 0,
    CREATED_ON DATETIME DEFAULT NULL,
    MODIFIED_ON DATETIME DEFAULT NULL
);

CREATE TABLE IMAGE_RENDITION (
    ID INT UNSIGNED PRIMARY KEY,
    IMAGE_ID INT UNSIGNED,
    HEIGHT SMALLINT UNSIGNED DEFAULT 0,
    WIDTH SMALLINT UNSIGNED DEFAULT 0,
    TARGET_DEVICE VARCHAR(16) DEFAULT NULL,
    NAME VARCHAR(24) DEFAULT NULL,
    SLUG VARCHAR(24) UNIQUE,
    PUBLISHED BOOLEAN DEFAULT FALSE,
    MIME_TYPE VARCHAR(32),       -- e.g. image/jpeg
    CREATED_BY SMALLINT UNSIGNED DEFAULT 0,
    MODIFIED_BY SMALLINT UNSIGNED DEFAULT 0,
    CREATED_ON DATETIME DEFAULT NOW(),
    MODIFIED_ON DATETIME DEFAULT NULL,

    FOREIGN KEY (IMAGE_ID) REFERENCES IMAGE(ID)
);
