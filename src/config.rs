use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub db: DbConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub address: String,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub url: String,
}

/**
 * load config
 */
pub(crate) fn load_config() -> Result<AppConfig, figment::Error> {
    let config = Figment::new()
        // 加载配置文件
        .merge(Toml::file("app.toml"))
        // 加载环境变量（如果环境变量中有匹配项，将覆盖文件中的配置）
        // .merge(figment::providers::Env::prefixed("APP_"))
        // 反序列化为结构体
        .extract::<AppConfig>()?;

    Ok(config)
}
