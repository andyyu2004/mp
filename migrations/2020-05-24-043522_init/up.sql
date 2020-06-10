create table artist (
  artist_id serial primary key,
  artist_name varchar(128) not null
);

create table album (
  album_id serial primary key,
  artist_id serial references artist not null
);

create table track (
  track_id serial primary key,
  title varchar(256) not null,
  album_id serial references album not null
);
