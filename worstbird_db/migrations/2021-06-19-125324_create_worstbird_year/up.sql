create table worstbird_year(
    bird_id integer references bird,
    year integer,
    votes integer not null,
    primary key(bird_id, year)
)
