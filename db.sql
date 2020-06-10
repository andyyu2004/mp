create table artist (
  artist_id serial primary key,
  artist_name varchar(128),
);

create table album (
  album_id serial primary key,

);

create table track (
    track_id serial primary key,
    title varchar(256),
);
