use taple_core::{
    ListenAddr, NetworkSettings, NodeSettings, Settings as CoreSettings,
};

use crate::{error::SettingsError, models::others::TapleKeyDerivator};

pub struct TapleSettings {
    pub listen_addr: Vec<String>,
    pub key_derivator: TapleKeyDerivator,
    pub private_key: Vec<u8>,
    pub known_nodes: Vec<String>,
}

impl TryInto<CoreSettings> for TapleSettings {
    type Error = SettingsError;
    fn try_into(self) -> Result<CoreSettings, SettingsError> {
        let hex_private_key = hex::encode(self.private_key);
        let default_settings = CoreSettings::default();
        let mut listen_addr = Vec::new();
        for addr in self.listen_addr {
            listen_addr
                .push(ListenAddr::try_from(addr).map_err(|_| SettingsError::InvalidListenAddr)?);
        }
        Ok(CoreSettings {
            network: NetworkSettings {
                listen_addr,
                known_nodes: self.known_nodes,
                external_address: Vec::new(),
            },
            node: NodeSettings {
                key_derivator: self.key_derivator.into(),
                secret_key: hex_private_key,
                digest_derivator: default_settings.node.digest_derivator,
                replication_factor: default_settings.node.replication_factor,
                timeout: default_settings.node.timeout,
                passvotation: default_settings.node.passvotation,
            },
        })
    }
}
