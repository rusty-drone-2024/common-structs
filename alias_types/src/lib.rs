#![warn(clippy::pedantic)]
use wg_2024::network::SourceRoutingHeader;

/// Type alias for FragmentIndex
pub type FragmentIdx = u64;
/// Type alias for SessionId
pub type Session = u64;
/// Type alias for SourceRoutingHeader
pub type Routing = SourceRoutingHeader;
