CREATE TABLE groups (
    id SERIAL  PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    url CHAR(32) NOT NULL ,
    is_del BOOLEARN,
    UNIQUE url
); 
CREATE TABLE nodes (
    id SERIAL  PRIMARY KEY,
    group_id INT NOT NULL,
    name VARCHAR(50) NOT NULL,
    scheme NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INT NOT NULL DEFAULT 443,
    password VARCHAR(255) NULL,
    path VARCHAR(255) NULL,
    uuid CHAR(36) NULL,
    alter_id INT NULL,
    cipher VARCHAR(255) NULL,
    username VARCHAR(255) NULL,
    is_del BOOLEARN
); 

CREATE TABLE cfips (
    id SERIAL  PRIMARY KEY,
    ip VARCHAR(15) NOT NULL,
    label VARCHAR(50) NOT NULL,
    code CHAR(10) NOT NULL,
    is_del BOOLEARN,
    UNIQUE(code)
);

CREATE TABLE auths (
    id SERIAL  PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_del BOOLEARN,
    UNIQUE(email)
);
