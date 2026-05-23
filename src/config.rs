#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    CN,
    EN,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionKey {
    Scan,
    CreateField,
    Capture,
    DestroyField,
    Deploy8th,
    CreateLink,
    DestroyLink,
    DeployMod,
    DeployRes,
    HackEnemy,
    DestroyRes,
    UpgradeRes,
}

#[derive(Clone, Debug)]
pub struct ActionConfig {
    pub id: usize,
    pub key: ActionKey,
    pub base_ap: i32,
    pub enabled: bool,
    pub global_multiplier: u32,
    pub apex_multiplier: u32,
}

impl ActionConfig {
    pub fn new(id: usize, key: ActionKey, base_ap: i32) -> Self {
        Self {
            id,
            key,
            base_ap,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        }
    }
}

pub fn default_configs() -> Vec<ActionConfig> {
    let mut configs = vec![
        ActionConfig::new(1, ActionKey::Scan, 500),
        ActionConfig::new(2, ActionKey::CreateField, 1250),
        ActionConfig::new(3, ActionKey::Capture, 800),
        ActionConfig::new(4, ActionKey::DestroyField, 750),
        ActionConfig::new(5, ActionKey::Deploy8th, 375),
        ActionConfig::new(6, ActionKey::CreateLink, 313),
        ActionConfig::new(7, ActionKey::DestroyLink, 187),
        ActionConfig::new(8, ActionKey::DeployMod, 125),
        ActionConfig::new(9, ActionKey::HackEnemy, 100),
        ActionConfig::new(10, ActionKey::DestroyRes, 75),
        ActionConfig::new(11, ActionKey::UpgradeRes, 65),
        ActionConfig::new(12, ActionKey::DeployRes, 125),
    ];
    
    // 默认关闭摧毁 Field 和 Link
    if let Some(c) = configs.iter_mut().find(|c| c.key == ActionKey::DestroyField) {
        c.enabled = false;
    }
    if let Some(c) = configs.iter_mut().find(|c| c.key == ActionKey::DestroyLink) {
        c.enabled = false;
    }
    
    configs
}
