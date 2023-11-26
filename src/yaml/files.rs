pub struct TargetFiles {
    left: serde_yaml::Value,
    right: serde_yaml::Value,
}

impl TargetFiles {
    
    pub fn new(left: serde_yaml::Value, right: serde_yaml::Value) -> Self {
        TargetFiles { left , right }
    }
}
