create table bird (
    id SERIAL PRIMARY KEY,
    name VARCHAR not null,
    description VARCHAR not null,
    assetID VARCHAR not null unique,
    url VARCHAR not null,
    width integer not null,
    height integer not null
)
