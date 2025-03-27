# zms-tradition-bot-template
Cryptocurrency quantitative trading robot template

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```
新的指标/自定义指标计算的项目会使用 https://github.com/ztNozdormu/zms-tradition-indicator-template.git 模版生成基本的代码：

> 首先进入工作空间目录 自定义项目名称【yourprojectname】

```bash
cargo generate --git https://github.com/ztNozdormu/zms-tradition-bot-template --name yourprojectname
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

### desiel 

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

* diesel migration generate create_tableName
* diesel migration run
* diesel migration revert
* diesel migration redo run+revert
-- create keyvaluestore
-- diesel migration generate create_KeyValueStore
```bash

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

```
-- create orders
-- diesel migration generate create_orders

```bash
-- orders definition

DROP TABLE IF EXISTS orders;

CREATE TABLE orders (
	id INTEGER NOT NULL, 
	ft_trade_id INTEGER NOT NULL, 
	ft_order_side VARCHAR(25) NOT NULL, 
	ft_pair VARCHAR(25) NOT NULL, 
	ft_is_open BOOLEAN NOT NULL, 
	ft_amount FLOAT NOT NULL, 
	ft_price FLOAT NOT NULL, 
	ft_cancel_reason VARCHAR(255), 
	order_id VARCHAR(255) NOT NULL, 
	status VARCHAR(255), 
	symbol VARCHAR(25), 
	order_type VARCHAR(50), 
	side VARCHAR(25), 
	price FLOAT, 
	average FLOAT, 
	amount FLOAT, 
	filled FLOAT, 
	remaining FLOAT, 
	cost FLOAT, 
	stop_price FLOAT, 
	order_date DATETIME, 
	order_filled_date DATETIME, 
	order_update_date DATETIME, 
	funding_fee FLOAT, 
	ft_fee_base FLOAT, 
	ft_order_tag VARCHAR(255), 
	PRIMARY KEY (id), 
	CONSTRAINT _order_pair_order_id UNIQUE (ft_pair, order_id), 
	FOREIGN KEY(ft_trade_id) REFERENCES trades (id)
);

CREATE INDEX ix_orders_order_id ON orders (order_id);
CREATE INDEX ix_orders_ft_trade_id ON orders (ft_trade_id);
CREATE INDEX ix_orders_ft_is_open ON orders (ft_is_open);



```