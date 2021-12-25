DROP SCHEMA IF EXISTS musicbrainz CASCADE;

CREATE SCHEMA musicbrainz;

CREATE TABLE artist_release (
    -- `is_track_artist` is TRUE only if the artist came from a track
    -- AC and does not also appear in the release AC. Track artists
    -- that appear in the release AC are not stored.
    is_track_artist                     BOOLEAN NOT NULL,
    artist                              INTEGER NOT NULL, -- references artist.id, CASCADE
    first_release_date                  INTEGER,
    catalog_numbers                     TEXT[],
    country_code                        CHAR(2),
    barcode                             BIGINT,
    -- Prior to adding these materialized tables, we'd order releases
    -- by name only if all other attributes where equal. It's not too
    -- common that an artist will have tons of releases with no dates,
    -- catalog numbers, countries, or barcodes (though it can be seen
    -- on some big composers). As a compromise between dropping the
    -- name sorting and having to store the entire name here (which,
    -- as a reminder, is duplicated for every artist on the release),
    -- we only store the first character of the name for sorting.
    sort_character                      CHAR(1) NOT NULL,
    release                             INTEGER NOT NULL -- references release.id, CASCADE
) PARTITION BY LIST (is_track_artist);

CREATE TABLE artist_release_nonva
    PARTITION OF artist_release FOR VALUES IN (FALSE);

CREATE TABLE artist_release_va
    PARTITION OF artist_release FOR VALUES IN (TRUE);

CREATE TABLE artist ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    type                INTEGER, -- references artist_type.id
    area                INTEGER, -- references area.id
    gender              INTEGER, -- references gender.id
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CONSTRAINT artist_ended_check CHECK (
        (
          -- If any end date fields are not null, then ended must be true
          (end_date_year IS NOT NULL OR
           end_date_month IS NOT NULL OR
           end_date_day IS NOT NULL) AND
          ended = TRUE
        ) OR (
          -- Otherwise, all end date fields must be null
          (end_date_year IS NULL AND
           end_date_month IS NULL AND
           end_date_day IS NULL)
        )
      ),
    begin_area          INTEGER, -- references area.id
    end_area            INTEGER -- references area.id
);

CREATE TABLE release ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    artist_credit       INTEGER NOT NULL, -- references artist_credit.id
    release_group       INTEGER NOT NULL, -- references release_group.id
    status              INTEGER, -- references release_status.id
    packaging           INTEGER, -- references release_packaging.id
    language            INTEGER, -- references language.id
    script              INTEGER, -- references script.id
    barcode             VARCHAR(255),
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    quality             SMALLINT NOT NULL DEFAULT -1,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE artist ADD CONSTRAINT artist_pkey PRIMARY KEY (id);
ALTER TABLE release ADD CONSTRAINT release_pkey PRIMARY KEY (id);

ALTER TABLE artist_release
   ADD CONSTRAINT artist_release_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE artist_release
   ADD CONSTRAINT artist_release_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id)
   ON DELETE CASCADE;
