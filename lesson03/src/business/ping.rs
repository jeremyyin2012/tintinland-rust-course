use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

pub struct PingBusiness {
    svc: Services,
}

impl PingBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}

impl PingBusiness {
    pub async fn do_ping(&self) -> Result<String, Error> {
        self.svc.ping.ping_pgsql().await?;
        self.svc.ping.ping_redis().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_mysql(&self) -> Result<String, Error> {
        self.svc.ping.ping_mysql().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_pgsql(&self) -> Result<String, Error> {
        self.svc.ping.ping_pgsql().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_redis(&self) -> Result<String, Error> {
        self.svc.ping.ping_redis().await?;
        Ok("pong".to_string())
    }
}
