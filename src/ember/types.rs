pub type ConfigTxPowerMod = u16;// TOOD: enum of EZSP_CONFIG_TX_POWER_MODE
/// Return type for stack functions.
pub type Status = u8;
/// Either marks an event as inactive or specifies the units for the event execution time.
pub type EventUnits = u8;
/// The type of the node.
pub type NodeType = u8;
/// The possible join states for a node.
pub type NetworkStatus = u8;
/// Incoming message types.
pub type IncomingMessageType = u8;
/// Outgoing message types.
pub type OutgoingMessageType = u8;
/// MAC passthrough message type flags.
pub type MacPassthroughType  = u8;
/// Binding types.
pub type BindingType = u8;
/// Options to use when sending a message.
pub type ApsOption = u16;
/// Decision made by the trust center when a node attempts to join.
pub type JoinDecision = u8;
/// This is the Initial Security Bitmask that controls
/// the use of various security features.
pub type InitialSecurityBitmask = u16;
/// This is the Current Security Bitmask that details
/// the use of various security features.
pub type CurrentSecurityBitmask = u16;
/// Describes the type of ZigBee security key.
pub type KeyType = u8;
/// Describes the presence of valid data within the EmberKeyStruct structure.
pub type KeyStructBitmask = u16;
/// The status of the device update.
pub type DeviceUpdate = u8;
/// The status of the attempt to establish a key.
pub type KeyStatus  = u8;
/// Defines the events reported to the application by the readAndClearCounters command.
pub type CounterType = u8;
/// The type of method used for joining.
pub type JoinMethod = u8;
/// Flags for controlling which incoming ZDO requests are passed to the application.
pub type ZdoConfigurationFlags= u8;
/// Type of concentrator.
pub type ConcentratorType = u16;
/// ZLL device state identifier.
pub type ZLLState = u16;
/// ZLL key encryption algorithm enumeration.
pub type ZLLKeyIndex = u8;
/// Bitmask options for emberNetworkInit().
pub type NetworkInitBitmask = u16;
/// Network configuration for the desired radio interface for multi-phy network.
pub type MultiPhyNwkConfig = u8;
/// Duty cycle states.
pub type DutyCycleState = u8;
/// Radio power modes.
pub type RadioPowerMode = u8;
/// Entropy sources.
pub type EntropySource = u8;
// TODO: sigbee security manager types?
pub type NodeId = u16;
/// Zigbee 802.15.4 network PAN ID.
pub type PanId = u16;
/// A 16-bit ZigBee multicast group identifier.
pub type MulticastId = u16;
/// An EUI 64-bit ID (an IEEE address).
pub type EUI64  = [u8; 8];
/// The percent of duty cycle for a limit.
/// 
/// Duty Cycle, Limits, and Thresholds are
/// reported in units of Percent * 100.
/// 
/// i.e. 10000 = 100.00%, 1 = 0.01%
pub type DutyCycleHectoPct = u16;
/// A library identifier.
pub type LibraryId  = u8;
/// The presence and status of the Ember library.
pub type LibraryStatus = u8;
/// The security level of the GPD.
pub type GpSecurityLevel  = u8;
/// The type of security key to use for the GPD.
pub type GpKeyType = u8;
/// The GPD proxy table status entry.
pub type GpProxyTableEntryStatus  = u8;
/// The GPD security frame counter.
pub type GpSecurityFrameCounter = u32;
/// The GPD sink table entry status.
pub type GpSinkTableEntryStatus = u8;