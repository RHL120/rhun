use std::collections::HashMap;
pub struct ConfigItem {
    /// An array of paths to binaries that the user can execute without entering
    /// a password, if None, all commands will be no_pass.
    pub no_pass: Option<&'static [&'static str]>,
    /// An array of paths to binaries that the user can execute by entering
    /// a password, if None, all commands will be pass. No pass is checked before
    /// pass
    pub pass: Option<&'static [&'static str]>
}

/// Creates the configuration. It should return a hashmap between the username and
/// the user's configuration item.
#[inline]
pub fn create_config() -> HashMap<&'static str, ConfigItem> {
    HashMap::from([
                  ("rhl120", ConfigItem {
                        no_pass: Some(&["/bin/poweroff"]),
                        pass: None,
                  })
    ])
}

// cut here for user config
