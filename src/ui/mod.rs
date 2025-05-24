//! User interface components for Monarch

pub mod status_view;
pub mod command_view;
pub mod settings_view;

/// UI initialization and configuration
pub mod app {
    use anyhow::Result;

    /// Available UI backends
    pub enum UiBackend {
        /// egui-based UI
        Egui,
        /// iced-based UI
        Iced,
        /// Tauri-based UI
        Tauri,
        /// Terminal UI
        Terminal,
    }

    /// UI application options
    pub struct UiOptions {
        /// UI backend to use
        pub backend: UiBackend,
        /// Window title
        pub title: String,
        /// Window width
        pub width: u32,
        /// Window height
        pub height: u32,
        /// Whether to use dark mode
        pub dark_mode: bool,
    }

    impl Default for UiOptions {
        fn default() -> Self {
            Self {
                backend: UiBackend::Egui,
                title: "Monarch: Advanced Git Management Suite".to_string(),
                width: 1280,
                height: 800,
                dark_mode: true,
            }
        }
    }

    /// Initialize the UI
    pub fn initialize(_options: UiOptions) -> Result<()> {
        // Implementation will depend on which UI backend is selected at compile time
        todo!("Implement UI initialization")
    }

    /// Start the UI main loop
    pub fn run() -> Result<()> {
        todo!("Implement UI run loop")
    }
}

#[cfg(feature = "egui-ui")]
pub mod egui_impl;

#[cfg(feature = "iced-ui")]
pub mod iced_impl;

#[cfg(feature = "tauri-ui")]
pub mod tauri_impl;