/// Identifies a configuration value.
#[repr(u8)]
pub enum ConfigID {
    /// The NCP no longer supports configuration of packet
    /// buffer count at runtime using this parameter.
    /// 
    /// Packet buffers must be configured using the
    /// EMBER_PACKET_BUFFER_COUNT macro when
    /// building the NCP project.
    /// 
    /// EZSP_CONFIG_PACKET_BUFFER_COUNT 
    PacketBuffercount = 0x01,
    /// The maximum number of router neighbors the stack can
    /// keep track of. A neighbor is a node within radio range.
    /// 
    /// EZSP_CONFIG_NEIGHBOR_TABLE_SIZE 
    NeighborTableSize = 0x02,

    /// The maximum number of APS retried messages the
    /// stack can be transmitting at any time.
    /// 
    /// EZSP_CONFIG_APS_UNICAST_MESSAGE_COUNT 
    APSUnicastMessageCount = 0x03,

    /// The maximum number of non-volatile bindings
    /// supported by the stack.
    /// 
    /// EZSP_CONFIG_BINDING_TABLE_SIZE 
    BindingTableSize = 0x04,

    /// The maximum number of EUI64 to network address
    /// associations that the stack can maintain for the
    /// application.
    /// 
    /// (Note, the total number of such address associations
    /// maintained by the NCP is the sum of the value of this
    /// setting and the value of
    /// EZSP_CONFIG_TRUST_CENTER_ADDRESS_CACHE_SIZE.).
    /// 
    /// 
    /// EZSP_CONFIG_ADDRESS_TABLE_SIZE 
    AddressTableSize = 0x05,

    /// The maximum number of multicast groups that the
    /// device may be a member of.
    /// 
    /// EZSP_CONFIG_MULTICAST_TABLE_SIZE 
    MulticastTableSize = 0x06,

    /// The maximum number of destinations to which a node
    /// can route messages. This includes both messages
    /// originating at this node and those relayed for others.
    /// 
    /// EZSP_CONFIG_ROUTE_TABLE_SIZE 
    RouteTableSize = 0x07,

    /// The number of simultaneous route discoveries that a
    /// node will support.
    /// 
    /// EZSP_CONFIG_DISCOVERY_TABLE_SIZE 
    DiscoveryTableSize = 0x08,

    /// Specifies the stack profile.
    /// 
    /// EZSP_CONFIG_STACK_PROFILE 
    StackProfile = 0x0C,

    /// The security level used for security at the MAC and
    ///  network layers.
    /// 
    /// The supported values are 0 (no security) and 5 (payload
    /// is encrypted and a four-byte MIC is used for authentication).
    /// 
    /// EZSP_CONFIG_SECURITY_LEVEL 
    SecurityLevel = 0x0D,

    /// The maximum number of hops for a message.
    /// 
    /// EZSP_CONFIG_MAX_HOPS 
    MaxHops = 0x10,

    /// The maximum number of end device children that a router will support.
    /// 
    /// EZSP_CONFIG_MAX_END_DEVICE_CHILDREN 
    MaxEndDeviceChildren = 0x11,

    /// The maximum amount of time that the MAC will hold a
    /// message for indirect transmission to a child.
    /// 
    /// EZSP_CONFIG_INDIRECT_TRANSMISSION_TIMEOUT 
    IndirectTransmissionTimeout = 0x12,

    /// The maximum amount of time that an end device child
    /// can wait between polls.
    /// 
    /// If no poll is heard within this timeout, then the parent
    /// removes the end device from its tables.
    /// 
    /// Value range 0-14.
    /// 
    /// The timeout corresponding to a value of zero is 10 seconds.
    /// The timeout corresponding to a nonzero value N is 2^N minutes,
    /// ranging from 2^1 = 2 minutes to 2^14 = 16384 minutes.
    /// 
    /// EZSP_CONFIG_END_DEVICE_POLL_TIMEOUT 
    EndDevicePollTimeout = 0x13,

    /// Enables boost power mode and/or the alternate transmitter output.
    /// 
    /// EZSP_CONFIG_TX_POWER_MODE 
    TXPowerMode = 0x17,

    /// 0: Allow this node to relay messages.
    /// 1: Prevent this node from relaying messages.
    /// 
    /// EZSP_CONFIG_DISABLE_RELAY 
    DisableRelay = 0x18,

    /// The maximum number of EUI64 to network address
    /// associations that the Trust Center can maintain.
    /// 
    /// These address cache entries are reserved for and
    /// reused by the Trust Center when processing device
    /// join/rejoin authentications.
    /// 
    /// This cache size limits the number of overlapping
    /// joins the Trust Center can process within a narrow
    /// time window (e.g. two seconds), and thus should be
    /// set to the maximum number of near simultaneous joins
    /// the Trust Center is expected to accommodate.
    /// 
    /// (Note, the total number of such address associations
    /// maintained by the NCP is the sum of the value of this
    /// setting and the value of EZSP_CONFIG_ADDRESS_TABLE_SIZE)
    /// 
    /// EZSP_CONFIG_TRUST_CENTER_ADDRESS_CACHE_SIZE 
    TrustCenterAddressCacheSize = 0x19,

    /// The size of the source route table.
    /// 
    /// EZSP_CONFIG_SOURCE_ROUTE_TABLE_SIZE 
    SourceRouteTableSize = 0x1A,

    /// The number of blocks of a fragmented message that
    /// can be sent in a single window.
    /// 
    /// EZSP_CONFIG_FRAGMENT_WINDOW_SIZE 
    FragmentWindowSize = 0x1C,

    /// The time the stack will wait (in milliseconds) between
    /// sending blocks of a fragmented message.
    /// 
    /// EZSP_CONFIG_FRAGMENT_DELAY_MS 
    FragmentDelayMS = 0x1D,

    /// The size of the Key Table used for storing individual link
    /// keys (if the device is a Trust Center) or Application Link
    /// Keys (if the device is a normal node).
    /// 
    /// EZSP_CONFIG_KEY_TABLE_SIZE 
    KeyTableSize = 0x1E,

    /// The APS ACK timeout value. The stack waits this
    /// amount of time between resends of APS retried
    /// messages.
    /// 
    /// EZSP_CONFIG_APS_ACK_TIMEOUT 
    APSACKTimeout = 0x1F,

    /// The duration of a beacon jitter, in the units used by the
    /// 15.4 scan parameter (((1 << duration) + 1) * 15ms),
    /// when responding to a beacon request.
    /// 
    /// EZSP_CONFIG_BEACON_JITTER_DURATION 
    BeaconJitterDuration = 0x20,

    /// The number of PAN id conflict reports that must be
    /// received by the network manager within one minute to
    /// trigger a PAN id change.
    /// 
    /// EZSP_CONFIG_PAN_ID_CONFLICT_REPORT_THRESHOLD 
    PANIDConflictReportThreshold = 0x22,

    /// The timeout value in minutes for how long the Trust
    /// Center or a normal node waits for the ZigBee Request
    /// Key to complete.
    /// 
    /// On the Trust Center this controls whether or not the
    /// device buffers the request, waiting for a matching pair
    /// of ZigBee Request Key.
    /// 
    /// If the value is non-zero, the Trust Center buffers and
    /// waits for that amount of time.
    /// 
    /// If the value is zero, the Trust Center does not buffer
    /// the request and immediately responds to the request.
    /// 
    /// Zero is the most compliant behavior.
    /// 
    /// EZSP_CONFIG_REQUEST_KEY_TIMEOUT 
    RequestKeyTimeout = 0x24,

    /// This value indicates the size of the runtime modifiable
    /// certificate table.
    /// 
    /// Normally certificates are stored in MFG tokens but this
    /// table can be used to field upgrade devices with new Smart
    /// Energy certificates.
    /// 
    /// This value cannot be set, it can only be queried.
    /// 
    /// EZSP_CONFIG_CERTIFICATE_TABLE_SIZE 
    CertificateTableSize = 0x29,

    /// This is a bitmask that controls which incoming ZDO
    /// request messages are passed to the application.
    /// 
    /// The bits are defined in the EmberZdoConfigurationFlags
    /// enumeration.
    /// 
    /// To see if the application is required to send a ZDO
    /// response in reply to an incoming message, the application
    /// must check the APS options bitfield within the
    /// incomingMessageHandler callback to see if the
    /// EMBER_APS_OPTION_ZDO_RESPONSE_REQUIRED flag is set.
    /// 
    /// EZSP_CONFIG_APPLICATION_ZDO_FLAGS 
    ApplicationZDOFlags = 0x2A,

    /// The maximum number of broadcasts during a single
    /// broadcast timeout period.
    /// 
    /// EZSP_CONFIG_BROADCAST_TABLE_SIZE 
    BroadcastTableSize = 0x2B,

    /// The size of the MAC filter list table.
    /// 
    /// EZSP_CONFIG_MAC_FILTER_TABLE_SIZE 
    MACFilterTableSize = 0x2C,

    /// The number of supported networks.
    /// 
    /// EZSP_CONFIG_SUPPORTED_NETWORKS 
    SupporedNetworks = 0x2D,

    /// Whether multicasts are sent to the RxOnWhenIdle=true
    /// address (0xFFFD) or the sleepy broadcast address (0xFFFF).
    /// 
    /// The RxOnWhenIdle=true address is the
    /// ZigBee compliant destination for multicasts.
    /// 
    /// EZSP_CONFIG_SEND_MULTICASTS_TO_SLEEPY_ADDRESS
    SendMulticastsToSleepyAddress = 0x2E,

    /// ZLL group address initial configuration.
    /// 
    /// EZSP_CONFIG_ZLL_GROUP_ADDRESSES 
    ZLLGroupAddresses = 0x2F,

    /// ZLL rssi threshold initial configuration.
    /// 
    /// EZSP_CONFIG_ZLL_RSSI_THRESHOLD 
    ZLLRSSIThreshold = 0x30,

    /// Toggles the MTORR flow control in the stack.
    /// 
    /// EZSP_CONFIG_MTORR_FLOW_CONTROL 
    MTORRFlowControl = 0x33,

    /// Setting the retry queue size. Applies to all queues.
    /// 
    /// Default value in the sample applications is 16.
    /// 
    /// EZSP_CONFIG_RETRY_QUEUE_SIZE 
    RetryQueueSize = 0x34,

    /// Setting the new broadcast entry threshold.
    /// 
    /// The number(BROADCAST_TABLE_SIZE - NEW_BROADCAST_ENTRY_THRESHOLD)
    /// of broadcast table entries are reserved for relaying the broadcast
    /// messages originated on other devices.
    /// 
    /// The local device will fail to originate a broadcast message after
    /// this threshold is reached. Setting this value to BROADCAST_TABLE_SIZE
    /// and greater will effectively kill this limitation.
    /// 
    /// EZSP_CONFIG_NEW_BROADCAST_ENTRY_THRESHOLD 
    NewBroadcastEntryThreshold = 0x35,

    /// The length of time, in seconds, that a trust center will
    /// store a transient link key that a device can use to join its
    /// network.
    /// 
    /// A transient key is added with a call to emberAddTransientLinkKey.
    /// 
    /// After the transient key is added, it will be removed once this
    /// amount of time has passed.
    /// 
    /// A joining device will not be able to use that key to join until it
    /// is added again on the trust center.
    /// 
    /// The default value is 300 seconds, i.e., 5 minutes.
    /// 
    /// (Deprecated) EZSP_CONFIG_TRANSIENT_KEY_TIMEOUT_S 
    #[deprecated]
    TransientKeyTimeoutS = 0x36,

    /// The number of passive acknowledgements to record
    /// from neighbors before we stop re-transmitting
    /// broadcasts.
    /// 
    /// EZSP_CONFIG_BROADCAST_MIN_ACKS_NEEDED 
    BroadcastMinACKsNeeded = 0x37,

    /// The length of time, in seconds, that a trust center will
    /// allow a Trust Center (insecure) rejoin for a device that is
    /// using the well-known link key.
    /// 
    /// This timeout takes effect once rejoins using the well-known
    /// key has been allowed.
    /// 
    /// This command updates the sli_zigbee_allow_tc_rejoins_using_well_known_key_timeout_sec value.
    /// 
    /// EZSP_CONFIG_TC_REJOINS_USING_WELL_KNOWN_KEY_TIMEOUT_S
    TCRejoinsUsingWellKnownKeyTimeoutS = 0x38,

    /// Valid range of a CTUNE value is 0x0000-0x01FF.
    /// 
    /// Higher order bits (0xFE00) of the 16-bit value are ignored.
    /// 
    /// EZSP_CONFIG_CTUNE_VALUE 
    CTUNEValue = 0x39,

    /// To configure non trust center node to assume a
    /// concentrator type of the trust center it join to, until it
    /// receive many-to-one route request from the trust center.
    /// 
    /// For the trust center node, concentrator type is
    /// configured from the concentrator plugin.
    /// 
    /// The stack by default assumes trust center be a low RAM concentrator
    /// that make other devices send route record to the trust center even
    /// without receiving a many-to-one route request.
    /// 
    /// The default concentrator type can be changed by setting appropriate
    /// EmberAssumeTrustCenterConcentratorType config value.
    /// 
    /// EZSP_CONFIG_ASSUME_TC_CONCENTRATOR_TYPE 
    AssumeToConcentratorType = 0x40,

    /// This is green power proxy table size.
    /// 
    /// This value is read-only and cannot be set at runtime.
    /// 
    /// EZSP_CONFIG_GP_PROXY_TABLE_SIZE 
    GPProxyTableSize = 0x41,

    /// This is green power sink table size.
    /// 
    /// This value is read-only and cannot be set at runtime.
    /// 
    /// EZSP_CONFIG_GP_SINK_TABLE_SIZE 
    GPSinkTableSize = 0x42,
}

/// Identifies a value.
#[repr(u8)]
pub enum ValueID {
    /// The contents of the node data stack token.
    /// 
    /// EZSP_VALUE_TOKEN_STACK_NODE_DATA 
    TokenStackNodeData = 0x00,

    /// The types of MAC passthrough messages that the host wishes to receive.
    /// 
    /// EZSP_VALUE_MAC_PASSTHROUGH_FLAGS 
    MACPassthroughFlags = 0x01,

    /// The source address used to filter legacy EmberNet
    /// messages when the EMBER_MAC_PASSTHROUGH_EMBERNET_SOURCE
    /// flag is set in EZSP_VALUE_MAC_PASSTHROUGH_FLAGS.
    /// 
    /// EZSP_VALUE_EMBERNET_PASSTHROUGH_SOURCE_ADDRESS 
    EmberNetPassthroughSourceAddress = 0x02,

    /// The number of available internal RAM general
    /// purpose buffers. Read only.
    /// 
    /// EZSP_VALUE_FREE_BUFFERS 
    FreeBuffers = 0x03,

    /// Selects sending synchronous callbacks in ezsp-uart.
    /// 
    /// EZSP_VALUE_UART_SYNCH_CALLBACKS 
    UARTSynchCallbacks = 0x04,

    /// The maximum incoming transfer size for the local node.
    /// 
    /// Default value is set to 82 and does not use fragmentation.
    /// 
    /// Sets the value in Node Descriptor.
    /// 
    /// To set, this takes the input of a uint8 array of length 2 where
    /// you pass the lower byte at index 0 and upper byte at index 1.
    /// 
    /// EZSP_VALUE_MAXIMUM_INCOMING_TRANSFER_SIZE 
    MaximumIncomingTransferSize = 0x05,

    /// The maximum outgoing transfer size for the local node.
    /// 
    /// Default value is set to 82 and does not use fragmentation.
    /// 
    /// Sets the value in Node Descriptor.
    /// 
    /// To set, this takes the input of a uint8 array of length 2
    /// where you pass the lower byte at index 0 and upper
    /// 
    /// 
    /// byte at index 1.
    /// 
    /// EZSP_VALUE_MAXIMUM_OUTGOING_TRANSFER_SIZE 
    MaximumOutgoingTransferSize = 0x06,

    /// A bool indicating whether stack tokens are written
    /// to persistent storage as they change.
    /// 
    /// EZSP_VALUE_STACK_TOKEN_WRITING 
    StackTokenWriting = 0x07,

    /// A bool indicating whether stack tokens are written to
    /// persistent storage as they change.
    /// 
    /// EZSP_VALUE_STACK_TOKEN_WRITING 
    StackIsPerformingRejoin = 0x08,

    /// A list of EmberMacFilterMatchData values.
    MACFilterList = 0x09,

    /// The Ember Extended Security Bitmask.
    /// 
    /// EZSP_VALUE_EXTENDED_SECURITY_BITMASK 
    ExtendedSecurityBitmask = 0x0A,

    /// The node short ID.
    /// 
    /// EZSP_VALUE_NODE_SHORT_ID 
    NodeShortID = 0x0B,

    /// The descriptor capability of the local node. Write only.
    /// 
    /// EZSP_VALUE_DESCRIPTOR_CAPABILITY 
    DecsriptionCapability = 0x0C,

    /// The stack device request sequence number of the local node.
    /// 
    /// EZSP_VALUE_STACK_DEVICE_REQUEST_SEQUENCE_NUMBER
    StackDeviceRequesetSequenceNumber = 0x0D,

    /// Enable or disable radio hold-off.
    /// 
    /// EZSP_VALUE_RADIO_HOLD_OFF 
    RadioHoldOff = 0x0E,

    /// The flags field associated with the endpoint data.
    /// 
    /// EZSP_VALUE_ENDPOINT_FLAGS 
    EndpointFlags = 0x0F,

    /// Enable/disable the Mfg security config key settings.
    /// 
    /// EZSP_VALUE_MFG_SECURITY_CONFIG 
    MGXSecurityConfig = 0x10,

    /// Retrieves the version information from the stack on the NCP
    /// 
    /// EZSP_VALUE_VERSION_INFO 
    VerisonInfo = 0x11,

    /// This will get/set the rejoin reason noted by the host
    /// for a subsequent call to emberFindAndRejoinNetwork().
    /// 
    /// After a call to emberFindAndRejoinNetwork() the host's rejoin
    /// reason will be set to EMBER_REJOIN_REASON_NONE.
    /// 
    /// The NCP will store the rejoin reason used by the call to
    /// emberFindAndRejoinNetwork().
    /// 
    /// Application is not required to do anything with this value.
    /// 
    /// The App Framework sets this for cases of emberFindAndRejoinNetwork
    /// that it initiates, but if the app is invoking a rejoin directly,
    /// it should/can set this value to aid in debugging of any rejoin state
    /// machine issues over EZSP logs after the fact.
    /// 
    /// The NCP doesn't do anything with this value other than
    /// cache it so you can read it later.
    /// 
    /// EZSP_VALUE_NEXT_HOST_REJOIN_REASON 
    NextHostRejoinReason = 0x12,

    /// This is the reason that the last rejoin took place.
    /// 
    /// This value may only be retrieved, not set.
    /// 
    /// The rejoin may have been initiated by the stack (NCP) or the
    /// application (host).
    /// 
    /// If a host initiated a rejoin the reason will be set by default to
    /// EMBER_REJOIN_DUE_TO_APP_EVENT_1.
    /// 
    /// If the application wishes to denote its own rejoin reasons it
    /// can do so by calling ezspSetValue(EMBER_VALUE_HOST_REJOIN_REASON,
    /// EMBER_REJOIN_DUE_TO_APP_EVENT_X).
    /// 
    /// X is a number corresponding to one of the app events defined.
    /// 
    /// If the NCP initiated a rejoin it will record this value internally
    /// for retrieval by ezspGetValue(EZSP_VALUE_REAL_REJOIN_REASON).
    /// 
    /// EZSP_VALUE_LAST_REJOIN_REASON 
    LastRejoinReason = 0x13,

    /// The next ZigBee sequence number.
    /// 
    /// EZSP_VALUE_NEXT_ZIGBEE_SEQUENCE_NUMBER 
    NextZigbeeSequenceNumber = 0x14,

    /// CCA energy detect threshold for radio.
    /// 
    /// EZSP_VALUE_CCA_THRESHOLD 
    CAAThreshold = 0x15,

    /// The threshold value for a counter
    /// 
    /// EZSP_VALUE_SET_COUNTER_THRESHOLD 
    SetCounterThreshold = 0x17,

    /// Resets all counters thresholds to 0xFF
    /// 
    /// EZSP_VALUE_RESET_COUNTER_THRESHOLDS 
    ResetCounterThresholds = 0x18,

    /// Clears all the counters.
    /// 
    /// EZSP_VALUE_CLEAR_COUNTERS 
    ClearCounters = 0x19,

    /// The node's new certificate signed by the CA.
    /// 
    /// EZSP_VALUE_CERTIFICATE_283K1 
    Certificate283K1 = 0x1A,

    /// The Certificate Authority's public key.
    /// 
    /// EZSP_VALUE_PUBLIC_KEY_283K1 
    PublicKey283K1 = 0x1B,

    /// The node's new static private key.
    /// 
    /// EZSP_VALUE_PRIVATE_KEY_283K1 
    PrivateKey283K1 = 0x1C,

    /// The NWK layer security frame counter value
    /// 
    /// EZSP_VALUE_NWK_FRAME_COUNTER 
    NWKFramecounter = 0x23,

    /// The APS layer security frame counter value.
    /// 
    /// Managed by the stack.
    /// 
    /// Users should not set these unless doing backup and restore.
    /// 
    /// EZSP_VALUE_APS_FRAME_COUNTER 
    APSFrmaEcounter = 0x24,

    /// Sets the device type to use on the next rejoin using device type.
    /// 
    /// EZSP_VALUE_RETRY_DEVICE_TYPE 
    RetryDeviceType = 0x25,

    /// Sets the device type to use on the next rejoin using device type.
    /// 
    /// EZSP_VALUE_ENABLE_R21_BEHAVIOR 
    EnableR21Behavior = 0x29,

    /// Configure the antenna mode.
    /// 
    /// 0-don't switch
    /// 1- primary
    /// 2-secondary
    /// 3-TX antenna diversity
    /// 
    /// EZSP_VALUE_ANTENNA_MODE 
    AntennaMode = 0x30,

    /// Enable or disable packet traffic arbitration.
    /// 
    /// EZSP_VALUE_ENABLE_PTA 
    EnablePTA = 0x31,

    /// Set packet traffic arbitration configuration options.
    /// 
    /// EZSP_VALUE_PTA_OPTIONS 
    PTAOptions = 0x32,

    /// Configure manufacturing library options.
    /// 
    /// 0-non-CSMA transmits
    /// 1-CSMA transmits
    /// 
    /// To be used with Manufacturing library.
    /// 
    /// EZSP_VALUE_MFGLIB_OPTIONS 
    MGFLibOptions = 0x33,

    /// Sets the flag to use either negotiated power by link
    /// power delta (LPD) or fixed power value provided by
    /// user while forming/joining a network for packet
    /// transmissions on sub-ghz interface.
    /// 
    /// This is mainly for testing purposes.
    /// 
    /// EZSP_VALUE_USE_NEGOTIATED_POWER_BY_LPD 
    UseNegotiatedPowerByLPD = 0x34,

    /// Set packet traffic arbitration PWM options.
    /// 
    /// EZSP_VALUE_PTA_PWM_OPTIONS 
    PTAPWNOptions = 0x35,

    /// Set packet traffic arbitration directional
    /// priority pulse width in microseconds.
    /// 
    /// EZSP_VALUE_PTA_DIRECTIONAL_PRIORITY_PULSE_WIDTH 
    PTADirectionalPriorityPulseWidth = 0x36,

    /// Set packet traffic arbitration phy select timeout(ms).
    /// 
    /// EZSP_VALUE_PTA_PHY_SELECT_TIMEOUT 
    PTAPHYSelectTimeout = 0x37,

    /// Configure the RX antenna mode:
    /// 0-do not switch
    /// 1-primary
    /// 2-secondary
    /// 3-RX antenna diversity
    /// 
    /// EZSP_VALUE_ANTENNA_RX_MODE 
    AntennaRXMode = 0x38,

    /// Configure the timeout to wait for the network key before failing a join.
    /// 
    /// Acceptable timeout range [3,255].
    /// 
    /// Value is in seconds.
    /// 
    /// EZSP_VALUE_NWK_KEY_TIMEOUT 
    NetworkKeyTimeout = 0x39,

    /// The number of failed CSMA attempts due to failed
    /// CCA made by the MAC before continuing
    /// transmission with CCA disabled.
    /// 
    /// This is the same as calling the emberForceTxAfterFailedCca(uint8_t
    /// csmaAttempts) API.
    /// 
    /// A value of 0 disables the feature.
    /// 
    /// EZSP_VALUE_FORCE_TX_AFTER_FAILED_CCA_ATTEMPTS 
    ForceTXAfterFailedCCAAttempts = 0x3A,

    /// The length of time, in seconds, that a trust center will
    /// store a transient link key that a device can use to join
    /// its network.
    /// 
    /// A transient key is added with a call to sl_zb_sec_man_import_transient_key.
    ///
    ///  After the transient key is added, it will be removed once this
    /// amount of time has passed.
    /// 
    /// A joining device will not be able to use that key to join
    /// until it is added again on the trust center.
    /// 
    /// The default value is 300 seconds (5 minutes).
    /// 
    /// EZSP_VALUE_TRANSIENT_KEY_TIMEOUT_S 
    TransientKeyTimeoutS = 0x3B,

    /// Cumulative energy usage metric since the last value
    /// reset of the coulomb counter plugin.
    /// 
    /// Setting this value will reset the coulomb counter.
    /// 
    /// EZSP_VALUE_COULOMB_COUNTER_USAGE 
    CoulombCounterUsage = 0x3C,

    /// When scanning, configure the maximum number of
    /// beacons to store in cache.
    /// 
    /// Each beacon consumes one packet buffer in RAM.
    /// 
    /// EZSP_VALUE_MAX_BEACONS_TO_STORE 
    MaxBeaconsToStore = 0x3D,

    /// Set the mask to filter out unacceptable child timeout options on a router.
    /// 
    /// EZSP_VALUE_END_DEVICE_TIMEOUT_OPTIONS_MASK 
    EndDeviceTimeoutOptionsMask = 0x3E,

    /// The end device keep-alive mode supported by the parent.
    EndDeviceKeepAliceSupportMode = 0x3F,

    /// Return the active radio config. Read only.
    /// 
    /// 0: Default
    /// 1: Antenna Diversity
    /// 2: Co-Existence
    /// 3: Antenna diversity and Co-Existence
    /// 
    /// EZSP_VALUE_ACTIVE_RADIO_CONFIG 
    ActiveRadioCnfig = 0x41,

    /// Return the number of seconds the network will remain open.
    /// 
    /// A return value of 0 indicates that the network is closed.
    /// 
    /// Read only.
    /// 
    /// EZSP_VALUE_NWK_OPEN_DURATION 
    NetworkOpenDuration = 0x42,

    /// Timeout in milliseconds to store entries in the transient
    /// device table.
    /// 
    /// If the devices are not authenticated before the timeout,
    /// the entry shall be purged.
    /// 
    /// EZSP_VALUE_TRANSIENT_DEVICE_TIMEOUT 
    TransientDeviceTimeout = 0x43,

    /// Return information about the key storage on an NCP.
    /// 
    /// Returns 0 if keys are in classic key storage, and 1
    /// if they are located in PSA key storage. Read only.
    /// 
    /// EZSP_VALUE_KEY_STORAGE_VERSION 
    KeyStorageVersion = 0x44,

    /// Return activation state about TC Delayed Join on an NCP.
    /// 
    /// A return value of 0 indicates that the feature is not activated.
    /// 
    /// EZSP_VALUE_DELAYED_JOIN_ACTIVATION 
    DelayedJoinActivation = 0x45,
}

/// Identifies a value based on specified characteristics.
#[repr(u8)]
pub enum ExtendedValueID {
    /// The flags field associated with the specified endpoint.
    /// 
    /// EZSP_EXTENDED_VALUE_ENDPOINT_FLAGS 
    EndpointFlags = 0x00,

    /// This is the reason for the node to leave the network
    /// as well as the device that told it to leave.
    /// 
    /// The leave reason is the 1st byte of the value while
    /// the node ID is the 2nd and 3rd byte.
    /// 
    /// If the leave was caused due to an API call rather than an
    /// over the air message, the node ID will be EMBER_UNKNOWN_NODE_ID
    /// (0xFFFD).
    /// 
    /// EZSP_EXTENDED_VALUE_LAST_LEAVE_REASON 
    LastLEaveReason = 0x01,

    /// This number of bytes of overhead required in the
    /// network frame for source routing to a particular
    /// destination.
    /// 
    /// EZSP_EXTENDED_VALUE_GET_SOURCE_ROUTE_OVERHEAD 
    GetSourceRouteOverhead = 0x02,
}

/// Flags associated with the endpoint data configured on the NCP.
#[repr(u16)]
pub enum EndpointFlags {
    /// Indicates that the endpoint is disabled and NOT discoverable via ZDO.
    Disabled = 0x00,

    /// Indicates that the endpoint is enabled and discoverable via ZDO.
    Enabled = 0x01
}

/// Identifies a policy.
#[repr(u8)]
pub enum PolicyID {
    /// Controls trust center behavior.
    /// 
    /// EZSP_TRUST_CENTER_POLICY 
    TrustCenter = 0x0,

    /// Controls how external binding modification requests are handled.
    /// 
    /// EZSP_BINDING_MODIFICATION_POLICY 
    BindingModification = 0x01,

    /// Controls whether the Host supplies unicast replies.
    /// 
    /// EZSP_UNICAST_REPLIES_POLICY 
    UnicastReplies = 0x02,

    /// Controls whether pollHandler callbacks are generated.
    /// 
    /// EZSP_POLL_HANDLER_POLICY 
    PollHandler = 0x03,

    /// Controls whether the message contents are included in the messageSentHandler callback.
    /// 
    /// EZSP_MESSAGE_CONTENTS_IN_CALLBACK_POLICY 
    MessageContentsInCallback = 0x04,

    /// Controls whether the Trust Center will respond
    /// to Trust Center link key requests.
    /// 
    /// EZSP_TC_KEY_REQUEST_POLICY 
    TCKeyRequest = 0x05,

    /// Controls whether the Trust Center will respond
    /// to application link key requests.
    /// 
    /// EZSP_APP_KEY_REQUEST_POLICY 
    AppKeyRequest = 0x06,

    /// Controls whether ZigBee packets that appear invalid are
    /// automatically dropped by the stack.
    /// 
    /// A counter will be incremented when this occurs.
    /// 
    /// EZSP_PACKET_VALIDATE_LIBRARY_POLICY 
    PacketValidatedLubrary = 0x07,

    /// Controls whether the stack will process ZLL messages.
    /// 
    /// EZSP_ZLL_POLICY 
    ZLL = 0x08,

    /// Controls whether Trust Center (insecure) rejoins for devices
    /// using the well-known link key are accepted.
    /// 
    /// If rejoining using the well-known key is allowed, it is disabled
    /// again after sli_zigbee_allow_tc_rejoins_using_well_known_key_timeout_sec
    /// seconds.
    /// 
    /// EZSP_TC_REJOINS_USING_WELL_KNOWN_KEY_POLICY
    TCRejoinsUsingWellKnownKey = 0x09
}

/// The policy decision bitmask that controls the trust center decision strategies.
#[repr(u16)]
// TODO: bitmask types
pub enum DecisionBitmask {
    /// Disallow joins and rejoins.
    /// 
    /// EZSP_DECISION_BITMASK_DEFAULT_CONFIGURATION 
    DefaultConfiguration = 0x0000,

    /// Send the network key to all joining devices.
    /// 
    /// EZSP_DECISION_ALLOW_JOINS 
    AllowJoins = 0x0001,
    
    /// Send the network key to all rejoining devices.
    /// 
    /// EZSP_DECISION_ALLOW_UNSECURED_REJOINS 
    AllowUnsecuredRejoins = 0x0002,
    
    /// Send the network key in the clear.
    /// 
    /// EZSP_DECISION_SEND_KEY_IN_CLEAR 
    SendKeyInClear = 0x0004,
    
    /// Do nothing for unsecured rejoins.
    /// 
    /// EZSP_DECISION_IGNORE_UNSECURED_REJOINS 
    IgnoreUnsecuredRejoins = 0x0008,
    
    /// Allow joins if there is an entry in the transient key table.
    /// 
    /// EZSP_DECISION_JOINS_USE_INSTALL_CODE_KEY 
    JoinsUseInstallCodeKey = 0x0010,

    /// Delay sending the network key to a new joining device.
    /// 
    /// EZSP_DECISION_DEFER_JOINS 
    DeferJoins = 0x0020
}

/// Identifies a policy decision.
#[repr(u8)]
pub enum DecisionId {
    /// Delay sending the network key to a new joining device.
    /// 
    /// EZSP_DEFER_JOINS_REJOINS_HAVE_LINK_KEY 
    DeferJoinsRejoinsHaveLinkKey = 0x07,

    /// EZSP_BINDING_MODIFICATION_POLICY default decision.
    /// 
    /// Do not allow the local binding table to be changed by remote nodes.
    /// 
    /// EZSP_DISALLOW_BINDING_MODIFICATION 
    DisallowBindingModification = 0x10,

    /// EZSP_BINDING_MODIFICATION_POLICY decision.
    /// 
    /// Allow remote nodes to change the local binding table.
    /// 
    /// EZSP_ALLOW_BINDING_MODIFICATION 
    AllowBindingModification = 0x11,

    /// EZSP_BINDING_MODIFICATION_POLICY decision.
    /// 
    /// Allows remote nodes to set local binding entries only if
    /// the entries correspond to endpoints defined on the device,
    /// and for output clusters bound to those endpoints.
    /// 
    /// EZSP_CHECK_BINDING_MODIFICATIONS_ARE_VALID_ENDPOINT_CLUSTERS
    CheckBindingModificaitonsAreValidEndpointClusters = 0x12,

    /// EZSP_UNICAST_REPLIES_POLICY default decision.
    /// 
    /// The NCP will automatically send an empty reply
    /// (containing no payload) for every unicast received.
    /// 
    /// EZSP_HOST_WILL_NOT_SUPPLY_REPLY 
    HostWillNotSupplyReply = 0x20,

    /// EZSP_UNICAST_REPLIES_POLICY decision.
    /// 
    /// The NCP will only send a reply if it receives
    /// a sendReply command from the Host.
    /// 
    /// EZSP_HOST_WILL_SUPPLY_REPLY 
    HostWillSupplyReply = 0x21,

    /// EZSP_POLL_HANDLER_POLICY default decision.
    /// 
    /// Do not inform the Host when a child polls.
    /// 
    /// EZSP_POLL_HANDLER_IGNORE 
    PollHandlerIgnore = 0x30,

    /// EZSP_POLL_HANDLER_POLICY decision.
    /// 
    /// Generate a pollHandler callback when a child polls.
    /// 
    /// EZSP_POLL_HANDLER_CALLBACK 
    PollHandlerCallback = 0x31,

    /// EZSP_MESSAGE_CONTENTS_IN_CALLBACK_POLICY default decision.
    /// 
    /// Include only the message tag in the messageSentHandler callback.
    /// 
    /// EZSP_MESSAGE_TAG_ONLY_IN_CALLBACK 
    MessageTagOnlyInCallback = 0x40,

    /// EZSP_MESSAGE_CONTENTS_IN_CALLBACK_POLICY decision.
    /// 
    /// Include both the message tag and the message contents
    /// in the messageSentHandler callback.
    /// 
    /// EZSP_MESSAGE_TAG_AND_CONTENTS_IN_CALLBACK 
    MessageTagAndContentsInCallback = 0x41,

    /// EZSP_TC_KEY_REQUEST_POLICY decision.
    /// 
    /// When the Trust Center receives a request for
    /// a Trust Center link key, it will be ignored.
    /// 
    /// EZSP_DENY_TC_KEY_REQUESTS 
    DenyTCKeyRequests = 0x50,

    /// EZSP_TC_KEY_REQUEST_POLICY decision.
    /// 
    /// When the Trust Center receives a request
    /// for a Trust Center link key, it will reply
    /// to it with the corresponding key.
    /// 
    /// EZSP_ALLOW_TC_KEY_REQUESTS_AND_SEND_CURRENT_KEY
    AllowTCKeyRequestsAndSendCurrentKey = 0x51,

    /// EZSP_TC_KEY_REQUEST_POLICY decision.
    /// 
    /// When the Trust Center receives a request for a
    /// Trust Center link key, it will generate a key
    /// to send to the joiner.
    /// 
    /// After generation, the key will be added to the
    /// transient key table and after verification this
    /// key will be added to the link key table.
    /// 
    /// EZSP_ALLOW_TC_KEY_REQUEST_AND_GENERATE_NEW_KEY
    AllowTCKeyRequestAndGenerateNewKey = 0x52,

    /// EZSP_APP_KEY_REQUEST_POLICY decision.
    /// 
    /// When the Trust Center receives a request for an
    /// application link key, it will be ignored.
    /// 
    /// EZSP_DENY_APP_KEY_REQUESTS 
    DenyAppKeyRquests = 0x60,

    /// EZSP_APP_KEY_REQUEST_POLICY decision.
    /// 
    /// When the Trust Center receives a request for an
    /// application link key, it will randomly generate a key
    /// and send it to both partners.
    /// 
    /// EZSP_ALLOW_APP_KEY_REQUESTS 
    AllowAppKeyRequests = 0x61,

    /// Indicates that packet validate library checks are enabled on the NCP.
    /// 
    /// EZSP_PACKET_VALIDATE_LIBRARY_CHECKS_ENABLED 
    PacketValidateLibraryChecksEnabled = 0x62,

    /// Indicates that packet validate library checks are NOT enabled on the NCP.
    /// 
    /// EZSP_PACKET_VALIDATE_LIBRARY_CHECKS_DISABLED 
    PacketValidateLibraryChecksDisabled = 0x63,
}

/// Manufacturing token ID.
#[repr(u8)]
pub enum  MaunfacturingTokenID {
    /// Custom version (2 bytes).
    /// 
    /// EZSP_MFG_CUSTOM_VERSION 
    CustomVersion = 0x00,

    /// Manufacturing string (16 bytes).
    /// 
    /// EZSP_MFG_STRING 
    String = 0x01,

    /// Board name (16 bytes).
    /// 
    /// EZSP_MFG_BOARD_NAME 
    BoardName = 0x02,

    /// Manufacturing ID (2 bytes).
    /// 
    /// EZSP_MFG_MANUF_ID 
    ManufactureID = 0x03,

    /// Radio configuration (2 bytes).
    /// 
    /// EZSP_MFG_PHY_CONFIG 
    PHYConfig = 0x04,

    /// Bootload AES key (16 bytes).
    /// 
    /// EZSP_MFG_BOOTLOAD_AES_KEY 
    BootloadAWSKey = 0x05,

    /// ASH configuration (40 bytes).
    /// 
    /// EZSP_MFG_ASH_CONFIG 
    ASHConfig = 0x06,

    /// EZSP storage (8 bytes).
    /// 
    /// EZSP_MFG_EZSP_STORAGE 
    EZSPStorage = 0x07,

    /// Radio calibration data (64 bytes).
    /// 
    /// 4 bytes are stored for each of the 16 channels.
    /// 
    /// This token is not stored in the Flash Information Area.
    /// 
    /// It is updated by the stack each time a calibration is performed.
    /// 
    /// EZSP_STACK_CAL_DATA 
    StackCalibrationData = 0x08,

    /// Certificate Based Key Exchange (CBKE) data (92 bytes).
    /// 
    /// EZSP_MFG_CBKE_DATA 
    CBKEData = 0x09,
    
    /// Installation code (20 bytes).
    /// 
    /// EZSP_MFG_INSTALLATION_CODE 
    InstallationCode = 0x0A,

    /// Radio channel filter calibration data (1 byte).
    /// 
    /// This token is not stored in the Flash Information Area.
    /// 
    /// It is updated by the stack each time a calibration is performed.
    /// 
    /// EZSP_STACK_CAL_FILTER 
    StackCalibrationFilter = 0x0B,

    /// Custom EUI64 MAC address (8 bytes).
    /// 
    /// EZSP_MFG_CUSTOM_EUI_64 
    CustomEUI64 = 0x0C,

    /// CTUNE value (2 byte).
    /// 
    /// EZSP_MFG_CTUNE 
    CTUNE = 0x0D
}

/// Status values used by EZSP.
#[repr(u8)]
pub enum Status {
    /// Success.
    /// 
    /// EZSP_SUCCESS 
    Success = 0x00,
    
    /// Fatal error.
    /// 
    /// EZSP_SPI_ERR_FATAL 
    SPIErrorFatal = 0x10,
    /// The Response frame of the current transaction indicates the
    /// NCP has reset.
    /// 
    /// EZSP_SPI_ERR_NCP_RESET 
    SPIErrorNCPReset = 0x11,
    /// The NCP is reporting that the Command frame of the current
    /// transaction is oversized (the length byte is too large).
    /// 
    /// EZSP_SPI_ERR_OVERSIZED_EZSP_FRAME 
    SPIErrorOversizedEZSPFrame = 0x12,
    /// The Response frame of the current transaction indicates the
    /// previous transaction was aborted (nSSEL deasserted too soon).
    /// 
    /// EZSP_SPI_ERR_ABORTED_TRANSACTION 
    SPIErrorAbortedTransaction = 0x13,
    /// The Response frame of the current transaction indicates the
    /// frame terminator is missing from the Command frame.
    /// 
    /// EZSP_SPI_ERR_MISSING_FRAME_TERMINATOR 
    SPIErrorMissingFrameTerminator = 0x14,
    /// The NCP has not provided a Response within the time limit
    /// defined by WAIT_SECTION_TIMEOUT.
    /// 
    /// EZSP_SPI_ERR_WAIT_SECTION_TIMEOUT 
    APIErrorWaitSectionTimeout = 0x15,
    /// The Response frame from the NCP is missing the frame terminator.
    /// 
    /// EZSP_SPI_ERR_NO_FRAME_TERMINATOR 
    SPIErrorNoFrameTerminator = 0x16,
    /// The Host attempted to send an oversized Command (the
    /// length byte is too large) and the AVR's spi-protocol.c blocked
    /// the transmission.
    /// 
    /// EZSP_SPI_ERR_EZSP_COMMAND_OVERSIZED 
    SPIErrorEZSPCommandOversized = 0x17,
    /// The NCP attempted to send an oversized Response (the
    /// length byte is too large) and the AVR's spi-protocol.c
    /// blocked the reception.
    /// 
    /// EZSP_SPI_ERR_EZSP_RESPONSE_OVERSIZED 
    SPIErrorEZSPResponseOversized = 0x18,
    /// The Host has sent the Command and is still waiting for the
    /// NCP to send a Response.
    /// 
    /// EZSP_SPI_WAITING_FOR_RESPONSE 
    SPIWaitingForResponse = 0x19,
    /// The NCP has not asserted nHOST_INT within the time limit
    /// defined by WAKE_HANDSHAKE_TIMEOUT.
    /// 
    /// EZSP_SPI_ERR_HANDSHAKE_TIMEOUT 
    SPIErrorHandshakeTimeout = 0x1A,
    /// The NCP has not asserted nHOST_INT after an NCP reset
    /// within the time limit defined by STARTUP_TIMEOUT.
    /// 
    /// EZSP_SPI_ERR_STARTUP_TIMEOUT 
    APIErrorStartupTimeout = 0x1B,
    /// The Host attempted to verify the SPI Protocol activity and
    /// version number, and the verification failed.
    /// 
    /// EZSP_SPI_ERR_STARTUP_FAIL 
    SPIErrorStartupFail = 0x1C,
    /// The Host has sent a command with a SPI Byte that is
    /// unsupported by the current mode the NCP is operating in.
    /// 
    /// EZSP_SPI_ERR_UNSUPPORTED_SPI_COMMAND 
    SPIErrorUnsupportedSPICommand = 0x1D,

    /// Operation not yet complete.
    /// 
    /// EZSP_ASH_IN_PROGRESS 
    ASHInProgress = 0x20,
    /// Fatal error detected by host.
    /// 
    /// EZSP_HOST_FATAL_ERROR 
    HostFatalError = 0x21,
    /// Fatal error detected by NCP.
    /// 
    /// EZSP_ASH_NCP_FATAL_ERROR 
    ASHNCPFatalError = 0x22,
    /// Tried to send DATA frame too long.
    /// 
    /// EZSP_DATA_FRAME_TOO_LONG 
    DataFrameTooLong = 0x23,
    /// Tried to send DATA frame too short.
    /// 
    /// EZSP_DATA_FRAME_TOO_SHORT 
    DataFrameTooShort = 0x24,
    /// No space for tx'ed DATA frame.
    /// 
    /// EZSP_NO_TX_SPACE 
    NoTXSpace = 0x25,
    /// No space for rec'd DATA frame.
    /// 
    /// EZSP_NO_RX_SPACE 
    NoRXSpace = 0x26,
    /// No receive data available.
    /// 
    /// EZSP_NO_RX_DATA 
    NoRXData = 0x27,
    /// Not in Connected state.
    /// 
    /// EZSP_NOT_CONNECTED 
    NotConnected = 0x28,
    
    /// The NCP received a command before the EZSP version had been set.
    /// 
    /// EZSP_ERROR_VERSION_NOT_SET 
    ErrorVersionNotSet = 0x30,
    /// The NCP received a command containing an unsupported frame ID.
    /// 
    /// EZSP_ERROR_INVALID_FRAME_ID 
    ErrorInvalidFrameID = 0x31,
    /// The direction flag in the frame control field was incorrect.
    /// 
    /// EZSP_ERROR_WRONG_DIRECTION 
    ErrorWrongdirection = 0x32,
    /// The truncated flag in the frame control field was set,
    /// indicating there was not enough memory available to
    /// complete the response or that the response would have
    /// exceeded the maximum EZSP frame length.
    /// 
    /// EZSP_ERROR_TRUNCATED
    ErrorTruncated = 0x33,
    /// The overflow flag in the frame control field was set, indicating
    /// one or more callbacks occurred since the previous response
    /// and there was not enough memory available to report them
    /// to the Host.
    /// 
    /// EZSP_ERROR_OVERFLOW 
    ErrorOverflow = 0x34,
    /// Insufficient memory was available.
    /// 
    /// EZSP_ERROR_OUT_OF_MEMORY 
    ErrorOutOfMemory = 0x35,
    /// The value was out of bounds.
    /// 
    /// EZSP_ERROR_INVALID_VALUE 
    ErrorInvalidValue = 0x36,
    /// The configuration id was not recognized.
    /// 
    /// EZSP_ERROR_INVALID_ID 
    ErrorInvalidID = 0x37,
    /// Configuration values can no longer be modified.
    /// 
    /// EZSP_ERROR_INVALID_CALL 
    ErrorInvalidCall = 0x38,
    /// The NCP failed to respond to a command.
    /// 
    /// EZSP_ERROR_NO_RESPONSE 
    ErrorNoResponse = 0x39,
    /// The length of the command exceeded the maximum EZSP
    /// frame length.
    /// 
    /// EZSP_ERROR_COMMAND_TOO_LONG 
    ErrorCommandTooLong = 0x40,
    /// The UART receive queue was full causing a callback
    /// response to be dropped.
    /// 
    /// EZSP_ERROR_QUEUE_FULL 
    ErrorQueueFull = 0x41,
    /// The command has been filtered out by NCP.
    /// 
    /// EZSP_ERROR_COMMAND_FILTERED 
    ErrorCommandFiltered = 0x42,
    /// EZSP Security Key is already set.
    /// 
    /// EZSP_ERROR_SECURITY_KEY_ALREADY_SET 
    ErrorSecurityKeyAlreadySet = 0x43,
    /// EZSP Security Type is invalid.
    /// 
    /// EZSP_ERROR_SECURITY_TYPE_INVALID 
    ErrorSecurityTypeInvalid = 0x44,
    /// EZSP Security Parameters are invalid.
    /// 
    /// EZSP_ERROR_SECURITY_PARAMETERS_INVALID 
    ErrorSecurityParametersInvalid = 0x45,
    /// EZSP Security Parameters are already set.
    /// 
    /// EZSP_ERROR_SECURITY_PARAMETERS_ALREADY_SET 
    ErrorSecurityParametersAlreadySet = 0x46,
    /// EZSP Security Key is not set
    /// 
    /// EZSP_ERROR_SECURITY_KEY_NOT_SET 
    ErrorSecurityKeyNotSet = 047,
    /// EZSP Security Parameters are not set.
    /// 
    /// EZSP_ERROR_SECURITY_PARAMETERS_NOT_SET 
    ErrorSecurityParametersNotSet = 0x48,
    /// Received frame with unsupported control byte.
    /// 
    /// EZSP_ERROR_UNSUPPORTED_CONTROL 
    ErrorUnsupportedControl = 0x49,
    /// Received frame is unsecure, when security is established.
    /// 
    /// EZSP_ERROR_UNSECURE_FRAME 
    ErrorUnsecureFrame = 0x4A,

    /// Incompatible ASH version.
    /// 
    /// EZSP_ASH_ERROR_VERSION 
    ASHErrorVersion = 0x50,
    /// Exceeded max ACK timeouts.
    /// 
    /// EZSP_ASH_ERROR_TIMEOUTS 
    ASHErrorTimeouts = 0x51,
    /// Timed out waiting for RSTACK.
    /// 
    /// EZSP_ASH_ERROR_RESET_FAIL 
    ASHErrorResetFail = 0x52,
    /// Unexpected ncp reset.
    /// 
    /// EZSP_ASH_ERROR_NCP_RESET 
    ASHErrorNCPReset = 0x53,
    /// Serial port initialization failed.
    /// 
    /// EZSP_ERROR_SERIAL_INIT 
    ErrorSerialInit = 0x54,
    /// Invalid ncp processor type.
    /// 
    /// EZSP_ASH_ERROR_NCP_TYPE 
    ASHErrorNCPTypes = 0x55,
    /// Invalid ncp reset method.
    /// 
    /// EZSP_ASH_ERROR_RESET_METHOD 
    ASHErrorResetMethod = 0x56,
    /// XON/XOFF not supported by host driver.
    /// 
    /// EZSP_ASH_ERROR_XON_XOFF 
    ASHErrorXOnXOff = 0x57,

    /// ASH protocol started.
    /// 
    /// EZSP_ASH_STARTED 
    ASHStarted = 0x70,
    /// ASH protocol connected.
    /// 
    /// EZSP_ASH_CONNECTED 
    ASHConnected = 0x71,
    /// ASH protocol disconnected.
    /// 
    /// EZSP_ASH_DISCONNECTED 
    ASHDisconnected = 0x72,
    /// Timer expired waiting for ack.
    /// 
    /// EZSP_ASH_ACK_TIMEOUT 
    ASHACKTimeout = 0x73,
    /// Frame in progress cancelled.
    /// 
    /// EZSP_ASH_CANCELLED 
    ASHCancelled = 0x74,
    /// Received frame out of sequence.
    /// 
    /// EZSP_ASH_OUT_OF_SEQUENCE 
    ASHOutOfSequence = 0x75,
    /// Received frame with CRC error.
    /// 
    /// EZSP_ASH_BAD_CRC 
    ASHBadCRC = 0x76,
    /// Received frame with comm error.
    /// 
    /// EZSP_ASH_COMM_ERROR 
    ASHCommError = 0x77,
    /// Received frame with bad ackNum.
    /// 
    /// EZSP_ASH_BAD_ACKNUM 
    ASHBadAcknum = 0x78,
    /// Received frame shorter than minimum.
    /// 
    /// EZSP_ASH_TOO_SHORT 
    ASHTooShort = 0x79,
    /// Received frame longer than maximum.
    /// 
    /// EZSP_ASH_TOO_LONG 
    ASHTooLong = 0x7A,
    /// Received frame with illegal control byte.
    /// 
    /// EZSP_ASH_BAD_CONTROL 
    ASHBadControl = 0x7B,
    /// Received frame with illegal length for its type.
    /// 
    /// EZSP_ASH_BAD_LENGTH 
    ASHBadLength = 0x7C,
    /// Received ASH Ack.
    /// 
    /// EZSP_ASH_ACK_RECEIVED 
    ASHACKReceived = 0x7D,
    /// Sent ASH Ack.
    /// 
    /// EZSP_ASH_ACK_SENT 
    ASHACKSent = 0x7E,
    /// Received ASH Nak.
    /// 
    /// EZSP_ASH_NAK_RECEIVED 
    ASHNAKReceived = 0x7F,
    /// Sent ASH Nak.
    /// 
    /// EZSP_ASH_NAK_SENT 
    ASHNAKSent = 0x80,
    /// Received ASH RST.
    /// 
    /// EZSP_ASH_RST_RECEIVED 
    ASHRSTReceived = 0x81,
    /// Sent ASH RST.
    /// 
    /// EZSP_ASH_RST_SENT 
    ASHRSTSent = 0x82,
    /// ASH Status.
    /// 
    /// EZSP_ASH_STATUS 
    ASHStatus = 0x83,
    /// ASH TX.
    /// 
    /// EZSP_ASH_TX 
    ASHTX = 0x84,
    /// ASH RX.
    /// 
    /// EZSP_ASH_RX 
    ASHRX = 0x85,
    /// Failed to connect to CPC daemon or failed to open CPC endpoint.
    /// 
    /// EZSP_CPC_ERROR_INIT 
    CPCErrorInit = 0x86,
    /// No reset or error
    /// 
    /// EZSP_NO_ERROR 
    NoError = 0xFF
}

/// Network scan types.
/// 
/// EzspNetworkScanType 
#[repr(u8)]
pub enum NetworkScanType {
    /// An energy scan scans each channel for its RSSI value.
    /// 
    /// EZSP_ENERGY_SCAN 
    EnergyScan = 0x00,
    /// An active scan scans each channel for available networks.
    /// 
    /// EZSP_ACTIVE_SCAN 
    ActiveScan = 0x01,
}

/// Differentiates ZLL network operations.
/// 
/// EzspZllNetworkOperation
#[repr(u8)]
pub enum ZLLNetworkOperation {
    /// ZLL form network command.
    /// 
    /// EZSP_ZLL_FORM_NETWORK 
    FormNetwork = 0x00,
    /// ZLL join target command.
    /// 
    /// EZSP_ZLL_JOIN_TARGET 
    JoinTarget = 0x01,
}

/// Validates Source Route Overhead Information cached.
/// 
/// EzspSourceRouteOverheadInformation 
#[repr(u8)]
pub enum SourceRouteOverheadInformation {
    /// EZSP source route overhead unknown.
    /// 
    /// EZSP_SOURCE_ROUTE_OVERHEAD_UNKNOWN 
    OverheadUnknown = 0xFF
}

