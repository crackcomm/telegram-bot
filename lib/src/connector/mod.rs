//! IO backend.

mod _base;

#[cfg(feature = "hyper_connector")]
pub mod hyper;

pub use self::_base::Connector;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::HyperConnector;

use crate::errors::Error;

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(feature = "hyper_connector")]
pub fn default_connector() -> Result<hyper::DefaultConnector, Error> {
    hyper::default_connector()
}
