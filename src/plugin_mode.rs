/// Determines whether a plugin runs in standard (with rendering) or headless mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginMode {
    /// Standard mode with all rendering systems
    Standard,
    /// Headless mode - logic only, no sprite/visual components
    Headless,
}

impl Default for PluginMode {
    fn default() -> Self {
        Self::Standard
    }
}
