
create table artists (
    artist_id integer primary key not null,
    artist_name varchar(64) not null,
    unique(artist_name)
);

create table albums(
    album_id integer primary key not null,
    album_title varchar(128) not null,
    artist_id integer not null,
    year int,
    total_tracks int,
    unique(album_title, artist_id),
    foreign key(artist_id) references artists(artist_id)
);

create table tracks(
    track_id integer primary key not null,
    title varchar(128) not null,
    album_id integer not null,
    lyrics text not null,
    comments varchar(256) not null,
    genre varchar(64) not null,
    track_number int,
    -- picture
    path varchar(4096) not null,
    duration int not null,
    bitrate int not null,
    samplerate int not null,
    channels int not null,
    unique(title, album_id),
    foreign key(album_id) references albums(album_id)
);
