use crate::ember::types;

pub struct NetworkParameters  {
    /// The network's extended PAN identifier.
    extended_pan_id: [u8; 8],

    /// The network's PAN identifier.
    pan_id: u16,

    /// A power setting, in dBm.
    radio_tx_power: u8,

    /// A radio channel.
    radio_channel: u8,

    /// The method used to initially join the network.
    join_method: types::JoinMethod,

    /// The ID of the network manager in the current network.
    /// 
    /// This may only be set at joining when using
    /// EMBER_USE_CONFIGURED_NWK_STATE as the join method.
    network_manager_id: types::NodeId,

    /// This is used to determine the newest instance of the
    /// network after a PAN ID or channel change.
    /// 
    /// This may only be set at joining when using
    /// EMBER_USE_CONFIGURED_NWK_STATE as the join method.
    network_update_id: u8,

    /// The network channel mask.
    /// 
    /// The list of preferred channels that the NWK
    /// manager has told this device to use when
    /// searching for the network.
    /// 
    /// This may only be set at joining when using
    /// EMBER_USE_CONFIGURED_NWK_STATE as the join
    /// method.
    channels: u32
}

pub struct MultiPhyRadioParameters {
    /// A power setting, in dBm.
    radio_tx_power: i8,

    /// A radio page.
    radio_page: u8,

    /// A radio channel.
    radio_channel: u8,
}

pub struct ZigbeeNetwork  {
    /// The 802.15.4 channel associated with the network.
    channel: u8,

    /// The network's PAN identifier.
    pan_id: u16,

    /// The network's extended PAN identifier.
    extended_pan_id: [u8;8],
    
    /// Whether the network is allowing MAC associations.
    allowing_join: bool,

    /// The Stack Profile associated with the network.
    stack_profile: u8,

    /// The instance of the Network.
    network_update_id: u8,
}

pub struct ApsFrame  {
    /// The application profile ID that
    /// describes the format of the message.
    profile_id: u16,

    /// The cluster ID for this message.
    cluster_id: u16,

    /// The source endpoint.
    source_endpoint: u8,

    /// The destination endpoint.
    destination_endpoint: u8,

    /// A bitmask of options.
    options: types::ApsOption,

    /// The group ID for this message, if it is multicast mode.
    group_id: u16,

    /// The sequence number.
    sequence: u8,
}

pub struct BindingTableEntry {
    /// The type of binding.
    binding_type: types::BindingType,

    /// The endpoint on the local node.
    local: u8,

    /// A cluster ID that matches one from the local endpoint's simple descriptor. 
    cluster_id: u16,

    /// The endpoint on the remote node (specified by identifier).
    remote: u8,

    /// A 64-bit identifier. This is either the destination EUI64
    /// (for unicasts) or the 64-bit group address (for multicasts).
    identifier: types::EUI64,

    /// The index of the network the binding belongs to.
    network_index: u8,
}

pub struct MulticastTableEntry  {
    /// The multicast group ID.
    multicast_id: types::MulticastId,

    /// The endpoint that is a member, or 0 if this entry is not
    /// in use (the ZDO is not a member of any multicast groups.)
    endpoint: u8,

    /// The network index of the network the entry is related to.
    network_index: u8,
}

/// A 128-bit key.
pub struct KeyData([u8; 16]);

/// The implicit certificate used in CBKE.
pub struct CertificateData ([u8;48]);

/// The public key data used in CBKE.
pub struct PublicKeyData  ([u8; 22]);

/// The private key data used in CBKE.
pub struct PrivateKeyData ([u8; 21]);

/// The Shared Message Authentication Code data used in CBKE.
pub struct SMACData ([u8;16]);

/// An ECDSA signature
pub struct SignatureData ([u8;42]);

/// The implicit certificate used in CBKE.
pub struct Certificate283k1Data ([u8; 74]);

/// The public key data used in CBKE.
pub struct PublicKey283k1Data ([u8; 37]);

/// The private key data used in CBKE.
pub struct PrivateKey283k1Data ([u8; 36]);

/// An ECDSA signature
pub struct Signature283k1Data ([u8; 72]);

/// The calculated digest of a message
pub struct MessageDigest  ([u8; 16]);

/// The hash context for an ongoing hash operation.
pub struct AesMmoHashContext {
    /// The result of ongoing the hash operation.
    result: [u8; 16],

    /// The total length of the data that has been hashed so far.
    length: u32
}

/// Beacon data structure.
pub struct BeaconData  {
    /// The channel of the received beacon.
    channel: u8,

    /// The LQI of the received beacon.
    lqi : u8,

    /// The RSSI of the received beacon.
    rssi: i8,

    /// The depth of the received beacon.
    depth: u8,

    /// The network update ID of the received beacon.
    network_update_id: u8,

    /// The power level of the received beacon.
    /// 
    /// This field is valid only if the beacon is an enhanced beacon.
    power: i8,

    /// The TC connectivity and long uptime from capacity field.
    parent_priority: i8,

    /// The PAN ID of the received beacon.
    pan_id: types::PanId,

    /// The extended PAN ID of the received beacon.
    extended_pan_id: [u8; 8],

    /// The sender of the received beacon.
    sender: types::NodeId,

    /// Whether or not the beacon is enhanced.
    enhanced: bool,

    /// Whether the beacon is advertising permit join.
    permit_join: bool,

    /// Whether the beacon is advertising capacity.
    has_capacity: bool,
}

/// Defines an iterator that is used to loop over cached beacons.
/// 
/// Do not write to fields denoted as Private.
pub struct BeaconIterator {
    /// The retrieved beacon.
    beacon: BeaconData,

    /// (Private) The index of the retrieved beacon.
    index: u8
}

/// Parameters related to beacon prioritization.
pub struct BeaconClassificationParams {
    /// The minimum RSSI value for receiving packets that
    /// is used in some beacon prioritization algorithms.
    min_rssi_for_receiving_pkts: i8,

    /// The beacon classification mask that identifies which
    /// beacon prioritization algorithm to pick and defines
    /// the relevant parameters.
    beacon_classification_mask: u16,
}

/// A neighbor table entry stores information about the
/// reliability of RF links to and from neighboring nodes.
pub struct NeighborTableEntry {
    /// The neighbor's two-byte network id
    short_id: u16,

    /// An exponentially weighted moving average of the
    /// link quality values of incoming packets from this
    /// neighbor as reported by the PHY.
    average_lqi: u8,

    /// The incoming cost for this neighbor, computed from
    /// the average LQI. Values range from 1 for a good link
    /// to 7 for a bad link.
    in_cost: u8,

    /// The outgoing cost for this neighbor, obtained from
    /// the most recently received neighbor exchange
    /// message from the neighbor. A value of zero means
    /// that a neighbor exchange message from the
    /// neighbor has not been received recently enough, or
    /// that our id was not present in the most recently
    /// received one.
    out_cost: u8,

    /// The number of aging periods elapsed since a link
    /// status message was last received from this
    /// neighbor. The aging period is 16 seconds.
    age: u8,

    /// The 8-byte EUI64 of the neighbor.
    long_id: types::EUI64
}

/// A route table entry stores information about the next
/// hop along the route to the destination.
pub struct RouteTableEntry {
    /// The short id of the destination.
    /// 
    /// A value of 0xFFFF indicates the entry is unused.
    destination: u16,

    /// The short id of the next hop to this destination.
    next_hop: u16,

    /// Indicates whether this entry is active (0), being
    /// discovered (1), unused (3), or validating (4).
    status: u8,

    /// The number of seconds since this route entry was
    /// last used to send a packet.
    age: u8,

    /// Indicates whether this destination is a High RAM
    /// Concentrator (2), a Low RAM Concentrator (1), or
    /// not a concentrator (0).
    concentrator_type: u8,

    /// For a High RAM Concentrator, indicates whether a
    /// route record is needed (2), has been sent (1), or is
    /// no long needed (0) because a source routed
    /// message from the concentrator has been received.
    route_record_state: u8,
}

/// The security data used to set the configuration for
/// the stack, or the retrieved configuration currently in
/// use.
pub struct InitialSecurityState {
    /// A bitmask indicating the security state used to
    /// indicate what the security configuration will be when
    /// the device forms or joins the network.
    bitmask : types::InitialSecurityBitmask,

    /// The pre-configured Key data that should be used
    /// when forming or joining the network. The security
    /// bitmask must be set with the
    /// EMBER_HAVE_PRECONFIGURED_KEY bit to
    /// indicate that the key contains valid data.
    preconfigured_key: KeyData,

    /// The Network Key that should be used by the Trust
    /// Center when it forms the network, or the Network
    /// Key currently in use by a joined device.
    /// 
    /// The security bitmask must be set with
    /// EMBER_HAVE_NETWORK_KEY to indicate that
    /// the key contains valid data.
    network_key: KeyData,

    /// The sequence number associated with the network key.
    /// 
    /// This is only valid if the EMBER_HAVE_NETWORK_KEY has
    /// been set in the security bitmask.
    network_key_sequence_number: u8,

    /// This is the long address of the trust center on the
    /// network that will be joined.
    /// 
    /// It is usually NOT set prior to joining the network
    /// and instead it is learned during the joining message
    /// exchange. This field is only examined if
    /// EMBER_HAVE_TRUST_CENTER_EUI64 is set in the
    /// EmberInitialSecurityState::bitmask.
    /// 
    /// Most devices should clear that bit and leave this field alone.
    /// 
    /// This field must be set when using commissioning mode.
    preconfigured_trust_center_eui: types::EUI64
} 

/// The security options and information currently used by the stack.
pub struct CurrentSecurityState  {
    /// A bitmask indicating the security options currently in
    /// use by a device joined in the network.
    bitmask: types::CurrentSecurityBitmask,

    /// The IEEE Address of the Trust Center device.
    trustCenterLongAddress: types::EUI64,
}

/// A structure containing a key and its associated data.
pub struct KeyStruct {
    /// A bitmask indicating the presence of data within the 
    /// various fields in the structure.
    bitmask : types::KeyStructBitmask,

    /// The type of the key.
    key_type: types::KeyType,

    /// The actual key data.
    key: KeyData,

    /// The outgoing frame counter associated with the key.
    outgoing_frame_counter: u32,

    /// The frame counter of the partner device associated with the key.
    incoming_frame_counter: u32,

    /// The sequence number associated with the key.
    sequence_number: u8,

    /// The IEEE address of the partner device also in
    /// possession of the key.
    partner_eui64: types::EUI64
}

/// Network Initialization parameters.
pub struct NetworkInitStruct {
    /// Configuration options for network init.
    bitmask: types::NetworkInitBitmask,
}

/// Data associated with the ZLL security algorithm.
pub struct ZllSecurityAlgorithmData {
    /// Transaction identifier.
    transaction_id: u32,

    /// Response identifier.
    response_id: u32,

    /// Bitmask.
    bitmask: u16,
}

/// The parameters of a ZLL network.
pub struct ZllNetwork  {
    /// The parameters of a ZigBee network.
    zigbee_network: ZigbeeNetwork,

    /// Data associated with the ZLL security algorithm.
    security_algorithm: ZllSecurityAlgorithmData,

    /// Associated EUI64.
    eui64: types::EUI64,

    /// The node id.
    node_id: types::NodeId,

    /// The ZLL state.
    state: types::ZLLState,

    /// The node type.
    node_type : types::NodeType,

    /// The number of sub devices.
    number_sub_devices: u8,

    /// The total number of group identifiers.
    total_group_identifiers: u8,

    /// RSSI correction value.
    rssi_correction: u8,
}

/// Describes the initial security features and
/// requirements that will be used when forming or
/// joining ZLL networks.
pub struct ZllInitialSecurityState {
    /// Unused bitmask; reserved for future use.
    bitmask: u32,

    /// The key encryption algorithm advertised by the
    /// application.
    key_index: types::ZLLKeyIndex,

    /// The encryption key for use by algorithms that
    /// require it.
    encryption_key: KeyData,

    /// The pre-configured link key used during classical
    // ZigBee commissioning.
    preconfigured_key: KeyData,
}

/// Information about a specific ZLL Device.
pub struct ZllDeviceInfoRecord {
    /// EUI64 associated with the device.
    ieee_address: types::EUI64,
    
    /// Endpoint id.
    endpoint_id: u8,
    
    /// Profile id.
    profile_id: u16,

    /// Device id.
    device_id: u16,

    /// Associated version.
    version: u8,

    /// Number of relevant group ids.
    group_id_count: u8,
}

/// ZLL address assignment data.
pub struct ZllAddressAssignment {
    /// Relevant node id.
    node_id : types::NodeId,

    /// Minimum free node id.
    free_node_id_min: types::NodeId,

    /// Maximum free node id.
    free_node_id_max: types::NodeId,

    /// Minimum group id.
    group_id_min: types::MulticastId,

    /// Maximum group id.
    group_id_max: types::MulticastId,

    /// Minimum free group id.
    free_group_id_min: types::MulticastId,

    /// Maximum free group id.
    free_group_id_max: types::MulticastId,
}

/// Public API for ZLL stack data token.
pub struct TokTypeStackZllData {
    /// Token bitmask.
    bitmask: u32,

    /// Minimum free node id.
    free_node_id_min: u16,

    /// Maximum free node id.
    free_node_id_max: u16,

    /// Local minimum group id.
    my_group_id_min: u16,

    /// Minimum free group id.
    free_group_id_min: u16,

    /// Maximum free group id.
    free_group_id_max: u16,

    /// RSSI correction value.
    rssi_correction: u8,
}

/// Public API for ZLL stack security token.
pub struct TokTypeStackZllSecurity {
    /// Token bitmask.
    bitmask: u32,

    /// Key index.
    key_index: u8,

    /// Encryption key.
    encryption_key: [u8; 16],

    /// Preconfigured key.
    preconfigured_key: [u8; 16],
}

/// A structure containing duty cycle limit configurations.
/// 
/// All limits are absolute, and are required to be as follows:
///  suspLimit > critThresh > limitThresh For
/// example:
///  suspLimit = 250 (2.5%), critThresh = 180 (1.8%), limitThresh 100 (1.00%).
pub struct DutyCycleLimits {
    /// The vendor identifier of the node.
    vendor_id: u16,

    /// The vendor string of the node.
    vendor_string: [u8; 7]
}

/// A structure containing per device overall duty
/// cycle consumed (up to the suspend limit).
pub struct PerDeviceDutyCycle {
    /// Node Id of device whose duty cycle is reported.
    node_id: types::NodeId,

    /// Amount of overall duty cycle consumed (up to suspend limit).
    duty_cycle_consumed: types::DutyCycleHectoPct,
}

/// The transient key data structure.
pub struct TransientKeyData {
    /// The IEEE address paired with the transient link key.
    eui64: types::EUI64,

    /// The key data structure matching the transient key.
    key_data: KeyData,

    /// This bitmask indicates whether various fields
    /// in the structure contain valid data.
    bitmask: types::KeyStructBitmask,

    /// The number of seconds remaining before the key is
    /// automatically timed out of the transient key table.
    remaining_time_seconds: u16,

    /// The network index indicates which NWK uses this key.
    network_index: u8,
}

/// A structure containing a child node's data.
pub struct ChildData {
    /// The EUI64 of the child.
    eui64: types::EUI64,

    /// The node type of the child.
    node_type : types::NodeType,

    /// The short address of the child.
    id: types::NodeId,

    /// The phy of the child.
    phy: u8,

    /// The power of the child.
    power: u8,

    /// The timeout of the child.
    timeout: u8,
    
    /// The GPD's EUI64.
    gpd_ieee_address: types::EUI64,

    /// The GPD's source ID.
    source_id: u32,

    /// The GPD Application ID.
    application_id: u8,

    /// The GPD endpoint.
    endpoint: u8,
}

// TODO: sl_zb_sec_man_key_t and related

/// A GP address structure.
pub struct GpAddress {
    /// Contains either a 4-byte source ID or an 8-byte
    /// IEEE address, as indicated by the value of the  
    /// application_id field.
    id: [u8; 8],

    /// The GPD Application ID specifying either source ID
    // (0x00) or IEEE address (0x02).
    application_id: u8, // TODO: convert to enum

    /// The GPD endpoint.
    endpoint: u8,
}

/// The internal representation of a proxy table entry
pub struct GpProxyTableEntry {
    /// Internal status of the proxy table entry.
    status: types::GpProxyTableEntryStatus,

    /// The tunneling options (this contains both options
    /// and extendedOptions from the spec).
    options: u32,

    /// The addressing info of the GPD.
    gpd: GpAddress,
    
    /// The assigned alias for the GPD.
    assigned_alias: types::NodeId,

    /// The security options field.
    security_options: u8,

    /// The security frame counter of the GPD.
    gpd_security_frame_counter: types::GpSecurityFrameCounter,

    /// The key to use for GPD.
    gpd_key: KeyData,

    /// The list of sinks (hardcoded to 2 which is the spec minimum).
    sink_list: [types::GpSinkListEntry; GP_SINK_LIST_ENTRIES],

    /// The groupcast radius.
    groupcast_radius: u8,

    /// The search counter.
    search_counter: u8,
}

/// The internal representation of a sink table entry.
pub struct GpSinkTableEntry {
    /// Internal status of the sink table entry.
    status: types::GpSinkTableEntryStatus,

    /// The tunneling options (this contains both options
    /// and extendedOptions from the spec).
    options: u32,

    /// The addressing info of the GPD.
    gpd: GpAddress,

    /// The device id for the GPD.
    device_id: u8,

    /// The list of sinks (hardcoded to 2 which is the spec minimum).
    sink_list: [GpSinkListEntry; GP_SINK_LIST_ENTRIES],

    /// The assigned alias for the GPD.
    assigned_alias: types::NodeId,

    /// The groupcast radius.
    groupcast_radius: u8,

    /// The security options field.
    security_options: u8,

    /// The security frame counter of the GPD.
    gpd_security_frame_counter: types::GpSecurityFrameCounter,

    /// The key to use for GPD.
    gpd_key: KeyData,
}

/// Information of a token in the token table.
pub struct TokenInfo {
    /// NVM3 key of the token
    nvm3_key: u32,

    /// Token is a counter type
    is_counter: bool,

    /// Token is an indexed token
    is_index: bool,

    /// Size of the token
    size: u8,

    /// Array size of the token
    array_size: u8,
}

/// Token Data.
pub struct TokenData {
    /// Token data size in bytes.
    size: u32,
    
    /// Token data pointer.
    data: [u8; 64]
}
