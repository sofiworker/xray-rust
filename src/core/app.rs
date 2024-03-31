use log::debug;
use crate::conf::AppConfig;
use crate::conf::SettingConfig;
use crate::core::server::Server;

pub struct App {
    app_config: AppConfig,
    setting_config: SettingConfig,
}

impl App {
    pub fn new() -> App {
        let app_config = AppConfig::new();
        debug!("{:?}", app_config);
        let setting_config = SettingConfig::new();
        return App{
            app_config,
            setting_config,
        };
    }

    pub async fn start(&self) -> Result<(), ()> {
        let server = Server::new(self.app_config.clone());
        server.start().await.unwrap();
        Ok(())
    }
}