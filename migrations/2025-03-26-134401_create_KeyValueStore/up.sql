DROP TABLE IF EXISTS KeyValueStore;

create TABLE KeyValueStore
(
    id             INTEGER     not null
        primary key,
    key            VARCHAR(25) not null,
    value_type     VARCHAR(20) not null,
    string_value   VARCHAR(255),
    datetime_value DATETIME,
    float_value    FLOAT,
    int_value      INTEGER
);

create index ix_KeyValueStore_key
    on KeyValueStore (key);