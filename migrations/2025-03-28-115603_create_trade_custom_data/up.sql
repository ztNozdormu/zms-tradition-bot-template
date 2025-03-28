-- Your SQL goes here

DROP TABLE IF EXISTS trade_custom_data;

CREATE TABLE trade_custom_data (
                                   id INTEGER NOT NULL,
                                   ft_trade_id INTEGER,
                                   cd_key VARCHAR(255) NOT NULL,
                                   cd_type VARCHAR(25) NOT NULL,
                                   cd_value TEXT NOT NULL,
                                   created_at DATETIME NOT NULL,
                                   updated_at DATETIME,
                                   PRIMARY KEY (id),
                                   CONSTRAINT _trade_id_cd_key UNIQUE (ft_trade_id, cd_key),
                                   FOREIGN KEY(ft_trade_id) REFERENCES trades (id)
);

CREATE INDEX ix_trade_custom_data_ft_trade_id ON trade_custom_data (ft_trade_id);