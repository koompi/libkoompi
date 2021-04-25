pub mod background;
pub mod desktop_item;
mod constants;
pub mod desktop_manager;
pub mod configs;
pub mod errors;

pub use desktop_item::DesktopItem;
pub use background::WallpaperItem;
pub use configs::PersistentData;
pub use errors::DesktopError;
pub use desktop_manager::DesktopManager;
