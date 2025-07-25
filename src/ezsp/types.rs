/// Identifies a configuration value.
pub type ConfigID = u8;
/// Identifies a value.
pub type ValueID = u8;
/// Identifies a value based on specified characteristics.
pub type ExtendedValueID = u8;
/// Flags associated with the endpoint data configured on the NCP.
pub type EndpointFlags = u16; // TODO: enum
/// Identifies a policy.
pub type PolicyID = u8;
/// The policy decision bitmask that controls the trust center decision strategies.
pub type DecisionBitmask = u16;
/// Identifies a policy decision.
pub type DecisionId = u8;
/// Manufacturing token ID.
pub type MfgTokenId = u8;
/// Status values used by EZSP.
pub type Status = u8;
/// Network scan types.
pub type NetworkScanType = u8;
/// Differentiates among ZLL network operations.
pub type ZllNetworkOperation = u8;
/// Validates Source Route Overhead Information cached.
pub type SourceRouteOverheadInformation = u8;

