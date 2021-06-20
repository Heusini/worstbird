create table worstbird_month (
    bird_id integer references bird,
    month integer,
    year integer,
    votes integer not null, 
    Primary key (bird_id, month, year)
)
