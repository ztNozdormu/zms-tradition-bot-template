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

INSERT INTO KeyValueStore
(id, "key", value_type, string_value, datetime_value, float_value, int_value)
VALUES(1, 'bot_start_time', 'datetime', NULL, '2024-10-14 04:48:01.483844', NULL, NULL);
INSERT INTO KeyValueStore
(id, "key", value_type, string_value, datetime_value, float_value, int_value)
VALUES(2, 'startup_time', 'datetime', NULL, '2025-01-08 12:48:39.537774', NULL, NULL);