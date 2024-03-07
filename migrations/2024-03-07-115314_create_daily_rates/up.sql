CREATE TABLE daily_rates(
    rate_type_id TINYINT UNSIGNED NOT NULL COMMENT 'レート種別ID',
    rate_date DATE COMMENT 'レート日付',
    closing_rate DECIMAL(15,4) NOT NULL COMMENT '終値',
    opening_rate DECIMAL(15,4) NOT NULL COMMENT '始値',
    high_rate DECIMAL(15,4) NOT NULL COMMENT '高値',
    low_rate DECIMAL(15,4) NOT NULL COMMENT '安値',
    volume DECIMAL(15,4) NOT NULL COMMENT '出来高',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '作成日時(UTC)',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新日時(UTC)',

    PRIMARY KEY (rate_type_id, rate_date),
    CONSTRAINT fk_daily_rates_rates_type_id FOREIGN KEY (rate_type_id) REFERENCES rate_types (id)
)
COMMENT '日足レート'
;
