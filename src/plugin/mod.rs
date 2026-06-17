// Open CAD Studio plugin runtime. Plugins are external cdylibs loaded from the
// user plugins folder (see `external`) and installed via the marketplace; the
// host ships no built-in add-ons. See `docs/plugin-architecture.md`.

pub mod external;
pub mod host;
pub mod marketplace;
pub mod registry;

pub use registry::{all_ribbon_modules, ribbon_modules_enabled};
pub(crate) use registry::try_dispatch;