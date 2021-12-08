DROP SCHEMA IF EXISTS musicbrainz CASCADE;

CREATE SCHEMA musicbrainz;

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/Extensions.sql

CREATE EXTENSION IF NOT EXISTS cube WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS earthdistance WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS unaccent WITH SCHEMA public;

-- Substitute public.ll_to_earth for a version that schema-qualifies
-- references to public.cube and public.earth, to avoid errors during
-- inlining/execution of the function while running an autovacuum or
-- pg_upgrade.
--
-- The issue in question arises since postgres 10.3: a change was made to
-- only include pg_catalog in the search_path settings of postgres client
-- programs in order to resolve a security issue. So, since `cube` and
-- `earth` are installed in the public schema, they're invisible in these
-- contexts without the qualification.

CREATE OR REPLACE FUNCTION musicbrainz.ll_to_earth(float8, float8)
RETURNS public.earth
LANGUAGE SQL
IMMUTABLE STRICT
PARALLEL SAFE
AS 'SELECT public.cube(public.cube(public.cube(public.earth()*cos(radians($1))*cos(radians($2))),public.earth()*cos(radians($1))*sin(radians($2))),public.earth()*sin(radians($1)))::public.earth';

-- The unaccent function, but IMMUTABLE. Based on a solution provided by
-- Erwin Brandstetter in [1], which removes the dependency on search_path.
-- Warning: changing the unaccent dictionary on the filesystem can still
-- break the IMMUTABLE assumption.
--
-- The answer in [1] suggests that using a C function for immutable_unaccent
-- allows it to be inlined in musicbrainz_unaccent below, and is 10x faster
-- than the fallback. Although this script is meant to execute as a
-- superuser, a fallback is provided specifically for instances where even
-- the "superuser" is restricted in creating C language functions, e.g.
-- Amazon RDS.
--
-- [1] https://stackoverflow.com/a/11007216

DO $$
BEGIN
  CREATE OR REPLACE FUNCTION public.immutable_unaccent(regdictionary, text)
    RETURNS text LANGUAGE c IMMUTABLE PARALLEL SAFE STRICT AS
  '$libdir/unaccent', 'unaccent_dict';

  CREATE OR REPLACE FUNCTION musicbrainz.musicbrainz_unaccent(text)
    RETURNS text LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT AS
  $func$
    SELECT public.immutable_unaccent(regdictionary 'public.unaccent', $1)
  $func$;
EXCEPTION
  WHEN insufficient_privilege THEN
    CREATE OR REPLACE FUNCTION musicbrainz.musicbrainz_unaccent(text)
      RETURNS text LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT AS
    $func$
      SELECT public.unaccent('public.unaccent', $1)
    $func$;
END
$$

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateCollations.sql


SET search_path = musicbrainz, public;

CREATE COLLATION musicbrainz (
    provider = icu,
    locale = '@colCaseFirst=lower;colNumeric=yes'
);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateTables.sql

CREATE TABLE alternative_release ( -- replicate
    id                      SERIAL, -- PK
    gid                     UUID NOT NULL,
    release                 INTEGER NOT NULL, -- references release.id
    name                    VARCHAR,
    artist_credit           INTEGER, -- references artist_credit.id
    type                    INTEGER NOT NULL, -- references alternative_release_type.id
    language                INTEGER NOT NULL, -- references language.id
    script                  INTEGER NOT NULL, -- references script.id
    comment                 VARCHAR(255) NOT NULL DEFAULT ''
    CHECK (name != '')
);

CREATE TABLE alternative_release_type ( -- replicate
    id                  SERIAL, -- PK
    name                TEXT NOT NULL,
    parent              INTEGER, -- references alternative_release_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 UUID NOT NULL
);

CREATE TABLE alternative_medium ( -- replicate
    id                      SERIAL, -- PK
    medium                  INTEGER NOT NULL, -- FK, references medium.id
    alternative_release     INTEGER NOT NULL, -- references alternative_release.id
    name                    VARCHAR
    CHECK (name != '')
);

CREATE TABLE alternative_track ( -- replicate
    id                      SERIAL, -- PK
    name                    VARCHAR,
    artist_credit           INTEGER, -- references artist_credit.id
    ref_count               INTEGER NOT NULL DEFAULT 0
    CHECK (name != '' AND (name IS NOT NULL OR artist_credit IS NOT NULL))
);

CREATE TABLE alternative_medium_track ( -- replicate
    alternative_medium      INTEGER NOT NULL, -- PK, references alternative_medium.id
    track                   INTEGER NOT NULL, -- PK, references track.id
    alternative_track       INTEGER NOT NULL -- references alternative_track.id
);

CREATE TABLE annotation ( -- replicate (verbose)
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    text                TEXT,
    changelog           VARCHAR(255),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE application
(
    id                  SERIAL,
    owner               INTEGER NOT NULL, -- references editor.id
    name                TEXT NOT NULL,
    oauth_id            TEXT NOT NULL,
    oauth_secret        TEXT NOT NULL,
    oauth_redirect_uri  TEXT
);

CREATE TABLE area_type ( -- replicate
    id                  SERIAL, -- PK
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references area_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE area ( -- replicate (verbose)
    id                  SERIAL, -- PK
    gid                 uuid NOT NULL,
    name                VARCHAR NOT NULL,
    type                INTEGER, -- references area_type.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    comment             VARCHAR(255) NOT NULL DEFAULT ''
);

CREATE TABLE area_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references area.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE area_alias_type ( -- replicate
    id                  SERIAL, -- PK,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references area_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE area_alias ( -- replicate (verbose)
    id                  SERIAL, --PK
    area                INTEGER NOT NULL, -- references area.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references area_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
             CONSTRAINT primary_check
                 CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)));

CREATE TABLE area_annotation ( -- replicate (verbose)
    area        INTEGER NOT NULL, -- PK, references area.id
    annotation  INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE area_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references area_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE area_attribute_type_allowed_value ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    area_attribute_type INTEGER NOT NULL, -- references area_attribute_type.id
    value               TEXT,
    parent              INTEGER, -- references area_attribute_type_allowed_value.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE area_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    area                                INTEGER NOT NULL, -- references area.id
    area_attribute_type                 INTEGER NOT NULL, -- references area_attribute_type.id
    area_attribute_type_allowed_value   INTEGER, -- references area_attribute_type_allowed_value.id
    area_attribute_text                 TEXT
    CHECK (
        (area_attribute_type_allowed_value IS NULL AND area_attribute_text IS NOT NULL)
        OR
        (area_attribute_type_allowed_value IS NOT NULL AND area_attribute_text IS NULL)
    )
);

CREATE TABLE area_tag ( -- replicate (verbose)
    area                INTEGER NOT NULL, -- PK, references area.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE area_tag_raw (
    area                INTEGER NOT NULL, -- PK, references area.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

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

CREATE TABLE artist_alias_type ( -- replicate
    id                  SERIAL,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references artist_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE artist_alias ( -- replicate (verbose)
    id                  SERIAL,
    artist              INTEGER NOT NULL, -- references artist.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references artist_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 3) OR (
          type = 3 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE artist_annotation ( -- replicate (verbose)
    artist              INTEGER NOT NULL, -- PK, references artist.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE artist_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references artist_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE artist_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    artist_attribute_type       INTEGER NOT NULL, -- references artist_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references artist_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE artist_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    artist                              INTEGER NOT NULL, -- references artist.id
    artist_attribute_type               INTEGER NOT NULL, -- references artist_attribute_type.id
    artist_attribute_type_allowed_value INTEGER, -- references artist_attribute_type_allowed_value.id
    artist_attribute_text               TEXT
    CHECK (
        (artist_attribute_type_allowed_value IS NULL AND artist_attribute_text IS NOT NULL)
        OR
        (artist_attribute_type_allowed_value IS NOT NULL AND artist_attribute_text IS NULL)
    )
);

CREATE TABLE artist_ipi ( -- replicate (verbose)
    artist              INTEGER NOT NULL, -- PK, references artist.id
    ipi                 CHAR(11) NOT NULL CHECK (ipi ~ E'^\\d{11}$'), -- PK
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE artist_isni ( -- replicate (verbose)
    artist              INTEGER NOT NULL, -- PK, references artist.id
    isni                CHAR(16) NOT NULL CHECK (isni ~ E'^\\d{15}[\\dX]$'), -- PK
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE artist_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references artist.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE artist_tag ( -- replicate (verbose)
    artist              INTEGER NOT NULL, -- PK, references artist.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE artist_rating_raw
(
    artist              INTEGER NOT NULL, -- PK, references artist.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE artist_tag_raw
(
    artist              INTEGER NOT NULL, -- PK, references artist.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE artist_credit ( -- replicate
    id                  SERIAL,
    name                VARCHAR NOT NULL,
    artist_count        SMALLINT NOT NULL,
    ref_count           INTEGER DEFAULT 0,
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0)
);

CREATE TABLE artist_credit_name ( -- replicate (verbose)
    artist_credit       INTEGER NOT NULL, -- PK, references artist_credit.id CASCADE
    position            SMALLINT NOT NULL, -- PK
    artist              INTEGER NOT NULL, -- references artist.id CASCADE
    name                VARCHAR NOT NULL,
    join_phrase         TEXT NOT NULL DEFAULT ''
);

CREATE TABLE artist_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references artist.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE artist_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references artist_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

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
    sort_character                      CHAR(1) COLLATE musicbrainz NOT NULL,
    release                             INTEGER NOT NULL -- references release.id, CASCADE
) PARTITION BY LIST (is_track_artist);

CREATE TABLE artist_release_nonva
    PARTITION OF artist_release FOR VALUES IN (FALSE);

CREATE TABLE artist_release_va
    PARTITION OF artist_release FOR VALUES IN (TRUE);

-- The set of triggers keeping the `artist_release` table up-to-date
-- (which can be found in admin/sql/CreateSlaveOnlyTriggers.sql) don't
-- update the table directly. The query to do that for a particular
-- release can be moderately heavy if there are a lot of tracks, so it
-- would degrade performance if identical updates to the same release
-- were triggered many times in the same transaction (which is very
-- easy to trigger when adding, editing, or removing tracks). The
-- strategy we use is to instead push release IDs that need updating
-- to the `artist_release_pending_update` table, and perform the
-- actual updates in a DEFERRED trigger at the end of the transaction.
-- (For where this happens, see the apply_*_pending_updates functions.)
CREATE TABLE artist_release_pending_update (
    release INTEGER NOT NULL
);

CREATE TABLE artist_release_group (
    -- See comment for `artist_release.is_track_artist`.
    is_track_artist                     BOOLEAN NOT NULL,
    artist                              INTEGER NOT NULL, -- references artist.id, CASCADE
    unofficial                          BOOLEAN NOT NULL,
    primary_type                        SMALLINT,
    secondary_types                     SMALLINT[],
    first_release_date                  INTEGER,
    -- See comment for `artist_release.sort_character`.
    sort_character                      CHAR(1) COLLATE musicbrainz NOT NULL,
    release_group                       INTEGER NOT NULL -- references release_group.id, CASCADE
) PARTITION BY LIST (is_track_artist);

CREATE TABLE artist_release_group_nonva
    PARTITION OF artist_release_group FOR VALUES IN (FALSE);

CREATE TABLE artist_release_group_va
    PARTITION OF artist_release_group FOR VALUES IN (TRUE);

-- Please see the comment above `artist_release_pending_update`
-- (the same idea applies).
CREATE TABLE artist_release_group_pending_update (
    release_group INTEGER NOT NULL
);

CREATE TABLE autoeditor_election
(
    id                  SERIAL,
    candidate           INTEGER NOT NULL, -- references editor.id
    proposer            INTEGER NOT NULL, -- references editor.id
    seconder_1          INTEGER, -- references editor.id
    seconder_2          INTEGER, -- references editor.id
    status              INTEGER NOT NULL DEFAULT 1
                            CHECK (status IN (1,2,3,4,5,6)),
                            -- 1 : has proposer
                            -- 2 : has seconder_1
                            -- 3 : has seconder_2 (voting open)
                            -- 4 : accepted!
                            -- 5 : rejected
                            -- 6 : cancelled (by proposer)
    yes_votes           INTEGER NOT NULL DEFAULT 0,
    no_votes            INTEGER NOT NULL DEFAULT 0,
    propose_time        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    open_time           TIMESTAMP WITH TIME ZONE,
    close_time          TIMESTAMP WITH TIME ZONE
);

CREATE TABLE autoeditor_election_vote
(
    id                  SERIAL,
    autoeditor_election INTEGER NOT NULL, -- references autoeditor_election.id
    voter               INTEGER NOT NULL, -- references editor.id
    vote                INTEGER NOT NULL CHECK (vote IN (-1,0,1)),
    vote_time           TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE cdtoc ( -- replicate
    id                  SERIAL,
    discid              CHAR(28) NOT NULL,
    freedb_id           CHAR(8) NOT NULL,
    track_count         INTEGER NOT NULL,
    leadout_offset      INTEGER NOT NULL,
    track_offset        INTEGER[] NOT NULL,
    degraded            BOOLEAN NOT NULL DEFAULT FALSE,
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE cdtoc_raw ( -- replicate
    id                  SERIAL, -- PK
    release             INTEGER NOT NULL, -- references release_raw.id
    discid              CHAR(28) NOT NULL,
    track_count          INTEGER NOT NULL,
    leadout_offset       INTEGER NOT NULL,
    track_offset         INTEGER[] NOT NULL
);

CREATE TABLE country_area ( -- replicate (verbose)
    area                INTEGER -- PK, references area.id
);

CREATE TABLE deleted_entity (
    gid UUID NOT NULL, -- PK
    data JSONB NOT NULL,
    deleted_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE edit
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    type                SMALLINT NOT NULL,
    status              SMALLINT NOT NULL,
    autoedit            SMALLINT NOT NULL DEFAULT 0,
    open_time            TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    close_time           TIMESTAMP WITH TIME ZONE,
    expire_time          TIMESTAMP WITH TIME ZONE NOT NULL,
    language            INTEGER, -- references language.id
    quality             SMALLINT NOT NULL DEFAULT 1
);

CREATE TABLE edit_data
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    data                JSONB NOT NULL
);

CREATE TABLE edit_note
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    edit                INTEGER NOT NULL, -- references edit.id
    text                TEXT NOT NULL,
    post_time            TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE edit_note_recipient (
    recipient           INTEGER NOT NULL, -- PK, references editor.id
    edit_note           INTEGER NOT NULL  -- PK, references edit_note.id
);

CREATE TABLE edit_area
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    area                INTEGER NOT NULL  -- PK, references area.id CASCADE
);

CREATE TABLE edit_artist
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    artist              INTEGER NOT NULL, -- PK, references artist.id CASCADE
    status              SMALLINT NOT NULL -- materialized from edit.status
);

CREATE TABLE edit_event
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    event               INTEGER NOT NULL  -- PK, references event.id CASCADE
);

CREATE TABLE edit_instrument
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    instrument          INTEGER NOT NULL  -- PK, references instrument.id CASCADE
);

CREATE TABLE edit_label
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    label               INTEGER NOT NULL, -- PK, references label.id CASCADE
    status              SMALLINT NOT NULL -- materialized from edit.status
);

CREATE TABLE edit_place
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    place               INTEGER NOT NULL  -- PK, references place.id CASCADE
);

CREATE TABLE edit_release
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    release             INTEGER NOT NULL  -- PK, references release.id CASCADE
);

CREATE TABLE edit_release_group
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    release_group       INTEGER NOT NULL  -- PK, references release_group.id CASCADE
);

CREATE TABLE edit_recording
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    recording           INTEGER NOT NULL  -- PK, references recording.id CASCADE
);

CREATE TABLE edit_series
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    series              INTEGER NOT NULL  -- PK, references series.id CASCADE
);

CREATE TABLE edit_work
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    work                INTEGER NOT NULL  -- PK, references work.id CASCADE
);

CREATE TABLE edit_url
(
    edit                INTEGER NOT NULL, -- PK, references edit.id
    url                 INTEGER NOT NULL  -- PK, references url.id CASCADE
);

CREATE TABLE editor
(
    id                  SERIAL,
    name                VARCHAR(64) NOT NULL,
    privs               INTEGER DEFAULT 0,
    email               VARCHAR(64) DEFAULT NULL,
    website             VARCHAR(255) DEFAULT NULL,
    bio                 TEXT DEFAULT NULL,
    member_since        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    email_confirm_date  TIMESTAMP WITH TIME ZONE,
    last_login_date     TIMESTAMP WITH TIME ZONE DEFAULT now(),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    birth_date          DATE,
    gender              INTEGER, -- references gender.id
    area                INTEGER, -- references area.id
    password            VARCHAR(128) NOT NULL,
    ha1                 CHAR(32) NOT NULL,
    deleted             BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE old_editor_name (
    name    VARCHAR(64) NOT NULL
);

CREATE TYPE FLUENCY AS ENUM ('basic', 'intermediate', 'advanced', 'native');

CREATE TABLE editor_language (
    editor   INTEGER NOT NULL,  -- PK, references editor.id
    language INTEGER NOT NULL,  -- PK, references language.id
    fluency  FLUENCY NOT NULL
);

CREATE TABLE editor_preference
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    name                VARCHAR(50) NOT NULL,
    value               VARCHAR(100) NOT NULL
);

CREATE TABLE editor_subscribe_artist
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    artist              INTEGER NOT NULL, -- references artist.id
    last_edit_sent      INTEGER NOT NULL -- references edit.id
);

CREATE TABLE editor_subscribe_artist_deleted
(
    editor INTEGER NOT NULL, -- PK, references editor.id
    gid UUID NOT NULL, -- PK, references deleted_entity.gid
    deleted_by INTEGER NOT NULL -- references edit.id
);

CREATE TABLE editor_subscribe_collection
(
    id                  SERIAL,
    editor              INTEGER NOT NULL,              -- references editor.id
    collection          INTEGER NOT NULL,              -- weakly references editor_collection.id
    last_edit_sent      INTEGER NOT NULL,              -- weakly references edit.id
    available           BOOLEAN NOT NULL DEFAULT TRUE,
    last_seen_name      VARCHAR(255)
);

CREATE TABLE editor_subscribe_label
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    label               INTEGER NOT NULL, -- references label.id
    last_edit_sent      INTEGER NOT NULL -- references edit.id
);

CREATE TABLE editor_subscribe_label_deleted
(
    editor INTEGER NOT NULL, -- PK, references editor.id
    gid UUID NOT NULL, -- PK, references deleted_entity.gid
    deleted_by INTEGER NOT NULL -- references edit.id
);

CREATE TABLE editor_subscribe_editor
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id (the one who has subscribed)
    subscribed_editor   INTEGER NOT NULL, -- references editor.id (the one being subscribed)
    last_edit_sent      INTEGER NOT NULL  -- weakly references edit.id
);

CREATE TABLE editor_subscribe_series
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    series              INTEGER NOT NULL, -- references series.id
    last_edit_sent      INTEGER NOT NULL -- references edit.id
);

CREATE TABLE editor_subscribe_series_deleted
(
    editor              INTEGER NOT NULL, -- PK, references editor.id
    gid                 UUID NOT NULL, -- PK, references deleted_entity.gid
    deleted_by          INTEGER NOT NULL -- references edit.id
);

CREATE TABLE event ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    time                TIME WITHOUT TIME ZONE,
    type                INTEGER, -- references event_type.id
    cancelled           BOOLEAN NOT NULL DEFAULT FALSE,
    setlist             TEXT,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CONSTRAINT event_ended_check CHECK (
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
      )
);

CREATE TYPE event_art_presence AS ENUM ('absent', 'present', 'darkened');

CREATE TABLE event_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references event.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER,
    event_art_presence  event_art_presence NOT NULL DEFAULT 'absent'
);

CREATE TABLE event_rating_raw (
    event               INTEGER NOT NULL, -- PK, references event.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE event_tag_raw (
    event               INTEGER NOT NULL, -- PK, references event.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE event_alias_type ( -- replicate
    id                  SERIAL,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references event_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE event_alias ( -- replicate (verbose)
    id                  SERIAL,
    event               INTEGER NOT NULL, -- references event.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references event_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE event_annotation ( -- replicate (verbose)
    event               INTEGER NOT NULL, -- PK, references event.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE event_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references event_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE event_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    event_attribute_type        INTEGER NOT NULL, -- references event_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references event_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE event_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    event                               INTEGER NOT NULL, -- references event.id
    event_attribute_type                INTEGER NOT NULL, -- references event_attribute_type.id
    event_attribute_type_allowed_value  INTEGER, -- references event_attribute_type_allowed_value.id
    event_attribute_text                TEXT
    CHECK (
        (event_attribute_type_allowed_value IS NULL AND event_attribute_text IS NOT NULL)
        OR
        (event_attribute_type_allowed_value IS NOT NULL AND event_attribute_text IS NULL)
    )
);

CREATE TABLE event_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references event.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE event_tag ( -- replicate (verbose)
    event               INTEGER NOT NULL, -- PK, references event.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE event_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references event_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_first_release_date (
    release     INTEGER NOT NULL, -- PK, references release.id CASCADE
    year        SMALLINT,
    month       SMALLINT,
    day         SMALLINT
);

CREATE TABLE recording_first_release_date (
    recording   INTEGER NOT NULL, -- PK, references recording.id CASCADE
    year        SMALLINT,
    month       SMALLINT,
    day         SMALLINT
);

CREATE TABLE gender ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references gender.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE genre ( -- replicate (verbose)
    id                  SERIAL, -- PK
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE genre_alias ( -- replicate (verbose)
    id                  SERIAL,
    genre               INTEGER NOT NULL, -- references genre.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    primary_for_locale  BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL))
);

CREATE TABLE instrument_type ( -- replicate
    id                  SERIAL, -- PK
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references instrument_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE instrument ( -- replicate (verbose)
    id                  SERIAL, -- PK
    gid                 uuid NOT NULL,
    name                VARCHAR NOT NULL,
    type                INTEGER, -- references instrument_type.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    description         TEXT NOT NULL DEFAULT ''
);

CREATE TABLE instrument_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references instrument.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE instrument_alias_type ( -- replicate
    id                  SERIAL, -- PK,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references instrument_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE instrument_alias ( -- replicate (verbose)
    id                  SERIAL, --PK
    instrument          INTEGER NOT NULL, -- references instrument.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references instrument_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE instrument_annotation ( -- replicate (verbose)
    instrument  INTEGER NOT NULL, -- PK, references instrument.id
    annotation  INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE instrument_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references instrument_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE instrument_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    instrument_attribute_type   INTEGER NOT NULL, -- references instrument_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references instrument_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE instrument_attribute ( -- replicate (verbose)
    id                                          SERIAL,  -- PK
    instrument                                  INTEGER NOT NULL, -- references instrument.id
    instrument_attribute_type                   INTEGER NOT NULL, -- references instrument_attribute_type.id
    instrument_attribute_type_allowed_value     INTEGER, -- references instrument_attribute_type_allowed_value.id
    instrument_attribute_text                   TEXT
    CHECK (
        (instrument_attribute_type_allowed_value IS NULL AND instrument_attribute_text IS NOT NULL)
        OR
        (instrument_attribute_type_allowed_value IS NOT NULL AND instrument_attribute_text IS NULL)
    )
);

CREATE TABLE instrument_tag ( -- replicate (verbose)
    instrument          INTEGER NOT NULL, -- PK, references instrument.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE instrument_tag_raw (
    instrument          INTEGER NOT NULL, -- PK, references instrument.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE iso_3166_1 ( -- replicate
    area      INTEGER NOT NULL, -- references area.id
    code      CHAR(2) -- PK
);
CREATE TABLE iso_3166_2 ( -- replicate
    area      INTEGER NOT NULL, -- references area.id
    code      VARCHAR(10) -- PK
);
CREATE TABLE iso_3166_3 ( -- replicate
    area      INTEGER NOT NULL, -- references area.id
    code      CHAR(4) -- PK
);

CREATE TABLE isrc ( -- replicate (verbose)
    id                  SERIAL,
    recording           INTEGER NOT NULL, -- references recording.id
    isrc                CHAR(12) NOT NULL CHECK (isrc ~ E'^[A-Z]{2}[A-Z0-9]{3}[0-9]{7}$'),
    source              SMALLINT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE iswc ( -- replicate (verbose)
    id SERIAL NOT NULL,
    work INTEGER NOT NULL, -- references work.id
    iswc CHARACTER(15) CHECK (iswc ~ E'^T-?\\d{3}.?\\d{3}.?\\d{3}[-.]?\\d$'),
    source SMALLINT,
    edits_pending INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE l_area_area ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references area.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_artist ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references artist.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_event ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references event.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_instrument ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references instrument.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_label ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references label.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_area_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references area.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_artist ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references artist.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_event ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references event.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_instrument ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references instrument.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_label ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references label.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_artist_work ( -- replicate (verbose)
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references artist.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_event ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references event.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_instrument ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references instrument.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_label ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references label.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_event_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references event.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_label ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references label.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_instrument ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references instrument.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_label ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references label.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_instrument_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references instrument.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_label_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references label.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_place ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references place.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_place_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references place.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_recording ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references recording.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_recording_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references recording.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_release ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release.id
    entity1             INTEGER NOT NULL, -- references release.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_group_release_group ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release_group.id
    entity1             INTEGER NOT NULL, -- references release_group.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_group_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release_group.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_group_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release_group.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_release_group_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references release_group.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_series_series ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references series.id
    entity1             INTEGER NOT NULL, -- references series.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_series_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references series.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_series_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references series.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_url_url ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references url.id
    entity1             INTEGER NOT NULL, -- references url.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_url_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references url.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE l_work_work ( -- replicate
    id                  SERIAL,
    link                INTEGER NOT NULL, -- references link.id
    entity0             INTEGER NOT NULL, -- references work.id
    entity1             INTEGER NOT NULL, -- references work.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link_order          INTEGER NOT NULL DEFAULT 0 CHECK (link_order >= 0),
    entity0_credit      TEXT NOT NULL DEFAULT '',
    entity1_credit      TEXT NOT NULL DEFAULT ''
);

CREATE TABLE label ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    label_code          INTEGER CHECK (label_code > 0 AND label_code < 100000),
    type                INTEGER, -- references label_type.id
    area                INTEGER, -- references area.id
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CONSTRAINT label_ended_check CHECK (
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
      )
);

CREATE TABLE label_rating_raw
(
    label               INTEGER NOT NULL, -- PK, references label.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE label_tag_raw
(
    label               INTEGER NOT NULL, -- PK, references label.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE label_alias_type ( -- replicate
    id                  SERIAL,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references label_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE label_alias ( -- replicate (verbose)
    id                  SERIAL,
    label               INTEGER NOT NULL, -- references label.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references label_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE label_annotation ( -- replicate (verbose)
    label               INTEGER NOT NULL, -- PK, references label.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE label_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references label_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE label_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    label_attribute_type        INTEGER NOT NULL, -- references label_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references label_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE label_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    label                               INTEGER NOT NULL, -- references label.id
    label_attribute_type                INTEGER NOT NULL, -- references label_attribute_type.id
    label_attribute_type_allowed_value  INTEGER, -- references label_attribute_type_allowed_value.id
    label_attribute_text                TEXT
    CHECK (
        (label_attribute_type_allowed_value IS NULL AND label_attribute_text IS NOT NULL)
        OR
        (label_attribute_type_allowed_value IS NOT NULL AND label_attribute_text IS NULL)
    )
);

CREATE TABLE label_ipi ( -- replicate (verbose)
    label               INTEGER NOT NULL, -- PK, references label.id
    ipi                 CHAR(11) NOT NULL CHECK (ipi ~ E'^\\d{11}$'), -- PK
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE label_isni ( -- replicate (verbose)
    label               INTEGER NOT NULL, -- PK, references label.id
    isni                CHAR(16) NOT NULL CHECK (isni ~ E'^\\d{15}[\\dX]$'), -- PK
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE label_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references label.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE label_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references label.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE label_tag ( -- replicate (verbose)
    label               INTEGER NOT NULL, -- PK, references label.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE label_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references label_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE language ( -- replicate
    id                  SERIAL,
    iso_code_2t         CHAR(3), -- ISO 639-2 (T)
    iso_code_2b         CHAR(3), -- ISO 639-2 (B)
    iso_code_1          CHAR(2), -- ISO 639
    name                VARCHAR(100) NOT NULL,
    frequency           SMALLINT NOT NULL DEFAULT 0,
    iso_code_3          CHAR(3)  -- ISO 639-3
);

ALTER TABLE language
      ADD CONSTRAINT iso_code_check
      CHECK (iso_code_2t IS NOT NULL OR iso_code_3  IS NOT NULL);

CREATE TABLE link ( -- replicate
    id                  SERIAL,
    link_type           INTEGER NOT NULL, -- references link_type.id
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    attribute_count     INTEGER NOT NULL DEFAULT 0,
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CONSTRAINT link_ended_check CHECK (
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
      )
);

CREATE TABLE link_attribute ( -- replicate
    link                INTEGER NOT NULL, -- PK, references link.id
    attribute_type      INTEGER NOT NULL, -- PK, references link_attribute_type.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE link_attribute_type ( -- replicate
    id                  SERIAL,
    parent              INTEGER, -- references link_attribute_type.id
    root                INTEGER NOT NULL, -- references link_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    gid                 UUID NOT NULL,
    name                VARCHAR(255) NOT NULL,
    description         TEXT,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE link_creditable_attribute_type ( -- replicate
  attribute_type INT NOT NULL -- PK, references link_attribute_type.id CASCADE
);

CREATE TABLE link_attribute_credit ( -- replicate
  link INT NOT NULL, -- PK, references link.id
  attribute_type INT NOT NULL, -- PK, references link_creditable_attribute_type.attribute_type
  credited_as TEXT NOT NULL
);

CREATE TABLE link_text_attribute_type ( -- replicate
    attribute_type      INT NOT NULL -- PK, references link_attribute_type.id CASCADE
);

CREATE TABLE link_attribute_text_value ( -- replicate
    link                INT NOT NULL, -- PK, references link.id
    attribute_type      INT NOT NULL, -- PK, references link_text_attribute_type.attribute_type
    text_value          TEXT NOT NULL
);

CREATE TABLE link_type ( -- replicate
    id                  SERIAL,
    parent              INTEGER, -- references link_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    gid                 UUID NOT NULL,
    entity_type0        VARCHAR(50) NOT NULL,
    entity_type1        VARCHAR(50) NOT NULL,
    name                VARCHAR(255) NOT NULL,
    description         TEXT,
    link_phrase         VARCHAR(255) NOT NULL,
    reverse_link_phrase VARCHAR(255) NOT NULL,
    long_link_phrase    VARCHAR(255) NOT NULL,
    priority            INTEGER NOT NULL DEFAULT 0,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_deprecated       BOOLEAN NOT NULL DEFAULT false,
    has_dates           BOOLEAN NOT NULL DEFAULT true,
    entity0_cardinality SMALLINT NOT NULL DEFAULT 0,
    entity1_cardinality SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE link_type_attribute_type ( -- replicate
    link_type           INTEGER NOT NULL, -- PK, references link_type.id
    attribute_type      INTEGER NOT NULL, -- PK, references link_attribute_type.id
    min                 SMALLINT,
    max                 SMALLINT,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE editor_collection
(
    id                  SERIAL,
    gid                 UUID NOT NULL,
    editor              INTEGER NOT NULL, -- references editor.id
    name                VARCHAR NOT NULL,
    public              BOOLEAN NOT NULL DEFAULT FALSE,
    description         TEXT DEFAULT '' NOT NULL,
    type                INTEGER NOT NULL -- references editor_collection_type.id
);

CREATE TABLE editor_collection_gid_redirect (
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references editor_collection.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE editor_collection_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    entity_type         VARCHAR(50) NOT NULL,
    parent              INTEGER, -- references editor_collection_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE editor_collection_collaborator (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    editor INTEGER NOT NULL -- PK, references editor.id
);

CREATE TABLE editor_collection_area (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    area INTEGER NOT NULL, -- PK, references area.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_artist (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    artist INTEGER NOT NULL, -- PK, references artist.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_event (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    event INTEGER NOT NULL, -- PK, references event.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_instrument (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    instrument INTEGER NOT NULL, -- PK, references instrument.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_label (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    label INTEGER NOT NULL, -- PK, references label.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_place (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    place INTEGER NOT NULL, -- PK, references place.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_recording (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    recording INTEGER NOT NULL, -- PK, references recording.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_release (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    release INTEGER NOT NULL, -- PK, references release.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_release_group (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    release_group INTEGER NOT NULL, -- PK, references release_group.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_series (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    series INTEGER NOT NULL, -- PK, references series.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_work (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    work INTEGER NOT NULL, -- PK, references work.id
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TABLE editor_collection_deleted_entity (
    collection INTEGER NOT NULL, -- PK, references editor_collection.id
    gid UUID NOT NULL, -- PK, references deleted_entity.gid
    added TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    position INTEGER NOT NULL DEFAULT 0 CHECK (position >= 0),
    comment TEXT DEFAULT '' NOT NULL
);

CREATE TYPE oauth_code_challenge_method AS ENUM ('plain', 'S256');

CREATE TABLE editor_oauth_token
(
    id                      SERIAL,
    editor                  INTEGER NOT NULL, -- references editor.id
    application             INTEGER NOT NULL, -- references application.id
    authorization_code      TEXT,
    refresh_token           TEXT,
    access_token            TEXT,
    expire_time             TIMESTAMP WITH TIME ZONE NOT NULL,
    scope                   INTEGER NOT NULL DEFAULT 0,
    granted                 TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    code_challenge          TEXT,
    code_challenge_method   oauth_code_challenge_method,
    CONSTRAINT valid_code_challenge CHECK (
        (code_challenge IS NULL) = (code_challenge_method IS NULL) AND
        (code_challenge IS NULL OR code_challenge ~ E'^[A-Za-z0-9.~_-]{43,128}$')
    )
);

CREATE TABLE editor_watch_preferences
(
    editor INTEGER NOT NULL, -- PK, references editor.id CASCADE
    notify_via_email BOOLEAN NOT NULL DEFAULT TRUE,
    notification_timeframe INTERVAL NOT NULL DEFAULT '1 week',
    last_checked TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE editor_watch_artist
(
    artist INTEGER NOT NULL, -- PK, references artist.id CASCADE
    editor INTEGER NOT NULL  -- PK, references editor.id CASCADE
);

CREATE TABLE editor_watch_release_group_type
(
    editor INTEGER NOT NULL, -- PK, references editor.id CASCADE
    release_group_type INTEGER NOT NULL -- PK, references release_group_primary_type.id
);

CREATE TABLE editor_watch_release_status
(
    editor INTEGER NOT NULL, -- PK, references editor.id CASCADE
    release_status INTEGER NOT NULL -- PK, references release_status.id
);

CREATE TABLE medium ( -- replicate (verbose)
    id                  SERIAL,
    release             INTEGER NOT NULL, -- references release.id
    position            INTEGER NOT NULL,
    format              INTEGER, -- references medium_format.id
    name                VARCHAR NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    track_count         INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE medium_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references medium_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE medium_attribute_type_allowed_format ( -- replicate (verbose)
    medium_format INTEGER NOT NULL, -- PK, references medium_format.id,
    medium_attribute_type INTEGER NOT NULL -- PK, references medium_attribute_type.id
);

CREATE TABLE medium_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    medium_attribute_type       INTEGER NOT NULL, -- references medium_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references medium_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE medium_attribute_type_allowed_value_allowed_format ( -- replicate (verbose)
    medium_format INTEGER NOT NULL, -- PK, references medium_format.id,
    medium_attribute_type_allowed_value INTEGER NOT NULL -- PK, references medium_attribute_type_allowed_value.id
);

CREATE TABLE medium_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    medium                              INTEGER NOT NULL, -- references medium.id
    medium_attribute_type               INTEGER NOT NULL, -- references medium_attribute_type.id
    medium_attribute_type_allowed_value INTEGER, -- references medium_attribute_type_allowed_value.id
    medium_attribute_text               TEXT
    CHECK (
        (medium_attribute_type_allowed_value IS NULL AND medium_attribute_text IS NOT NULL)
        OR
        (medium_attribute_type_allowed_value IS NOT NULL AND medium_attribute_text IS NULL)
    )
);

CREATE TABLE medium_cdtoc ( -- replicate (verbose)
    id                  SERIAL,
    medium              INTEGER NOT NULL, -- references medium.id
    cdtoc               INTEGER NOT NULL, -- references cdtoc.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE medium_format ( -- replicate
    id                  SERIAL,
    name                VARCHAR(100) NOT NULL,
    parent              INTEGER, -- references medium_format.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    year                SMALLINT,
    has_discids         BOOLEAN NOT NULL DEFAULT FALSE,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE orderable_link_type ( -- replicate
    link_type           INTEGER NOT NULL, -- PK, references link_type.id
    direction           SMALLINT NOT NULL DEFAULT 1 CHECK (direction = 1 OR direction = 2)
);

CREATE TABLE place ( -- replicate (verbose)
    id                  SERIAL, -- PK
    gid                 uuid NOT NULL,
    name                VARCHAR NOT NULL,
    type                INTEGER, -- references place_type.id
    address             VARCHAR NOT NULL DEFAULT '',
    area                INTEGER, -- references area.id
    coordinates         POINT,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
      )
);

CREATE TABLE place_alias ( -- replicate (verbose)
    id                  SERIAL,
    place               INTEGER NOT NULL, -- references place.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references place_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE place_alias_type ( -- replicate
    id                  SERIAL,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references place_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE place_annotation ( -- replicate (verbose)
    place               INTEGER NOT NULL, -- PK, references place.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE place_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references place_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE place_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    place_attribute_type        INTEGER NOT NULL, -- references place_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references place_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE place_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    place                               INTEGER NOT NULL, -- references place.id
    place_attribute_type                INTEGER NOT NULL, -- references place_attribute_type.id
    place_attribute_type_allowed_value  INTEGER, -- references place_attribute_type_allowed_value.id
    place_attribute_text                TEXT
    CHECK (
        (place_attribute_type_allowed_value IS NULL AND place_attribute_text IS NOT NULL)
        OR
        (place_attribute_type_allowed_value IS NOT NULL AND place_attribute_text IS NULL)
    )
);

CREATE TABLE place_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references place.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE place_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references place.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE place_rating_raw
(
    place               INTEGER NOT NULL, -- PK, references place.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE place_tag ( -- replicate (verbose)
    place               INTEGER NOT NULL, -- PK, references place.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE place_tag_raw
(
    place               INTEGER NOT NULL, -- PK, references place.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE place_type ( -- replicate
    id                  SERIAL, -- PK
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references place_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE replication_control ( -- replicate
    id                              SERIAL,
    current_schema_sequence         INTEGER NOT NULL,
    current_replication_sequence    INTEGER,
    last_replication_date           TIMESTAMP WITH TIME ZONE
);

CREATE TABLE recording ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    artist_credit       INTEGER NOT NULL, -- references artist_credit.id
    length              INTEGER CHECK (length IS NULL OR length > 0),
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    video               BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE recording_alias_type ( -- replicate
    id                  SERIAL, -- PK,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references recording_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE recording_alias ( -- replicate (verbose)
    id                  SERIAL, --PK
    recording           INTEGER NOT NULL, -- references recording.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references recording_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
             CONSTRAINT primary_check
                 CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)));

CREATE TABLE recording_rating_raw
(
    recording           INTEGER NOT NULL, -- PK, references recording.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE recording_tag_raw
(
    recording           INTEGER NOT NULL, -- PK, references recording.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE recording_annotation ( -- replicate (verbose)
    recording           INTEGER NOT NULL, -- PK, references recording.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE recording_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references recording_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE recording_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    recording_attribute_type    INTEGER NOT NULL, -- references recording_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references recording_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE recording_attribute ( -- replicate (verbose)
    id                                          SERIAL,  -- PK
    recording                                   INTEGER NOT NULL, -- references recording.id
    recording_attribute_type                    INTEGER NOT NULL, -- references recording_attribute_type.id
    recording_attribute_type_allowed_value      INTEGER, -- references recording_attribute_type_allowed_value.id
    recording_attribute_text                    TEXT
    CHECK (
        (recording_attribute_type_allowed_value IS NULL AND recording_attribute_text IS NOT NULL)
        OR
        (recording_attribute_type_allowed_value IS NOT NULL AND recording_attribute_text IS NULL)
    )
);

CREATE TABLE recording_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references recording.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE recording_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references recording.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE recording_tag ( -- replicate (verbose)
    recording           INTEGER NOT NULL, -- PK, references recording.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
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

CREATE TABLE release_alias_type ( -- replicate
    id                  SERIAL, -- PK,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references release_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_alias ( -- replicate (verbose)
    id                  SERIAL, --PK
    release             INTEGER NOT NULL, -- references release.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references release_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
             CONSTRAINT primary_check
                 CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)));

CREATE TABLE release_country ( -- replicate (verbose)
  release INTEGER NOT NULL,  -- PK, references release.id
  country INTEGER NOT NULL,  -- PK, references country_area.area
  date_year SMALLINT,
  date_month SMALLINT,
  date_day SMALLINT
);

CREATE TABLE release_unknown_country ( -- replicate (verbose)
  release INTEGER NOT NULL,  -- PK, references release.id
  date_year SMALLINT,
  date_month SMALLINT,
  date_day SMALLINT
);

CREATE TABLE release_raw ( -- replicate
    id                  SERIAL, -- PK
    title               VARCHAR(255) NOT NULL,
    artist              VARCHAR(255),
    added               TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_modified        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    lookup_count         INTEGER DEFAULT 0,
    modify_count         INTEGER DEFAULT 0,
    source              INTEGER DEFAULT 0,
    barcode             VARCHAR(255),
    comment             VARCHAR(255) NOT NULL DEFAULT ''
);

CREATE TABLE release_tag_raw
(
    release             INTEGER NOT NULL, -- PK, references release.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE release_annotation ( -- replicate (verbose)
    release             INTEGER NOT NULL, -- PK, references release.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE release_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references release_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    release_attribute_type      INTEGER NOT NULL, -- references release_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references release_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE release_attribute ( -- replicate (verbose)
    id                                          SERIAL,  -- PK
    release                                     INTEGER NOT NULL, -- references release.id
    release_attribute_type                      INTEGER NOT NULL, -- references release_attribute_type.id
    release_attribute_type_allowed_value        INTEGER, -- references release_attribute_type_allowed_value.id
    release_attribute_text                      TEXT
    CHECK (
        (release_attribute_type_allowed_value IS NULL AND release_attribute_text IS NOT NULL)
        OR
        (release_attribute_type_allowed_value IS NOT NULL AND release_attribute_text IS NULL)
    )
);

CREATE TABLE release_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references release.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TYPE cover_art_presence AS ENUM ('absent', 'present', 'darkened');

CREATE TABLE release_meta ( -- replicate (verbose)
    id                  INTEGER NOT NULL, -- PK, references release.id CASCADE
    date_added          TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    info_url            VARCHAR(255),
    amazon_asin         VARCHAR(10),
    amazon_store        VARCHAR(20),
    cover_art_presence  cover_art_presence NOT NULL DEFAULT 'absent'
);

CREATE TABLE release_coverart
(
    id                  INTEGER NOT NULL, -- PK, references release.id CASCADE
    last_updated        TIMESTAMP WITH TIME ZONE,
    cover_art_url       VARCHAR(255)
);

CREATE TABLE release_label ( -- replicate (verbose)
    id                  SERIAL,
    release             INTEGER NOT NULL, -- references release.id
    label               INTEGER, -- references label.id
    catalog_number      VARCHAR(255),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE release_packaging ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references release_packaging.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_status ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references release_status.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_tag ( -- replicate (verbose)
    release             INTEGER NOT NULL, -- PK, references release.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE release_group ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    artist_credit       INTEGER NOT NULL, -- references artist_credit.id
    type                INTEGER, -- references release_group_primary_type.id
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE release_group_alias_type ( -- replicate
    id                  SERIAL, -- PK,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references release_group_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_group_alias ( -- replicate (verbose)
    id                  SERIAL, --PK
    release_group       INTEGER NOT NULL, -- references release_group.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >=0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references release_group_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
             CONSTRAINT primary_check
                 CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)));

CREATE TABLE release_group_rating_raw
(
    release_group       INTEGER NOT NULL, -- PK, references release_group.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE release_group_tag_raw
(
    release_group       INTEGER NOT NULL, -- PK, references release_group.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE release_group_annotation ( -- replicate (verbose)
    release_group       INTEGER NOT NULL, -- PK, references release_group.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE release_group_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references release_group_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_group_attribute_type_allowed_value ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    release_group_attribute_type        INTEGER NOT NULL, -- references release_group_attribute_type.id
    value                               TEXT,
    parent                              INTEGER, -- references release_group_attribute_type_allowed_value.id
    child_order                         INTEGER NOT NULL DEFAULT 0,
    description                         TEXT,
    gid                                 uuid NOT NULL
);

CREATE TABLE release_group_attribute ( -- replicate (verbose)
    id                                          SERIAL,  -- PK
    release_group                               INTEGER NOT NULL, -- references release_group.id
    release_group_attribute_type                INTEGER NOT NULL, -- references release_group_attribute_type.id
    release_group_attribute_type_allowed_value  INTEGER, -- references release_group_attribute_type_allowed_value.id
    release_group_attribute_text                TEXT
    CHECK (
        (release_group_attribute_type_allowed_value IS NULL AND release_group_attribute_text IS NOT NULL)
        OR
        (release_group_attribute_type_allowed_value IS NOT NULL AND release_group_attribute_text IS NULL)
    )
);

CREATE TABLE release_group_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references release_group.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE release_group_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references release_group.id CASCADE
    release_count       INTEGER NOT NULL DEFAULT 0,
    first_release_date_year   SMALLINT,
    first_release_date_month  SMALLINT,
    first_release_date_day    SMALLINT,
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE release_group_tag ( -- replicate (verbose)
    release_group       INTEGER NOT NULL, -- PK, references release_group.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE release_group_primary_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references release_group_primary_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_group_secondary_type ( -- replicate
    id                  SERIAL NOT NULL, -- PK
    name                TEXT NOT NULL,
    parent              INTEGER, -- references release_group_secondary_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE release_group_secondary_type_join ( -- replicate (verbose)
    release_group INTEGER NOT NULL, -- PK, references release_group.id,
    secondary_type INTEGER NOT NULL, -- PK, references release_group_secondary_type.id
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE script ( -- replicate
    id                  SERIAL,
    iso_code            CHAR(4) NOT NULL, -- ISO 15924
    iso_number          CHAR(3) NOT NULL, -- ISO 15924
    name                VARCHAR(100) NOT NULL,
    frequency           SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE series ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    type                INTEGER NOT NULL, -- references series_type.id
    ordering_attribute  INTEGER NOT NULL, -- references link_text_attribute_type.attribute_type
    ordering_type       INTEGER NOT NULL, -- references series_ordering_type.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE series_type ( -- replicate (verbose)
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    entity_type         VARCHAR(50) NOT NULL,
    parent              INTEGER, -- references series_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE series_ordering_type ( -- replicate (verbose)
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references series_ordering_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE series_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references series.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE series_alias_type ( -- replicate (verbose)
    id                  SERIAL, -- PK
    name                TEXT NOT NULL,
    parent              INTEGER, -- references series_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE series_alias ( -- replicate (verbose)
    id                  SERIAL, -- PK
    series              INTEGER NOT NULL, -- references series.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references series_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT FALSE,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE series_annotation ( -- replicate (verbose)
    series              INTEGER NOT NULL, -- PK, references series.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE series_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references series_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE series_attribute_type_allowed_value ( -- replicate (verbose)
    id                          SERIAL,  -- PK
    series_attribute_type       INTEGER NOT NULL, -- references series_attribute_type.id
    value                       TEXT,
    parent                      INTEGER, -- references series_attribute_type_allowed_value.id
    child_order                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    gid                         uuid NOT NULL
);

CREATE TABLE series_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    series                              INTEGER NOT NULL, -- references series.id
    series_attribute_type               INTEGER NOT NULL, -- references series_attribute_type.id
    series_attribute_type_allowed_value INTEGER, -- references series_attribute_type_allowed_value.id
    series_attribute_text               TEXT
    CHECK (
        (series_attribute_type_allowed_value IS NULL AND series_attribute_text IS NOT NULL)
        OR
        (series_attribute_type_allowed_value IS NOT NULL AND series_attribute_text IS NULL)
    )
);

CREATE TABLE series_tag ( -- replicate (verbose)
    series              INTEGER NOT NULL, -- PK, references series.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE series_tag_raw (
    series              INTEGER NOT NULL, -- PK, references series.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE tag ( -- replicate (verbose)
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    ref_count           INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE tag_relation
(
    tag1                INTEGER NOT NULL, -- PK, references tag.id
    tag2                INTEGER NOT NULL, -- PK, references tag.id
    weight              INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CHECK (tag1 < tag2)
);

CREATE TABLE track ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    recording           INTEGER NOT NULL, -- references recording.id
    medium              INTEGER NOT NULL, -- references medium.id
    position            INTEGER NOT NULL,
    number              TEXT NOT NULL,
    name                VARCHAR NOT NULL,
    artist_credit       INTEGER NOT NULL, -- references artist_credit.id
    length              INTEGER CHECK (length IS NULL OR length > 0),
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_data_track       BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE track_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references track.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE track_raw ( -- replicate
    id                  SERIAL, -- PK
    release             INTEGER NOT NULL,   -- references release_raw.id
    title               VARCHAR(255) NOT NULL,
    artist              VARCHAR(255),   -- For VA albums, otherwise empty
    sequence            INTEGER NOT NULL
);

CREATE TABLE medium_index ( -- replicate
    medium              INTEGER, -- PK, references medium.id CASCADE
    toc                 CUBE
);

CREATE TABLE url ( -- replicate
    id                  SERIAL,
    gid                 UUID NOT NULL,
    url                 TEXT NOT NULL,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE url_gid_redirect ( -- replicate
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references url.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE vote
(
    id                  SERIAL,
    editor              INTEGER NOT NULL, -- references editor.id
    edit                INTEGER NOT NULL, -- references edit.id
    vote                SMALLINT NOT NULL,
    vote_time            TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    superseded          BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE work ( -- replicate (verbose)
    id                  SERIAL,
    gid                 UUID NOT NULL,
    name                VARCHAR NOT NULL,
    type                INTEGER, -- references work_type.id
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE work_language ( -- replicate (verbose)
    work                INTEGER NOT NULL, -- PK, references work.id
    language            INTEGER NOT NULL, -- PK, references language.id
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE work_rating_raw
(
    work                INTEGER NOT NULL, -- PK, references work.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    rating              SMALLINT NOT NULL CHECK (rating >= 0 AND rating <= 100)
);

CREATE TABLE work_tag_raw
(
    work                INTEGER NOT NULL, -- PK, references work.id
    editor              INTEGER NOT NULL, -- PK, references editor.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    is_upvote           BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE work_alias_type ( -- replicate
    id                  SERIAL,
    name                TEXT NOT NULL,
    parent              INTEGER, -- references work_alias_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE work_alias ( -- replicate (verbose)
    id                  SERIAL,
    work                INTEGER NOT NULL, -- references work.id
    name                VARCHAR NOT NULL,
    locale              TEXT,
    edits_pending       INTEGER NOT NULL DEFAULT 0 CHECK (edits_pending >= 0),
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    type                INTEGER, -- references work_alias_type.id
    sort_name           VARCHAR NOT NULL,
    begin_date_year     SMALLINT,
    begin_date_month    SMALLINT,
    begin_date_day      SMALLINT,
    end_date_year       SMALLINT,
    end_date_month      SMALLINT,
    end_date_day        SMALLINT,
    primary_for_locale  BOOLEAN NOT NULL DEFAULT false,
    ended               BOOLEAN NOT NULL DEFAULT FALSE
      CHECK (
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
    CONSTRAINT primary_check CHECK ((locale IS NULL AND primary_for_locale IS FALSE) OR (locale IS NOT NULL)),
    CONSTRAINT search_hints_are_empty
      CHECK (
        (type <> 2) OR (
          type = 2 AND sort_name = name AND
          begin_date_year IS NULL AND begin_date_month IS NULL AND begin_date_day IS NULL AND
          end_date_year IS NULL AND end_date_month IS NULL AND end_date_day IS NULL AND
          primary_for_locale IS FALSE AND locale IS NULL
        )
      )
);

CREATE TABLE work_annotation ( -- replicate (verbose)
    work                INTEGER NOT NULL, -- PK, references work.id
    annotation          INTEGER NOT NULL -- PK, references annotation.id
);

CREATE TABLE work_gid_redirect ( -- replicate (verbose)
    gid                 UUID NOT NULL, -- PK
    new_id              INTEGER NOT NULL, -- references work.id
    created             TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE work_meta ( -- replicate
    id                  INTEGER NOT NULL, -- PK, references work.id CASCADE
    rating              SMALLINT CHECK (rating >= 0 AND rating <= 100),
    rating_count        INTEGER
);

CREATE TABLE work_tag ( -- replicate (verbose)
    work                INTEGER NOT NULL, -- PK, references work.id
    tag                 INTEGER NOT NULL, -- PK, references tag.id
    count               INTEGER NOT NULL,
    last_updated        TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE work_type ( -- replicate
    id                  SERIAL,
    name                VARCHAR(255) NOT NULL,
    parent              INTEGER, -- references work_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE work_attribute_type ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    name                VARCHAR(255) NOT NULL,
    comment             VARCHAR(255) NOT NULL DEFAULT '',
    free_text           BOOLEAN NOT NULL,
    parent              INTEGER, -- references work_attribute_type.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE work_attribute_type_allowed_value ( -- replicate (verbose)
    id                  SERIAL,  -- PK
    work_attribute_type INTEGER NOT NULL, -- references work_attribute_type.id
    value               TEXT,
    parent              INTEGER, -- references work_attribute_type_allowed_value.id
    child_order         INTEGER NOT NULL DEFAULT 0,
    description         TEXT,
    gid                 uuid NOT NULL
);

CREATE TABLE work_attribute ( -- replicate (verbose)
    id                                  SERIAL,  -- PK
    work                                INTEGER NOT NULL, -- references work.id
    work_attribute_type                 INTEGER NOT NULL, -- references work_attribute_type.id
    work_attribute_type_allowed_value   INTEGER, -- references work_attribute_type_allowed_value.id
    work_attribute_text                 TEXT
    CHECK (
        (work_attribute_type_allowed_value IS NULL AND work_attribute_text IS NOT NULL)
        OR
        (work_attribute_type_allowed_value IS NOT NULL AND work_attribute_text IS NULL)
    )
);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateViews.sql


CREATE OR REPLACE VIEW release_event AS
    SELECT
        release, date_year, date_month, date_day, country
    FROM (
        SELECT release, date_year, date_month, date_day, country
        FROM release_country
        UNION ALL
        SELECT release, date_year, date_month, date_day, NULL
        FROM release_unknown_country
    ) as q;

CREATE OR REPLACE VIEW artist_series AS
    SELECT entity0 AS artist,
           entity1 AS series,
           las.id AS relationship,
           link_order,
           las.link,
           COALESCE(text_value, '') AS text_value
    FROM l_artist_series las
    JOIN series s ON s.id = las.entity1
    JOIN link l ON l.id = las.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = 'd1a845d1-8c03-3191-9454-e4e8d37fa5e0')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = 788 AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW event_series AS
    SELECT entity0 AS event,
           entity1 AS series,
           lrs.id AS relationship,
           link_order,
           lrs.link,
           COALESCE(text_value, '') AS text_value
    FROM l_event_series lrs
    JOIN series s ON s.id = lrs.entity1
    JOIN link l ON l.id = lrs.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = '707d947d-9563-328a-9a7d-0c5b9c3a9791')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = s.ordering_attribute AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW recording_series AS
    SELECT entity0 AS recording,
           entity1 AS series,
           lrs.id AS relationship,
           link_order,
           lrs.link,
           COALESCE(text_value, '') AS text_value
    FROM l_recording_series lrs
    JOIN series s ON s.id = lrs.entity1
    JOIN link l ON l.id = lrs.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = 'ea6f0698-6782-30d6-b16d-293081b66774')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = s.ordering_attribute AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW release_series AS
    SELECT entity0 AS release,
           entity1 AS series,
           lrs.id AS relationship,
           link_order,
           lrs.link,
           COALESCE(text_value, '') AS text_value
    FROM l_release_series lrs
    JOIN series s ON s.id = lrs.entity1
    JOIN link l ON l.id = lrs.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = '3fa29f01-8e13-3e49-9b0a-ad212aa2f81d')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = s.ordering_attribute AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW release_group_series AS
    SELECT entity0 AS release_group,
           entity1 AS series,
           lrgs.id AS relationship,
           link_order,
           lrgs.link,
           COALESCE(text_value, '') AS text_value
    FROM l_release_group_series lrgs
    JOIN series s ON s.id = lrgs.entity1
    JOIN link l ON l.id = lrgs.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = '01018437-91d8-36b9-bf89-3f885d53b5bd')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = s.ordering_attribute AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW work_series AS
    SELECT entity1 AS work,
           entity0 AS series,
           lsw.id AS relationship,
           link_order,
           lsw.link,
           COALESCE(text_value, '') AS text_value
    FROM l_series_work lsw
    JOIN series s ON s.id = lsw.entity0
    JOIN link l ON l.id = lsw.link
    JOIN link_type lt ON (lt.id = l.link_type AND lt.gid = 'b0d44366-cdf0-3acb-bee6-0f65a77a6ef0')
    LEFT OUTER JOIN link_attribute_text_value latv ON (latv.attribute_type = s.ordering_attribute AND latv.link = l.id)
    ORDER BY series, link_order;

CREATE OR REPLACE VIEW medium_track_durations AS
    SELECT
        medium.id AS medium,
        array_agg(track.length ORDER BY track.position) FILTER (WHERE track.position = 0) AS pregap_length,
        array_agg(track.length ORDER BY track.position) FILTER (WHERE track.position > 0 AND track.is_data_track = false) AS cdtoc_track_lengths,
        array_agg(track.length ORDER BY track.position) FILTER (WHERE track.is_data_track = true) AS data_track_lengths
    FROM medium
    JOIN track ON track.medium = medium.id
    GROUP BY medium.id;

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateFunctions.sql

CREATE OR REPLACE FUNCTION _median(anyarray) RETURNS anyelement AS $$
  WITH q AS (
      SELECT val
      FROM unnest($1) val
      WHERE VAL IS NOT NULL
      ORDER BY val
  )
  SELECT val
  FROM q
  LIMIT 1
  -- Subtracting (n + 1) % 2 creates a left bias
  OFFSET greatest(0, floor((select count(*) FROM q) / 2.0) - ((select count(*) + 1 FROM q) % 2));
$$ LANGUAGE sql IMMUTABLE;

CREATE AGGREGATE median(anyelement) (
  SFUNC=array_append,
  STYPE=anyarray,
  FINALFUNC=_median,
  INITCOND='{}'
);

-- We may want to create a CreateAggregate.sql script, but it seems silly to do that for one aggregate
CREATE AGGREGATE array_accum (basetype = anyelement, sfunc = array_append, stype = anyarray, initcond = '{}');

-- Generates UUID version 4 (random-based)
CREATE OR REPLACE FUNCTION generate_uuid_v4() RETURNS uuid
    AS $$
DECLARE
    value VARCHAR(36);
BEGIN
    value =          lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || '-';
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || '-';
    value = value || lpad((to_hex((ceil(random() * 255)::int & 15) | 64)), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || '-';
    value = value || lpad((to_hex((ceil(random() * 255)::int & 63) | 128)), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || '-';
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    value = value || lpad(to_hex(ceil(random() * 255)::int), 2, '0');
    RETURN value::uuid;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION from_hex(t text) RETURNS integer
    AS $$
DECLARE
    r RECORD;
BEGIN
    FOR r IN EXECUTE 'SELECT x'''||t||'''::integer AS hex' LOOP
        RETURN r.hex;
    END LOOP;
END
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

-- NameSpace_URL = '6ba7b8119dad11d180b400c04fd430c8'
CREATE OR REPLACE FUNCTION generate_uuid_v3(namespace varchar, name varchar) RETURNS uuid
    AS $$
DECLARE
    value varchar(36);
    bytes varchar;
BEGIN
    bytes = md5(decode(namespace, 'hex') || decode(name, 'escape'));
    value = substr(bytes, 1+0, 8);
    value = value || '-';
    value = value || substr(bytes, 1+2*4, 4);
    value = value || '-';
    value = value || lpad(to_hex((from_hex(substr(bytes, 1+2*6, 2)) & 15) | 48), 2, '0');
    value = value || substr(bytes, 1+2*7, 2);
    value = value || '-';
    value = value || lpad(to_hex((from_hex(substr(bytes, 1+2*8, 2)) & 63) | 128), 2, '0');
    value = value || substr(bytes, 1+2*9, 2);
    value = value || '-';
    value = value || substr(bytes, 1+2*10, 12);
    return value::uuid;
END;
$$ LANGUAGE 'plpgsql' IMMUTABLE STRICT;


CREATE OR REPLACE FUNCTION inc_ref_count(tbl varchar, row_id integer, val integer) RETURNS void AS $$
BEGIN
    -- increment ref_count for the new name
    EXECUTE 'SELECT ref_count FROM ' || tbl || ' WHERE id = ' || row_id || ' FOR UPDATE';
    EXECUTE 'UPDATE ' || tbl || ' SET ref_count = ref_count + ' || val || ' WHERE id = ' || row_id;
    RETURN;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION dec_ref_count(tbl varchar, row_id integer, val integer) RETURNS void AS $$
DECLARE
    ref_count integer;
BEGIN
    -- decrement ref_count for the old name,
    -- or delete it if ref_count would drop to 0
    EXECUTE 'SELECT ref_count FROM ' || tbl || ' WHERE id = ' || row_id || ' FOR UPDATE' INTO ref_count;
    IF ref_count <= val THEN
        EXECUTE 'DELETE FROM ' || tbl || ' WHERE id = ' || row_id;
    ELSE
        EXECUTE 'UPDATE ' || tbl || ' SET ref_count = ref_count - ' || val || ' WHERE id = ' || row_id;
    END IF;
    RETURN;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION integer_date(year SMALLINT, month SMALLINT, day SMALLINT)
RETURNS INTEGER AS $$
    -- Returns an integer representation of the given date, keeping
    -- NULL values sorted last.
    SELECT (
        CASE
            WHEN year IS NULL AND month IS NULL AND day IS NULL
            THEN NULL
            ELSE (
                coalesce(year::TEXT, '9999') ||
                lpad(coalesce(month::TEXT, '99'), 2, '0') ||
                lpad(coalesce(day::TEXT, '99'), 2, '0')
            )::INTEGER
        END
    )
$$ LANGUAGE SQL IMMUTABLE PARALLEL SAFE;

-----------------------------------------------------------------------
-- area triggers
-----------------------------------------------------------------------

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_area_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.area_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM area_attribute_type
             WHERE area_attribute_type.id = NEW.area_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- artist triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_artist() RETURNS trigger AS $$
BEGIN
    -- add a new entry to the artist_meta table
    INSERT INTO artist_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_artist_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.artist_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM artist_attribute_type
             WHERE artist_attribute_type.id = NEW.artist_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- artist_credit triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION b_upd_artist_credit_name() RETURNS trigger AS $$
BEGIN
    -- Artist credits are assumed to be immutable. When changes need to
    -- be made, we `find_or_insert` the new artist credits and swap
    -- them with the old ones rather than mutate existing entries.
    --
    -- This simplifies a lot of assumptions we can make about their
    -- cacheability, and the consistency of materialized tables like
    -- artist_release_group.
    RAISE EXCEPTION 'Cannot update artist_credit_name';
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- editor triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_editor() RETURNS trigger AS $$
BEGIN
    -- add a new entry to the editor_watch_preference table
    INSERT INTO editor_watch_preferences (editor) VALUES (NEW.id);

    -- by default watch for new official albums
    INSERT INTO editor_watch_release_group_type (editor, release_group_type)
        VALUES (NEW.id, 2);
    INSERT INTO editor_watch_release_status (editor, release_status)
        VALUES (NEW.id, 1);

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION check_editor_name() RETURNS trigger AS $$
BEGIN
    IF (SELECT 1 FROM old_editor_name WHERE lower(name) = lower(NEW.name))
    THEN
        RAISE EXCEPTION 'Attempt to use a previously-used editor name.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- event triggers
-----------------------------------------------------------------------

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_event_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.event_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM event_attribute_type
             WHERE event_attribute_type.id = NEW.event_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- event triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_event() RETURNS trigger AS $$
BEGIN
    -- add a new entry to the event_meta table
    INSERT INTO event_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- instrument triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_instrument() RETURNS trigger AS $$
BEGIN
    WITH inserted_rows (id) AS (
        INSERT INTO link_attribute_type (parent, root, child_order, gid, name, description)
        VALUES (14, 14, 0, NEW.gid, NEW.name, NEW.description)
        RETURNING id
    ) INSERT INTO link_creditable_attribute_type (attribute_type) SELECT id FROM inserted_rows;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION a_upd_instrument() RETURNS trigger AS $$
BEGIN
    UPDATE link_attribute_type SET name = NEW.name, description = NEW.description WHERE gid = NEW.gid;
    IF NOT FOUND THEN
        RAISE EXCEPTION 'no link_attribute_type found for instrument %', NEW.gid;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION a_del_instrument() RETURNS trigger AS $$
BEGIN
    DELETE FROM link_attribute_type WHERE gid = OLD.gid;
    IF NOT FOUND THEN
        RAISE EXCEPTION 'no link_attribute_type found for instrument %', NEW.gid;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_instrument_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.instrument_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM instrument_attribute_type
             WHERE instrument_attribute_type.id = NEW.instrument_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- label triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_label() RETURNS trigger AS $$
BEGIN
    INSERT INTO label_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_label_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.label_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM label_attribute_type
             WHERE label_attribute_type.id = NEW.label_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- medium triggers
-----------------------------------------------------------------------

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_medium_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.medium_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM medium_attribute_type
             WHERE medium_attribute_type.id = NEW.medium_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- place triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_place() RETURNS trigger AS $$
BEGIN
    -- add a new entry to the place_meta table
    INSERT INTO place_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_place_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.place_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM place_attribute_type
             WHERE place_attribute_type.id = NEW.place_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- recording triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION median_track_length(recording_id integer)
RETURNS integer AS $$
  SELECT median(track.length) FROM track WHERE recording = $1;
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION b_upd_recording() RETURNS TRIGGER AS $$
BEGIN
  IF OLD.length IS DISTINCT FROM NEW.length
    AND EXISTS (SELECT TRUE FROM track WHERE recording = NEW.id)
    AND NEW.length IS DISTINCT FROM median_track_length(NEW.id)
  THEN
    NEW.length = median_track_length(NEW.id);
  END IF;

  NEW.last_updated = now();
  RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_ins_recording() RETURNS trigger AS $$
BEGIN
    PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    INSERT INTO recording_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_recording() RETURNS trigger AS $$
BEGIN
    IF NEW.artist_credit != OLD.artist_credit THEN
        PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
        PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_recording() RETURNS trigger AS $$
BEGIN
    PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_recording_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.recording_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM recording_attribute_type
             WHERE recording_attribute_type.id = NEW.recording_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- release triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_release() RETURNS trigger AS $$
BEGIN
    -- increment ref_count of the name
    PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    -- increment release_count of the parent release group
    UPDATE release_group_meta SET release_count = release_count + 1 WHERE id = NEW.release_group;
    -- add new release_meta
    INSERT INTO release_meta (id) VALUES (NEW.id);
    INSERT INTO release_coverart (id) VALUES (NEW.id);
    INSERT INTO artist_release_pending_update VALUES (NEW.id);
    INSERT INTO artist_release_group_pending_update VALUES (NEW.release_group);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_release() RETURNS trigger AS $$
BEGIN
    IF NEW.artist_credit != OLD.artist_credit THEN
        PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
        PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    END IF;
    IF NEW.release_group != OLD.release_group THEN
        -- release group is changed, decrement release_count in the original RG, increment in the new one
        UPDATE release_group_meta SET release_count = release_count - 1 WHERE id = OLD.release_group;
        UPDATE release_group_meta SET release_count = release_count + 1 WHERE id = NEW.release_group;
        PERFORM set_release_group_first_release_date(OLD.release_group);
        PERFORM set_release_group_first_release_date(NEW.release_group);
    END IF;
    IF (
        NEW.status IS DISTINCT FROM OLD.status OR
        NEW.release_group != OLD.release_group OR
        NEW.artist_credit != OLD.artist_credit
    ) THEN
        INSERT INTO artist_release_group_pending_update
        VALUES (NEW.release_group), (OLD.release_group);
    END IF;
    IF (
        NEW.barcode IS DISTINCT FROM OLD.barcode OR
        NEW.name != OLD.name OR
        NEW.artist_credit != OLD.artist_credit
    ) THEN
        INSERT INTO artist_release_pending_update VALUES (OLD.id);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_release() RETURNS trigger AS $$
BEGIN
    -- decrement ref_count of the name
    PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
    -- decrement release_count of the parent release group
    UPDATE release_group_meta SET release_count = release_count - 1 WHERE id = OLD.release_group;
    INSERT INTO artist_release_pending_update VALUES (OLD.id);
    INSERT INTO artist_release_group_pending_update VALUES (OLD.release_group);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_ins_release_group_secondary_type_join()
RETURNS trigger AS $$
BEGIN
    INSERT INTO artist_release_group_pending_update VALUES (NEW.release_group);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_release_group_secondary_type_join()
RETURNS trigger AS $$
BEGIN
    INSERT INTO artist_release_group_pending_update VALUES (OLD.release_group);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_ins_release_label()
RETURNS trigger AS $$
BEGIN
    INSERT INTO artist_release_pending_update VALUES (NEW.release);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_release_label()
RETURNS trigger AS $$
BEGIN
    IF NEW.catalog_number IS DISTINCT FROM OLD.catalog_number THEN
        INSERT INTO artist_release_pending_update VALUES (OLD.release);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_release_label()
RETURNS trigger AS $$
BEGIN
    INSERT INTO artist_release_pending_update VALUES (OLD.release);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_release_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.release_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM release_attribute_type
             WHERE release_attribute_type.id = NEW.release_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- release_group triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_release_group() RETURNS trigger AS $$
BEGIN
    PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    INSERT INTO release_group_meta (id) VALUES (NEW.id);
    INSERT INTO artist_release_group_pending_update VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_release_group() RETURNS trigger AS $$
BEGIN
    IF NEW.artist_credit != OLD.artist_credit THEN
        PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
        PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    END IF;
    IF (
        NEW.name != OLD.name OR
        NEW.artist_credit != OLD.artist_credit OR
        NEW.type IS DISTINCT FROM OLD.type
     ) THEN
        INSERT INTO artist_release_group_pending_update VALUES (OLD.id);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_release_group() RETURNS trigger AS $$
BEGIN
    PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
    INSERT INTO artist_release_group_pending_update VALUES (OLD.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION b_upd_release_group_secondary_type_join() RETURNS trigger AS $$
BEGIN
    -- Like artist credits, rows in release_group_secondary_type_join
    -- are immutable. When updates need to be made for a particular
    -- release group, they're deleted and re-inserted.
    --
    -- A benefit of this is that we don't need UPDATE triggers to keep
    -- artist_release_group up-to-date.
    RAISE EXCEPTION 'Cannot update release_group_secondary_type_join';
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_release_group_attribute_type_allows_text()
RETURNS trigger AS $$
  BEGIN
    IF NEW.release_group_attribute_text IS NOT NULL
        AND NOT EXISTS (
           SELECT TRUE FROM release_group_attribute_type
        WHERE release_group_attribute_type.id = NEW.release_group_attribute_type
        AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE RETURN NEW;
    END IF;
  END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- series triggers
-----------------------------------------------------------------------

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_series_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.series_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE
              FROM series_attribute_type
             WHERE series_attribute_type.id = NEW.series_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- track triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_track() RETURNS trigger AS $$
BEGIN
    PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
    -- increment track_count in the parent medium
    UPDATE medium SET track_count = track_count + 1 WHERE id = NEW.medium;
    PERFORM materialise_recording_length(NEW.recording);
    PERFORM set_recordings_first_release_dates(ARRAY[NEW.recording]);
    INSERT INTO artist_release_pending_update (
        SELECT release FROM medium
        WHERE id = NEW.medium
    );
    INSERT INTO artist_release_group_pending_update (
        SELECT release_group FROM release
        JOIN medium ON medium.release = release.id
        WHERE medium.id = NEW.medium
    );
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_track() RETURNS trigger AS $$
BEGIN
    IF NEW.artist_credit != OLD.artist_credit THEN
        PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
        PERFORM inc_ref_count('artist_credit', NEW.artist_credit, 1);
        INSERT INTO artist_release_pending_update (
            SELECT release FROM medium
            WHERE id = OLD.medium
        );
        INSERT INTO artist_release_group_pending_update (
            SELECT release_group FROM release
            JOIN medium ON medium.release = release.id
            WHERE medium.id = OLD.medium
        );
    END IF;
    IF NEW.medium != OLD.medium THEN
        IF (
            SELECT count(DISTINCT release)
              FROM medium
             WHERE id IN (NEW.medium, OLD.medium)
        ) = 2
        THEN
            -- I don't believe this code path should ever be hit.
            -- We have no functionality to move tracks between
            -- mediums. If this is ever allowed, however, we should
            -- ensure that both old and new mediums share the same
            -- release, otherwise we'd have to carefully handle this
            -- case when when updating materialized tables for
            -- recordings' first release dates and artists' release
            -- groups. -mwiencek, 2021-03-14
            RAISE EXCEPTION 'Cannot move a track between releases';
        END IF;

        -- medium is changed, decrement track_count in the original medium, increment in the new one
        UPDATE medium SET track_count = track_count - 1 WHERE id = OLD.medium;
        UPDATE medium SET track_count = track_count + 1 WHERE id = NEW.medium;
    END IF;
    IF OLD.recording <> NEW.recording THEN
      PERFORM materialise_recording_length(OLD.recording);
      PERFORM set_recordings_first_release_dates(ARRAY[OLD.recording, NEW.recording]);
    END IF;
    PERFORM materialise_recording_length(NEW.recording);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_track() RETURNS trigger AS $$
BEGIN
    PERFORM dec_ref_count('artist_credit', OLD.artist_credit, 1);
    -- decrement track_count in the parent medium
    UPDATE medium SET track_count = track_count - 1 WHERE id = OLD.medium;
    PERFORM materialise_recording_length(OLD.recording);
    PERFORM set_recordings_first_release_dates(ARRAY[OLD.recording]);
    INSERT INTO artist_release_pending_update (
        SELECT release FROM medium
        WHERE id = OLD.medium
    );
    INSERT INTO artist_release_group_pending_update (
        SELECT release_group FROM release
        JOIN medium ON medium.release = release.id
        WHERE medium.id = OLD.medium
    );
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- work triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_work() RETURNS trigger AS $$
BEGIN
    INSERT INTO work_meta (id) VALUES (NEW.id);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- Ensure attribute type allows free text if free text is added
CREATE OR REPLACE FUNCTION ensure_work_attribute_type_allows_text()
RETURNS trigger AS $$
BEGIN
    IF NEW.work_attribute_text IS NOT NULL
        AND NOT EXISTS (
            SELECT TRUE FROM work_attribute_type
             WHERE work_attribute_type.id = NEW.work_attribute_type
               AND free_text
    )
    THEN
        RAISE EXCEPTION 'This attribute type can not contain free text';
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- alternative tracklist triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION inc_nullable_artist_credit(row_id integer) RETURNS void AS $$
BEGIN
    IF row_id IS NOT NULL THEN
        PERFORM inc_ref_count('artist_credit', row_id, 1);
    END IF;
    RETURN;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION dec_nullable_artist_credit(row_id integer) RETURNS void AS $$
BEGIN
    IF row_id IS NOT NULL THEN
        PERFORM dec_ref_count('artist_credit', row_id, 1);
    END IF;
    RETURN;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_ins_alternative_release_or_track() RETURNS trigger AS $$
BEGIN
    PERFORM inc_nullable_artist_credit(NEW.artist_credit);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_alternative_release_or_track() RETURNS trigger AS $$
BEGIN
    IF NEW.artist_credit IS DISTINCT FROM OLD.artist_credit THEN
        PERFORM inc_nullable_artist_credit(NEW.artist_credit);
        PERFORM dec_nullable_artist_credit(OLD.artist_credit);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_alternative_release_or_track() RETURNS trigger AS $$
BEGIN
    PERFORM dec_nullable_artist_credit(OLD.artist_credit);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_ins_alternative_medium_track() RETURNS trigger AS $$
BEGIN
    PERFORM inc_ref_count('alternative_track', NEW.alternative_track, 1);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_alternative_medium_track() RETURNS trigger AS $$
BEGIN
    IF NEW.alternative_track IS DISTINCT FROM OLD.alternative_track THEN
        PERFORM inc_ref_count('alternative_track', NEW.alternative_track, 1);
        PERFORM dec_ref_count('alternative_track', OLD.alternative_track, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_alternative_medium_track() RETURNS trigger AS $$
BEGIN
    PERFORM dec_ref_count('alternative_track', OLD.alternative_track, 1);
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- lastupdate triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION b_upd_last_updated_table() RETURNS trigger AS $$
BEGIN
    NEW.last_updated = NOW();
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_edit() RETURNS trigger AS $$
BEGIN
    IF NEW.status != OLD.status THEN
       UPDATE edit_artist SET status = NEW.status WHERE edit = NEW.id;
       UPDATE edit_label  SET status = NEW.status WHERE edit = NEW.id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION b_ins_edit_materialize_status() RETURNS trigger AS $$
BEGIN
    NEW.status = (SELECT status FROM edit WHERE id = NEW.edit);
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

------------------------
-- Collection deletion and hiding triggers
------------------------

CREATE OR REPLACE FUNCTION replace_old_sub_on_add()
RETURNS trigger AS $$
  BEGIN
    UPDATE editor_subscribe_collection
     SET available = TRUE, last_seen_name = NULL,
      last_edit_sent = NEW.last_edit_sent
     WHERE editor = NEW.editor AND collection = NEW.collection;

    IF FOUND THEN
      RETURN NULL;
    ELSE
      RETURN NEW;
    END IF;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION del_collection_sub_on_delete()
RETURNS trigger AS $$
  BEGIN
    UPDATE editor_subscribe_collection sub
     SET available = FALSE, last_seen_name = OLD.name
     FROM editor_collection coll
     WHERE sub.collection = OLD.id AND sub.collection = coll.id;

    RETURN OLD;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION del_collection_sub_on_private()
RETURNS trigger AS $$
  BEGIN
    IF NEW.public = FALSE AND OLD.public = TRUE THEN
      UPDATE editor_subscribe_collection sub
         SET available = FALSE,
             last_seen_name = OLD.name
       WHERE sub.collection = OLD.id
         AND sub.editor != NEW.editor
         AND sub.editor NOT IN (SELECT ecc.editor
                                  FROM editor_collection_collaborator ecc
                                 WHERE ecc.collection = sub.collection);
    END IF;

    RETURN NEW;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION restore_collection_sub_on_public()
RETURNS trigger AS $$
  BEGIN
    IF NEW.public = TRUE AND OLD.public = FALSE THEN
      UPDATE editor_subscribe_collection sub
         SET available = TRUE,
             last_seen_name = NEW.name
       WHERE sub.collection = OLD.id
         AND sub.available = FALSE;
    END IF;

    RETURN NULL;
  END;
$$ LANGUAGE 'plpgsql';

------------------------
-- CD Lookup
------------------------

CREATE OR REPLACE FUNCTION create_cube_from_durations(durations INTEGER[]) RETURNS cube AS $$
DECLARE
    point    cube;
    str      VARCHAR;
    i        INTEGER;
    count    INTEGER;
    dest     INTEGER;
    dim      CONSTANT INTEGER = 6;
    selected INTEGER[];
BEGIN

    count = array_upper(durations, 1);
    FOR i IN 0..dim LOOP
        selected[i] = 0;
    END LOOP;

    IF count < dim THEN
        FOR i IN 1..count LOOP
            selected[i] = durations[i];
        END LOOP;
    ELSE
        FOR i IN 1..count LOOP
            dest = (dim * (i-1) / count) + 1;
            selected[dest] = selected[dest] + durations[i];
        END LOOP;
    END IF;

    str = '(';
    FOR i IN 1..dim LOOP
        IF i > 1 THEN
            str = str || ',';
        END IF;
        str = str || cast(selected[i] as text);
    END LOOP;
    str = str || ')';

    RETURN str::cube;
END;
$$ LANGUAGE 'plpgsql' IMMUTABLE;

CREATE OR REPLACE FUNCTION create_bounding_cube(durations INTEGER[], fuzzy INTEGER) RETURNS cube AS $$
DECLARE
    point    cube;
    str      VARCHAR;
    i        INTEGER;
    dest     INTEGER;
    count    INTEGER;
    dim      CONSTANT INTEGER = 6;
    selected INTEGER[];
    scalers  INTEGER[];
BEGIN

    count = array_upper(durations, 1);
    IF count < dim THEN
        FOR i IN 1..dim LOOP
            selected[i] = 0;
            scalers[i] = 0;
        END LOOP;
        FOR i IN 1..count LOOP
            selected[i] = durations[i];
            scalers[i] = 1;
        END LOOP;
    ELSE
        FOR i IN 1..dim LOOP
            selected[i] = 0;
            scalers[i] = 0;
        END LOOP;
        FOR i IN 1..count LOOP
            dest = (dim * (i-1) / count) + 1;
            selected[dest] = selected[dest] + durations[i];
            scalers[dest] = scalers[dest] + 1;
        END LOOP;
    END IF;

    str = '(';
    FOR i IN 1..dim LOOP
        IF i > 1 THEN
            str = str || ',';
        END IF;
        str = str || cast((selected[i] - (fuzzy * scalers[i])) as text);
    END LOOP;
    str = str || '),(';
    FOR i IN 1..dim LOOP
        IF i > 1 THEN
            str = str || ',';
        END IF;
        str = str || cast((selected[i] + (fuzzy * scalers[i])) as text);
    END LOOP;
    str = str || ')';

    RETURN str::cube;
END;
$$ LANGUAGE 'plpgsql' IMMUTABLE;

-------------------------------------------------------------------
-- Maintain musicbrainz.release_first_release_date
-------------------------------------------------------------------
CREATE OR REPLACE FUNCTION get_release_first_release_date_rows(condition TEXT)
RETURNS SETOF release_first_release_date AS $$
BEGIN
    RETURN QUERY EXECUTE '
        SELECT DISTINCT ON (release) release,
            date_year AS year,
            date_month AS month,
            date_day AS day
        FROM (
            SELECT release, date_year, date_month, date_day FROM release_country
            WHERE (date_year IS NOT NULL OR date_month IS NOT NULL OR date_day IS NOT NULL)
            UNION ALL
            SELECT release, date_year, date_month, date_day FROM release_unknown_country
        ) all_dates
        WHERE ' || condition ||
        ' ORDER BY release, year NULLS LAST, month NULLS LAST, day NULLS LAST';
END;
$$ LANGUAGE 'plpgsql' STRICT;

CREATE OR REPLACE FUNCTION set_release_first_release_date(release_id INTEGER)
RETURNS VOID AS $$
BEGIN
  -- DO NOT modify any replicated tables in this function; it's used
  -- by a trigger on slaves.
  DELETE FROM release_first_release_date
  WHERE release = release_id;

  INSERT INTO release_first_release_date
  SELECT * FROM get_release_first_release_date_rows(
    format('release = %L', release_id)
  );

  INSERT INTO artist_release_pending_update VALUES (release_id);
END;
$$ LANGUAGE 'plpgsql' STRICT;

-------------------------------------------------------------------
-- Maintain release_group_meta.first_release_date
-------------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_release_group_first_release_date(release_group_id INTEGER)
RETURNS VOID AS $$
BEGIN
    UPDATE release_group_meta SET first_release_date_year = first.year,
                                  first_release_date_month = first.month,
                                  first_release_date_day = first.day
      FROM (
        SELECT rd.year, rd.month, rd.day
        FROM release
        LEFT JOIN release_first_release_date rd ON (rd.release = release.id)
        WHERE release.release_group = release_group_id
        ORDER BY
          rd.year NULLS LAST,
          rd.month NULLS LAST,
          rd.day NULLS LAST
        LIMIT 1
      ) AS first
    WHERE id = release_group_id;
    INSERT INTO artist_release_group_pending_update VALUES (release_group_id);
END;
$$ LANGUAGE 'plpgsql';

-------------------------------------------------------------------
-- Maintain musicbrainz.recording_first_release_date
-------------------------------------------------------------------
CREATE OR REPLACE FUNCTION get_recording_first_release_date_rows(condition TEXT)
RETURNS SETOF recording_first_release_date AS $$
BEGIN
    RETURN QUERY EXECUTE '
        SELECT DISTINCT ON (track.recording)
            track.recording, rd.year, rd.month, rd.day
        FROM track
        JOIN medium ON medium.id = track.medium
        JOIN release_first_release_date rd ON rd.release = medium.release
        WHERE ' || condition || '
        ORDER BY track.recording,
            rd.year NULLS LAST,
            rd.month NULLS LAST,
            rd.day NULLS LAST';
END;
$$ LANGUAGE 'plpgsql' STRICT;

CREATE OR REPLACE FUNCTION set_recordings_first_release_dates(recording_ids INTEGER[])
RETURNS VOID AS $$
BEGIN
  -- DO NOT modify any replicated tables in this function; it's used
  -- by a trigger on slaves.
  DELETE FROM recording_first_release_date
  WHERE recording = ANY(recording_ids);

  INSERT INTO recording_first_release_date
  SELECT * FROM get_recording_first_release_date_rows(
    format('track.recording = any(%L)', recording_ids)
  );
END;
$$ LANGUAGE 'plpgsql' STRICT;

CREATE OR REPLACE FUNCTION set_releases_recordings_first_release_dates(release_ids INTEGER[])
RETURNS VOID AS $$
BEGIN
  PERFORM set_recordings_first_release_dates((
    SELECT array_agg(recording)
      FROM track
      JOIN medium ON medium.id = track.medium
     WHERE medium.release = any(release_ids)
  ));
  RETURN;
END;
$$ LANGUAGE 'plpgsql' STRICT;

CREATE OR REPLACE FUNCTION a_ins_release_event()
RETURNS TRIGGER AS $$
BEGIN
  PERFORM set_release_first_release_date(NEW.release);

  PERFORM set_release_group_first_release_date(release_group)
  FROM release
  WHERE release.id = NEW.release;

  PERFORM set_releases_recordings_first_release_dates(ARRAY[NEW.release]);

  IF TG_TABLE_NAME = 'release_country' THEN
    INSERT INTO artist_release_pending_update VALUES (NEW.release);
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_upd_release_event()
RETURNS TRIGGER AS $$
BEGIN
  IF (
    NEW.release != OLD.release OR
    NEW.date_year IS DISTINCT FROM OLD.date_year OR
    NEW.date_month IS DISTINCT FROM OLD.date_month OR
    NEW.date_day IS DISTINCT FROM OLD.date_day
  ) THEN
    PERFORM set_release_first_release_date(OLD.release);
    IF NEW.release != OLD.release THEN
        PERFORM set_release_first_release_date(NEW.release);
    END IF;

    PERFORM set_release_group_first_release_date(release_group)
    FROM release
    WHERE release.id IN (NEW.release, OLD.release);

    PERFORM set_releases_recordings_first_release_dates(ARRAY[NEW.release, OLD.release]);
  END IF;

  IF TG_TABLE_NAME = 'release_country' THEN
    IF NEW.country != OLD.country THEN
      INSERT INTO artist_release_pending_update VALUES (OLD.release);
    END IF;
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION a_del_release_event()
RETURNS TRIGGER AS $$
BEGIN
  PERFORM set_release_first_release_date(OLD.release);

  PERFORM set_release_group_first_release_date(release_group)
  FROM release
  WHERE release.id = OLD.release;

  PERFORM set_releases_recordings_first_release_dates(ARRAY[OLD.release]);

  IF TG_TABLE_NAME = 'release_country' THEN
    INSERT INTO artist_release_pending_update VALUES (OLD.release);
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION deny_special_purpose_deletion() RETURNS trigger AS $$
BEGIN
    RAISE EXCEPTION 'Attempted to delete a special purpose row';
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION delete_ratings(enttype TEXT, ids INTEGER[])
RETURNS TABLE(editor INT, rating SMALLINT) AS $$
DECLARE
    tablename TEXT;
BEGIN
    tablename = enttype || '_rating_raw';
    RETURN QUERY
       EXECUTE 'DELETE FROM ' || tablename || ' WHERE ' || enttype || ' = any($1)
                RETURNING editor, rating'
         USING ids;
    RETURN;
END;
$$ LANGUAGE 'plpgsql';

-------------------------------------------------------------------
-- Prevent link attributes being used on links that don't support them
-------------------------------------------------------------------
CREATE OR REPLACE FUNCTION prevent_invalid_attributes()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (
        SELECT TRUE
        FROM (VALUES (NEW.link, NEW.attribute_type)) la (link, attribute_type)
        JOIN link l ON l.id = la.link
        JOIN link_type lt ON l.link_type = lt.id
        JOIN link_attribute_type lat ON lat.id = la.attribute_type
        JOIN link_type_attribute_type ltat ON ltat.attribute_type = lat.root AND ltat.link_type = lt.id
    ) THEN
        RAISE EXCEPTION 'Attribute type % is invalid for link %', NEW.attribute_type, NEW.link;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

--------------------------------------------------------------------------------
-- Remove unused link rows when a relationship is changed
--------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION remove_unused_links()
RETURNS TRIGGER AS $$
DECLARE
    other_ars_exist BOOLEAN;
BEGIN
    EXECUTE 'SELECT EXISTS (SELECT TRUE FROM ' || quote_ident(TG_TABLE_NAME) ||
            ' WHERE link = $1)'
    INTO other_ars_exist
    USING OLD.link;

    IF NOT other_ars_exist THEN
       DELETE FROM link_attribute WHERE link = OLD.link;
       DELETE FROM link_attribute_credit WHERE link = OLD.link;
       DELETE FROM link_attribute_text_value WHERE link = OLD.link;
       DELETE FROM link WHERE id = OLD.link;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION delete_unused_url(ids INTEGER[])
RETURNS VOID AS $$
DECLARE
  clear_up INTEGER[];
BEGIN
  SELECT ARRAY(
    SELECT id FROM url url_row WHERE id = any(ids)
    EXCEPT
    SELECT url FROM edit_url JOIN edit ON (edit.id = edit_url.edit) WHERE edit.status = 1
    EXCEPT
    SELECT entity1 FROM l_area_url
    EXCEPT
    SELECT entity1 FROM l_artist_url
    EXCEPT
    SELECT entity1 FROM l_event_url
    EXCEPT
    SELECT entity1 FROM l_instrument_url
    EXCEPT
    SELECT entity1 FROM l_label_url
    EXCEPT
    SELECT entity1 FROM l_place_url
    EXCEPT
    SELECT entity1 FROM l_recording_url
    EXCEPT
    SELECT entity1 FROM l_release_url
    EXCEPT
    SELECT entity1 FROM l_release_group_url
    EXCEPT
    SELECT entity1 FROM l_series_url
    EXCEPT
    SELECT entity1 FROM l_url_url
    EXCEPT
    SELECT entity0 FROM l_url_url
    EXCEPT
    SELECT entity0 FROM l_url_work
  ) INTO clear_up;

  DELETE FROM url_gid_redirect WHERE new_id = any(clear_up);
  DELETE FROM url WHERE id = any(clear_up);
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION remove_unused_url()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_TABLE_NAME LIKE 'l_url_%' THEN
      EXECUTE delete_unused_url(ARRAY[OLD.entity0]);
    END IF;

    IF TG_TABLE_NAME LIKE 'l_%_url' THEN
      EXECUTE delete_unused_url(ARRAY[OLD.entity1]);
    END IF;

    IF TG_TABLE_NAME LIKE 'url' THEN
      EXECUTE delete_unused_url(ARRAY[OLD.id, NEW.id]);
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION simplify_search_hints()
RETURNS trigger AS $$
BEGIN
    IF NEW.type::int = TG_ARGV[0]::int THEN
        NEW.sort_name := NEW.name;
        NEW.begin_date_year := NULL;
        NEW.begin_date_month := NULL;
        NEW.begin_date_day := NULL;
        NEW.end_date_year := NULL;
        NEW.end_date_month := NULL;
        NEW.end_date_day := NULL;
        NEW.end_date_day := NULL;
        NEW.ended := FALSE;
        NEW.locale := NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION end_date_implies_ended()
RETURNS trigger AS $$
BEGIN
    IF NEW.end_date_year IS NOT NULL OR
       NEW.end_date_month IS NOT NULL OR
       NEW.end_date_day IS NOT NULL
    THEN
        NEW.ended = TRUE;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION end_area_implies_ended()
RETURNS trigger AS $$
BEGIN
    IF NEW.end_area IS NOT NULL
    THEN
        NEW.ended = TRUE;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION delete_orphaned_recordings()
RETURNS TRIGGER
AS $$
  BEGIN
    PERFORM TRUE
    FROM recording outer_r
    WHERE id = OLD.recording
      AND edits_pending = 0
      AND NOT EXISTS (
        SELECT TRUE
        FROM edit JOIN edit_recording er ON edit.id = er.edit
        WHERE er.recording = outer_r.id
          AND type IN (71, 207, 218)
          LIMIT 1
      ) AND NOT EXISTS (
        SELECT TRUE FROM track WHERE track.recording = outer_r.id LIMIT 1
      ) AND NOT EXISTS (
        SELECT TRUE FROM l_area_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_artist_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_event_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_instrument_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_label_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_place_recording WHERE entity1 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_recording_recording WHERE entity1 = outer_r.id OR entity0 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_recording_release WHERE entity0 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_recording_release_group WHERE entity0 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_recording_series WHERE entity0 = outer_r.id
          UNION ALL
        SELECT TRUE FROM l_recording_work WHERE entity0 = outer_r.id
          UNION ALL
         SELECT TRUE FROM l_recording_url WHERE entity0 = outer_r.id
      );

    IF FOUND THEN
      -- Remove references from tables that don't change whether or not this recording
      -- is orphaned.
      DELETE FROM isrc WHERE recording = OLD.recording;
      DELETE FROM recording_alias WHERE recording = OLD.recording;
      DELETE FROM recording_annotation WHERE recording = OLD.recording;
      DELETE FROM recording_gid_redirect WHERE new_id = OLD.recording;
      DELETE FROM recording_rating_raw WHERE recording = OLD.recording;
      DELETE FROM recording_tag WHERE recording = OLD.recording;
      DELETE FROM recording_tag_raw WHERE recording = OLD.recording;
      DELETE FROM editor_collection_recording WHERE recording = OLD.recording;

      DELETE FROM recording WHERE id = OLD.recording;
    END IF;

    RETURN NULL;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION padded_by_whitespace(TEXT) RETURNS boolean AS $$
  SELECT btrim($1) <> $1;
$$ LANGUAGE SQL IMMUTABLE;

CREATE OR REPLACE FUNCTION whitespace_collapsed(TEXT) RETURNS boolean AS $$
  SELECT $1 !~ E'\\s{2,}';
$$ LANGUAGE SQL IMMUTABLE;

CREATE OR REPLACE FUNCTION controlled_for_whitespace(TEXT) RETURNS boolean AS $$
  SELECT NOT padded_by_whitespace($1) AND whitespace_collapsed($1);
$$ LANGUAGE SQL IMMUTABLE SET search_path = musicbrainz, public;

CREATE OR REPLACE FUNCTION delete_unused_tag(tag_id INT)
RETURNS void AS $$
  BEGIN
    DELETE FROM tag WHERE id = tag_id;
  EXCEPTION
    WHEN foreign_key_violation THEN RETURN;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION trg_delete_unused_tag()
RETURNS trigger AS $$
  BEGIN
    PERFORM delete_unused_tag(NEW.id);
    RETURN NULL;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION trg_delete_unused_tag_ref()
RETURNS trigger AS $$
  BEGIN
    PERFORM delete_unused_tag(OLD.tag);
    RETURN NULL;
  END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION inserting_edits_requires_confirmed_email_address()
RETURNS trigger AS $$
BEGIN
  IF NOT (
    SELECT email_confirm_date IS NOT NULL AND email_confirm_date <= now()
    FROM editor
    WHERE editor.id = NEW.editor
  ) THEN
    RAISE EXCEPTION 'Editor tried to create edit without a confirmed email address';
  ELSE
    RETURN NEW;
  END IF;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION deny_deprecated_links()
RETURNS trigger AS $$
BEGIN
  IF (TG_OP = 'INSERT' OR (TG_OP = 'UPDATE' AND OLD.link_type <> NEW.link_type))
    AND (SELECT is_deprecated FROM link_type WHERE id = NEW.link_type)
  THEN
    RAISE EXCEPTION 'Attempt to create or change a relationship into a deprecated relationship type';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION check_has_dates()
RETURNS trigger AS $$
BEGIN
    IF (NEW.begin_date_year IS NOT NULL OR
       NEW.begin_date_month IS NOT NULL OR
       NEW.begin_date_day IS NOT NULL OR
       NEW.end_date_year IS NOT NULL OR
       NEW.end_date_month IS NOT NULL OR
       NEW.end_date_day IS NOT NULL OR
       NEW.ended = TRUE)
       AND NOT (SELECT has_dates FROM link_type WHERE id = NEW.link_type)
  THEN
    RAISE EXCEPTION 'Attempt to add dates to a relationship type that does not support dates.';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION materialise_recording_length(recording_id INT)
RETURNS void as $$
BEGIN
  UPDATE recording SET length = median
   FROM (SELECT median_track_length(recording_id) median) track
  WHERE recording.id = recording_id
    AND recording.length IS DISTINCT FROM track.median;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION track_count_matches_cdtoc(medium, int) RETURNS boolean AS $$
    SELECT $1.track_count = $2 + COALESCE(
        (SELECT count(*) FROM track
         WHERE medium = $1.id AND (position = 0 OR is_data_track = true)
    ), 0);
$$ LANGUAGE SQL IMMUTABLE;

-----------------------------------------------------------------------
-- edit_note triggers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION a_ins_edit_note() RETURNS trigger AS $$
BEGIN
    INSERT INTO edit_note_recipient (recipient, edit_note) (
        SELECT edit.editor, NEW.id
          FROM edit
         WHERE edit.id = NEW.edit
           AND edit.editor != NEW.editor
    );
    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- Text search helpers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION mb_lower(input text) RETURNS text AS $$
  SELECT lower(input COLLATE musicbrainz.musicbrainz);
$$ LANGUAGE SQL IMMUTABLE PARALLEL SAFE STRICT;

CREATE OR REPLACE FUNCTION mb_simple_tsvector(input text) RETURNS tsvector AS $$
  -- The builtin 'simple' dictionary, which the mb_simple text search
  -- configuration makes use of, would normally lowercase the input string
  -- for us, but internally it hardcodes DEFAULT_COLLATION_OID; therefore
  -- we first lowercase the input string ourselves using mb_lower.
  SELECT to_tsvector('musicbrainz.mb_simple', musicbrainz.mb_lower(input));
$$ LANGUAGE SQL IMMUTABLE PARALLEL SAFE STRICT;

-----------------------------------------------------------------------
-- Edit data helpers
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION edit_data_type_info(data JSONB) RETURNS TEXT AS $$
BEGIN
    CASE jsonb_typeof(data)
    WHEN 'object' THEN
        RETURN '{' ||
            (SELECT string_agg(
                to_json(key) || ':' ||
                edit_data_type_info(jsonb_extract_path(data, key)),
                ',' ORDER BY key)
               FROM jsonb_object_keys(data) AS key) ||
            '}';
    WHEN 'array' THEN
        RETURN '[' ||
            (SELECT string_agg(
                DISTINCT edit_data_type_info(item),
                ',' ORDER BY edit_data_type_info(item))
               FROM jsonb_array_elements(data) AS item) ||
            ']';
    WHEN 'string' THEN
        RETURN '1';
    WHEN 'number' THEN
        RETURN '2';
    WHEN 'boolean' THEN
        RETURN '4';
    WHEN 'null' THEN
        RETURN '8';
    END CASE;
    RETURN '';
END;
$$ LANGUAGE plpgsql IMMUTABLE PARALLEL SAFE STRICT;

-----------------------------------------------------------------------
-- Maintain musicbrainz.artist_release
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION get_artist_release_rows(
    release_id INTEGER
) RETURNS SETOF artist_release AS $$
BEGIN
    -- PostgreSQL 12 generates a vastly more efficient plan when only
    -- one release ID is passed. A condition like `r.id = any(...)`
    -- can be over 200x slower, even with only one release ID in the
    -- array.
    RETURN QUERY EXECUTE $SQL$
        SELECT DISTINCT ON (ar.artist, r.id)
            ar.is_track_artist,
            ar.artist,
            integer_date(rfrd.year, rfrd.month, rfrd.day) AS first_release_date,
            array_agg(
                DISTINCT rl.catalog_number ORDER BY rl.catalog_number
            ) FILTER (WHERE rl.catalog_number IS NOT NULL)::TEXT[] AS catalog_numbers,
            min(iso.code ORDER BY iso.code)::CHAR(2) AS country_code,
            left(regexp_replace(
                (CASE r.barcode WHEN '' THEN '0' ELSE r.barcode END),
                '[^0-9]+', '', 'g'
            ), 18)::BIGINT AS barcode,
            left(r.name, 1)::CHAR(1) AS sort_character,
            r.id
        FROM (
            SELECT FALSE AS is_track_artist, racn.artist, r.id AS release
            FROM release r
            JOIN artist_credit_name racn ON racn.artist_credit = r.artist_credit
            UNION ALL
            SELECT TRUE AS is_track_artist, tacn.artist, m.release
            FROM medium m
            JOIN track t ON t.medium = m.id
            JOIN artist_credit_name tacn ON tacn.artist_credit = t.artist_credit
        ) ar
        JOIN release r ON r.id = ar.release
        LEFT JOIN release_first_release_date rfrd ON rfrd.release = r.id
        LEFT JOIN release_label rl ON rl.release = r.id
        LEFT JOIN release_country rc ON rc.release = r.id
        LEFT JOIN iso_3166_1 iso ON iso.area = rc.country
    $SQL$ || (CASE WHEN release_id IS NULL THEN '' ELSE 'WHERE r.id = $1' END) ||
    $SQL$
        GROUP BY ar.is_track_artist, ar.artist, rfrd.release, r.id
        ORDER BY ar.artist, r.id, ar.is_track_artist
    $SQL$
    USING release_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION apply_artist_release_pending_updates()
RETURNS trigger AS $$
DECLARE
    release_ids INTEGER[];
    release_id INTEGER;
BEGIN
    -- DO NOT modify any replicated tables in this function; it's used
    -- by a trigger on slaves.
    WITH pending AS (
        DELETE FROM artist_release_pending_update
        RETURNING release
    )
    SELECT array_agg(DISTINCT release)
    INTO release_ids
    FROM pending;

    IF coalesce(array_length(release_ids, 1), 0) > 0 THEN
        -- If the user hasn't generated `artist_release`, then we
        -- shouldn't update or insert to it. MBS determines whether to
        -- use this table based on it being non-empty, so a partial
        -- table would manifest as partial data on the website and
        -- webservice.
        PERFORM 1 FROM artist_release LIMIT 1;
        IF FOUND THEN
            DELETE FROM artist_release WHERE release = any(release_ids);

            FOREACH release_id IN ARRAY release_ids LOOP
                -- We handle each release ID separately because the
                -- `get_artist_release_rows` query can be planned much
                -- more efficiently that way.
                INSERT INTO artist_release
                SELECT * FROM get_artist_release_rows(release_id);
            END LOOP;
        END IF;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-----------------------------------------------------------------------
-- Maintain musicbrainz.artist_release_group
-----------------------------------------------------------------------

CREATE OR REPLACE FUNCTION get_artist_release_group_rows(
    release_group_id INTEGER
) RETURNS SETOF artist_release_group AS $$
BEGIN
    -- PostgreSQL 12 generates a vastly more efficient plan when only
    -- one release group ID is passed. A condition like
    -- `rg.id = any(...)` can be over 200x slower, even with only one
    -- release group ID in the array.
    RETURN QUERY EXECUTE $SQL$
        SELECT DISTINCT ON (arg.artist, rg.id)
            arg.is_track_artist,
            arg.artist,
            bool_and(r.status IS NOT NULL AND r.status != 1),
            rg.type::SMALLINT,
            array_agg(
                DISTINCT st.secondary_type ORDER BY st.secondary_type)
                FILTER (WHERE st.secondary_type IS NOT NULL
            )::SMALLINT[],
            integer_date(
                rgm.first_release_date_year,
                rgm.first_release_date_month,
                rgm.first_release_date_day
            ),
            left(rg.name, 1)::CHAR(1),
            rg.id
        FROM (
            SELECT FALSE AS is_track_artist, rgacn.artist, rg.id AS release_group
            FROM release_group rg
            JOIN artist_credit_name rgacn ON rgacn.artist_credit = rg.artist_credit
            UNION ALL
            SELECT TRUE AS is_track_artist, tacn.artist, r.release_group
            FROM release r
            JOIN medium m ON m.release = r.id
            JOIN track t ON t.medium = m.id
            JOIN artist_credit_name tacn ON tacn.artist_credit = t.artist_credit
        ) arg
        JOIN release_group rg ON rg.id = arg.release_group
        LEFT JOIN release r ON r.release_group = rg.id
        JOIN release_group_meta rgm ON rgm.id = rg.id
        LEFT JOIN release_group_secondary_type_join st ON st.release_group = rg.id
    $SQL$ || (CASE WHEN release_group_id IS NULL THEN '' ELSE 'WHERE rg.id = $1' END) ||
    $SQL$
        GROUP BY arg.is_track_artist, arg.artist, rgm.id, rg.id
        ORDER BY arg.artist, rg.id, arg.is_track_artist
    $SQL$
    USING release_group_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION apply_artist_release_group_pending_updates()
RETURNS trigger AS $$
DECLARE
    release_group_ids INTEGER[];
    release_group_id INTEGER;
BEGIN
    -- DO NOT modify any replicated tables in this function; it's used
    -- by a trigger on slaves.
    WITH pending AS (
        DELETE FROM artist_release_group_pending_update
        RETURNING release_group
    )
    SELECT array_agg(DISTINCT release_group)
    INTO release_group_ids
    FROM pending;

    IF coalesce(array_length(release_group_ids, 1), 0) > 0 THEN
        -- If the user hasn't generated `artist_release_group`, then we
        -- shouldn't update or insert to it. MBS determines whether to
        -- use this table based on it being non-empty, so a partial
        -- table would manifest as partial data on the website and
        -- webservice.
        PERFORM 1 FROM artist_release_group LIMIT 1;
        IF FOUND THEN
            DELETE FROM artist_release_group WHERE release_group = any(release_group_ids);

            FOREACH release_group_id IN ARRAY release_group_ids LOOP
                -- We handle each release group ID separately because
                -- the `get_artist_release_group_rows` query can be
                -- planned much more efficiently that way.
                INSERT INTO artist_release_group
                SELECT * FROM get_artist_release_group_rows(release_group_id);
            END LOOP;
        END IF;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE 'plpgsql';

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateConstraints.sql

ALTER TABLE area          ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE artist        ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE event         ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE instrument    ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE label         ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE medium        ADD CHECK (controlled_for_whitespace(name));
ALTER TABLE place         ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE recording     ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE release       ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE release_group ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE release_label ADD CHECK (controlled_for_whitespace(catalog_number));
ALTER TABLE series        ADD CHECK (controlled_for_whitespace(comment));
ALTER TABLE track         ADD CHECK (controlled_for_whitespace(number));
ALTER TABLE work          ADD CHECK (controlled_for_whitespace(comment));

ALTER TABLE area
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE area_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE artist
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE artist_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE editor_collection_type
      ADD CONSTRAINT allowed_collection_entity_type CHECK (
          entity_type IN (
            'area', 'artist', 'event', 'instrument', 'label',
            'place', 'recording', 'release', 'release_group',
            'series', 'work'
          )
      );

ALTER TABLE event
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name));

ALTER TABLE event_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE artist_credit
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE artist_credit_name
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE instrument
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE instrument_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE label
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE label_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE link_attribute_text_value
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(text_value)),
  ADD CONSTRAINT only_non_empty CHECK (text_value != '');

ALTER TABLE place
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE place_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE release
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE release_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE release_group
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE release_group_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE track
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE recording
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE recording_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE series
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE series_type ADD CONSTRAINT allowed_series_entity_type
  CHECK (
    entity_type IN (
      'artist',
      'event',
      'recording',
      'release',
      'release_group',
      'work'
    )
  );

ALTER TABLE work
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != '');

ALTER TABLE work_alias
  ADD CONSTRAINT control_for_whitespace CHECK (controlled_for_whitespace(name)),
  ADD CONSTRAINT only_non_empty CHECK (name != ''),
  ADD CONSTRAINT control_for_whitespace_sort_name CHECK (controlled_for_whitespace(sort_name)),
  ADD CONSTRAINT only_non_empty_sort_name CHECK (sort_name != '');

ALTER TABLE artist
ADD CONSTRAINT group_type_implies_null_gender CHECK (
  (gender IS NULL AND type IN (2, 5, 6))
  OR type NOT IN (2, 5, 6)
  OR type IS NULL
);

ALTER TABLE release_label
ADD CHECK (catalog_number IS NOT NULL OR label IS NOT NULL);

ALTER TABLE artist ADD CONSTRAINT artist_va_check
    CHECK (id <> 1 OR
           (type = 3 AND
            gender IS NULL AND
            area IS NULL AND
            begin_area IS NULL AND
            end_area IS NULL AND
            begin_date_year IS NULL AND
            begin_date_month IS NULL AND
            begin_date_day IS NULL AND
            end_date_year IS NULL AND
            end_date_month IS NULL AND
            end_date_day IS NULL));

ALTER TABLE release_unknown_country ADD CONSTRAINT non_empty_date
    CHECK (date_year IS NOT NULL OR date_month IS NOT NULL OR date_day IS NOT NULL);

ALTER TABLE medium ADD CONSTRAINT medium_uniq
    UNIQUE (release, position) DEFERRABLE INITIALLY IMMEDIATE;

ALTER TABLE track ADD CONSTRAINT track_uniq_medium_position
    UNIQUE (medium, position) DEFERRABLE INITIALLY IMMEDIATE;

ALTER TABLE l_area_area ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_area ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_area_area ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_area_artist ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_artist ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_event ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_event ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_instrument ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_instrument ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_label ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_label ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_area_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_area_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_artist ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_artist ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_artist_artist ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_artist_event ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_event ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_instrument ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_instrument ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_label ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_label ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_artist_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_artist_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_event ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_event ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_event_event ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_event_instrument ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_instrument ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_label ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_label ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_event_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_event_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_instrument ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_instrument ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_instrument_instrument ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_instrument_label ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_label ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_instrument_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_instrument_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_label ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_label ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_label_label ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_label_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_label_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_label_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_place ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_place ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_place_place ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_place_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_place_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_place_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_recording_recording ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_recording ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_recording_recording ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_recording_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_recording_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_recording_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_recording_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_recording_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_recording_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_release ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_release ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_release_release ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_release_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_group_release_group ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_group_release_group ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_release_group_release_group ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_release_group_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_group_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_group_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_group_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_release_group_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_release_group_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_series_series ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_series_series ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_series_series ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_series_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_series_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_series_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_series_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_url_url ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_url_url ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_url_url ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

ALTER TABLE l_url_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_url_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));

ALTER TABLE l_work_work ADD CONSTRAINT control_for_whitespace_entity0_credit CHECK (controlled_for_whitespace(entity0_credit));
ALTER TABLE l_work_work ADD CONSTRAINT control_for_whitespace_entity1_credit CHECK (controlled_for_whitespace(entity1_credit));
ALTER TABLE l_work_work ADD CONSTRAINT non_loop_relationship CHECK (entity0 != entity1);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreatePrimaryKeys.sql

-- Automatically generated, do not edit.

ALTER TABLE alternative_medium ADD CONSTRAINT alternative_medium_pkey PRIMARY KEY (id);
ALTER TABLE alternative_medium_track ADD CONSTRAINT alternative_medium_track_pkey PRIMARY KEY (alternative_medium, track);
ALTER TABLE alternative_release ADD CONSTRAINT alternative_release_pkey PRIMARY KEY (id);
ALTER TABLE alternative_release_type ADD CONSTRAINT alternative_release_type_pkey PRIMARY KEY (id);
ALTER TABLE alternative_track ADD CONSTRAINT alternative_track_pkey PRIMARY KEY (id);
ALTER TABLE annotation ADD CONSTRAINT annotation_pkey PRIMARY KEY (id);
ALTER TABLE application ADD CONSTRAINT application_pkey PRIMARY KEY (id);
ALTER TABLE area ADD CONSTRAINT area_pkey PRIMARY KEY (id);
ALTER TABLE area_alias ADD CONSTRAINT area_alias_pkey PRIMARY KEY (id);
ALTER TABLE area_alias_type ADD CONSTRAINT area_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE area_annotation ADD CONSTRAINT area_annotation_pkey PRIMARY KEY (area, annotation);
ALTER TABLE area_attribute ADD CONSTRAINT area_attribute_pkey PRIMARY KEY (id);
ALTER TABLE area_attribute_type ADD CONSTRAINT area_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE area_attribute_type_allowed_value ADD CONSTRAINT area_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE area_gid_redirect ADD CONSTRAINT area_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE area_tag ADD CONSTRAINT area_tag_pkey PRIMARY KEY (area, tag);
ALTER TABLE area_tag_raw ADD CONSTRAINT area_tag_raw_pkey PRIMARY KEY (area, editor, tag);
ALTER TABLE area_type ADD CONSTRAINT area_type_pkey PRIMARY KEY (id);
ALTER TABLE artist ADD CONSTRAINT artist_pkey PRIMARY KEY (id);
ALTER TABLE artist_alias ADD CONSTRAINT artist_alias_pkey PRIMARY KEY (id);
ALTER TABLE artist_alias_type ADD CONSTRAINT artist_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE artist_annotation ADD CONSTRAINT artist_annotation_pkey PRIMARY KEY (artist, annotation);
ALTER TABLE artist_attribute ADD CONSTRAINT artist_attribute_pkey PRIMARY KEY (id);
ALTER TABLE artist_attribute_type ADD CONSTRAINT artist_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE artist_attribute_type_allowed_value ADD CONSTRAINT artist_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE artist_credit ADD CONSTRAINT artist_credit_pkey PRIMARY KEY (id);
ALTER TABLE artist_credit_name ADD CONSTRAINT artist_credit_name_pkey PRIMARY KEY (artist_credit, position);
ALTER TABLE artist_gid_redirect ADD CONSTRAINT artist_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE artist_ipi ADD CONSTRAINT artist_ipi_pkey PRIMARY KEY (artist, ipi);
ALTER TABLE artist_isni ADD CONSTRAINT artist_isni_pkey PRIMARY KEY (artist, isni);
ALTER TABLE artist_meta ADD CONSTRAINT artist_meta_pkey PRIMARY KEY (id);
ALTER TABLE artist_rating_raw ADD CONSTRAINT artist_rating_raw_pkey PRIMARY KEY (artist, editor);
ALTER TABLE artist_tag ADD CONSTRAINT artist_tag_pkey PRIMARY KEY (artist, tag);
ALTER TABLE artist_tag_raw ADD CONSTRAINT artist_tag_raw_pkey PRIMARY KEY (artist, editor, tag);
ALTER TABLE artist_type ADD CONSTRAINT artist_type_pkey PRIMARY KEY (id);
ALTER TABLE autoeditor_election ADD CONSTRAINT autoeditor_election_pkey PRIMARY KEY (id);
ALTER TABLE autoeditor_election_vote ADD CONSTRAINT autoeditor_election_vote_pkey PRIMARY KEY (id);
ALTER TABLE cdtoc ADD CONSTRAINT cdtoc_pkey PRIMARY KEY (id);
ALTER TABLE cdtoc_raw ADD CONSTRAINT cdtoc_raw_pkey PRIMARY KEY (id);
ALTER TABLE country_area ADD CONSTRAINT country_area_pkey PRIMARY KEY (area);
ALTER TABLE deleted_entity ADD CONSTRAINT deleted_entity_pkey PRIMARY KEY (gid);
ALTER TABLE edit ADD CONSTRAINT edit_pkey PRIMARY KEY (id);
ALTER TABLE edit_area ADD CONSTRAINT edit_area_pkey PRIMARY KEY (edit, area);
ALTER TABLE edit_artist ADD CONSTRAINT edit_artist_pkey PRIMARY KEY (edit, artist);
ALTER TABLE edit_data ADD CONSTRAINT edit_data_pkey PRIMARY KEY (edit);
ALTER TABLE edit_event ADD CONSTRAINT edit_event_pkey PRIMARY KEY (edit, event);
ALTER TABLE edit_instrument ADD CONSTRAINT edit_instrument_pkey PRIMARY KEY (edit, instrument);
ALTER TABLE edit_label ADD CONSTRAINT edit_label_pkey PRIMARY KEY (edit, label);
ALTER TABLE edit_note ADD CONSTRAINT edit_note_pkey PRIMARY KEY (id);
ALTER TABLE edit_note_recipient ADD CONSTRAINT edit_note_recipient_pkey PRIMARY KEY (recipient, edit_note);
ALTER TABLE edit_place ADD CONSTRAINT edit_place_pkey PRIMARY KEY (edit, place);
ALTER TABLE edit_recording ADD CONSTRAINT edit_recording_pkey PRIMARY KEY (edit, recording);
ALTER TABLE edit_release ADD CONSTRAINT edit_release_pkey PRIMARY KEY (edit, release);
ALTER TABLE edit_release_group ADD CONSTRAINT edit_release_group_pkey PRIMARY KEY (edit, release_group);
ALTER TABLE edit_series ADD CONSTRAINT edit_series_pkey PRIMARY KEY (edit, series);
ALTER TABLE edit_url ADD CONSTRAINT edit_url_pkey PRIMARY KEY (edit, url);
ALTER TABLE edit_work ADD CONSTRAINT edit_work_pkey PRIMARY KEY (edit, work);
ALTER TABLE editor ADD CONSTRAINT editor_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection ADD CONSTRAINT editor_collection_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection_area ADD CONSTRAINT editor_collection_area_pkey PRIMARY KEY (collection, area);
ALTER TABLE editor_collection_artist ADD CONSTRAINT editor_collection_artist_pkey PRIMARY KEY (collection, artist);
ALTER TABLE editor_collection_collaborator ADD CONSTRAINT editor_collection_collaborator_pkey PRIMARY KEY (collection, editor);
ALTER TABLE editor_collection_deleted_entity ADD CONSTRAINT editor_collection_deleted_entity_pkey PRIMARY KEY (collection, gid);
ALTER TABLE editor_collection_event ADD CONSTRAINT editor_collection_event_pkey PRIMARY KEY (collection, event);
ALTER TABLE editor_collection_gid_redirect ADD CONSTRAINT editor_collection_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE editor_collection_instrument ADD CONSTRAINT editor_collection_instrument_pkey PRIMARY KEY (collection, instrument);
ALTER TABLE editor_collection_label ADD CONSTRAINT editor_collection_label_pkey PRIMARY KEY (collection, label);
ALTER TABLE editor_collection_place ADD CONSTRAINT editor_collection_place_pkey PRIMARY KEY (collection, place);
ALTER TABLE editor_collection_recording ADD CONSTRAINT editor_collection_recording_pkey PRIMARY KEY (collection, recording);
ALTER TABLE editor_collection_release ADD CONSTRAINT editor_collection_release_pkey PRIMARY KEY (collection, release);
ALTER TABLE editor_collection_release_group ADD CONSTRAINT editor_collection_release_group_pkey PRIMARY KEY (collection, release_group);
ALTER TABLE editor_collection_series ADD CONSTRAINT editor_collection_series_pkey PRIMARY KEY (collection, series);
ALTER TABLE editor_collection_type ADD CONSTRAINT editor_collection_type_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection_work ADD CONSTRAINT editor_collection_work_pkey PRIMARY KEY (collection, work);
ALTER TABLE editor_language ADD CONSTRAINT editor_language_pkey PRIMARY KEY (editor, language);
ALTER TABLE editor_oauth_token ADD CONSTRAINT editor_oauth_token_pkey PRIMARY KEY (id);
ALTER TABLE editor_preference ADD CONSTRAINT editor_preference_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_artist ADD CONSTRAINT editor_subscribe_artist_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_artist_deleted ADD CONSTRAINT editor_subscribe_artist_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE editor_subscribe_collection ADD CONSTRAINT editor_subscribe_collection_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_editor ADD CONSTRAINT editor_subscribe_editor_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_label ADD CONSTRAINT editor_subscribe_label_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_label_deleted ADD CONSTRAINT editor_subscribe_label_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE editor_subscribe_series ADD CONSTRAINT editor_subscribe_series_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_series_deleted ADD CONSTRAINT editor_subscribe_series_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE editor_watch_artist ADD CONSTRAINT editor_watch_artist_pkey PRIMARY KEY (artist, editor);
ALTER TABLE editor_watch_preferences ADD CONSTRAINT editor_watch_preferences_pkey PRIMARY KEY (editor);
ALTER TABLE editor_watch_release_group_type ADD CONSTRAINT editor_watch_release_group_type_pkey PRIMARY KEY (editor, release_group_type);
ALTER TABLE editor_watch_release_status ADD CONSTRAINT editor_watch_release_status_pkey PRIMARY KEY (editor, release_status);
ALTER TABLE event ADD CONSTRAINT event_pkey PRIMARY KEY (id);
ALTER TABLE event_alias ADD CONSTRAINT event_alias_pkey PRIMARY KEY (id);
ALTER TABLE event_alias_type ADD CONSTRAINT event_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE event_annotation ADD CONSTRAINT event_annotation_pkey PRIMARY KEY (event, annotation);
ALTER TABLE event_attribute ADD CONSTRAINT event_attribute_pkey PRIMARY KEY (id);
ALTER TABLE event_attribute_type ADD CONSTRAINT event_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE event_attribute_type_allowed_value ADD CONSTRAINT event_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE event_gid_redirect ADD CONSTRAINT event_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE event_meta ADD CONSTRAINT event_meta_pkey PRIMARY KEY (id);
ALTER TABLE event_rating_raw ADD CONSTRAINT event_rating_raw_pkey PRIMARY KEY (event, editor);
ALTER TABLE event_tag ADD CONSTRAINT event_tag_pkey PRIMARY KEY (event, tag);
ALTER TABLE event_tag_raw ADD CONSTRAINT event_tag_raw_pkey PRIMARY KEY (event, editor, tag);
ALTER TABLE event_type ADD CONSTRAINT event_type_pkey PRIMARY KEY (id);
ALTER TABLE gender ADD CONSTRAINT gender_pkey PRIMARY KEY (id);
ALTER TABLE genre ADD CONSTRAINT genre_pkey PRIMARY KEY (id);
ALTER TABLE genre_alias ADD CONSTRAINT genre_alias_pkey PRIMARY KEY (id);
ALTER TABLE instrument ADD CONSTRAINT instrument_pkey PRIMARY KEY (id);
ALTER TABLE instrument_alias ADD CONSTRAINT instrument_alias_pkey PRIMARY KEY (id);
ALTER TABLE instrument_alias_type ADD CONSTRAINT instrument_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE instrument_annotation ADD CONSTRAINT instrument_annotation_pkey PRIMARY KEY (instrument, annotation);
ALTER TABLE instrument_attribute ADD CONSTRAINT instrument_attribute_pkey PRIMARY KEY (id);
ALTER TABLE instrument_attribute_type ADD CONSTRAINT instrument_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE instrument_attribute_type_allowed_value ADD CONSTRAINT instrument_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE instrument_gid_redirect ADD CONSTRAINT instrument_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE instrument_tag ADD CONSTRAINT instrument_tag_pkey PRIMARY KEY (instrument, tag);
ALTER TABLE instrument_tag_raw ADD CONSTRAINT instrument_tag_raw_pkey PRIMARY KEY (instrument, editor, tag);
ALTER TABLE instrument_type ADD CONSTRAINT instrument_type_pkey PRIMARY KEY (id);
ALTER TABLE iso_3166_1 ADD CONSTRAINT iso_3166_1_pkey PRIMARY KEY (code);
ALTER TABLE iso_3166_2 ADD CONSTRAINT iso_3166_2_pkey PRIMARY KEY (code);
ALTER TABLE iso_3166_3 ADD CONSTRAINT iso_3166_3_pkey PRIMARY KEY (code);
ALTER TABLE isrc ADD CONSTRAINT isrc_pkey PRIMARY KEY (id);
ALTER TABLE iswc ADD CONSTRAINT iswc_pkey PRIMARY KEY (id);
ALTER TABLE l_area_area ADD CONSTRAINT l_area_area_pkey PRIMARY KEY (id);
ALTER TABLE l_area_artist ADD CONSTRAINT l_area_artist_pkey PRIMARY KEY (id);
ALTER TABLE l_area_event ADD CONSTRAINT l_area_event_pkey PRIMARY KEY (id);
ALTER TABLE l_area_instrument ADD CONSTRAINT l_area_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_area_label ADD CONSTRAINT l_area_label_pkey PRIMARY KEY (id);
ALTER TABLE l_area_place ADD CONSTRAINT l_area_place_pkey PRIMARY KEY (id);
ALTER TABLE l_area_recording ADD CONSTRAINT l_area_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_area_release ADD CONSTRAINT l_area_release_pkey PRIMARY KEY (id);
ALTER TABLE l_area_release_group ADD CONSTRAINT l_area_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_area_series ADD CONSTRAINT l_area_series_pkey PRIMARY KEY (id);
ALTER TABLE l_area_url ADD CONSTRAINT l_area_url_pkey PRIMARY KEY (id);
ALTER TABLE l_area_work ADD CONSTRAINT l_area_work_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_artist ADD CONSTRAINT l_artist_artist_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_event ADD CONSTRAINT l_artist_event_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_instrument ADD CONSTRAINT l_artist_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_label ADD CONSTRAINT l_artist_label_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_place ADD CONSTRAINT l_artist_place_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_recording ADD CONSTRAINT l_artist_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_release ADD CONSTRAINT l_artist_release_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_release_group ADD CONSTRAINT l_artist_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_series ADD CONSTRAINT l_artist_series_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_url ADD CONSTRAINT l_artist_url_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_work ADD CONSTRAINT l_artist_work_pkey PRIMARY KEY (id);
ALTER TABLE l_event_event ADD CONSTRAINT l_event_event_pkey PRIMARY KEY (id);
ALTER TABLE l_event_instrument ADD CONSTRAINT l_event_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_event_label ADD CONSTRAINT l_event_label_pkey PRIMARY KEY (id);
ALTER TABLE l_event_place ADD CONSTRAINT l_event_place_pkey PRIMARY KEY (id);
ALTER TABLE l_event_recording ADD CONSTRAINT l_event_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_event_release ADD CONSTRAINT l_event_release_pkey PRIMARY KEY (id);
ALTER TABLE l_event_release_group ADD CONSTRAINT l_event_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_event_series ADD CONSTRAINT l_event_series_pkey PRIMARY KEY (id);
ALTER TABLE l_event_url ADD CONSTRAINT l_event_url_pkey PRIMARY KEY (id);
ALTER TABLE l_event_work ADD CONSTRAINT l_event_work_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_instrument ADD CONSTRAINT l_instrument_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_label ADD CONSTRAINT l_instrument_label_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_place ADD CONSTRAINT l_instrument_place_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_recording ADD CONSTRAINT l_instrument_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_release ADD CONSTRAINT l_instrument_release_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_release_group ADD CONSTRAINT l_instrument_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_series ADD CONSTRAINT l_instrument_series_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_url ADD CONSTRAINT l_instrument_url_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_work ADD CONSTRAINT l_instrument_work_pkey PRIMARY KEY (id);
ALTER TABLE l_label_label ADD CONSTRAINT l_label_label_pkey PRIMARY KEY (id);
ALTER TABLE l_label_place ADD CONSTRAINT l_label_place_pkey PRIMARY KEY (id);
ALTER TABLE l_label_recording ADD CONSTRAINT l_label_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_label_release ADD CONSTRAINT l_label_release_pkey PRIMARY KEY (id);
ALTER TABLE l_label_release_group ADD CONSTRAINT l_label_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_label_series ADD CONSTRAINT l_label_series_pkey PRIMARY KEY (id);
ALTER TABLE l_label_url ADD CONSTRAINT l_label_url_pkey PRIMARY KEY (id);
ALTER TABLE l_label_work ADD CONSTRAINT l_label_work_pkey PRIMARY KEY (id);
ALTER TABLE l_place_place ADD CONSTRAINT l_place_place_pkey PRIMARY KEY (id);
ALTER TABLE l_place_recording ADD CONSTRAINT l_place_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_place_release ADD CONSTRAINT l_place_release_pkey PRIMARY KEY (id);
ALTER TABLE l_place_release_group ADD CONSTRAINT l_place_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_place_series ADD CONSTRAINT l_place_series_pkey PRIMARY KEY (id);
ALTER TABLE l_place_url ADD CONSTRAINT l_place_url_pkey PRIMARY KEY (id);
ALTER TABLE l_place_work ADD CONSTRAINT l_place_work_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_recording ADD CONSTRAINT l_recording_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_release ADD CONSTRAINT l_recording_release_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_release_group ADD CONSTRAINT l_recording_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_series ADD CONSTRAINT l_recording_series_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_url ADD CONSTRAINT l_recording_url_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_work ADD CONSTRAINT l_recording_work_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_release_group ADD CONSTRAINT l_release_group_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_series ADD CONSTRAINT l_release_group_series_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_url ADD CONSTRAINT l_release_group_url_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_work ADD CONSTRAINT l_release_group_work_pkey PRIMARY KEY (id);
ALTER TABLE l_release_release ADD CONSTRAINT l_release_release_pkey PRIMARY KEY (id);
ALTER TABLE l_release_release_group ADD CONSTRAINT l_release_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_release_series ADD CONSTRAINT l_release_series_pkey PRIMARY KEY (id);
ALTER TABLE l_release_url ADD CONSTRAINT l_release_url_pkey PRIMARY KEY (id);
ALTER TABLE l_release_work ADD CONSTRAINT l_release_work_pkey PRIMARY KEY (id);
ALTER TABLE l_series_series ADD CONSTRAINT l_series_series_pkey PRIMARY KEY (id);
ALTER TABLE l_series_url ADD CONSTRAINT l_series_url_pkey PRIMARY KEY (id);
ALTER TABLE l_series_work ADD CONSTRAINT l_series_work_pkey PRIMARY KEY (id);
ALTER TABLE l_url_url ADD CONSTRAINT l_url_url_pkey PRIMARY KEY (id);
ALTER TABLE l_url_work ADD CONSTRAINT l_url_work_pkey PRIMARY KEY (id);
ALTER TABLE l_work_work ADD CONSTRAINT l_work_work_pkey PRIMARY KEY (id);
ALTER TABLE label ADD CONSTRAINT label_pkey PRIMARY KEY (id);
ALTER TABLE label_alias ADD CONSTRAINT label_alias_pkey PRIMARY KEY (id);
ALTER TABLE label_alias_type ADD CONSTRAINT label_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE label_annotation ADD CONSTRAINT label_annotation_pkey PRIMARY KEY (label, annotation);
ALTER TABLE label_attribute ADD CONSTRAINT label_attribute_pkey PRIMARY KEY (id);
ALTER TABLE label_attribute_type ADD CONSTRAINT label_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE label_attribute_type_allowed_value ADD CONSTRAINT label_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE label_gid_redirect ADD CONSTRAINT label_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE label_ipi ADD CONSTRAINT label_ipi_pkey PRIMARY KEY (label, ipi);
ALTER TABLE label_isni ADD CONSTRAINT label_isni_pkey PRIMARY KEY (label, isni);
ALTER TABLE label_meta ADD CONSTRAINT label_meta_pkey PRIMARY KEY (id);
ALTER TABLE label_rating_raw ADD CONSTRAINT label_rating_raw_pkey PRIMARY KEY (label, editor);
ALTER TABLE label_tag ADD CONSTRAINT label_tag_pkey PRIMARY KEY (label, tag);
ALTER TABLE label_tag_raw ADD CONSTRAINT label_tag_raw_pkey PRIMARY KEY (label, editor, tag);
ALTER TABLE label_type ADD CONSTRAINT label_type_pkey PRIMARY KEY (id);
ALTER TABLE language ADD CONSTRAINT language_pkey PRIMARY KEY (id);
ALTER TABLE link ADD CONSTRAINT link_pkey PRIMARY KEY (id);
ALTER TABLE link_attribute ADD CONSTRAINT link_attribute_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_credit ADD CONSTRAINT link_attribute_credit_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_text_value ADD CONSTRAINT link_attribute_text_value_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_type ADD CONSTRAINT link_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE link_creditable_attribute_type ADD CONSTRAINT link_creditable_attribute_type_pkey PRIMARY KEY (attribute_type);
ALTER TABLE link_text_attribute_type ADD CONSTRAINT link_text_attribute_type_pkey PRIMARY KEY (attribute_type);
ALTER TABLE link_type ADD CONSTRAINT link_type_pkey PRIMARY KEY (id);
ALTER TABLE link_type_attribute_type ADD CONSTRAINT link_type_attribute_type_pkey PRIMARY KEY (link_type, attribute_type);
ALTER TABLE medium ADD CONSTRAINT medium_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute ADD CONSTRAINT medium_attribute_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type ADD CONSTRAINT medium_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type_allowed_format ADD CONSTRAINT medium_attribute_type_allowed_format_pkey PRIMARY KEY (medium_format, medium_attribute_type);
ALTER TABLE medium_attribute_type_allowed_value ADD CONSTRAINT medium_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type_allowed_value_allowed_format ADD CONSTRAINT medium_attribute_type_allowed_value_allowed_format_pkey PRIMARY KEY (medium_format, medium_attribute_type_allowed_value);
ALTER TABLE medium_cdtoc ADD CONSTRAINT medium_cdtoc_pkey PRIMARY KEY (id);
ALTER TABLE medium_format ADD CONSTRAINT medium_format_pkey PRIMARY KEY (id);
ALTER TABLE medium_index ADD CONSTRAINT medium_index_pkey PRIMARY KEY (medium);
ALTER TABLE orderable_link_type ADD CONSTRAINT orderable_link_type_pkey PRIMARY KEY (link_type);
ALTER TABLE place ADD CONSTRAINT place_pkey PRIMARY KEY (id);
ALTER TABLE place_alias ADD CONSTRAINT place_alias_pkey PRIMARY KEY (id);
ALTER TABLE place_alias_type ADD CONSTRAINT place_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE place_annotation ADD CONSTRAINT place_annotation_pkey PRIMARY KEY (place, annotation);
ALTER TABLE place_attribute ADD CONSTRAINT place_attribute_pkey PRIMARY KEY (id);
ALTER TABLE place_attribute_type ADD CONSTRAINT place_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE place_attribute_type_allowed_value ADD CONSTRAINT place_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE place_gid_redirect ADD CONSTRAINT place_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE place_meta ADD CONSTRAINT place_meta_pkey PRIMARY KEY (id);
ALTER TABLE place_rating_raw ADD CONSTRAINT place_rating_raw_pkey PRIMARY KEY (place, editor);
ALTER TABLE place_tag ADD CONSTRAINT place_tag_pkey PRIMARY KEY (place, tag);
ALTER TABLE place_tag_raw ADD CONSTRAINT place_tag_raw_pkey PRIMARY KEY (place, editor, tag);
ALTER TABLE place_type ADD CONSTRAINT place_type_pkey PRIMARY KEY (id);
ALTER TABLE recording ADD CONSTRAINT recording_pkey PRIMARY KEY (id);
ALTER TABLE recording_alias ADD CONSTRAINT recording_alias_pkey PRIMARY KEY (id);
ALTER TABLE recording_alias_type ADD CONSTRAINT recording_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE recording_annotation ADD CONSTRAINT recording_annotation_pkey PRIMARY KEY (recording, annotation);
ALTER TABLE recording_attribute ADD CONSTRAINT recording_attribute_pkey PRIMARY KEY (id);
ALTER TABLE recording_attribute_type ADD CONSTRAINT recording_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE recording_attribute_type_allowed_value ADD CONSTRAINT recording_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE recording_first_release_date ADD CONSTRAINT recording_first_release_date_pkey PRIMARY KEY (recording);
ALTER TABLE recording_gid_redirect ADD CONSTRAINT recording_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE recording_meta ADD CONSTRAINT recording_meta_pkey PRIMARY KEY (id);
ALTER TABLE recording_rating_raw ADD CONSTRAINT recording_rating_raw_pkey PRIMARY KEY (recording, editor);
ALTER TABLE recording_tag ADD CONSTRAINT recording_tag_pkey PRIMARY KEY (recording, tag);
ALTER TABLE recording_tag_raw ADD CONSTRAINT recording_tag_raw_pkey PRIMARY KEY (recording, editor, tag);
ALTER TABLE release ADD CONSTRAINT release_pkey PRIMARY KEY (id);
ALTER TABLE release_alias ADD CONSTRAINT release_alias_pkey PRIMARY KEY (id);
ALTER TABLE release_alias_type ADD CONSTRAINT release_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE release_annotation ADD CONSTRAINT release_annotation_pkey PRIMARY KEY (release, annotation);
ALTER TABLE release_attribute ADD CONSTRAINT release_attribute_pkey PRIMARY KEY (id);
ALTER TABLE release_attribute_type ADD CONSTRAINT release_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE release_attribute_type_allowed_value ADD CONSTRAINT release_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE release_country ADD CONSTRAINT release_country_pkey PRIMARY KEY (release, country);
ALTER TABLE release_coverart ADD CONSTRAINT release_coverart_pkey PRIMARY KEY (id);
ALTER TABLE release_first_release_date ADD CONSTRAINT release_first_release_date_pkey PRIMARY KEY (release);
ALTER TABLE release_gid_redirect ADD CONSTRAINT release_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE release_group ADD CONSTRAINT release_group_pkey PRIMARY KEY (id);
ALTER TABLE release_group_alias ADD CONSTRAINT release_group_alias_pkey PRIMARY KEY (id);
ALTER TABLE release_group_alias_type ADD CONSTRAINT release_group_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_annotation ADD CONSTRAINT release_group_annotation_pkey PRIMARY KEY (release_group, annotation);
ALTER TABLE release_group_attribute ADD CONSTRAINT release_group_attribute_pkey PRIMARY KEY (id);
ALTER TABLE release_group_attribute_type ADD CONSTRAINT release_group_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_attribute_type_allowed_value ADD CONSTRAINT release_group_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE release_group_gid_redirect ADD CONSTRAINT release_group_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE release_group_meta ADD CONSTRAINT release_group_meta_pkey PRIMARY KEY (id);
ALTER TABLE release_group_primary_type ADD CONSTRAINT release_group_primary_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_rating_raw ADD CONSTRAINT release_group_rating_raw_pkey PRIMARY KEY (release_group, editor);
ALTER TABLE release_group_secondary_type ADD CONSTRAINT release_group_secondary_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_secondary_type_join ADD CONSTRAINT release_group_secondary_type_join_pkey PRIMARY KEY (release_group, secondary_type);
ALTER TABLE release_group_tag ADD CONSTRAINT release_group_tag_pkey PRIMARY KEY (release_group, tag);
ALTER TABLE release_group_tag_raw ADD CONSTRAINT release_group_tag_raw_pkey PRIMARY KEY (release_group, editor, tag);
ALTER TABLE release_label ADD CONSTRAINT release_label_pkey PRIMARY KEY (id);
ALTER TABLE release_meta ADD CONSTRAINT release_meta_pkey PRIMARY KEY (id);
ALTER TABLE release_packaging ADD CONSTRAINT release_packaging_pkey PRIMARY KEY (id);
ALTER TABLE release_raw ADD CONSTRAINT release_raw_pkey PRIMARY KEY (id);
ALTER TABLE release_status ADD CONSTRAINT release_status_pkey PRIMARY KEY (id);
ALTER TABLE release_tag ADD CONSTRAINT release_tag_pkey PRIMARY KEY (release, tag);
ALTER TABLE release_tag_raw ADD CONSTRAINT release_tag_raw_pkey PRIMARY KEY (release, editor, tag);
ALTER TABLE release_unknown_country ADD CONSTRAINT release_unknown_country_pkey PRIMARY KEY (release);
ALTER TABLE replication_control ADD CONSTRAINT replication_control_pkey PRIMARY KEY (id);
ALTER TABLE script ADD CONSTRAINT script_pkey PRIMARY KEY (id);
ALTER TABLE series ADD CONSTRAINT series_pkey PRIMARY KEY (id);
ALTER TABLE series_alias ADD CONSTRAINT series_alias_pkey PRIMARY KEY (id);
ALTER TABLE series_alias_type ADD CONSTRAINT series_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE series_annotation ADD CONSTRAINT series_annotation_pkey PRIMARY KEY (series, annotation);
ALTER TABLE series_attribute ADD CONSTRAINT series_attribute_pkey PRIMARY KEY (id);
ALTER TABLE series_attribute_type ADD CONSTRAINT series_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE series_attribute_type_allowed_value ADD CONSTRAINT series_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE series_gid_redirect ADD CONSTRAINT series_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE series_ordering_type ADD CONSTRAINT series_ordering_type_pkey PRIMARY KEY (id);
ALTER TABLE series_tag ADD CONSTRAINT series_tag_pkey PRIMARY KEY (series, tag);
ALTER TABLE series_tag_raw ADD CONSTRAINT series_tag_raw_pkey PRIMARY KEY (series, editor, tag);
ALTER TABLE series_type ADD CONSTRAINT series_type_pkey PRIMARY KEY (id);
ALTER TABLE tag ADD CONSTRAINT tag_pkey PRIMARY KEY (id);
ALTER TABLE tag_relation ADD CONSTRAINT tag_relation_pkey PRIMARY KEY (tag1, tag2);
ALTER TABLE track ADD CONSTRAINT track_pkey PRIMARY KEY (id);
ALTER TABLE track_gid_redirect ADD CONSTRAINT track_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE track_raw ADD CONSTRAINT track_raw_pkey PRIMARY KEY (id);
ALTER TABLE url ADD CONSTRAINT url_pkey PRIMARY KEY (id);
ALTER TABLE url_gid_redirect ADD CONSTRAINT url_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE vote ADD CONSTRAINT vote_pkey PRIMARY KEY (id);
ALTER TABLE work ADD CONSTRAINT work_pkey PRIMARY KEY (id);
ALTER TABLE work_alias ADD CONSTRAINT work_alias_pkey PRIMARY KEY (id);
ALTER TABLE work_alias_type ADD CONSTRAINT work_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE work_annotation ADD CONSTRAINT work_annotation_pkey PRIMARY KEY (work, annotation);
ALTER TABLE work_attribute ADD CONSTRAINT work_attribute_pkey PRIMARY KEY (id);
ALTER TABLE work_attribute_type ADD CONSTRAINT work_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE work_attribute_type_allowed_value ADD CONSTRAINT work_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE work_gid_redirect ADD CONSTRAINT work_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE work_language ADD CONSTRAINT work_language_pkey PRIMARY KEY (work, language);
ALTER TABLE work_meta ADD CONSTRAINT work_meta_pkey PRIMARY KEY (id);
ALTER TABLE work_rating_raw ADD CONSTRAINT work_rating_raw_pkey PRIMARY KEY (work, editor);
ALTER TABLE work_tag ADD CONSTRAINT work_tag_pkey PRIMARY KEY (work, tag);
ALTER TABLE work_tag_raw ADD CONSTRAINT work_tag_raw_pkey PRIMARY KEY (work, editor, tag);
ALTER TABLE work_type ADD CONSTRAINT work_type_pkey PRIMARY KEY (id);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateFKConstraints.sql

-- Automatically generated, do not edit.

ALTER TABLE alternative_medium
   ADD CONSTRAINT alternative_medium_fk_medium
   FOREIGN KEY (medium)
   REFERENCES medium(id);

ALTER TABLE alternative_medium
   ADD CONSTRAINT alternative_medium_fk_alternative_release
   FOREIGN KEY (alternative_release)
   REFERENCES alternative_release(id);

ALTER TABLE alternative_medium_track
   ADD CONSTRAINT alternative_medium_track_fk_alternative_medium
   FOREIGN KEY (alternative_medium)
   REFERENCES alternative_medium(id);

ALTER TABLE alternative_medium_track
   ADD CONSTRAINT alternative_medium_track_fk_track
   FOREIGN KEY (track)
   REFERENCES track(id);

ALTER TABLE alternative_medium_track
   ADD CONSTRAINT alternative_medium_track_fk_alternative_track
   FOREIGN KEY (alternative_track)
   REFERENCES alternative_track(id);

ALTER TABLE alternative_release
   ADD CONSTRAINT alternative_release_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE alternative_release
   ADD CONSTRAINT alternative_release_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE alternative_release
   ADD CONSTRAINT alternative_release_fk_type
   FOREIGN KEY (type)
   REFERENCES alternative_release_type(id);

ALTER TABLE alternative_release
   ADD CONSTRAINT alternative_release_fk_language
   FOREIGN KEY (language)
   REFERENCES language(id);

ALTER TABLE alternative_release
   ADD CONSTRAINT alternative_release_fk_script
   FOREIGN KEY (script)
   REFERENCES script(id);

ALTER TABLE alternative_release_type
   ADD CONSTRAINT alternative_release_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES alternative_release_type(id);

ALTER TABLE alternative_track
   ADD CONSTRAINT alternative_track_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE annotation
   ADD CONSTRAINT annotation_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE application
   ADD CONSTRAINT application_fk_owner
   FOREIGN KEY (owner)
   REFERENCES editor(id);

ALTER TABLE area
   ADD CONSTRAINT area_fk_type
   FOREIGN KEY (type)
   REFERENCES area_type(id);

ALTER TABLE area_alias
   ADD CONSTRAINT area_alias_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE area_alias
   ADD CONSTRAINT area_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES area_alias_type(id);

ALTER TABLE area_alias_type
   ADD CONSTRAINT area_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES area_alias_type(id);

ALTER TABLE area_annotation
   ADD CONSTRAINT area_annotation_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE area_annotation
   ADD CONSTRAINT area_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE area_attribute
   ADD CONSTRAINT area_attribute_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE area_attribute
   ADD CONSTRAINT area_attribute_fk_area_attribute_type
   FOREIGN KEY (area_attribute_type)
   REFERENCES area_attribute_type(id);

ALTER TABLE area_attribute
   ADD CONSTRAINT area_attribute_fk_area_attribute_type_allowed_value
   FOREIGN KEY (area_attribute_type_allowed_value)
   REFERENCES area_attribute_type_allowed_value(id);

ALTER TABLE area_attribute_type
   ADD CONSTRAINT area_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES area_attribute_type(id);

ALTER TABLE area_attribute_type_allowed_value
   ADD CONSTRAINT area_attribute_type_allowed_value_fk_area_attribute_type
   FOREIGN KEY (area_attribute_type)
   REFERENCES area_attribute_type(id);

ALTER TABLE area_attribute_type_allowed_value
   ADD CONSTRAINT area_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES area_attribute_type_allowed_value(id);

ALTER TABLE area_gid_redirect
   ADD CONSTRAINT area_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES area(id);

ALTER TABLE area_tag
   ADD CONSTRAINT area_tag_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE area_tag
   ADD CONSTRAINT area_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE area_tag_raw
   ADD CONSTRAINT area_tag_raw_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE area_tag_raw
   ADD CONSTRAINT area_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE area_tag_raw
   ADD CONSTRAINT area_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE area_type
   ADD CONSTRAINT area_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES area_type(id);

ALTER TABLE artist
   ADD CONSTRAINT artist_fk_type
   FOREIGN KEY (type)
   REFERENCES artist_type(id);

ALTER TABLE artist
   ADD CONSTRAINT artist_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE artist
   ADD CONSTRAINT artist_fk_gender
   FOREIGN KEY (gender)
   REFERENCES gender(id);

ALTER TABLE artist
   ADD CONSTRAINT artist_fk_begin_area
   FOREIGN KEY (begin_area)
   REFERENCES area(id);

ALTER TABLE artist
   ADD CONSTRAINT artist_fk_end_area
   FOREIGN KEY (end_area)
   REFERENCES area(id);

ALTER TABLE artist_alias
   ADD CONSTRAINT artist_alias_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_alias
   ADD CONSTRAINT artist_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES artist_alias_type(id);

ALTER TABLE artist_alias_type
   ADD CONSTRAINT artist_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES artist_alias_type(id);

ALTER TABLE artist_annotation
   ADD CONSTRAINT artist_annotation_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_annotation
   ADD CONSTRAINT artist_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE artist_attribute
   ADD CONSTRAINT artist_attribute_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_attribute
   ADD CONSTRAINT artist_attribute_fk_artist_attribute_type
   FOREIGN KEY (artist_attribute_type)
   REFERENCES artist_attribute_type(id);

ALTER TABLE artist_attribute
   ADD CONSTRAINT artist_attribute_fk_artist_attribute_type_allowed_value
   FOREIGN KEY (artist_attribute_type_allowed_value)
   REFERENCES artist_attribute_type_allowed_value(id);

ALTER TABLE artist_attribute_type
   ADD CONSTRAINT artist_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES artist_attribute_type(id);

ALTER TABLE artist_attribute_type_allowed_value
   ADD CONSTRAINT artist_attribute_type_allowed_value_fk_artist_attribute_type
   FOREIGN KEY (artist_attribute_type)
   REFERENCES artist_attribute_type(id);

ALTER TABLE artist_attribute_type_allowed_value
   ADD CONSTRAINT artist_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES artist_attribute_type_allowed_value(id);

ALTER TABLE artist_credit_name
   ADD CONSTRAINT artist_credit_name_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id)
   ON DELETE CASCADE;

ALTER TABLE artist_credit_name
   ADD CONSTRAINT artist_credit_name_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE artist_gid_redirect
   ADD CONSTRAINT artist_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES artist(id);

ALTER TABLE artist_ipi
   ADD CONSTRAINT artist_ipi_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_isni
   ADD CONSTRAINT artist_isni_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_meta
   ADD CONSTRAINT artist_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE artist_rating_raw
   ADD CONSTRAINT artist_rating_raw_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_rating_raw
   ADD CONSTRAINT artist_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

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

ALTER TABLE artist_release_group
   ADD CONSTRAINT artist_release_group_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE artist_release_group
   ADD CONSTRAINT artist_release_group_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id)
   ON DELETE CASCADE;

ALTER TABLE artist_tag
   ADD CONSTRAINT artist_tag_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_tag
   ADD CONSTRAINT artist_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE artist_tag_raw
   ADD CONSTRAINT artist_tag_raw_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE artist_tag_raw
   ADD CONSTRAINT artist_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE artist_tag_raw
   ADD CONSTRAINT artist_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE artist_type
   ADD CONSTRAINT artist_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES artist_type(id);

ALTER TABLE autoeditor_election
   ADD CONSTRAINT autoeditor_election_fk_candidate
   FOREIGN KEY (candidate)
   REFERENCES editor(id);

ALTER TABLE autoeditor_election
   ADD CONSTRAINT autoeditor_election_fk_proposer
   FOREIGN KEY (proposer)
   REFERENCES editor(id);

ALTER TABLE autoeditor_election
   ADD CONSTRAINT autoeditor_election_fk_seconder_1
   FOREIGN KEY (seconder_1)
   REFERENCES editor(id);

ALTER TABLE autoeditor_election
   ADD CONSTRAINT autoeditor_election_fk_seconder_2
   FOREIGN KEY (seconder_2)
   REFERENCES editor(id);

ALTER TABLE autoeditor_election_vote
   ADD CONSTRAINT autoeditor_election_vote_fk_autoeditor_election
   FOREIGN KEY (autoeditor_election)
   REFERENCES autoeditor_election(id);

ALTER TABLE autoeditor_election_vote
   ADD CONSTRAINT autoeditor_election_vote_fk_voter
   FOREIGN KEY (voter)
   REFERENCES editor(id);

ALTER TABLE cdtoc_raw
   ADD CONSTRAINT cdtoc_raw_fk_release
   FOREIGN KEY (release)
   REFERENCES release_raw(id);

ALTER TABLE country_area
   ADD CONSTRAINT country_area_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE edit
   ADD CONSTRAINT edit_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE edit
   ADD CONSTRAINT edit_fk_language
   FOREIGN KEY (language)
   REFERENCES language(id);

ALTER TABLE edit_area
   ADD CONSTRAINT edit_area_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_area
   ADD CONSTRAINT edit_area_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id)
   ON DELETE CASCADE;

ALTER TABLE edit_artist
   ADD CONSTRAINT edit_artist_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_artist
   ADD CONSTRAINT edit_artist_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE edit_data
   ADD CONSTRAINT edit_data_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_event
   ADD CONSTRAINT edit_event_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_event
   ADD CONSTRAINT edit_event_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id)
   ON DELETE CASCADE;

ALTER TABLE edit_instrument
   ADD CONSTRAINT edit_instrument_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_instrument
   ADD CONSTRAINT edit_instrument_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id)
   ON DELETE CASCADE;

ALTER TABLE edit_label
   ADD CONSTRAINT edit_label_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_label
   ADD CONSTRAINT edit_label_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id)
   ON DELETE CASCADE;

ALTER TABLE edit_note
   ADD CONSTRAINT edit_note_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE edit_note
   ADD CONSTRAINT edit_note_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_note_recipient
   ADD CONSTRAINT edit_note_recipient_fk_recipient
   FOREIGN KEY (recipient)
   REFERENCES editor(id);

ALTER TABLE edit_note_recipient
   ADD CONSTRAINT edit_note_recipient_fk_edit_note
   FOREIGN KEY (edit_note)
   REFERENCES edit_note(id);

ALTER TABLE edit_place
   ADD CONSTRAINT edit_place_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_place
   ADD CONSTRAINT edit_place_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id)
   ON DELETE CASCADE;

ALTER TABLE edit_recording
   ADD CONSTRAINT edit_recording_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_recording
   ADD CONSTRAINT edit_recording_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id)
   ON DELETE CASCADE;

ALTER TABLE edit_release
   ADD CONSTRAINT edit_release_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_release
   ADD CONSTRAINT edit_release_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id)
   ON DELETE CASCADE;

ALTER TABLE edit_release_group
   ADD CONSTRAINT edit_release_group_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_release_group
   ADD CONSTRAINT edit_release_group_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id)
   ON DELETE CASCADE;

ALTER TABLE edit_series
   ADD CONSTRAINT edit_series_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_series
   ADD CONSTRAINT edit_series_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id)
   ON DELETE CASCADE;

ALTER TABLE edit_url
   ADD CONSTRAINT edit_url_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_url
   ADD CONSTRAINT edit_url_fk_url
   FOREIGN KEY (url)
   REFERENCES url(id)
   ON DELETE CASCADE;

ALTER TABLE edit_work
   ADD CONSTRAINT edit_work_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE edit_work
   ADD CONSTRAINT edit_work_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id)
   ON DELETE CASCADE;

ALTER TABLE editor
   ADD CONSTRAINT editor_fk_gender
   FOREIGN KEY (gender)
   REFERENCES gender(id);

ALTER TABLE editor
   ADD CONSTRAINT editor_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE editor_collection
   ADD CONSTRAINT editor_collection_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_collection
   ADD CONSTRAINT editor_collection_fk_type
   FOREIGN KEY (type)
   REFERENCES editor_collection_type(id);

ALTER TABLE editor_collection_area
   ADD CONSTRAINT editor_collection_area_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_area
   ADD CONSTRAINT editor_collection_area_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE editor_collection_artist
   ADD CONSTRAINT editor_collection_artist_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_artist
   ADD CONSTRAINT editor_collection_artist_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE editor_collection_collaborator
   ADD CONSTRAINT editor_collection_collaborator_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_collaborator
   ADD CONSTRAINT editor_collection_collaborator_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_collection_deleted_entity
   ADD CONSTRAINT editor_collection_deleted_entity_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_deleted_entity
   ADD CONSTRAINT editor_collection_deleted_entity_fk_gid
   FOREIGN KEY (gid)
   REFERENCES deleted_entity(gid);

ALTER TABLE editor_collection_event
   ADD CONSTRAINT editor_collection_event_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_event
   ADD CONSTRAINT editor_collection_event_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE editor_collection_gid_redirect
   ADD CONSTRAINT editor_collection_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_instrument
   ADD CONSTRAINT editor_collection_instrument_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_instrument
   ADD CONSTRAINT editor_collection_instrument_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE editor_collection_label
   ADD CONSTRAINT editor_collection_label_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_label
   ADD CONSTRAINT editor_collection_label_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE editor_collection_place
   ADD CONSTRAINT editor_collection_place_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_place
   ADD CONSTRAINT editor_collection_place_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE editor_collection_recording
   ADD CONSTRAINT editor_collection_recording_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_recording
   ADD CONSTRAINT editor_collection_recording_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE editor_collection_release
   ADD CONSTRAINT editor_collection_release_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_release
   ADD CONSTRAINT editor_collection_release_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE editor_collection_release_group
   ADD CONSTRAINT editor_collection_release_group_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_release_group
   ADD CONSTRAINT editor_collection_release_group_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE editor_collection_series
   ADD CONSTRAINT editor_collection_series_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_series
   ADD CONSTRAINT editor_collection_series_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE editor_collection_type
   ADD CONSTRAINT editor_collection_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES editor_collection_type(id);

ALTER TABLE editor_collection_work
   ADD CONSTRAINT editor_collection_work_fk_collection
   FOREIGN KEY (collection)
   REFERENCES editor_collection(id);

ALTER TABLE editor_collection_work
   ADD CONSTRAINT editor_collection_work_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE editor_language
   ADD CONSTRAINT editor_language_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_language
   ADD CONSTRAINT editor_language_fk_language
   FOREIGN KEY (language)
   REFERENCES language(id);

ALTER TABLE editor_oauth_token
   ADD CONSTRAINT editor_oauth_token_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_oauth_token
   ADD CONSTRAINT editor_oauth_token_fk_application
   FOREIGN KEY (application)
   REFERENCES application(id);

ALTER TABLE editor_preference
   ADD CONSTRAINT editor_preference_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_artist
   ADD CONSTRAINT editor_subscribe_artist_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_artist
   ADD CONSTRAINT editor_subscribe_artist_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id);

ALTER TABLE editor_subscribe_artist
   ADD CONSTRAINT editor_subscribe_artist_fk_last_edit_sent
   FOREIGN KEY (last_edit_sent)
   REFERENCES edit(id);

ALTER TABLE editor_subscribe_artist_deleted
   ADD CONSTRAINT editor_subscribe_artist_deleted_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_artist_deleted
   ADD CONSTRAINT editor_subscribe_artist_deleted_fk_gid
   FOREIGN KEY (gid)
   REFERENCES deleted_entity(gid);

ALTER TABLE editor_subscribe_artist_deleted
   ADD CONSTRAINT editor_subscribe_artist_deleted_fk_deleted_by
   FOREIGN KEY (deleted_by)
   REFERENCES edit(id);

ALTER TABLE editor_subscribe_collection
   ADD CONSTRAINT editor_subscribe_collection_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_editor
   ADD CONSTRAINT editor_subscribe_editor_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_editor
   ADD CONSTRAINT editor_subscribe_editor_fk_subscribed_editor
   FOREIGN KEY (subscribed_editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_label
   ADD CONSTRAINT editor_subscribe_label_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_label
   ADD CONSTRAINT editor_subscribe_label_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE editor_subscribe_label
   ADD CONSTRAINT editor_subscribe_label_fk_last_edit_sent
   FOREIGN KEY (last_edit_sent)
   REFERENCES edit(id);

ALTER TABLE editor_subscribe_label_deleted
   ADD CONSTRAINT editor_subscribe_label_deleted_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_label_deleted
   ADD CONSTRAINT editor_subscribe_label_deleted_fk_gid
   FOREIGN KEY (gid)
   REFERENCES deleted_entity(gid);

ALTER TABLE editor_subscribe_label_deleted
   ADD CONSTRAINT editor_subscribe_label_deleted_fk_deleted_by
   FOREIGN KEY (deleted_by)
   REFERENCES edit(id);

ALTER TABLE editor_subscribe_series
   ADD CONSTRAINT editor_subscribe_series_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_series
   ADD CONSTRAINT editor_subscribe_series_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE editor_subscribe_series
   ADD CONSTRAINT editor_subscribe_series_fk_last_edit_sent
   FOREIGN KEY (last_edit_sent)
   REFERENCES edit(id);

ALTER TABLE editor_subscribe_series_deleted
   ADD CONSTRAINT editor_subscribe_series_deleted_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE editor_subscribe_series_deleted
   ADD CONSTRAINT editor_subscribe_series_deleted_fk_gid
   FOREIGN KEY (gid)
   REFERENCES deleted_entity(gid);

ALTER TABLE editor_subscribe_series_deleted
   ADD CONSTRAINT editor_subscribe_series_deleted_fk_deleted_by
   FOREIGN KEY (deleted_by)
   REFERENCES edit(id);

ALTER TABLE editor_watch_artist
   ADD CONSTRAINT editor_watch_artist_fk_artist
   FOREIGN KEY (artist)
   REFERENCES artist(id)
   ON DELETE CASCADE;

ALTER TABLE editor_watch_artist
   ADD CONSTRAINT editor_watch_artist_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id)
   ON DELETE CASCADE;

ALTER TABLE editor_watch_preferences
   ADD CONSTRAINT editor_watch_preferences_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id)
   ON DELETE CASCADE;

ALTER TABLE editor_watch_release_group_type
   ADD CONSTRAINT editor_watch_release_group_type_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id)
   ON DELETE CASCADE;

ALTER TABLE editor_watch_release_group_type
   ADD CONSTRAINT editor_watch_release_group_type_fk_release_group_type
   FOREIGN KEY (release_group_type)
   REFERENCES release_group_primary_type(id);

ALTER TABLE editor_watch_release_status
   ADD CONSTRAINT editor_watch_release_status_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id)
   ON DELETE CASCADE;

ALTER TABLE editor_watch_release_status
   ADD CONSTRAINT editor_watch_release_status_fk_release_status
   FOREIGN KEY (release_status)
   REFERENCES release_status(id);

ALTER TABLE event
   ADD CONSTRAINT event_fk_type
   FOREIGN KEY (type)
   REFERENCES event_type(id);

ALTER TABLE event_alias
   ADD CONSTRAINT event_alias_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_alias
   ADD CONSTRAINT event_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES event_alias_type(id);

ALTER TABLE event_alias_type
   ADD CONSTRAINT event_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES event_alias_type(id);

ALTER TABLE event_annotation
   ADD CONSTRAINT event_annotation_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_annotation
   ADD CONSTRAINT event_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE event_attribute
   ADD CONSTRAINT event_attribute_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_attribute
   ADD CONSTRAINT event_attribute_fk_event_attribute_type
   FOREIGN KEY (event_attribute_type)
   REFERENCES event_attribute_type(id);

ALTER TABLE event_attribute
   ADD CONSTRAINT event_attribute_fk_event_attribute_type_allowed_value
   FOREIGN KEY (event_attribute_type_allowed_value)
   REFERENCES event_attribute_type_allowed_value(id);

ALTER TABLE event_attribute_type
   ADD CONSTRAINT event_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES event_attribute_type(id);

ALTER TABLE event_attribute_type_allowed_value
   ADD CONSTRAINT event_attribute_type_allowed_value_fk_event_attribute_type
   FOREIGN KEY (event_attribute_type)
   REFERENCES event_attribute_type(id);

ALTER TABLE event_attribute_type_allowed_value
   ADD CONSTRAINT event_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES event_attribute_type_allowed_value(id);

ALTER TABLE event_gid_redirect
   ADD CONSTRAINT event_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES event(id);

ALTER TABLE event_meta
   ADD CONSTRAINT event_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES event(id)
   ON DELETE CASCADE;

ALTER TABLE event_rating_raw
   ADD CONSTRAINT event_rating_raw_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_rating_raw
   ADD CONSTRAINT event_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE event_tag
   ADD CONSTRAINT event_tag_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_tag
   ADD CONSTRAINT event_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE event_tag_raw
   ADD CONSTRAINT event_tag_raw_fk_event
   FOREIGN KEY (event)
   REFERENCES event(id);

ALTER TABLE event_tag_raw
   ADD CONSTRAINT event_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE event_tag_raw
   ADD CONSTRAINT event_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE event_type
   ADD CONSTRAINT event_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES event_type(id);

ALTER TABLE gender
   ADD CONSTRAINT gender_fk_parent
   FOREIGN KEY (parent)
   REFERENCES gender(id);

ALTER TABLE genre_alias
   ADD CONSTRAINT genre_alias_fk_genre
   FOREIGN KEY (genre)
   REFERENCES genre(id);

ALTER TABLE instrument
   ADD CONSTRAINT instrument_fk_type
   FOREIGN KEY (type)
   REFERENCES instrument_type(id);

ALTER TABLE instrument_alias
   ADD CONSTRAINT instrument_alias_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE instrument_alias
   ADD CONSTRAINT instrument_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES instrument_alias_type(id);

ALTER TABLE instrument_alias_type
   ADD CONSTRAINT instrument_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES instrument_alias_type(id);

ALTER TABLE instrument_annotation
   ADD CONSTRAINT instrument_annotation_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE instrument_annotation
   ADD CONSTRAINT instrument_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE instrument_attribute
   ADD CONSTRAINT instrument_attribute_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE instrument_attribute
   ADD CONSTRAINT instrument_attribute_fk_instrument_attribute_type
   FOREIGN KEY (instrument_attribute_type)
   REFERENCES instrument_attribute_type(id);

ALTER TABLE instrument_attribute
   ADD CONSTRAINT instrument_attribute_fk_instrument_attribute_type_allowed_value
   FOREIGN KEY (instrument_attribute_type_allowed_value)
   REFERENCES instrument_attribute_type_allowed_value(id);

ALTER TABLE instrument_attribute_type
   ADD CONSTRAINT instrument_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES instrument_attribute_type(id);

ALTER TABLE instrument_attribute_type_allowed_value
   ADD CONSTRAINT instrument_attribute_type_allowed_value_fk_instrument_attribute_type
   FOREIGN KEY (instrument_attribute_type)
   REFERENCES instrument_attribute_type(id);

ALTER TABLE instrument_attribute_type_allowed_value
   ADD CONSTRAINT instrument_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES instrument_attribute_type_allowed_value(id);

ALTER TABLE instrument_gid_redirect
   ADD CONSTRAINT instrument_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES instrument(id);

ALTER TABLE instrument_tag
   ADD CONSTRAINT instrument_tag_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE instrument_tag
   ADD CONSTRAINT instrument_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE instrument_tag_raw
   ADD CONSTRAINT instrument_tag_raw_fk_instrument
   FOREIGN KEY (instrument)
   REFERENCES instrument(id);

ALTER TABLE instrument_tag_raw
   ADD CONSTRAINT instrument_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE instrument_tag_raw
   ADD CONSTRAINT instrument_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE instrument_type
   ADD CONSTRAINT instrument_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES instrument_type(id);

ALTER TABLE iso_3166_1
   ADD CONSTRAINT iso_3166_1_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE iso_3166_2
   ADD CONSTRAINT iso_3166_2_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE iso_3166_3
   ADD CONSTRAINT iso_3166_3_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE isrc
   ADD CONSTRAINT isrc_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE iswc
   ADD CONSTRAINT iswc_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE l_area_area
   ADD CONSTRAINT l_area_area_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_area
   ADD CONSTRAINT l_area_area_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_area
   ADD CONSTRAINT l_area_area_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES area(id);

ALTER TABLE l_area_artist
   ADD CONSTRAINT l_area_artist_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_artist
   ADD CONSTRAINT l_area_artist_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_artist
   ADD CONSTRAINT l_area_artist_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES artist(id);

ALTER TABLE l_area_event
   ADD CONSTRAINT l_area_event_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_event
   ADD CONSTRAINT l_area_event_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_event
   ADD CONSTRAINT l_area_event_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES event(id);

ALTER TABLE l_area_instrument
   ADD CONSTRAINT l_area_instrument_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_instrument
   ADD CONSTRAINT l_area_instrument_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_instrument
   ADD CONSTRAINT l_area_instrument_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES instrument(id);

ALTER TABLE l_area_label
   ADD CONSTRAINT l_area_label_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_label
   ADD CONSTRAINT l_area_label_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_label
   ADD CONSTRAINT l_area_label_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES label(id);

ALTER TABLE l_area_place
   ADD CONSTRAINT l_area_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_place
   ADD CONSTRAINT l_area_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_place
   ADD CONSTRAINT l_area_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_area_recording
   ADD CONSTRAINT l_area_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_recording
   ADD CONSTRAINT l_area_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_recording
   ADD CONSTRAINT l_area_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_area_release
   ADD CONSTRAINT l_area_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_release
   ADD CONSTRAINT l_area_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_release
   ADD CONSTRAINT l_area_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_area_release_group
   ADD CONSTRAINT l_area_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_release_group
   ADD CONSTRAINT l_area_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_release_group
   ADD CONSTRAINT l_area_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_area_series
   ADD CONSTRAINT l_area_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_series
   ADD CONSTRAINT l_area_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_series
   ADD CONSTRAINT l_area_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_area_url
   ADD CONSTRAINT l_area_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_url
   ADD CONSTRAINT l_area_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_url
   ADD CONSTRAINT l_area_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_area_work
   ADD CONSTRAINT l_area_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_area_work
   ADD CONSTRAINT l_area_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES area(id);

ALTER TABLE l_area_work
   ADD CONSTRAINT l_area_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_artist_artist
   ADD CONSTRAINT l_artist_artist_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_artist
   ADD CONSTRAINT l_artist_artist_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_artist
   ADD CONSTRAINT l_artist_artist_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES artist(id);

ALTER TABLE l_artist_event
   ADD CONSTRAINT l_artist_event_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_event
   ADD CONSTRAINT l_artist_event_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_event
   ADD CONSTRAINT l_artist_event_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES event(id);

ALTER TABLE l_artist_instrument
   ADD CONSTRAINT l_artist_instrument_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_instrument
   ADD CONSTRAINT l_artist_instrument_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_instrument
   ADD CONSTRAINT l_artist_instrument_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES instrument(id);

ALTER TABLE l_artist_label
   ADD CONSTRAINT l_artist_label_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_label
   ADD CONSTRAINT l_artist_label_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_label
   ADD CONSTRAINT l_artist_label_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES label(id);

ALTER TABLE l_artist_place
   ADD CONSTRAINT l_artist_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_place
   ADD CONSTRAINT l_artist_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_place
   ADD CONSTRAINT l_artist_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_artist_recording
   ADD CONSTRAINT l_artist_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_recording
   ADD CONSTRAINT l_artist_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_recording
   ADD CONSTRAINT l_artist_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_artist_release
   ADD CONSTRAINT l_artist_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_release
   ADD CONSTRAINT l_artist_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_release
   ADD CONSTRAINT l_artist_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_artist_release_group
   ADD CONSTRAINT l_artist_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_release_group
   ADD CONSTRAINT l_artist_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_release_group
   ADD CONSTRAINT l_artist_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_artist_series
   ADD CONSTRAINT l_artist_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_series
   ADD CONSTRAINT l_artist_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_series
   ADD CONSTRAINT l_artist_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_artist_url
   ADD CONSTRAINT l_artist_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_url
   ADD CONSTRAINT l_artist_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_url
   ADD CONSTRAINT l_artist_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_artist_work
   ADD CONSTRAINT l_artist_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_artist_work
   ADD CONSTRAINT l_artist_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES artist(id);

ALTER TABLE l_artist_work
   ADD CONSTRAINT l_artist_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_event_event
   ADD CONSTRAINT l_event_event_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_event
   ADD CONSTRAINT l_event_event_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_event
   ADD CONSTRAINT l_event_event_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES event(id);

ALTER TABLE l_event_instrument
   ADD CONSTRAINT l_event_instrument_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_instrument
   ADD CONSTRAINT l_event_instrument_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_instrument
   ADD CONSTRAINT l_event_instrument_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES instrument(id);

ALTER TABLE l_event_label
   ADD CONSTRAINT l_event_label_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_label
   ADD CONSTRAINT l_event_label_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_label
   ADD CONSTRAINT l_event_label_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES label(id);

ALTER TABLE l_event_place
   ADD CONSTRAINT l_event_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_place
   ADD CONSTRAINT l_event_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_place
   ADD CONSTRAINT l_event_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_event_recording
   ADD CONSTRAINT l_event_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_recording
   ADD CONSTRAINT l_event_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_recording
   ADD CONSTRAINT l_event_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_event_release
   ADD CONSTRAINT l_event_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_release
   ADD CONSTRAINT l_event_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_release
   ADD CONSTRAINT l_event_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_event_release_group
   ADD CONSTRAINT l_event_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_release_group
   ADD CONSTRAINT l_event_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_release_group
   ADD CONSTRAINT l_event_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_event_series
   ADD CONSTRAINT l_event_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_series
   ADD CONSTRAINT l_event_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_series
   ADD CONSTRAINT l_event_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_event_url
   ADD CONSTRAINT l_event_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_url
   ADD CONSTRAINT l_event_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_url
   ADD CONSTRAINT l_event_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_event_work
   ADD CONSTRAINT l_event_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_event_work
   ADD CONSTRAINT l_event_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES event(id);

ALTER TABLE l_event_work
   ADD CONSTRAINT l_event_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_instrument_instrument
   ADD CONSTRAINT l_instrument_instrument_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_instrument
   ADD CONSTRAINT l_instrument_instrument_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_instrument
   ADD CONSTRAINT l_instrument_instrument_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_label
   ADD CONSTRAINT l_instrument_label_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_label
   ADD CONSTRAINT l_instrument_label_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_label
   ADD CONSTRAINT l_instrument_label_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES label(id);

ALTER TABLE l_instrument_place
   ADD CONSTRAINT l_instrument_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_place
   ADD CONSTRAINT l_instrument_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_place
   ADD CONSTRAINT l_instrument_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_instrument_recording
   ADD CONSTRAINT l_instrument_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_recording
   ADD CONSTRAINT l_instrument_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_recording
   ADD CONSTRAINT l_instrument_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_instrument_release
   ADD CONSTRAINT l_instrument_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_release
   ADD CONSTRAINT l_instrument_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_release
   ADD CONSTRAINT l_instrument_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_instrument_release_group
   ADD CONSTRAINT l_instrument_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_release_group
   ADD CONSTRAINT l_instrument_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_release_group
   ADD CONSTRAINT l_instrument_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_instrument_series
   ADD CONSTRAINT l_instrument_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_series
   ADD CONSTRAINT l_instrument_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_series
   ADD CONSTRAINT l_instrument_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_instrument_url
   ADD CONSTRAINT l_instrument_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_url
   ADD CONSTRAINT l_instrument_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_url
   ADD CONSTRAINT l_instrument_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_instrument_work
   ADD CONSTRAINT l_instrument_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_instrument_work
   ADD CONSTRAINT l_instrument_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES instrument(id);

ALTER TABLE l_instrument_work
   ADD CONSTRAINT l_instrument_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_label_label
   ADD CONSTRAINT l_label_label_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_label
   ADD CONSTRAINT l_label_label_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_label
   ADD CONSTRAINT l_label_label_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES label(id);

ALTER TABLE l_label_place
   ADD CONSTRAINT l_label_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_place
   ADD CONSTRAINT l_label_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_place
   ADD CONSTRAINT l_label_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_label_recording
   ADD CONSTRAINT l_label_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_recording
   ADD CONSTRAINT l_label_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_recording
   ADD CONSTRAINT l_label_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_label_release
   ADD CONSTRAINT l_label_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_release
   ADD CONSTRAINT l_label_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_release
   ADD CONSTRAINT l_label_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_label_release_group
   ADD CONSTRAINT l_label_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_release_group
   ADD CONSTRAINT l_label_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_release_group
   ADD CONSTRAINT l_label_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_label_series
   ADD CONSTRAINT l_label_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_series
   ADD CONSTRAINT l_label_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_series
   ADD CONSTRAINT l_label_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_label_url
   ADD CONSTRAINT l_label_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_url
   ADD CONSTRAINT l_label_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_url
   ADD CONSTRAINT l_label_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_label_work
   ADD CONSTRAINT l_label_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_label_work
   ADD CONSTRAINT l_label_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES label(id);

ALTER TABLE l_label_work
   ADD CONSTRAINT l_label_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_place_place
   ADD CONSTRAINT l_place_place_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_place
   ADD CONSTRAINT l_place_place_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_place
   ADD CONSTRAINT l_place_place_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES place(id);

ALTER TABLE l_place_recording
   ADD CONSTRAINT l_place_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_recording
   ADD CONSTRAINT l_place_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_recording
   ADD CONSTRAINT l_place_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_place_release
   ADD CONSTRAINT l_place_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_release
   ADD CONSTRAINT l_place_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_release
   ADD CONSTRAINT l_place_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_place_release_group
   ADD CONSTRAINT l_place_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_release_group
   ADD CONSTRAINT l_place_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_release_group
   ADD CONSTRAINT l_place_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_place_series
   ADD CONSTRAINT l_place_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_series
   ADD CONSTRAINT l_place_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_series
   ADD CONSTRAINT l_place_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_place_url
   ADD CONSTRAINT l_place_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_url
   ADD CONSTRAINT l_place_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_url
   ADD CONSTRAINT l_place_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_place_work
   ADD CONSTRAINT l_place_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_place_work
   ADD CONSTRAINT l_place_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES place(id);

ALTER TABLE l_place_work
   ADD CONSTRAINT l_place_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_recording_recording
   ADD CONSTRAINT l_recording_recording_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_recording
   ADD CONSTRAINT l_recording_recording_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_recording
   ADD CONSTRAINT l_recording_recording_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES recording(id);

ALTER TABLE l_recording_release
   ADD CONSTRAINT l_recording_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_release
   ADD CONSTRAINT l_recording_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_release
   ADD CONSTRAINT l_recording_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_recording_release_group
   ADD CONSTRAINT l_recording_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_release_group
   ADD CONSTRAINT l_recording_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_release_group
   ADD CONSTRAINT l_recording_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_recording_series
   ADD CONSTRAINT l_recording_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_series
   ADD CONSTRAINT l_recording_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_series
   ADD CONSTRAINT l_recording_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_recording_url
   ADD CONSTRAINT l_recording_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_url
   ADD CONSTRAINT l_recording_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_url
   ADD CONSTRAINT l_recording_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_recording_work
   ADD CONSTRAINT l_recording_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_recording_work
   ADD CONSTRAINT l_recording_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES recording(id);

ALTER TABLE l_recording_work
   ADD CONSTRAINT l_recording_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_release_group_release_group
   ADD CONSTRAINT l_release_group_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_group_release_group
   ADD CONSTRAINT l_release_group_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release_group(id);

ALTER TABLE l_release_group_release_group
   ADD CONSTRAINT l_release_group_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_release_group_series
   ADD CONSTRAINT l_release_group_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_group_series
   ADD CONSTRAINT l_release_group_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release_group(id);

ALTER TABLE l_release_group_series
   ADD CONSTRAINT l_release_group_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_release_group_url
   ADD CONSTRAINT l_release_group_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_group_url
   ADD CONSTRAINT l_release_group_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release_group(id);

ALTER TABLE l_release_group_url
   ADD CONSTRAINT l_release_group_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_release_group_work
   ADD CONSTRAINT l_release_group_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_group_work
   ADD CONSTRAINT l_release_group_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release_group(id);

ALTER TABLE l_release_group_work
   ADD CONSTRAINT l_release_group_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_release_release
   ADD CONSTRAINT l_release_release_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_release
   ADD CONSTRAINT l_release_release_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release(id);

ALTER TABLE l_release_release
   ADD CONSTRAINT l_release_release_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release(id);

ALTER TABLE l_release_release_group
   ADD CONSTRAINT l_release_release_group_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_release_group
   ADD CONSTRAINT l_release_release_group_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release(id);

ALTER TABLE l_release_release_group
   ADD CONSTRAINT l_release_release_group_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES release_group(id);

ALTER TABLE l_release_series
   ADD CONSTRAINT l_release_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_series
   ADD CONSTRAINT l_release_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release(id);

ALTER TABLE l_release_series
   ADD CONSTRAINT l_release_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_release_url
   ADD CONSTRAINT l_release_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_url
   ADD CONSTRAINT l_release_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release(id);

ALTER TABLE l_release_url
   ADD CONSTRAINT l_release_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_release_work
   ADD CONSTRAINT l_release_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_release_work
   ADD CONSTRAINT l_release_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES release(id);

ALTER TABLE l_release_work
   ADD CONSTRAINT l_release_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_series_series
   ADD CONSTRAINT l_series_series_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_series_series
   ADD CONSTRAINT l_series_series_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES series(id);

ALTER TABLE l_series_series
   ADD CONSTRAINT l_series_series_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES series(id);

ALTER TABLE l_series_url
   ADD CONSTRAINT l_series_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_series_url
   ADD CONSTRAINT l_series_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES series(id);

ALTER TABLE l_series_url
   ADD CONSTRAINT l_series_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_series_work
   ADD CONSTRAINT l_series_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_series_work
   ADD CONSTRAINT l_series_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES series(id);

ALTER TABLE l_series_work
   ADD CONSTRAINT l_series_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_url_url
   ADD CONSTRAINT l_url_url_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_url_url
   ADD CONSTRAINT l_url_url_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES url(id);

ALTER TABLE l_url_url
   ADD CONSTRAINT l_url_url_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES url(id);

ALTER TABLE l_url_work
   ADD CONSTRAINT l_url_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_url_work
   ADD CONSTRAINT l_url_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES url(id);

ALTER TABLE l_url_work
   ADD CONSTRAINT l_url_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE l_work_work
   ADD CONSTRAINT l_work_work_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE l_work_work
   ADD CONSTRAINT l_work_work_fk_entity0
   FOREIGN KEY (entity0)
   REFERENCES work(id);

ALTER TABLE l_work_work
   ADD CONSTRAINT l_work_work_fk_entity1
   FOREIGN KEY (entity1)
   REFERENCES work(id);

ALTER TABLE label
   ADD CONSTRAINT label_fk_type
   FOREIGN KEY (type)
   REFERENCES label_type(id);

ALTER TABLE label
   ADD CONSTRAINT label_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE label_alias
   ADD CONSTRAINT label_alias_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_alias
   ADD CONSTRAINT label_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES label_alias_type(id);

ALTER TABLE label_alias_type
   ADD CONSTRAINT label_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES label_alias_type(id);

ALTER TABLE label_annotation
   ADD CONSTRAINT label_annotation_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_annotation
   ADD CONSTRAINT label_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE label_attribute
   ADD CONSTRAINT label_attribute_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_attribute
   ADD CONSTRAINT label_attribute_fk_label_attribute_type
   FOREIGN KEY (label_attribute_type)
   REFERENCES label_attribute_type(id);

ALTER TABLE label_attribute
   ADD CONSTRAINT label_attribute_fk_label_attribute_type_allowed_value
   FOREIGN KEY (label_attribute_type_allowed_value)
   REFERENCES label_attribute_type_allowed_value(id);

ALTER TABLE label_attribute_type
   ADD CONSTRAINT label_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES label_attribute_type(id);

ALTER TABLE label_attribute_type_allowed_value
   ADD CONSTRAINT label_attribute_type_allowed_value_fk_label_attribute_type
   FOREIGN KEY (label_attribute_type)
   REFERENCES label_attribute_type(id);

ALTER TABLE label_attribute_type_allowed_value
   ADD CONSTRAINT label_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES label_attribute_type_allowed_value(id);

ALTER TABLE label_gid_redirect
   ADD CONSTRAINT label_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES label(id);

ALTER TABLE label_ipi
   ADD CONSTRAINT label_ipi_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_isni
   ADD CONSTRAINT label_isni_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_meta
   ADD CONSTRAINT label_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES label(id)
   ON DELETE CASCADE;

ALTER TABLE label_rating_raw
   ADD CONSTRAINT label_rating_raw_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_rating_raw
   ADD CONSTRAINT label_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE label_tag
   ADD CONSTRAINT label_tag_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_tag
   ADD CONSTRAINT label_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE label_tag_raw
   ADD CONSTRAINT label_tag_raw_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE label_tag_raw
   ADD CONSTRAINT label_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE label_tag_raw
   ADD CONSTRAINT label_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE label_type
   ADD CONSTRAINT label_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES label_type(id);

ALTER TABLE link
   ADD CONSTRAINT link_fk_link_type
   FOREIGN KEY (link_type)
   REFERENCES link_type(id);

ALTER TABLE link_attribute
   ADD CONSTRAINT link_attribute_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE link_attribute
   ADD CONSTRAINT link_attribute_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_attribute_type(id);

ALTER TABLE link_attribute_credit
   ADD CONSTRAINT link_attribute_credit_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE link_attribute_credit
   ADD CONSTRAINT link_attribute_credit_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_creditable_attribute_type(attribute_type);

ALTER TABLE link_attribute_text_value
   ADD CONSTRAINT link_attribute_text_value_fk_link
   FOREIGN KEY (link)
   REFERENCES link(id);

ALTER TABLE link_attribute_text_value
   ADD CONSTRAINT link_attribute_text_value_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_text_attribute_type(attribute_type);

ALTER TABLE link_attribute_type
   ADD CONSTRAINT link_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES link_attribute_type(id);

ALTER TABLE link_attribute_type
   ADD CONSTRAINT link_attribute_type_fk_root
   FOREIGN KEY (root)
   REFERENCES link_attribute_type(id);

ALTER TABLE link_creditable_attribute_type
   ADD CONSTRAINT link_creditable_attribute_type_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_attribute_type(id)
   ON DELETE CASCADE;

ALTER TABLE link_text_attribute_type
   ADD CONSTRAINT link_text_attribute_type_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_attribute_type(id)
   ON DELETE CASCADE;

ALTER TABLE link_type
   ADD CONSTRAINT link_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES link_type(id);

ALTER TABLE link_type_attribute_type
   ADD CONSTRAINT link_type_attribute_type_fk_link_type
   FOREIGN KEY (link_type)
   REFERENCES link_type(id);

ALTER TABLE link_type_attribute_type
   ADD CONSTRAINT link_type_attribute_type_fk_attribute_type
   FOREIGN KEY (attribute_type)
   REFERENCES link_attribute_type(id);

ALTER TABLE medium
   ADD CONSTRAINT medium_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE medium
   ADD CONSTRAINT medium_fk_format
   FOREIGN KEY (format)
   REFERENCES medium_format(id);

ALTER TABLE medium_attribute
   ADD CONSTRAINT medium_attribute_fk_medium
   FOREIGN KEY (medium)
   REFERENCES medium(id);

ALTER TABLE medium_attribute
   ADD CONSTRAINT medium_attribute_fk_medium_attribute_type
   FOREIGN KEY (medium_attribute_type)
   REFERENCES medium_attribute_type(id);

ALTER TABLE medium_attribute
   ADD CONSTRAINT medium_attribute_fk_medium_attribute_type_allowed_value
   FOREIGN KEY (medium_attribute_type_allowed_value)
   REFERENCES medium_attribute_type_allowed_value(id);

ALTER TABLE medium_attribute_type
   ADD CONSTRAINT medium_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES medium_attribute_type(id);

ALTER TABLE medium_attribute_type_allowed_format
   ADD CONSTRAINT medium_attribute_type_allowed_format_fk_medium_format
   FOREIGN KEY (medium_format)
   REFERENCES medium_format(id);

ALTER TABLE medium_attribute_type_allowed_format
   ADD CONSTRAINT medium_attribute_type_allowed_format_fk_medium_attribute_type
   FOREIGN KEY (medium_attribute_type)
   REFERENCES medium_attribute_type(id);

ALTER TABLE medium_attribute_type_allowed_value
   ADD CONSTRAINT medium_attribute_type_allowed_value_fk_medium_attribute_type
   FOREIGN KEY (medium_attribute_type)
   REFERENCES medium_attribute_type(id);

ALTER TABLE medium_attribute_type_allowed_value
   ADD CONSTRAINT medium_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES medium_attribute_type_allowed_value(id);

ALTER TABLE medium_attribute_type_allowed_value_allowed_format
   ADD CONSTRAINT medium_attribute_type_allowed_value_allowed_format_fk_medium_format
   FOREIGN KEY (medium_format)
   REFERENCES medium_format(id);

ALTER TABLE medium_attribute_type_allowed_value_allowed_format
   ADD CONSTRAINT medium_attribute_type_allowed_value_allowed_format_fk_medium_attribute_type_allowed_value
   FOREIGN KEY (medium_attribute_type_allowed_value)
   REFERENCES medium_attribute_type_allowed_value(id);

ALTER TABLE medium_cdtoc
   ADD CONSTRAINT medium_cdtoc_fk_medium
   FOREIGN KEY (medium)
   REFERENCES medium(id);

ALTER TABLE medium_cdtoc
   ADD CONSTRAINT medium_cdtoc_fk_cdtoc
   FOREIGN KEY (cdtoc)
   REFERENCES cdtoc(id);

ALTER TABLE medium_format
   ADD CONSTRAINT medium_format_fk_parent
   FOREIGN KEY (parent)
   REFERENCES medium_format(id);

ALTER TABLE medium_index
   ADD CONSTRAINT medium_index_fk_medium
   FOREIGN KEY (medium)
   REFERENCES medium(id)
   ON DELETE CASCADE;

ALTER TABLE orderable_link_type
   ADD CONSTRAINT orderable_link_type_fk_link_type
   FOREIGN KEY (link_type)
   REFERENCES link_type(id);

ALTER TABLE place
   ADD CONSTRAINT place_fk_type
   FOREIGN KEY (type)
   REFERENCES place_type(id);

ALTER TABLE place
   ADD CONSTRAINT place_fk_area
   FOREIGN KEY (area)
   REFERENCES area(id);

ALTER TABLE place_alias
   ADD CONSTRAINT place_alias_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_alias
   ADD CONSTRAINT place_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES place_alias_type(id);

ALTER TABLE place_alias_type
   ADD CONSTRAINT place_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES place_alias_type(id);

ALTER TABLE place_annotation
   ADD CONSTRAINT place_annotation_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_annotation
   ADD CONSTRAINT place_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE place_attribute
   ADD CONSTRAINT place_attribute_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_attribute
   ADD CONSTRAINT place_attribute_fk_place_attribute_type
   FOREIGN KEY (place_attribute_type)
   REFERENCES place_attribute_type(id);

ALTER TABLE place_attribute
   ADD CONSTRAINT place_attribute_fk_place_attribute_type_allowed_value
   FOREIGN KEY (place_attribute_type_allowed_value)
   REFERENCES place_attribute_type_allowed_value(id);

ALTER TABLE place_attribute_type
   ADD CONSTRAINT place_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES place_attribute_type(id);

ALTER TABLE place_attribute_type_allowed_value
   ADD CONSTRAINT place_attribute_type_allowed_value_fk_place_attribute_type
   FOREIGN KEY (place_attribute_type)
   REFERENCES place_attribute_type(id);

ALTER TABLE place_attribute_type_allowed_value
   ADD CONSTRAINT place_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES place_attribute_type_allowed_value(id);

ALTER TABLE place_gid_redirect
   ADD CONSTRAINT place_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES place(id);

ALTER TABLE place_meta
   ADD CONSTRAINT place_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES place(id)
   ON DELETE CASCADE;

ALTER TABLE place_rating_raw
   ADD CONSTRAINT place_rating_raw_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_rating_raw
   ADD CONSTRAINT place_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE place_tag
   ADD CONSTRAINT place_tag_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_tag
   ADD CONSTRAINT place_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE place_tag_raw
   ADD CONSTRAINT place_tag_raw_fk_place
   FOREIGN KEY (place)
   REFERENCES place(id);

ALTER TABLE place_tag_raw
   ADD CONSTRAINT place_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE place_tag_raw
   ADD CONSTRAINT place_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE place_type
   ADD CONSTRAINT place_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES place_type(id);

ALTER TABLE recording
   ADD CONSTRAINT recording_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE recording_alias
   ADD CONSTRAINT recording_alias_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_alias
   ADD CONSTRAINT recording_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES recording_alias_type(id);

ALTER TABLE recording_alias_type
   ADD CONSTRAINT recording_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES recording_alias_type(id);

ALTER TABLE recording_annotation
   ADD CONSTRAINT recording_annotation_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_annotation
   ADD CONSTRAINT recording_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE recording_attribute
   ADD CONSTRAINT recording_attribute_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_attribute
   ADD CONSTRAINT recording_attribute_fk_recording_attribute_type
   FOREIGN KEY (recording_attribute_type)
   REFERENCES recording_attribute_type(id);

ALTER TABLE recording_attribute
   ADD CONSTRAINT recording_attribute_fk_recording_attribute_type_allowed_value
   FOREIGN KEY (recording_attribute_type_allowed_value)
   REFERENCES recording_attribute_type_allowed_value(id);

ALTER TABLE recording_attribute_type
   ADD CONSTRAINT recording_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES recording_attribute_type(id);

ALTER TABLE recording_attribute_type_allowed_value
   ADD CONSTRAINT recording_attribute_type_allowed_value_fk_recording_attribute_type
   FOREIGN KEY (recording_attribute_type)
   REFERENCES recording_attribute_type(id);

ALTER TABLE recording_attribute_type_allowed_value
   ADD CONSTRAINT recording_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES recording_attribute_type_allowed_value(id);

ALTER TABLE recording_first_release_date
   ADD CONSTRAINT recording_first_release_date_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id)
   ON DELETE CASCADE;

ALTER TABLE recording_gid_redirect
   ADD CONSTRAINT recording_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES recording(id);

ALTER TABLE recording_meta
   ADD CONSTRAINT recording_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES recording(id)
   ON DELETE CASCADE;

ALTER TABLE recording_rating_raw
   ADD CONSTRAINT recording_rating_raw_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_rating_raw
   ADD CONSTRAINT recording_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE recording_tag
   ADD CONSTRAINT recording_tag_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_tag
   ADD CONSTRAINT recording_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE recording_tag_raw
   ADD CONSTRAINT recording_tag_raw_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE recording_tag_raw
   ADD CONSTRAINT recording_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE recording_tag_raw
   ADD CONSTRAINT recording_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_status
   FOREIGN KEY (status)
   REFERENCES release_status(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_packaging
   FOREIGN KEY (packaging)
   REFERENCES release_packaging(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_language
   FOREIGN KEY (language)
   REFERENCES language(id);

ALTER TABLE release
   ADD CONSTRAINT release_fk_script
   FOREIGN KEY (script)
   REFERENCES script(id);

ALTER TABLE release_alias
   ADD CONSTRAINT release_alias_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_alias
   ADD CONSTRAINT release_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES release_alias_type(id);

ALTER TABLE release_alias_type
   ADD CONSTRAINT release_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_alias_type(id);

ALTER TABLE release_annotation
   ADD CONSTRAINT release_annotation_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_annotation
   ADD CONSTRAINT release_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE release_attribute
   ADD CONSTRAINT release_attribute_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_attribute
   ADD CONSTRAINT release_attribute_fk_release_attribute_type
   FOREIGN KEY (release_attribute_type)
   REFERENCES release_attribute_type(id);

ALTER TABLE release_attribute
   ADD CONSTRAINT release_attribute_fk_release_attribute_type_allowed_value
   FOREIGN KEY (release_attribute_type_allowed_value)
   REFERENCES release_attribute_type_allowed_value(id);

ALTER TABLE release_attribute_type
   ADD CONSTRAINT release_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_attribute_type(id);

ALTER TABLE release_attribute_type_allowed_value
   ADD CONSTRAINT release_attribute_type_allowed_value_fk_release_attribute_type
   FOREIGN KEY (release_attribute_type)
   REFERENCES release_attribute_type(id);

ALTER TABLE release_attribute_type_allowed_value
   ADD CONSTRAINT release_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_attribute_type_allowed_value(id);

ALTER TABLE release_country
   ADD CONSTRAINT release_country_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_country
   ADD CONSTRAINT release_country_fk_country
   FOREIGN KEY (country)
   REFERENCES country_area(area);

ALTER TABLE release_coverart
   ADD CONSTRAINT release_coverart_fk_id
   FOREIGN KEY (id)
   REFERENCES release(id)
   ON DELETE CASCADE;

ALTER TABLE release_first_release_date
   ADD CONSTRAINT release_first_release_date_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id)
   ON DELETE CASCADE;

ALTER TABLE release_gid_redirect
   ADD CONSTRAINT release_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES release(id);

ALTER TABLE release_group
   ADD CONSTRAINT release_group_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE release_group
   ADD CONSTRAINT release_group_fk_type
   FOREIGN KEY (type)
   REFERENCES release_group_primary_type(id);

ALTER TABLE release_group_alias
   ADD CONSTRAINT release_group_alias_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_alias
   ADD CONSTRAINT release_group_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES release_group_alias_type(id);

ALTER TABLE release_group_alias_type
   ADD CONSTRAINT release_group_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_group_alias_type(id);

ALTER TABLE release_group_annotation
   ADD CONSTRAINT release_group_annotation_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_annotation
   ADD CONSTRAINT release_group_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE release_group_attribute
   ADD CONSTRAINT release_group_attribute_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_attribute
   ADD CONSTRAINT release_group_attribute_fk_release_group_attribute_type
   FOREIGN KEY (release_group_attribute_type)
   REFERENCES release_group_attribute_type(id);

ALTER TABLE release_group_attribute
   ADD CONSTRAINT release_group_attribute_fk_release_group_attribute_type_allowed_value
   FOREIGN KEY (release_group_attribute_type_allowed_value)
   REFERENCES release_group_attribute_type_allowed_value(id);

ALTER TABLE release_group_attribute_type
   ADD CONSTRAINT release_group_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_group_attribute_type(id);

ALTER TABLE release_group_attribute_type_allowed_value
   ADD CONSTRAINT release_group_attribute_type_allowed_value_fk_release_group_attribute_type
   FOREIGN KEY (release_group_attribute_type)
   REFERENCES release_group_attribute_type(id);

ALTER TABLE release_group_attribute_type_allowed_value
   ADD CONSTRAINT release_group_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_group_attribute_type_allowed_value(id);

ALTER TABLE release_group_gid_redirect
   ADD CONSTRAINT release_group_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES release_group(id);

ALTER TABLE release_group_meta
   ADD CONSTRAINT release_group_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES release_group(id)
   ON DELETE CASCADE;

ALTER TABLE release_group_primary_type
   ADD CONSTRAINT release_group_primary_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_group_primary_type(id);

ALTER TABLE release_group_rating_raw
   ADD CONSTRAINT release_group_rating_raw_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_rating_raw
   ADD CONSTRAINT release_group_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE release_group_secondary_type
   ADD CONSTRAINT release_group_secondary_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_group_secondary_type(id);

ALTER TABLE release_group_secondary_type_join
   ADD CONSTRAINT release_group_secondary_type_join_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_secondary_type_join
   ADD CONSTRAINT release_group_secondary_type_join_fk_secondary_type
   FOREIGN KEY (secondary_type)
   REFERENCES release_group_secondary_type(id);

ALTER TABLE release_group_tag
   ADD CONSTRAINT release_group_tag_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_tag
   ADD CONSTRAINT release_group_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE release_group_tag_raw
   ADD CONSTRAINT release_group_tag_raw_fk_release_group
   FOREIGN KEY (release_group)
   REFERENCES release_group(id);

ALTER TABLE release_group_tag_raw
   ADD CONSTRAINT release_group_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE release_group_tag_raw
   ADD CONSTRAINT release_group_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE release_label
   ADD CONSTRAINT release_label_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_label
   ADD CONSTRAINT release_label_fk_label
   FOREIGN KEY (label)
   REFERENCES label(id);

ALTER TABLE release_meta
   ADD CONSTRAINT release_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES release(id)
   ON DELETE CASCADE;

ALTER TABLE release_packaging
   ADD CONSTRAINT release_packaging_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_packaging(id);

ALTER TABLE release_status
   ADD CONSTRAINT release_status_fk_parent
   FOREIGN KEY (parent)
   REFERENCES release_status(id);

ALTER TABLE release_tag
   ADD CONSTRAINT release_tag_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_tag
   ADD CONSTRAINT release_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE release_tag_raw
   ADD CONSTRAINT release_tag_raw_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE release_tag_raw
   ADD CONSTRAINT release_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE release_tag_raw
   ADD CONSTRAINT release_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE release_unknown_country
   ADD CONSTRAINT release_unknown_country_fk_release
   FOREIGN KEY (release)
   REFERENCES release(id);

ALTER TABLE series
   ADD CONSTRAINT series_fk_type
   FOREIGN KEY (type)
   REFERENCES series_type(id);

ALTER TABLE series
   ADD CONSTRAINT series_fk_ordering_attribute
   FOREIGN KEY (ordering_attribute)
   REFERENCES link_text_attribute_type(attribute_type);

ALTER TABLE series
   ADD CONSTRAINT series_fk_ordering_type
   FOREIGN KEY (ordering_type)
   REFERENCES series_ordering_type(id);

ALTER TABLE series_alias
   ADD CONSTRAINT series_alias_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE series_alias
   ADD CONSTRAINT series_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES series_alias_type(id);

ALTER TABLE series_alias_type
   ADD CONSTRAINT series_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES series_alias_type(id);

ALTER TABLE series_annotation
   ADD CONSTRAINT series_annotation_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE series_annotation
   ADD CONSTRAINT series_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE series_attribute
   ADD CONSTRAINT series_attribute_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE series_attribute
   ADD CONSTRAINT series_attribute_fk_series_attribute_type
   FOREIGN KEY (series_attribute_type)
   REFERENCES series_attribute_type(id);

ALTER TABLE series_attribute
   ADD CONSTRAINT series_attribute_fk_series_attribute_type_allowed_value
   FOREIGN KEY (series_attribute_type_allowed_value)
   REFERENCES series_attribute_type_allowed_value(id);

ALTER TABLE series_attribute_type
   ADD CONSTRAINT series_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES series_attribute_type(id);

ALTER TABLE series_attribute_type_allowed_value
   ADD CONSTRAINT series_attribute_type_allowed_value_fk_series_attribute_type
   FOREIGN KEY (series_attribute_type)
   REFERENCES series_attribute_type(id);

ALTER TABLE series_attribute_type_allowed_value
   ADD CONSTRAINT series_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES series_attribute_type_allowed_value(id);

ALTER TABLE series_gid_redirect
   ADD CONSTRAINT series_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES series(id);

ALTER TABLE series_ordering_type
   ADD CONSTRAINT series_ordering_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES series_ordering_type(id);

ALTER TABLE series_tag
   ADD CONSTRAINT series_tag_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE series_tag
   ADD CONSTRAINT series_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE series_tag_raw
   ADD CONSTRAINT series_tag_raw_fk_series
   FOREIGN KEY (series)
   REFERENCES series(id);

ALTER TABLE series_tag_raw
   ADD CONSTRAINT series_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE series_tag_raw
   ADD CONSTRAINT series_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE series_type
   ADD CONSTRAINT series_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES series_type(id);

ALTER TABLE tag_relation
   ADD CONSTRAINT tag_relation_fk_tag1
   FOREIGN KEY (tag1)
   REFERENCES tag(id);

ALTER TABLE tag_relation
   ADD CONSTRAINT tag_relation_fk_tag2
   FOREIGN KEY (tag2)
   REFERENCES tag(id);

ALTER TABLE track
   ADD CONSTRAINT track_fk_recording
   FOREIGN KEY (recording)
   REFERENCES recording(id);

ALTER TABLE track
   ADD CONSTRAINT track_fk_medium
   FOREIGN KEY (medium)
   REFERENCES medium(id);

ALTER TABLE track
   ADD CONSTRAINT track_fk_artist_credit
   FOREIGN KEY (artist_credit)
   REFERENCES artist_credit(id);

ALTER TABLE track_gid_redirect
   ADD CONSTRAINT track_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES track(id);

ALTER TABLE track_raw
   ADD CONSTRAINT track_raw_fk_release
   FOREIGN KEY (release)
   REFERENCES release_raw(id);

ALTER TABLE url_gid_redirect
   ADD CONSTRAINT url_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES url(id);

ALTER TABLE vote
   ADD CONSTRAINT vote_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE vote
   ADD CONSTRAINT vote_fk_edit
   FOREIGN KEY (edit)
   REFERENCES edit(id);

ALTER TABLE work
   ADD CONSTRAINT work_fk_type
   FOREIGN KEY (type)
   REFERENCES work_type(id);

ALTER TABLE work_alias
   ADD CONSTRAINT work_alias_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_alias
   ADD CONSTRAINT work_alias_fk_type
   FOREIGN KEY (type)
   REFERENCES work_alias_type(id);

ALTER TABLE work_alias_type
   ADD CONSTRAINT work_alias_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES work_alias_type(id);

ALTER TABLE work_annotation
   ADD CONSTRAINT work_annotation_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_annotation
   ADD CONSTRAINT work_annotation_fk_annotation
   FOREIGN KEY (annotation)
   REFERENCES annotation(id);

ALTER TABLE work_attribute
   ADD CONSTRAINT work_attribute_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_attribute
   ADD CONSTRAINT work_attribute_fk_work_attribute_type
   FOREIGN KEY (work_attribute_type)
   REFERENCES work_attribute_type(id);

ALTER TABLE work_attribute
   ADD CONSTRAINT work_attribute_fk_work_attribute_type_allowed_value
   FOREIGN KEY (work_attribute_type_allowed_value)
   REFERENCES work_attribute_type_allowed_value(id);

ALTER TABLE work_attribute_type
   ADD CONSTRAINT work_attribute_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES work_attribute_type(id);

ALTER TABLE work_attribute_type_allowed_value
   ADD CONSTRAINT work_attribute_type_allowed_value_fk_work_attribute_type
   FOREIGN KEY (work_attribute_type)
   REFERENCES work_attribute_type(id);

ALTER TABLE work_attribute_type_allowed_value
   ADD CONSTRAINT work_attribute_type_allowed_value_fk_parent
   FOREIGN KEY (parent)
   REFERENCES work_attribute_type_allowed_value(id);

ALTER TABLE work_gid_redirect
   ADD CONSTRAINT work_gid_redirect_fk_new_id
   FOREIGN KEY (new_id)
   REFERENCES work(id);

ALTER TABLE work_language
   ADD CONSTRAINT work_language_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_language
   ADD CONSTRAINT work_language_fk_language
   FOREIGN KEY (language)
   REFERENCES language(id);

ALTER TABLE work_meta
   ADD CONSTRAINT work_meta_fk_id
   FOREIGN KEY (id)
   REFERENCES work(id)
   ON DELETE CASCADE;

ALTER TABLE work_rating_raw
   ADD CONSTRAINT work_rating_raw_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_rating_raw
   ADD CONSTRAINT work_rating_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE work_tag
   ADD CONSTRAINT work_tag_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_tag
   ADD CONSTRAINT work_tag_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE work_tag_raw
   ADD CONSTRAINT work_tag_raw_fk_work
   FOREIGN KEY (work)
   REFERENCES work(id);

ALTER TABLE work_tag_raw
   ADD CONSTRAINT work_tag_raw_fk_editor
   FOREIGN KEY (editor)
   REFERENCES editor(id);

ALTER TABLE work_tag_raw
   ADD CONSTRAINT work_tag_raw_fk_tag
   FOREIGN KEY (tag)
   REFERENCES tag(id);

ALTER TABLE work_type
   ADD CONSTRAINT work_type_fk_parent
   FOREIGN KEY (parent)
   REFERENCES work_type(id);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateTriggers.sql

CREATE TRIGGER b_upd_area BEFORE UPDATE ON area
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_area_alias BEFORE UPDATE ON area_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_area_tag BEFORE UPDATE ON area_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON area_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(3);

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON area_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON area
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER a_ins_artist AFTER INSERT ON artist
    FOR EACH ROW EXECUTE PROCEDURE a_ins_artist();

CREATE TRIGGER b_upd_artist BEFORE UPDATE ON artist
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_artist_credit_name BEFORE UPDATE ON artist_credit_name
    FOR EACH ROW EXECUTE PROCEDURE b_upd_artist_credit_name();

CREATE TRIGGER b_del_artist_special BEFORE DELETE ON artist
    FOR EACH ROW WHEN (OLD.id IN (1, 2)) EXECUTE PROCEDURE deny_special_purpose_deletion();

CREATE TRIGGER end_area_implies_ended BEFORE UPDATE OR INSERT ON artist
    FOR EACH ROW EXECUTE PROCEDURE end_area_implies_ended();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON artist
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON artist_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_artist_alias BEFORE UPDATE ON artist_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER replace_old_sub_on_add BEFORE INSERT ON editor_subscribe_collection
    FOR EACH ROW EXECUTE PROCEDURE replace_old_sub_on_add();

CREATE TRIGGER del_collection_sub_on_delete BEFORE DELETE ON editor_collection
    FOR EACH ROW EXECUTE PROCEDURE del_collection_sub_on_delete();

CREATE TRIGGER del_collection_sub_on_private BEFORE UPDATE ON editor_collection
    FOR EACH ROW EXECUTE PROCEDURE del_collection_sub_on_private();

CREATE TRIGGER restore_collection_sub_on_public AFTER UPDATE ON editor_collection
    FOR EACH ROW EXECUTE PROCEDURE restore_collection_sub_on_public();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON artist_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(3);

CREATE TRIGGER b_upd_artist_tag BEFORE UPDATE ON artist_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_editor BEFORE UPDATE ON editor
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_editor AFTER INSERT ON editor
    FOR EACH ROW EXECUTE PROCEDURE a_ins_editor();

CREATE TRIGGER check_editor_name BEFORE UPDATE OR INSERT ON editor
    FOR EACH ROW EXECUTE PROCEDURE check_editor_name();

CREATE TRIGGER a_ins_event AFTER INSERT ON event
    FOR EACH ROW EXECUTE PROCEDURE a_ins_event();

CREATE TRIGGER b_upd_event BEFORE UPDATE ON event
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON event
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_event_alias BEFORE UPDATE ON event_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON event_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON event_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_event_tag BEFORE UPDATE ON event_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_genre BEFORE UPDATE ON genre
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_genre_alias BEFORE UPDATE ON genre_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_instrument BEFORE UPDATE ON instrument
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON instrument_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_instrument_alias BEFORE UPDATE ON instrument_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_instrument_tag BEFORE UPDATE ON instrument_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON instrument_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_l_area_area BEFORE UPDATE ON l_area_area
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_artist BEFORE UPDATE ON l_area_artist
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_event BEFORE UPDATE ON l_area_event
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_instrument BEFORE UPDATE ON l_area_instrument
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_label BEFORE UPDATE ON l_area_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_place BEFORE UPDATE ON l_area_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_recording BEFORE UPDATE ON l_area_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_release BEFORE UPDATE ON l_area_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_release_group BEFORE UPDATE ON l_area_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_url BEFORE UPDATE ON l_area_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_area_work BEFORE UPDATE ON l_area_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_artist BEFORE UPDATE ON l_artist_artist
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_event BEFORE UPDATE ON l_artist_event
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_instrument BEFORE UPDATE ON l_artist_instrument
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_label BEFORE UPDATE ON l_artist_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_place BEFORE UPDATE ON l_artist_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_recording BEFORE UPDATE ON l_artist_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_release BEFORE UPDATE ON l_artist_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_release_group BEFORE UPDATE ON l_artist_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_url BEFORE UPDATE ON l_artist_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_artist_work BEFORE UPDATE ON l_artist_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_event BEFORE UPDATE ON l_event_event
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_instrument BEFORE UPDATE ON l_event_instrument
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_label BEFORE UPDATE ON l_event_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_place BEFORE UPDATE ON l_event_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_recording BEFORE UPDATE ON l_event_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_release BEFORE UPDATE ON l_event_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_release_group BEFORE UPDATE ON l_event_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_url BEFORE UPDATE ON l_event_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_event_work BEFORE UPDATE ON l_event_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_instrument BEFORE UPDATE ON l_instrument_instrument
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_label BEFORE UPDATE ON l_instrument_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_place BEFORE UPDATE ON l_instrument_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_recording BEFORE UPDATE ON l_instrument_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_release BEFORE UPDATE ON l_instrument_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_release_group BEFORE UPDATE ON l_instrument_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_url BEFORE UPDATE ON l_instrument_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_instrument_work BEFORE UPDATE ON l_instrument_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_label BEFORE UPDATE ON l_label_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_place BEFORE UPDATE ON l_label_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_recording BEFORE UPDATE ON l_label_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_release BEFORE UPDATE ON l_label_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_release_group BEFORE UPDATE ON l_label_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_url BEFORE UPDATE ON l_label_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_label_work BEFORE UPDATE ON l_label_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_place BEFORE UPDATE ON l_place_place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_recording BEFORE UPDATE ON l_place_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_release BEFORE UPDATE ON l_place_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_release_group BEFORE UPDATE ON l_place_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_url BEFORE UPDATE ON l_place_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_place_work BEFORE UPDATE ON l_place_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_recording_recording BEFORE UPDATE ON l_recording_recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_recording_release BEFORE UPDATE ON l_recording_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_recording_release_group BEFORE UPDATE ON l_recording_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_recording_url BEFORE UPDATE ON l_recording_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_recording_work BEFORE UPDATE ON l_recording_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_release BEFORE UPDATE ON l_release_release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_release_group BEFORE UPDATE ON l_release_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_url BEFORE UPDATE ON l_release_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_work BEFORE UPDATE ON l_release_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_group_release_group BEFORE UPDATE ON l_release_group_release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_group_url BEFORE UPDATE ON l_release_group_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_release_group_work BEFORE UPDATE ON l_release_group_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_url_url BEFORE UPDATE ON l_url_url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_url_work BEFORE UPDATE ON l_url_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_l_work_work BEFORE UPDATE ON l_work_work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_label AFTER INSERT ON label
    FOR EACH ROW EXECUTE PROCEDURE a_ins_label();

CREATE TRIGGER b_del_label_special BEFORE DELETE ON label
    FOR EACH ROW WHEN (OLD.id IN (1, 3267)) EXECUTE PROCEDURE deny_special_purpose_deletion();

CREATE TRIGGER b_upd_label BEFORE UPDATE ON label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON label
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON label_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_label_alias BEFORE UPDATE ON label_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON label_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_label_tag BEFORE UPDATE ON label_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON link
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER deny_deprecated BEFORE UPDATE OR INSERT ON link
    FOR EACH ROW EXECUTE PROCEDURE deny_deprecated_links();

CREATE TRIGGER check_has_dates BEFORE UPDATE OR INSERT ON link
    FOR EACH ROW EXECUTE PROCEDURE check_has_dates();

CREATE TRIGGER b_upd_link_attribute BEFORE UPDATE OR INSERT ON link_attribute
    FOR EACH ROW EXECUTE PROCEDURE prevent_invalid_attributes();

CREATE TRIGGER b_upd_link_attribute_type BEFORE UPDATE ON link_attribute_type
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_link_type BEFORE UPDATE ON link_type
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_link_type_attribute_type BEFORE UPDATE ON link_type_attribute_type
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_medium BEFORE UPDATE ON medium
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_medium_cdtoc BEFORE UPDATE ON medium_cdtoc
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_place AFTER INSERT ON place
    FOR EACH ROW EXECUTE PROCEDURE a_ins_place();

CREATE TRIGGER b_upd_place BEFORE UPDATE ON place
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON place
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON place_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_place_alias BEFORE UPDATE ON place_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON place_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_place_tag BEFORE UPDATE ON place_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_recording AFTER INSERT ON recording
    FOR EACH ROW EXECUTE PROCEDURE a_ins_recording();

CREATE TRIGGER b_upd_recording BEFORE UPDATE ON recording
    FOR EACH ROW EXECUTE PROCEDURE b_upd_recording();

CREATE TRIGGER a_upd_recording AFTER UPDATE ON recording
    FOR EACH ROW EXECUTE PROCEDURE a_upd_recording();

CREATE TRIGGER a_del_recording AFTER DELETE ON recording
    FOR EACH ROW EXECUTE PROCEDURE a_del_recording();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON recording_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_recording_alias BEFORE UPDATE ON recording_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON recording_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_recording_tag BEFORE UPDATE ON recording_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_release AFTER INSERT ON release
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release();

CREATE TRIGGER a_upd_release AFTER UPDATE ON release
    FOR EACH ROW EXECUTE PROCEDURE a_upd_release();

CREATE TRIGGER a_del_release AFTER DELETE ON release
    FOR EACH ROW EXECUTE PROCEDURE a_del_release();

CREATE TRIGGER b_upd_release BEFORE UPDATE ON release
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON release_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_release_alias BEFORE UPDATE ON release_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON release_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER a_ins_release_event AFTER INSERT ON release_country
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release_event();

CREATE TRIGGER a_upd_release_event AFTER UPDATE ON release_country
    FOR EACH ROW EXECUTE PROCEDURE a_upd_release_event();

CREATE TRIGGER a_del_release_event AFTER DELETE ON release_country
    FOR EACH ROW EXECUTE PROCEDURE a_del_release_event();

CREATE TRIGGER a_ins_release_event AFTER INSERT ON release_unknown_country
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release_event();

CREATE TRIGGER a_upd_release_event AFTER UPDATE ON release_unknown_country
    FOR EACH ROW EXECUTE PROCEDURE a_upd_release_event();

CREATE TRIGGER a_del_release_event AFTER DELETE ON release_unknown_country
    FOR EACH ROW EXECUTE PROCEDURE a_del_release_event();

CREATE TRIGGER b_upd_release_label BEFORE UPDATE ON release_label
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_release_label AFTER INSERT ON release_label
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release_label();

CREATE TRIGGER a_upd_release_label AFTER UPDATE ON release_label
    FOR EACH ROW EXECUTE PROCEDURE a_upd_release_label();

CREATE TRIGGER a_del_release_label AFTER DELETE ON release_label
    FOR EACH ROW EXECUTE PROCEDURE a_del_release_label();

CREATE TRIGGER a_ins_release_group AFTER INSERT ON release_group
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release_group();

CREATE TRIGGER a_upd_release_group AFTER UPDATE ON release_group
    FOR EACH ROW EXECUTE PROCEDURE a_upd_release_group();

CREATE TRIGGER a_del_release_group AFTER DELETE ON release_group
    FOR EACH ROW EXECUTE PROCEDURE a_del_release_group();

CREATE TRIGGER b_upd_release_group BEFORE UPDATE ON release_group
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_release_group_secondary_type_join AFTER INSERT ON release_group_secondary_type_join
    FOR EACH ROW EXECUTE PROCEDURE a_ins_release_group_secondary_type_join();

CREATE TRIGGER a_del_release_group_secondary_type_join AFTER DELETE ON release_group_secondary_type_join
    FOR EACH ROW EXECUTE PROCEDURE a_del_release_group_secondary_type_join();

CREATE TRIGGER b_upd_release_group_secondary_type_join BEFORE UPDATE ON release_group_secondary_type_join
    FOR EACH ROW EXECUTE PROCEDURE b_upd_release_group_secondary_type_join();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON release_group_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER b_upd_release_group_alias BEFORE UPDATE ON release_group_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON release_group_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_release_group_tag BEFORE UPDATE ON release_group_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_series BEFORE UPDATE ON series
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_series_alias BEFORE UPDATE ON series_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_series_tag BEFORE UPDATE ON series_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON series_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON series_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_tag_relation BEFORE UPDATE ON tag_relation
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_track AFTER INSERT ON track
    FOR EACH ROW EXECUTE PROCEDURE a_ins_track();

CREATE TRIGGER a_upd_track AFTER UPDATE ON track
    FOR EACH ROW EXECUTE PROCEDURE a_upd_track();

CREATE TRIGGER a_del_track AFTER DELETE ON track
    FOR EACH ROW EXECUTE PROCEDURE a_del_track();

CREATE TRIGGER b_upd_track BEFORE UPDATE ON track
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE CONSTRAINT TRIGGER remove_orphaned_tracks
    AFTER DELETE OR UPDATE ON track DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE delete_orphaned_recordings();

CREATE TRIGGER b_upd_url BEFORE UPDATE ON url
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER a_ins_work AFTER INSERT ON work
    FOR EACH ROW EXECUTE PROCEDURE a_ins_work();

CREATE TRIGGER b_upd_work BEFORE UPDATE ON work
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER b_upd_work_alias BEFORE UPDATE ON work_alias
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER end_date_implies_ended BEFORE UPDATE OR INSERT ON work_alias
    FOR EACH ROW EXECUTE PROCEDURE end_date_implies_ended();

CREATE TRIGGER search_hint BEFORE UPDATE OR INSERT ON work_alias
    FOR EACH ROW EXECUTE PROCEDURE simplify_search_hints(2);

CREATE TRIGGER b_upd_work_tag BEFORE UPDATE ON work_tag
    FOR EACH ROW EXECUTE PROCEDURE b_upd_last_updated_table();

CREATE TRIGGER inserting_edits_requires_confirmed_email_address BEFORE INSERT ON edit
    FOR EACH ROW EXECUTE PROCEDURE inserting_edits_requires_confirmed_email_address();

CREATE TRIGGER a_upd_edit AFTER UPDATE ON edit
    FOR EACH ROW EXECUTE PROCEDURE a_upd_edit();

CREATE TRIGGER a_ins_edit_artist BEFORE INSERT ON edit_artist
    FOR EACH ROW EXECUTE PROCEDURE b_ins_edit_materialize_status();

CREATE TRIGGER a_ins_edit_artist BEFORE INSERT ON edit_label
    FOR EACH ROW EXECUTE PROCEDURE b_ins_edit_materialize_status();

CREATE TRIGGER a_ins_instrument AFTER INSERT ON instrument
    FOR EACH ROW EXECUTE PROCEDURE a_ins_instrument();

CREATE TRIGGER a_upd_instrument AFTER UPDATE ON instrument
    FOR EACH ROW EXECUTE PROCEDURE a_upd_instrument();

CREATE TRIGGER a_del_instrument AFTER DELETE ON instrument
    FOR EACH ROW EXECUTE PROCEDURE a_del_instrument();

CREATE TRIGGER a_ins_edit_note AFTER INSERT ON edit_note
    FOR EACH ROW EXECUTE PROCEDURE a_ins_edit_note();

CREATE TRIGGER a_ins_alternative_release AFTER INSERT ON alternative_release
    FOR EACH ROW EXECUTE PROCEDURE a_ins_alternative_release_or_track();

CREATE TRIGGER a_ins_alternative_track AFTER INSERT ON alternative_track
    FOR EACH ROW EXECUTE PROCEDURE a_ins_alternative_release_or_track();

CREATE TRIGGER a_upd_alternative_release AFTER UPDATE ON alternative_release
    FOR EACH ROW EXECUTE PROCEDURE a_upd_alternative_release_or_track();

CREATE TRIGGER a_upd_alternative_track AFTER UPDATE ON alternative_track
    FOR EACH ROW EXECUTE PROCEDURE a_upd_alternative_release_or_track();

CREATE TRIGGER a_del_alternative_release AFTER DELETE ON alternative_release
    FOR EACH ROW EXECUTE PROCEDURE a_del_alternative_release_or_track();

CREATE TRIGGER a_del_alternative_track AFTER DELETE ON alternative_track
    FOR EACH ROW EXECUTE PROCEDURE a_del_alternative_release_or_track();

CREATE TRIGGER a_ins_alternative_medium_track AFTER INSERT ON alternative_medium_track
    FOR EACH ROW EXECUTE PROCEDURE a_ins_alternative_medium_track();

CREATE TRIGGER a_upd_alternative_medium_track AFTER UPDATE ON alternative_medium_track
    FOR EACH ROW EXECUTE PROCEDURE a_upd_alternative_medium_track();

CREATE TRIGGER a_del_alternative_medium_track AFTER DELETE ON alternative_medium_track
    FOR EACH ROW EXECUTE PROCEDURE a_del_alternative_medium_track();

CREATE TRIGGER ensure_area_attribute_type_allows_text BEFORE INSERT OR UPDATE ON area_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_area_attribute_type_allows_text();

CREATE TRIGGER ensure_artist_attribute_type_allows_text BEFORE INSERT OR UPDATE ON artist_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_artist_attribute_type_allows_text();

CREATE TRIGGER ensure_event_attribute_type_allows_text BEFORE INSERT OR UPDATE ON event_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_event_attribute_type_allows_text();

CREATE TRIGGER ensure_instrument_attribute_type_allows_text BEFORE INSERT OR UPDATE ON instrument_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_instrument_attribute_type_allows_text();

CREATE TRIGGER ensure_label_attribute_type_allows_text BEFORE INSERT OR UPDATE ON label_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_label_attribute_type_allows_text();

CREATE TRIGGER ensure_medium_attribute_type_allows_text BEFORE INSERT OR UPDATE ON medium_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_medium_attribute_type_allows_text();

CREATE TRIGGER ensure_place_attribute_type_allows_text BEFORE INSERT OR UPDATE ON place_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_place_attribute_type_allows_text();

CREATE TRIGGER ensure_recording_attribute_type_allows_text BEFORE INSERT OR UPDATE ON recording_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_recording_attribute_type_allows_text();

CREATE TRIGGER ensure_release_attribute_type_allows_text BEFORE INSERT OR UPDATE ON release_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_release_attribute_type_allows_text();

CREATE TRIGGER ensure_release_group_attribute_type_allows_text BEFORE INSERT OR UPDATE ON release_group_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_release_group_attribute_type_allows_text();

CREATE TRIGGER ensure_series_attribute_type_allows_text BEFORE INSERT OR UPDATE ON series_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_series_attribute_type_allows_text();

CREATE TRIGGER ensure_work_attribute_type_allows_text BEFORE INSERT OR UPDATE ON work_attribute
    FOR EACH ROW EXECUTE PROCEDURE ensure_work_attribute_type_allows_text();

--------------------------------------------------------------------------------
CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_area DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_artist DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_event DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_instrument DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_area_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_artist DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_event DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_instrument DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_artist_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_event DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_instrument DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_event_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_instrument DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_instrument_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_label_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_place DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_place_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_recording_recording DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_recording_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_recording_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_recording_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_recording_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_group_release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_group_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_release_group_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_url_url DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_url_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();

CREATE CONSTRAINT TRIGGER remove_unused_links
    AFTER DELETE OR UPDATE ON l_work_work DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE remove_unused_links();
--------------------------------------------------------------------------------

CREATE CONSTRAINT TRIGGER url_gc_a_upd_url
AFTER UPDATE ON url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_area_url
AFTER UPDATE ON l_area_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_area_url
AFTER DELETE ON l_area_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_artist_url
AFTER UPDATE ON l_artist_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_artist_url
AFTER DELETE ON l_artist_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_event_url
AFTER UPDATE ON l_event_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_event_url
AFTER DELETE ON l_event_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_instrument_url
AFTER UPDATE ON l_instrument_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_instrument_url
AFTER DELETE ON l_instrument_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_label_url
AFTER UPDATE ON l_label_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_label_url
AFTER DELETE ON l_label_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_place_url
AFTER UPDATE ON l_place_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_place_url
AFTER DELETE ON l_place_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_recording_url
AFTER UPDATE ON l_recording_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_recording_url
AFTER DELETE ON l_recording_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_release_url
AFTER UPDATE ON l_release_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_release_url
AFTER DELETE ON l_release_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_release_group_url
AFTER UPDATE ON l_release_group_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_release_group_url
AFTER DELETE ON l_release_group_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_series_url
AFTER UPDATE ON l_series_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_series_url
AFTER DELETE ON l_series_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_url_url
AFTER UPDATE ON l_url_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_url_url
AFTER DELETE ON l_url_url DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_upd_l_url_work
AFTER UPDATE ON l_url_work DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

CREATE CONSTRAINT TRIGGER url_gc_a_del_l_url_work
AFTER DELETE ON l_url_work DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE remove_unused_url();

--------------------------------------------------------------------------------
CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER INSERT ON tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON area_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON artist_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON event_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON instrument_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON label_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON place_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON recording_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON release_group_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON release_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON series_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

CREATE CONSTRAINT TRIGGER delete_unused_tag
AFTER DELETE ON work_tag DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW EXECUTE PROCEDURE trg_delete_unused_tag_ref();

--------------------------------------------------------------------------------
CREATE CONSTRAINT TRIGGER apply_artist_release_group_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_group_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release_country DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release_first_release_date DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_group_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release_group DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_group_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_group_pending_updates
    AFTER UPDATE ON release_group_meta DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_group_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_group_pending_updates
    AFTER INSERT OR DELETE ON release_group_secondary_type_join DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_group_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON release_label DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_group_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON track DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_group_pending_updates();

CREATE CONSTRAINT TRIGGER apply_artist_release_pending_updates
    AFTER INSERT OR UPDATE OR DELETE ON track DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE PROCEDURE apply_artist_release_pending_updates();

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateIndexes.sql

CREATE INDEX application_idx_owner ON application (owner);
CREATE UNIQUE INDEX application_idx_oauth_id ON application (oauth_id);

CREATE UNIQUE INDEX area_type_idx_gid ON area_type (gid);

CREATE UNIQUE INDEX area_idx_gid ON area (gid);
CREATE INDEX area_idx_name ON area (name);

CREATE UNIQUE INDEX area_alias_type_idx_gid ON area_alias_type (gid);

CREATE UNIQUE INDEX artist_alias_type_idx_gid ON artist_alias_type (gid);

CREATE INDEX iso_3166_1_idx_area ON iso_3166_1 (area);
CREATE INDEX iso_3166_2_idx_area ON iso_3166_2 (area);
CREATE INDEX iso_3166_3_idx_area ON iso_3166_3 (area);

CREATE INDEX area_alias_idx_area ON area_alias (area);
CREATE UNIQUE INDEX area_alias_idx_primary ON area_alias (area, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX area_attribute_idx_area ON area_attribute (area);

CREATE UNIQUE INDEX area_attribute_type_idx_gid ON area_attribute_type (gid);

CREATE INDEX area_attribute_type_allowed_value_idx_name ON area_attribute_type_allowed_value (area_attribute_type);
CREATE UNIQUE INDEX area_attribute_type_allowed_value_idx_gid ON area_attribute_type_allowed_value (gid);

CREATE INDEX area_tag_idx_tag ON area_tag (tag);

CREATE INDEX area_tag_raw_idx_area ON area_tag_raw (area);
CREATE INDEX area_tag_raw_idx_tag ON area_tag_raw (tag);
CREATE INDEX area_tag_raw_idx_editor ON area_tag_raw (editor);

CREATE UNIQUE INDEX artist_idx_gid ON artist (gid);
CREATE INDEX artist_idx_name ON artist (name);
CREATE INDEX artist_idx_sort_name ON artist (sort_name);

CREATE INDEX artist_idx_area ON artist (area);
CREATE INDEX artist_idx_begin_area ON artist (begin_area);
CREATE INDEX artist_idx_end_area ON artist (end_area);

CREATE UNIQUE INDEX artist_idx_null_comment ON artist (name) WHERE comment IS NULL;
CREATE UNIQUE INDEX artist_idx_uniq_name_comment ON artist (name, comment) WHERE comment IS NOT NULL;

CREATE INDEX artist_alias_idx_artist ON artist_alias (artist);
CREATE UNIQUE INDEX artist_alias_idx_primary ON artist_alias (artist, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX artist_attribute_idx_artist ON artist_attribute (artist);

CREATE UNIQUE INDEX artist_attribute_type_idx_gid ON artist_attribute_type (gid);

CREATE INDEX artist_attribute_type_allowed_value_idx_name ON artist_attribute_type_allowed_value (artist_attribute_type);
CREATE UNIQUE INDEX artist_attribute_type_allowed_value_idx_gid ON artist_attribute_type_allowed_value (gid);

CREATE INDEX artist_credit_name_idx_artist ON artist_credit_name (artist);

CREATE UNIQUE INDEX artist_type_idx_gid ON artist_type (gid);

CREATE INDEX artist_tag_idx_tag ON artist_tag (tag);

CREATE INDEX artist_rating_raw_idx_editor ON artist_rating_raw (editor);

CREATE INDEX artist_tag_raw_idx_tag ON artist_tag_raw (tag);
CREATE INDEX artist_tag_raw_idx_editor ON artist_tag_raw (editor);

CREATE INDEX artist_release_nonva_idx_sort ON artist_release_nonva (artist, first_release_date NULLS LAST, catalog_numbers NULLS LAST, country_code NULLS LAST, barcode NULLS LAST, sort_character, release);
CREATE INDEX artist_release_va_idx_sort ON artist_release_va (artist, first_release_date NULLS LAST, catalog_numbers NULLS LAST, country_code NULLS LAST, barcode NULLS LAST, sort_character, release);

CREATE UNIQUE INDEX artist_release_nonva_idx_uniq ON artist_release_nonva (release, artist);
CREATE UNIQUE INDEX artist_release_va_idx_uniq ON artist_release_va (release, artist);

CREATE INDEX artist_release_pending_update_idx_release ON artist_release_pending_update USING HASH (release);

CREATE INDEX artist_release_group_nonva_idx_sort ON artist_release_group_nonva (artist, unofficial, primary_type NULLS FIRST, secondary_types NULLS FIRST, first_release_date NULLS LAST, sort_character, release_group);
CREATE INDEX artist_release_group_va_idx_sort ON artist_release_group_va (artist, unofficial, primary_type NULLS FIRST, secondary_types NULLS FIRST, first_release_date NULLS LAST, sort_character, release_group);

CREATE UNIQUE INDEX artist_release_group_nonva_idx_uniq ON artist_release_group_nonva (release_group, artist);
CREATE UNIQUE INDEX artist_release_group_va_idx_uniq ON artist_release_group_va (release_group, artist);

CREATE INDEX artist_release_group_pending_update_idx_release_group ON artist_release_group_pending_update USING HASH (release_group);

CREATE INDEX cdtoc_raw_discid ON cdtoc_raw (discid);
CREATE UNIQUE INDEX cdtoc_raw_toc ON cdtoc_raw (track_count, leadout_offset, track_offset);

CREATE UNIQUE INDEX editor_idx_name ON editor (LOWER(name));
CREATE UNIQUE INDEX old_editor_name_idx_name ON old_editor_name (LOWER(name));
CREATE INDEX editor_language_idx_language ON editor_language (language);

CREATE INDEX editor_oauth_token_idx_editor ON editor_oauth_token (editor);
CREATE UNIQUE INDEX editor_oauth_token_idx_access_token ON editor_oauth_token (access_token);
CREATE UNIQUE INDEX editor_oauth_token_idx_refresh_token ON editor_oauth_token (refresh_token);

CREATE UNIQUE INDEX editor_preference_idx_editor_name ON editor_preference (editor, name);

CREATE UNIQUE INDEX editor_subscribe_artist_idx_uniq ON editor_subscribe_artist (editor, artist);
CREATE INDEX editor_subscribe_artist_idx_artist ON editor_subscribe_artist (artist);
CREATE UNIQUE INDEX editor_subscribe_collection_idx_uniq ON editor_subscribe_collection (editor, collection);
CREATE INDEX editor_subscribe_collection_idx_collection ON editor_subscribe_collection (collection);
CREATE UNIQUE INDEX editor_subscribe_label_idx_uniq ON editor_subscribe_label (editor, label);
CREATE INDEX editor_subscribe_label_idx_label ON editor_subscribe_label (label);
CREATE UNIQUE INDEX editor_subscribe_series_idx_uniq ON editor_subscribe_series (editor, series);
CREATE INDEX editor_subscribe_series_idx_series ON editor_subscribe_series (series);
CREATE UNIQUE INDEX editor_subscribe_editor_idx_uniq ON editor_subscribe_editor (editor, subscribed_editor);

CREATE UNIQUE INDEX event_type_idx_gid ON event_type (gid);

CREATE UNIQUE INDEX gender_idx_gid ON gender (gid);

CREATE UNIQUE INDEX genre_idx_gid ON genre (gid);
CREATE UNIQUE INDEX genre_idx_name ON genre (LOWER(name));

CREATE INDEX genre_alias_idx_genre ON genre_alias (genre);
CREATE UNIQUE INDEX genre_alias_idx_primary ON genre_alias (genre, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE UNIQUE INDEX instrument_type_idx_gid ON instrument_type (gid);

CREATE UNIQUE INDEX instrument_alias_type_idx_gid ON instrument_alias_type (gid);

CREATE INDEX edit_idx_editor_id_desc ON edit (editor, id DESC); -- DESC only for historical reasons
CREATE INDEX edit_idx_type_id ON edit (type, id);

-- Index for the "last 24 hours" edit count on the user profile
CREATE INDEX edit_idx_editor_open_time ON edit (editor, open_time);

-- Partial index for status (excludes applied edits)
CREATE INDEX edit_idx_status_id ON edit (status, id) WHERE status <> 2;

-- Indexes for materialized edit status
CREATE INDEX edit_artist_idx_status ON edit_artist (status);
CREATE INDEX edit_label_idx_status ON edit_label (status);

CREATE INDEX edit_idx_open_time ON edit USING BRIN (open_time);
CREATE INDEX edit_idx_close_time ON edit USING BRIN (close_time);
CREATE INDEX edit_idx_expire_time ON edit USING BRIN (expire_time);

CREATE INDEX edit_data_idx_link_type ON edit_data USING GIN (
    array_remove(ARRAY[
                     (data#>>'{link_type,id}')::int,
                     (data#>>'{link,link_type,id}')::int,
                     (data#>>'{old,link_type,id}')::int,
                     (data#>>'{new,link_type,id}')::int,
                     (data#>>'{relationship,link_type,id}')::int
                 ], NULL)
);

-- Entity indexes
CREATE INDEX edit_area_idx ON edit_area (area);
CREATE INDEX edit_artist_idx ON edit_artist (artist);
CREATE INDEX edit_event_idx ON edit_event (event);
CREATE INDEX edit_instrument_idx ON edit_instrument (instrument);
CREATE INDEX edit_label_idx ON edit_label (label);
CREATE INDEX edit_place_idx ON edit_place (place);
CREATE INDEX edit_release_idx ON edit_release (release);
CREATE INDEX edit_release_group_idx ON edit_release_group (release_group);
CREATE INDEX edit_recording_idx ON edit_recording (recording);
CREATE INDEX edit_series_idx ON edit_series (series);
CREATE INDEX edit_work_idx ON edit_work (work);
CREATE INDEX edit_url_idx ON edit_url (url);

CREATE INDEX edit_note_idx_edit ON edit_note (edit);
CREATE INDEX edit_note_idx_editor ON edit_note (editor);

CREATE INDEX edit_note_recipient_idx_recipient ON edit_note_recipient (recipient);

CREATE UNIQUE INDEX event_idx_gid ON event (gid);
CREATE INDEX event_idx_name ON event (name);

CREATE UNIQUE INDEX event_alias_type_idx_gid ON event_alias_type (gid);

CREATE INDEX event_alias_idx_event ON event_alias (event);
CREATE UNIQUE INDEX event_alias_idx_primary ON event_alias (event, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX event_attribute_idx_event ON event_attribute (event);

CREATE UNIQUE INDEX event_attribute_type_idx_gid ON event_attribute_type (gid);

CREATE INDEX event_attribute_type_allowed_value_idx_name ON event_attribute_type_allowed_value (event_attribute_type);
CREATE UNIQUE INDEX event_attribute_type_allowed_value_idx_gid ON event_attribute_type_allowed_value (gid);

CREATE INDEX event_rating_raw_idx_editor ON event_rating_raw (editor);

CREATE INDEX event_tag_idx_tag ON event_tag (tag);

CREATE INDEX event_tag_raw_idx_tag ON event_tag_raw (tag);
CREATE INDEX event_tag_raw_idx_editor ON event_tag_raw (editor);

CREATE UNIQUE INDEX instrument_idx_gid ON instrument (gid);
CREATE INDEX instrument_idx_name ON instrument (name);

CREATE INDEX instrument_alias_idx_instrument ON instrument_alias (instrument);
CREATE UNIQUE INDEX instrument_alias_idx_primary ON instrument_alias (instrument, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX instrument_attribute_idx_instrument ON instrument_attribute (instrument);

CREATE UNIQUE INDEX instrument_attribute_type_idx_gid ON instrument_attribute_type (gid);

CREATE INDEX instrument_attribute_type_allowed_value_idx_name ON instrument_attribute_type_allowed_value (instrument_attribute_type);
CREATE UNIQUE INDEX instrument_attribute_type_allowed_value_idx_gid ON instrument_attribute_type_allowed_value (gid);

CREATE INDEX instrument_tag_idx_tag ON instrument_tag (tag);

CREATE INDEX instrument_tag_raw_idx_instrument ON instrument_tag_raw (instrument);
CREATE INDEX instrument_tag_raw_idx_tag ON instrument_tag_raw (tag);
CREATE INDEX instrument_tag_raw_idx_editor ON instrument_tag_raw (editor);

CREATE INDEX isrc_idx_isrc ON isrc (isrc);
CREATE INDEX isrc_idx_recording ON isrc (recording);
CREATE UNIQUE INDEX isrc_idx_isrc_recording ON isrc (isrc, recording);

CREATE INDEX iswc_idx_work ON iswc (work);
CREATE UNIQUE INDEX iswc_idx_iswc ON iswc (iswc, work);

CREATE UNIQUE INDEX l_area_area_idx_uniq ON l_area_area (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_artist_idx_uniq ON l_area_artist (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_event_idx_uniq ON l_area_event (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_instrument_idx_uniq ON l_area_instrument (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_label_idx_uniq ON l_area_label (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_place_idx_uniq ON l_area_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_recording_idx_uniq ON l_area_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_release_idx_uniq ON l_area_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_release_group_idx_uniq ON l_area_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_series_idx_uniq ON l_area_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_url_idx_uniq ON l_area_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_area_work_idx_uniq ON l_area_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_artist_artist_idx_uniq ON l_artist_artist (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_event_idx_uniq ON l_artist_event (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_instrument_idx_uniq ON l_artist_instrument (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_label_idx_uniq ON l_artist_label (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_place_idx_uniq ON l_artist_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_recording_idx_uniq ON l_artist_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_release_idx_uniq ON l_artist_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_release_group_idx_uniq ON l_artist_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_series_idx_uniq ON l_artist_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_url_idx_uniq ON l_artist_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_artist_work_idx_uniq ON l_artist_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_event_event_idx_uniq ON l_event_event (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_instrument_idx_uniq ON l_event_instrument (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_label_idx_uniq ON l_event_label (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_place_idx_uniq ON l_event_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_recording_idx_uniq ON l_event_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_release_idx_uniq ON l_event_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_release_group_idx_uniq ON l_event_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_series_idx_uniq ON l_event_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_url_idx_uniq ON l_event_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_event_work_idx_uniq ON l_event_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_instrument_instrument_idx_uniq ON l_instrument_instrument (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_label_idx_uniq ON l_instrument_label (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_place_idx_uniq ON l_instrument_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_recording_idx_uniq ON l_instrument_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_release_idx_uniq ON l_instrument_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_release_group_idx_uniq ON l_instrument_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_series_idx_uniq ON l_instrument_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_url_idx_uniq ON l_instrument_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_instrument_work_idx_uniq ON l_instrument_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_label_label_idx_uniq ON l_label_label (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_place_idx_uniq ON l_label_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_recording_idx_uniq ON l_label_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_release_idx_uniq ON l_label_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_release_group_idx_uniq ON l_label_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_series_idx_uniq ON l_label_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_url_idx_uniq ON l_label_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_label_work_idx_uniq ON l_label_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_place_place_idx_uniq ON l_place_place (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_recording_idx_uniq ON l_place_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_release_idx_uniq ON l_place_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_release_group_idx_uniq ON l_place_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_series_idx_uniq ON l_place_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_url_idx_uniq ON l_place_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_place_work_idx_uniq ON l_place_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_recording_recording_idx_uniq ON l_recording_recording (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_recording_release_idx_uniq ON l_recording_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_recording_release_group_idx_uniq ON l_recording_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_recording_series_idx_uniq ON l_recording_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_recording_url_idx_uniq ON l_recording_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_recording_work_idx_uniq ON l_recording_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_release_release_idx_uniq ON l_release_release (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_release_group_idx_uniq ON l_release_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_series_idx_uniq ON l_release_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_url_idx_uniq ON l_release_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_work_idx_uniq ON l_release_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_release_group_release_group_idx_uniq ON l_release_group_release_group (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_group_series_idx_uniq ON l_release_group_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_group_url_idx_uniq ON l_release_group_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_release_group_work_idx_uniq ON l_release_group_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_series_series_idx_uniq ON l_series_series (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_series_url_idx_uniq ON l_series_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_series_work_idx_uniq ON l_series_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_url_url_idx_uniq ON l_url_url (entity0, entity1, link, link_order);
CREATE UNIQUE INDEX l_url_work_idx_uniq ON l_url_work (entity0, entity1, link, link_order);

CREATE UNIQUE INDEX l_work_work_idx_uniq ON l_work_work (entity0, entity1, link, link_order);

CREATE INDEX l_area_area_idx_entity1 ON l_area_area (entity1);
CREATE INDEX l_area_artist_idx_entity1 ON l_area_artist (entity1);
CREATE INDEX l_area_event_idx_entity1 ON l_area_event (entity1);
CREATE INDEX l_area_instrument_idx_entity1 ON l_area_instrument (entity1);
CREATE INDEX l_area_label_idx_entity1 ON l_area_label (entity1);
CREATE INDEX l_area_place_idx_entity1 ON l_area_place (entity1);
CREATE INDEX l_area_recording_idx_entity1 ON l_area_recording (entity1);
CREATE INDEX l_area_release_idx_entity1 ON l_area_release (entity1);
CREATE INDEX l_area_release_group_idx_entity1 ON l_area_release_group (entity1);
CREATE INDEX l_area_series_idx_entity1 ON l_area_series (entity1);
CREATE INDEX l_area_url_idx_entity1 ON l_area_url (entity1);
CREATE INDEX l_area_work_idx_entity1 ON l_area_work (entity1);

CREATE INDEX l_artist_artist_idx_entity1 ON l_artist_artist (entity1);
CREATE INDEX l_artist_event_idx_entity1 ON l_artist_event (entity1);
CREATE INDEX l_artist_instrument_idx_entity1 ON l_artist_instrument (entity1);
CREATE INDEX l_artist_label_idx_entity1 ON l_artist_label (entity1);
CREATE INDEX l_artist_place_idx_entity1 ON l_artist_place (entity1);
CREATE INDEX l_artist_recording_idx_entity1 ON l_artist_recording (entity1);
CREATE INDEX l_artist_release_idx_entity1 ON l_artist_release (entity1);
CREATE INDEX l_artist_release_group_idx_entity1 ON l_artist_release_group (entity1);
CREATE INDEX l_artist_series_idx_entity1 ON l_artist_series (entity1);
CREATE INDEX l_artist_url_idx_entity1 ON l_artist_url (entity1);
CREATE INDEX l_artist_work_idx_entity1 ON l_artist_work (entity1);

CREATE INDEX l_event_event_idx_entity1 ON l_event_event (entity1);
CREATE INDEX l_event_instrument_idx_entity1 ON l_event_instrument (entity1);
CREATE INDEX l_event_label_idx_entity1 ON l_event_label (entity1);
CREATE INDEX l_event_place_idx_entity1 ON l_event_place (entity1);
CREATE INDEX l_event_recording_idx_entity1 ON l_event_recording (entity1);
CREATE INDEX l_event_release_idx_entity1 ON l_event_release (entity1);
CREATE INDEX l_event_release_group_idx_entity1 ON l_event_release_group (entity1);
CREATE INDEX l_event_series_idx_entity1 ON l_event_series (entity1);
CREATE INDEX l_event_url_idx_entity1 ON l_event_url (entity1);
CREATE INDEX l_event_work_idx_entity1 ON l_event_work (entity1);

CREATE INDEX l_instrument_instrument_idx_entity1 ON l_instrument_instrument (entity1);
CREATE INDEX l_instrument_label_idx_entity1 ON l_instrument_label (entity1);
CREATE INDEX l_instrument_place_idx_entity1 ON l_instrument_place (entity1);
CREATE INDEX l_instrument_recording_idx_entity1 ON l_instrument_recording (entity1);
CREATE INDEX l_instrument_release_idx_entity1 ON l_instrument_release (entity1);
CREATE INDEX l_instrument_release_group_idx_entity1 ON l_instrument_release_group (entity1);
CREATE INDEX l_instrument_series_idx_entity1 ON l_instrument_series (entity1);
CREATE INDEX l_instrument_url_idx_entity1 ON l_instrument_url (entity1);
CREATE INDEX l_instrument_work_idx_entity1 ON l_instrument_work (entity1);

CREATE INDEX l_label_label_idx_entity1 ON l_label_label (entity1);
CREATE INDEX l_label_place_idx_entity1 ON l_label_place (entity1);
CREATE INDEX l_label_recording_idx_entity1 ON l_label_recording (entity1);
CREATE INDEX l_label_release_idx_entity1 ON l_label_release (entity1);
CREATE INDEX l_label_release_group_idx_entity1 ON l_label_release_group (entity1);
CREATE INDEX l_label_series_idx_entity1 ON l_label_series (entity1);
CREATE INDEX l_label_url_idx_entity1 ON l_label_url (entity1);
CREATE INDEX l_label_work_idx_entity1 ON l_label_work (entity1);

CREATE INDEX l_place_place_idx_entity1 ON l_place_place (entity1);
CREATE INDEX l_place_recording_idx_entity1 ON l_place_recording (entity1);
CREATE INDEX l_place_release_idx_entity1 ON l_place_release (entity1);
CREATE INDEX l_place_release_group_idx_entity1 ON l_place_release_group (entity1);
CREATE INDEX l_place_series_idx_entity1 ON l_place_series (entity1);
CREATE INDEX l_place_url_idx_entity1 ON l_place_url (entity1);
CREATE INDEX l_place_work_idx_entity1 ON l_place_work (entity1);

CREATE INDEX l_recording_recording_idx_entity1 ON l_recording_recording (entity1);
CREATE INDEX l_recording_release_idx_entity1 ON l_recording_release (entity1);
CREATE INDEX l_recording_release_group_idx_entity1 ON l_recording_release_group (entity1);
CREATE INDEX l_recording_series_idx_entity1 ON l_recording_series (entity1);
CREATE INDEX l_recording_url_idx_entity1 ON l_recording_url (entity1);
CREATE INDEX l_recording_work_idx_entity1 ON l_recording_work (entity1);

CREATE INDEX l_release_release_idx_entity1 ON l_release_release (entity1);
CREATE INDEX l_release_release_group_idx_entity1 ON l_release_release_group (entity1);
CREATE INDEX l_release_series_idx_entity1 ON l_release_series (entity1);
CREATE INDEX l_release_url_idx_entity1 ON l_release_url (entity1);
CREATE INDEX l_release_work_idx_entity1 ON l_release_work (entity1);

CREATE INDEX l_release_group_release_group_idx_entity1 ON l_release_group_release_group (entity1);
CREATE INDEX l_release_group_series_idx_entity1 ON l_release_group_series (entity1);
CREATE INDEX l_release_group_url_idx_entity1 ON l_release_group_url (entity1);
CREATE INDEX l_release_group_work_idx_entity1 ON l_release_group_work (entity1);

CREATE INDEX l_series_series_idx_entity1 ON l_series_series (entity1);
CREATE INDEX l_series_url_idx_entity1 ON l_series_url (entity1);
CREATE INDEX l_series_work_idx_entity1 ON l_series_work (entity1);

CREATE INDEX l_url_url_idx_entity1 ON l_url_url (entity1);
CREATE INDEX l_url_work_idx_entity1 ON l_url_work (entity1);

CREATE INDEX l_work_work_idx_entity1 ON l_work_work (entity1);

CREATE UNIQUE INDEX link_type_idx_gid ON link_type (gid);
CREATE UNIQUE INDEX link_attribute_type_idx_gid ON link_attribute_type (gid);

CREATE INDEX link_idx_type_attr ON link (link_type, attribute_count);

CREATE UNIQUE INDEX label_idx_gid ON label (gid);
CREATE INDEX label_idx_name ON label (name);

CREATE INDEX label_idx_area ON label (area);

CREATE UNIQUE INDEX label_idx_null_comment ON label (name) WHERE comment IS NULL;
CREATE UNIQUE INDEX label_idx_uniq_name_comment ON label (name, comment) WHERE comment IS NOT NULL;

CREATE UNIQUE INDEX label_alias_type_idx_gid ON label_alias_type (gid);

CREATE INDEX label_alias_idx_label ON label_alias (label);
CREATE UNIQUE INDEX label_alias_idx_primary ON label_alias (label, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX label_attribute_idx_label ON label_attribute (label);

CREATE UNIQUE INDEX label_attribute_type_idx_gid ON label_attribute_type (gid);

CREATE INDEX label_attribute_type_allowed_value_idx_name ON label_attribute_type_allowed_value (label_attribute_type);
CREATE UNIQUE INDEX label_attribute_type_allowed_value_idx_gid ON label_attribute_type_allowed_value (gid);

CREATE INDEX label_tag_idx_tag ON label_tag (tag);

CREATE INDEX label_tag_raw_idx_tag ON label_tag_raw (tag);
CREATE INDEX label_tag_raw_idx_editor ON label_tag_raw (editor);

CREATE INDEX label_rating_raw_idx_editor ON label_rating_raw (editor);

CREATE UNIQUE INDEX label_type_idx_gid ON label_type (gid);

CREATE UNIQUE INDEX language_idx_iso_code_2b ON language (iso_code_2b);
CREATE UNIQUE INDEX language_idx_iso_code_2t ON language (iso_code_2t);
CREATE UNIQUE INDEX language_idx_iso_code_1 ON language (iso_code_1);
CREATE UNIQUE INDEX language_idx_iso_code_3 ON language (iso_code_3);

CREATE UNIQUE INDEX editor_collection_idx_gid ON editor_collection (gid);
CREATE INDEX editor_collection_idx_editor ON editor_collection (editor);

CREATE UNIQUE INDEX editor_collection_type_idx_gid ON editor_collection_type (gid);

CREATE UNIQUE INDEX cdtoc_idx_discid ON cdtoc (discid);
CREATE INDEX cdtoc_idx_freedb_id ON cdtoc (freedb_id);

CREATE INDEX medium_attribute_idx_medium ON medium_attribute (medium);

CREATE UNIQUE INDEX medium_attribute_type_idx_gid ON medium_attribute_type (gid);

CREATE INDEX medium_attribute_type_allowed_value_idx_name ON medium_attribute_type_allowed_value (medium_attribute_type);
CREATE UNIQUE INDEX medium_attribute_type_allowed_value_idx_gid ON medium_attribute_type_allowed_value (gid);

CREATE INDEX medium_cdtoc_idx_medium ON medium_cdtoc (medium);
CREATE INDEX medium_cdtoc_idx_cdtoc ON medium_cdtoc (cdtoc);
CREATE UNIQUE INDEX medium_cdtoc_idx_uniq ON medium_cdtoc (medium, cdtoc);

CREATE UNIQUE INDEX medium_format_idx_gid ON medium_format (gid);

CREATE UNIQUE INDEX place_idx_gid ON place (gid);
CREATE INDEX place_idx_name ON place (name);
CREATE INDEX place_idx_area ON place (area);
CREATE INDEX place_idx_geo ON place USING gist (musicbrainz.ll_to_earth(coordinates[0], coordinates[1])) WHERE coordinates IS NOT NULL;

CREATE INDEX place_alias_idx_place ON place_alias (place);
CREATE UNIQUE INDEX place_alias_idx_primary ON place_alias (place, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX place_attribute_idx_place ON place_attribute (place);

CREATE UNIQUE INDEX place_attribute_type_idx_gid ON place_attribute_type (gid);

CREATE INDEX place_attribute_type_allowed_value_idx_name ON place_attribute_type_allowed_value (place_attribute_type);
CREATE UNIQUE INDEX place_attribute_type_allowed_value_idx_gid ON place_attribute_type_allowed_value (gid);

CREATE UNIQUE INDEX place_alias_type_idx_gid ON place_alias_type (gid);

CREATE INDEX place_rating_raw_idx_editor ON place_rating_raw (editor);

CREATE INDEX place_tag_idx_tag ON place_tag (tag);

CREATE INDEX place_tag_raw_idx_tag ON place_tag_raw (tag);
CREATE INDEX place_tag_raw_idx_editor ON place_tag_raw (editor);

CREATE UNIQUE INDEX place_type_idx_gid ON place_type (gid);

CREATE UNIQUE INDEX recording_idx_gid ON recording (gid);
CREATE INDEX recording_idx_name ON recording (name);
CREATE INDEX recording_idx_artist_credit ON recording (artist_credit);

CREATE UNIQUE INDEX recording_alias_type_idx_gid ON recording_alias_type (gid);

CREATE INDEX recording_alias_idx_recording ON recording_alias (recording);
CREATE UNIQUE INDEX recording_alias_idx_primary ON recording_alias (recording, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX recording_attribute_idx_recording ON recording_attribute (recording);

CREATE UNIQUE INDEX recording_attribute_type_idx_gid ON recording_attribute_type (gid);

CREATE INDEX recording_attribute_type_allowed_value_idx_name ON recording_attribute_type_allowed_value (recording_attribute_type);
CREATE UNIQUE INDEX recording_attribute_type_allowed_value_idx_gid ON recording_attribute_type_allowed_value (gid);

CREATE INDEX recording_tag_idx_tag ON recording_tag (tag);

CREATE INDEX recording_rating_raw_idx_editor ON recording_rating_raw (editor);

CREATE INDEX recording_tag_raw_idx_track ON recording_tag_raw (recording);
CREATE INDEX recording_tag_raw_idx_tag ON recording_tag_raw (tag);
CREATE INDEX recording_tag_raw_idx_editor ON recording_tag_raw (editor);

CREATE UNIQUE INDEX release_idx_gid ON release (gid);
CREATE INDEX release_idx_name ON release (name);
CREATE INDEX release_idx_release_group ON release (release_group);
CREATE INDEX release_idx_artist_credit ON release (artist_credit);

CREATE INDEX release_alias_idx_release ON release_alias (release);
CREATE UNIQUE INDEX release_alias_idx_primary ON release_alias (release, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX release_attribute_idx_release ON release_attribute (release);

CREATE UNIQUE INDEX release_attribute_type_idx_gid ON release_attribute_type (gid);

CREATE INDEX release_attribute_type_allowed_value_idx_name ON release_attribute_type_allowed_value (release_attribute_type);
CREATE UNIQUE INDEX release_attribute_type_allowed_value_idx_gid ON release_attribute_type_allowed_value (gid);

CREATE INDEX release_tag_idx_tag ON release_tag (tag);

CREATE INDEX release_tag_raw_idx_tag ON release_tag_raw (tag);
CREATE INDEX release_tag_raw_idx_editor ON release_tag_raw (editor);

CREATE INDEX release_label_idx_release ON release_label (release);
CREATE INDEX release_label_idx_label ON release_label (label);

CREATE UNIQUE INDEX release_packaging_idx_gid ON release_packaging (gid);

CREATE UNIQUE INDEX release_status_idx_gid ON release_status (gid);

CREATE INDEX release_country_idx_country ON release_country (country);

CREATE UNIQUE INDEX release_group_idx_gid ON release_group (gid);
CREATE INDEX release_group_idx_name ON release_group (name);
CREATE INDEX release_group_idx_artist_credit ON release_group (artist_credit);

CREATE UNIQUE INDEX release_group_alias_type_idx_gid ON release_group_alias_type (gid);

CREATE INDEX release_group_alias_idx_release_group ON release_group_alias (release_group);
CREATE UNIQUE INDEX release_group_alias_idx_primary ON release_group_alias (release_group, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX release_group_attribute_idx_release_group ON release_group_attribute (release_group);

CREATE UNIQUE INDEX release_group_attribute_type_idx_gid ON release_group_attribute_type (gid);

CREATE INDEX release_group_attribute_type_allowed_value_idx_name ON release_group_attribute_type_allowed_value (release_group_attribute_type);
CREATE UNIQUE INDEX release_group_attribute_type_allowed_value_idx_gid ON release_group_attribute_type_allowed_value (gid);

CREATE INDEX release_group_tag_idx_tag ON release_group_tag (tag);

CREATE INDEX release_group_rating_raw_idx_editor ON release_group_rating_raw (editor);

CREATE INDEX release_group_tag_raw_idx_tag ON release_group_tag_raw (tag);
CREATE INDEX release_group_tag_raw_idx_editor ON release_group_tag_raw (editor);

CREATE UNIQUE INDEX release_group_primary_type_idx_gid ON release_group_primary_type (gid);

CREATE UNIQUE INDEX release_group_secondary_type_idx_gid ON release_group_secondary_type (gid);

CREATE UNIQUE INDEX script_idx_iso_code ON script (iso_code);

CREATE UNIQUE INDEX series_idx_gid ON series (gid);
CREATE INDEX series_idx_name ON series (name);

CREATE UNIQUE INDEX series_type_idx_gid ON series_type (gid);

CREATE UNIQUE INDEX series_ordering_type_idx_gid ON series_ordering_type (gid);

CREATE UNIQUE INDEX series_alias_type_idx_gid ON series_alias_type (gid);

CREATE INDEX series_alias_idx_series ON series_alias (series);
CREATE UNIQUE INDEX series_alias_idx_primary ON series_alias (series, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX series_attribute_idx_series ON series_attribute (series);

CREATE UNIQUE INDEX series_attribute_type_idx_gid ON series_attribute_type (gid);

CREATE INDEX series_attribute_type_allowed_value_idx_name ON series_attribute_type_allowed_value (series_attribute_type);
CREATE UNIQUE INDEX series_attribute_type_allowed_value_idx_gid ON series_attribute_type_allowed_value (gid);

CREATE INDEX series_tag_idx_tag ON series_tag (tag);

CREATE INDEX series_tag_raw_idx_series ON series_tag_raw (series);
CREATE INDEX series_tag_raw_idx_tag ON series_tag_raw (tag);
CREATE INDEX series_tag_raw_idx_editor ON series_tag_raw (editor);

CREATE UNIQUE INDEX tag_idx_name ON tag (name);

CREATE UNIQUE INDEX track_idx_gid ON track (gid);

CREATE INDEX track_idx_recording ON track (recording);
CREATE INDEX track_idx_artist_credit ON track (artist_credit);

CREATE INDEX track_raw_idx_release ON track_raw (release);

CREATE INDEX medium_idx_track_count ON medium (track_count);
CREATE INDEX medium_index_idx ON medium_index USING gist (toc);

CREATE UNIQUE INDEX url_idx_gid ON url (gid);
CREATE UNIQUE INDEX url_idx_url ON url (url);

CREATE INDEX vote_idx_edit ON vote (edit);
CREATE INDEX vote_idx_editor_vote_time ON vote (editor, vote_time);
CREATE INDEX vote_idx_editor_edit ON vote (editor, edit) WHERE superseded = FALSE;

CREATE UNIQUE INDEX work_idx_gid ON work (gid);
CREATE INDEX work_idx_name ON work (name);

CREATE UNIQUE INDEX work_alias_type_idx_gid ON work_alias_type (gid);

CREATE INDEX work_alias_idx_work ON work_alias (work);
CREATE UNIQUE INDEX work_alias_idx_primary ON work_alias (work, locale) WHERE primary_for_locale = TRUE AND locale IS NOT NULL;

CREATE INDEX work_attribute_idx_work ON work_attribute (work);

CREATE UNIQUE INDEX work_attribute_type_idx_gid ON work_attribute_type (gid);

CREATE INDEX work_attribute_type_allowed_value_idx_name ON work_attribute_type_allowed_value (work_attribute_type);
CREATE UNIQUE INDEX work_attribute_type_allowed_value_idx_gid ON work_attribute_type_allowed_value (gid);

CREATE INDEX work_tag_idx_tag ON work_tag (tag);

CREATE INDEX work_tag_raw_idx_tag ON work_tag_raw (tag);

CREATE UNIQUE INDEX work_type_idx_gid ON work_type (gid);

-- indexes for /ws/js/check_duplicates
CREATE INDEX artist_idx_lower_unaccent_name_comment ON artist (lower(musicbrainz_unaccent(name)), lower(musicbrainz_unaccent(comment)));
CREATE INDEX label_idx_lower_unaccent_name_comment ON label (lower(musicbrainz_unaccent(name)), lower(musicbrainz_unaccent(comment)));
CREATE INDEX place_idx_lower_unaccent_name_comment ON place (lower(musicbrainz_unaccent(name)), lower(musicbrainz_unaccent(comment)));
CREATE INDEX series_idx_lower_unaccent_name_comment ON series (lower(musicbrainz_unaccent(name)), lower(musicbrainz_unaccent(comment)));
CREATE INDEX artist_alias_idx_lower_unaccent_name ON artist_alias (lower(musicbrainz_unaccent(name)));
CREATE INDEX label_alias_idx_lower_unaccent_name ON label_alias (lower(musicbrainz_unaccent(name)));
CREATE INDEX place_alias_idx_lower_unaccent_name ON place_alias (lower(musicbrainz_unaccent(name)));
CREATE INDEX series_alias_idx_lower_unaccent_name ON series_alias (lower(musicbrainz_unaccent(name)));

-- collated name indexes for unicode sorting
CREATE INDEX release_idx_musicbrainz_collate ON release (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX release_group_idx_musicbrainz_collate ON release_group (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX artist_idx_musicbrainz_collate ON artist (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX artist_credit_idx_musicbrainz_collate ON artist_credit (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX artist_credit_name_idx_musicbrainz_collate ON artist_credit_name (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX label_idx_musicbrainz_collate ON label (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX recording_idx_musicbrainz_collate ON recording (name COLLATE musicbrainz.musicbrainz);
CREATE INDEX work_idx_musicbrainz_collate ON work (name COLLATE musicbrainz.musicbrainz);

CREATE INDEX alternative_release_idx_release ON alternative_release (release);
CREATE INDEX alternative_release_idx_name ON alternative_release (name);
CREATE INDEX alternative_release_idx_artist_credit ON alternative_release (artist_credit);
CREATE INDEX alternative_release_idx_language_script ON alternative_release (language, script);
CREATE UNIQUE INDEX alternative_release_idx_gid ON alternative_release (gid);
CREATE INDEX alternative_medium_idx_alternative_release ON alternative_medium (alternative_release);
CREATE INDEX alternative_track_idx_name ON alternative_track (name);
CREATE INDEX alternative_track_idx_artist_credit ON alternative_track (artist_credit);

CREATE INDEX area_gid_redirect_idx_new_id ON area_gid_redirect (new_id);
CREATE INDEX artist_gid_redirect_idx_new_id ON artist_gid_redirect (new_id);
CREATE INDEX editor_collection_gid_redirect_idx_new_id ON editor_collection_gid_redirect (new_id);
CREATE INDEX event_gid_redirect_idx_new_id ON event_gid_redirect (new_id);
CREATE INDEX instrument_gid_redirect_idx_new_id ON instrument_gid_redirect (new_id);
CREATE INDEX label_gid_redirect_idx_new_id ON label_gid_redirect (new_id);
CREATE INDEX place_gid_redirect_idx_new_id ON place_gid_redirect (new_id);
CREATE INDEX recording_gid_redirect_idx_new_id ON recording_gid_redirect (new_id);
CREATE INDEX release_gid_redirect_idx_new_id ON release_gid_redirect (new_id);
CREATE INDEX release_group_gid_redirect_idx_new_id ON release_group_gid_redirect (new_id);
CREATE INDEX series_gid_redirect_idx_new_id ON series_gid_redirect (new_id);
CREATE INDEX track_gid_redirect_idx_new_id ON track_gid_redirect (new_id);
CREATE INDEX url_gid_redirect_idx_new_id ON url_gid_redirect (new_id);
CREATE INDEX work_gid_redirect_idx_new_id ON work_gid_redirect (new_id);

-- https://github.com/metabrainz/musicbrainz-server/blob/master/admin/sql/CreateSearchIndexes.sql

\unset ON_ERROR_STOP

CREATE INDEX artist_idx_txt ON artist USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX artist_idx_txt_sort ON artist USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX artist_alias_idx_txt ON artist_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX artist_alias_idx_txt_sort ON artist_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX artist_credit_idx_txt ON artist_credit USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX artist_credit_name_idx_txt ON artist_credit_name USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX event_idx_txt ON event USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX event_alias_idx_txt ON event_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX event_alias_idx_txt_sort ON event_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX instrument_idx_txt ON instrument USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX label_idx_txt ON label USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX label_alias_idx_txt ON label_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX label_alias_idx_txt_sort ON label_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX release_idx_txt ON release USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX release_alias_idx_txt ON release_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX release_alias_idx_txt_sort ON release_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX release_group_idx_txt ON release_group USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX release_group_alias_idx_txt ON release_group_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX release_group_alias_idx_txt_sort ON release_group_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX recording_idx_txt ON recording USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX recording_alias_idx_txt ON recording_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX recording_alias_idx_txt_sort ON recording_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX series_idx_txt ON series USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX series_alias_idx_txt ON series_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX series_alias_idx_txt_sort ON series_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX work_idx_txt ON work USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX work_alias_idx_txt ON work_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX work_alias_idx_txt_sort ON work_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX area_idx_name_txt ON area USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX area_alias_idx_txt ON area_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX area_alias_idx_txt_sort ON area_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX place_idx_name_txt ON place USING gin(musicbrainz.mb_simple_tsvector(name));

CREATE INDEX place_alias_idx_txt ON place_alias USING gin(musicbrainz.mb_simple_tsvector(name));
CREATE INDEX place_alias_idx_txt_sort ON place_alias USING gin(musicbrainz.mb_simple_tsvector(sort_name));

CREATE INDEX tag_idx_name_txt ON tag USING gin(musicbrainz.mb_simple_tsvector(name));
