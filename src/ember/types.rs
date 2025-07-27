#[repr(u16)]
pub enum ConfigTXPowerMode {
    /// Normal power mode and bi-directional RF transmitter output.
    Default = 0x00,

    /// Enable boost power mode.
    /// 
    /// This is a high-performance radio mode which offers
    /// increased receive sensitivity and transmit power at
    /// the cost of an increase in power consumption.
    Boost = 0x01,
    Alternate = 0x02,
    BoostandAlternate = 0x03
}

/// Return type for stack functions.
/// 
/// https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf (P.g. 33)
#[repr(u8)]
pub enum Status {
    /// The generic 'no error' message.
    /// 
    /// EMBER_SUCCESS 
    Success = 0x00,
    
    /// The generic 'fatal error' message.
    /// 
    /// EMBER_ERR_FATAL 
    ErrFatal = 0x01,
    /// An invalid value was passed as an argument to a function.
    /// 
    /// EMBER_BAD_ARGUMENT 
    BadArgument = 0x02,
    
    /// The manufacturing and stack token format in non-
    /// volatile memory is different than what the stack
    /// expects (returned at initialization).
    /// 
    /// EMBER_EEPROM_MFG_STACK_VERSION_MISMATCH 
    EEPROMMfgStackVersionMismatch = 0x04,
    /// The manufacturing token format in non-volatile
    /// memory is different than what the stack expects
    /// (returned at initialization).
    /// 
    /// EMBER_EEPROM_MFG_VERSION_MISMATCH 
    EEPROMMfgVersionMismatch = 0x06,
    /// The stack token format in non-volatile memory is
    /// different than what the stack expects (returned at
    /// initialization).
    /// 
    /// EMBER_EEPROM_STACK_VERSION_MISMATCH 
    EEPROMStackVersionMismatch = 0x07,

    /// There are no more buffers.
    /// 
    /// EMBER_NO_BUFFERS 
    NoBuffers = 0x18,

    /// Specified an invalid baud rate.
    /// 
    /// EMBER_SERIAL_INVALID_BAUD_RATE 
    SerialInvalidBaudRate = 0x20,
    /// Specified an invalid serial port.
    /// 
    /// EMBER_SERIAL_INVALID_PORT 
    SerialInvalidPort = 0x21,
    /// Tried to send too much data.
    /// 
    /// EMBER_SERIAL_TX_OVERFLOW 
    SerialTXOverflow = 0x022,
    /// There was not enough space to store a received
    /// character and the character was dropped.
    /// 
    /// EMBER_SERIAL_RX_OVERFLOW 
    SerialRXOverflow = 0x23,
    /// Detected a UART framing error.
    /// 
    /// EMBER_SERIAL_RX_FRAME_ERROR 
    SerialRXFrameError = 0x24,
    /// Detected a UART parity error.
    /// 
    /// EMBER_SERIAL_RX_PARITY_ERROR 
    SerialRXParityError = 0x25,
    /// There is no received data to process.
    /// 
    /// EMBER_SERIAL_RX_EMPTY 
    SerialRXEmpty = 0x26,
    /// The receive interrupt was not handled in time, and a
    /// character was dropped.
    /// 
    /// EMBER_SERIAL_RX_OVERRUN_ERROR 
    SerialRXOverrunError = 0x27,

    /// The MAC transmit queue is full.
    /// 
    /// EMBER_MAC_TRANSMIT_QUEUE_FULL 
    MACTransmitQueueFull = 0x39,
    /// MAC header FCR error on receive.
    /// 
    /// EMBER_MAC_UNKNOWN_HEADER_TYPE 
    MACUnknownHeaderType = 0x3A,
    /// The MAC can't complete this task because it is scanning.
    /// 
    /// EMBER_MAC_SCANNING 
    MACScanning = 0x3D,
    /// No pending data exists for device doing a data poll.
    /// 
    /// EMBER_MAC_NO_DATA 
    MACNoData = 0x31,
    /// Attempt to scan when we are joined to a network.
    /// 
    /// EMBER_MAC_JOINED_NETWORK 
    MACJoinedNetwork = 0x32,
    /// Scan duration must be 0 to 14 inclusive. Attempt was
    /// made to scan with an incorrect duration value.
    /// 
    /// EMBER_MAC_BAD_SCAN_DURATION 
    MACBadScanDuration = 0x33,
    /// emberStartScan was called with an incorrect scan type.
    /// 
    /// EMBER_MAC_INCORRECT_SCAN_TYPE 
    MACIncorrectScanType = 0x34,
    /// emberStartScan was called with an invalid channel mask.
    /// 
    /// EMBER_MAC_INVALID_CHANNEL_MASK 
    MACInvalidChannelMask = 0x35,
    /// Failed to scan current channel because we were
    /// unable to transmit the relevant MAC command.
    /// 
    /// EMBER_MAC_COMMAND_TRANSMIT_FAILURE 
    MACCommandTransmitFailure = 0x36,
    /// We expected to receive an ACK following the
    /// transmission, but the MAC level ACK was never
    /// received.
    /// 
    /// EMBER_MAC_NO_ACK_RECEIVED 
    MACNoACKReceived = 0x40,
    /// Indirect data message timed out before polled.
    /// 
    /// EMBER_MAC_INDIRECT_TIMEOUT 
    MACIndirectTimeout = 0x42,

    /// The Simulated EEPROM is telling the application
    /// that there is at least one flash page to be erased.
    /// 
    /// The GREEN status means the current page has not
    /// filled above the ERASE_CRITICAL_THRESHOLD.
    /// 
    /// The application should call the function
    /// halSimEepromErasePage when it can to erase a
    /// page.
    /// 
    /// EMBER_SIM_EEPROM_ERASE_PAGE_GREEN 
    SimulatedEEPROMErasePageGreen = 0x43,
    /// The Simulated EEPROM is telling the application
    /// that there is at least one flash page to be erased.
    /// 
    /// The RED status means the current page has filled
    /// above the ERASE_CRITICAL_THRESHOLD.
    /// 
    /// Due to the shrinking availability of write space,
    /// there is a danger of data loss.
    /// 
    /// The application must call the function halSimEepromErasePage
    /// as soon as possible to erase a page.
    /// 
    /// EMBER_SIM_EEPROM_ERASE_PAGE_RED 
    SimulatedEEPROMErasePageRed = 0x44,
    /// The Simulated EEPROM has run out of room to write
    /// any new data and the data trying to be set has been
    /// lost.
    /// 
    /// This error code is the result of ignoring the
    /// SIM_EEPROM_ERASE_PAGE_RED error code.
    /// 
    /// The application must call the function
    /// halSimEepromErasePage to make room for any
    /// further calls to set a token.
    /// 
    /// EMBER_SIM_EEPROM_FULL 
    SimulatedEEPROMFull = 0x45,
    /// A fatal error has occurred while trying to write data to the Flash.
    /// 
    /// The target memory attempting to be programmed is already programmed.
    /// 
    /// The flash write routines were asked to flip a bit from a 0 to 1, which
    /// is physically impossible and the write was therefore inhibited.
    /// 
    /// The data in the flash cannot be trusted after this error.
    /// 
    /// EMBER_ERR_FLASH_WRITE_INHIBITED 
    ErrorFlashWriteInhibited = 0x46,
    /// A fatal error has occurred while trying to write data to
    /// the Flash and the write verification has failed. The
    /// data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding
    /// the life cycles of the flash.
    /// 
    /// EMBER_ERR_FLASH_VERIFY_FAILED 
    ErrorFlashVerifyFailed = 0x47,
    /// Attempt 1 to initialize the Simulated EEPROM has
    /// failed. This failure means the information already
    /// stored in Flash (or a lack thereof), is fatally
    /// incompatible with the token information compiled into
    /// the code image being run.
    /// 
    /// EMBER_SIM_EEPROM_INIT_1_FAILED 
    SimulatedEEPROMInit1Failed = 0x48,
    /// Attempt 2 to initialize the Simulated EEPROM has
    /// failed. This failure means Attempt 1 failed, and the
    /// token system failed to properly reload default tokens
    /// and reset the Simulated EEPROM.
    /// 
    /// EMBER_SIM_EEPROM_INIT_2_FAILED 
    SimulatedEEPROMInit2Failed = 0x49,
    /// Attempt 3 to initialize the Simulated EEPROM has failed.
    /// 
    /// This failure means one or both of the tokens
    /// TOKEN_MFG_NVDATA_VERSION or TOKEN_STACK_NVDATA_VERSION
    /// were incorrect and the token system failed to properly reload default
    /// tokens and reset the Simulated EEPROM.
    /// 
    /// EMBER_SIM_EEPROM_INIT_3_FAILED 
    SimulatedEEPORMInit3Failed = 0x4A,
    /// A fatal error has occurred while trying to write data to
    /// the flash, possibly due to write protection or an
    /// invalid address. The data in the flash cannot be
    /// trusted after this error, and it is possible this error is
    /// the result of exceeding the life cycles of the flash.
    /// 
    /// EMBER_ERR_FLASH_PROG_FAIL 
    ErrorFlashProgFail = 0x4B,
    /// A fatal error has occurred while trying to erase flash,
    /// possibly due to write protection. The data in the flash
    /// cannot be trusted after this error, and it is possible
    /// this error is the result of exceeding the life cycles of
    /// the flash.
    /// 
    /// EMBER_ERR_FLASH_ERASE_FAIL 
    ErrorFlashEraseFail = 0x4C,

    /// The bootloader received an invalid message (failed
    /// attempt to go into bootloader).
    ///
    /// EMBER_ERR_BOOTLOADER_TRAP_TABLE_BAD 
    ErrorBootloaderTrapTableBad = 0x58,
    /// Bootloader received an invalid message (failed
    /// attempt to go into bootloader).
    /// 
    /// EMBER_ERR_BOOTLOADER_TRAP_UNKNOWN 
    ErrorBootloaderTrapUknown = 0x59,
    /// The bootloader cannot complete the bootload
    /// operation because either an image was not found or
    /// the image exceeded memory bounds.
    /// 
    /// EMBER_ERR_BOOTLOADER_NO_IMAGE 
    ErrorBootloaderNoImage = 0x5A,

    /// The APS layer attempted to send or deliver a
    /// message, but it failed.
    /// 
    /// EMBER_DELIVERY_FAILED 
    DeliveryFailed = 0x66,
    /// This binding index is out of range of the current binding table.
    /// 
    /// EMBER_BINDING_INDEX_OUT_OF_RANGE 
    BindingIndexOutOfRange = 0x69,
    /// This address table index is out of range for the
    /// current address table.
    /// 
    /// EMBER_ADDRESS_TABLE_INDEX_OUT_OF_RANGE 
    AddressTableIndexOutOfRange = 0x6A,
    /// An invalid binding table index was given to a function.
    /// 
    /// EMBER_INVALID_BINDING_INDEX 
    InvalidBindingIndex = 0x6C,
    /// The API call is not allowed given the current state of the stack.
    /// 
    /// EMBER_INVALID_CALL 
    InvalidCall = 0x70,
    /// The link cost to a node is not known.
    /// 
    /// EMBER_COST_NOT_KNOWN 
    CostNotKnown = 0x71,
    /// The maximum number of in-flight messages (i.e.
    /// EMBER_APS_UNICAST_MESSAGE_COUNT) has
    /// been reached.
    /// 
    /// EMBER_MAX_MESSAGE_LIMIT_REACHED 
    MaxMessageLimitReached = 0x72,
    /// The message to be transmitted is too big to fit into a
    /// single over-the-air packet.
    /// 
    /// EMBER_MESSAGE_TOO_LONG 
    MessageTooLong = 0x74,
    /// The application is trying to delete or overwrite a
    /// binding that is in use.
    /// 
    /// EMBER_BINDING_IS_ACTIVE 
    BindingIsActive = 0x75,
    /// The application is trying to overwrite an address
    /// table entry that is in use.
    /// 
    /// EMBER_ADDRESS_TABLE_ENTRY_IS_ACTIVE 
    AddressTableEntryIsActive = 0x76,

    /// Conversion is complete.
    /// 
    /// EMBER_ADC_CONVERSION_DONE 
    ADCConversionDone = 0x80,
    /// Conversion cannot be done because a request is being processed.
    /// 
    /// EMBER_ADC_CONVERSION_BUSY 
    ADCConversionBusy = 0x81,
    /// Conversion is deferred until the current request has
    /// been processed.
    /// 
    /// EMBER_ADC_CONVERSION_DEFERRED 
    ADCConversionDeferrred = 0x82,
    /// No results are pending.
    /// 
    /// EMBER_ADC_NO_CONVERSION_PENDING 
    ADCNoConversionPending = 0x84,
    /// Sleeping (for a duration) has been abnormally
    /// interrupted and exited prematurely.
    /// 
    /// EMBER_SLEEP_INTERRUPTED 
    SleepInterruped = 0x85,

    /// The transmit hardware buffer underflowed.
    /// 
    /// EMBER_PHY_TX_UNDERFLOW 
    PHYTXUnderflow = 0x88,
    /// The transmit hardware did not finish transmitting a packet.
    /// 
    /// EMBER_PHY_TX_INCOMPLETE 
    PHYTXIncomplete = 0x89,
    /// An unsupported channel setting was specified.
    /// 
    /// EMBER_PHY_INVALID_CHANNEL 
    PHYInvalidChannel = 0x8A,
    /// An unsupported power setting was specified.
    /// 
    /// EMBER_PHY_INVALID_POWER 
    PHYInvalidPower = 0x8B,
    /// The packet cannot be transmitted because the
    /// physical MAC layer is currently transmitting a packet.
    /// 
    /// This is used for the MAC backoff algorithm.
    /// 
    /// EMBER_PHY_TX_BUSY 
    PHYTXBusy = 0x8C,
    /// The transmit attempt failed because all CCA
    /// attempts indicated that the channel was busy.
    /// 
    /// EMBER_PHY_TX_CCA_FAIL 
    PHYTXCAAFail = 0x8D,
    /// The software installed on the hardware doesn't
    /// recognize the hardware radio type.
    /// 
    /// EMBER_PHY_OSCILLATOR_CHECK_FAILED 
    PHYOcillatorCheckFailed = 0x8E,
    /// The expected ACK was received after the last transmission.
    /// 
    /// EMBER_PHY_ACK_RECEIVED 
    PHYACKReceived = 0x8F,
    /// The stack software has completed initialization and is
    /// ready to send and receive packets over the air.
    /// 
    /// EMBER_NETWORK_UP 
    NetworkUp = 0x90,
    /// The network is not operating.
    /// 
    /// EMBER_NETWORK_DOWN 
    NetworkDown = 0x91,
    /// An attempt to join a network failed.
    /// 
    /// EMBER_JOIN_FAILED 
    JoinFailed = 0x94,
    /// After moving, a mobile node's attempt to re-establish
    /// contact with the network failed.
    /// 
    /// EMBER_MOVE_FAILED 
    MoveFailed = 0x96,
    /// An attempt to join as a router failed due to a ZigBee
    /// versus ZigBee Pro incompatibility. ZigBee devices
    /// joining ZigBee Pro networks (or vice versa) must join
    /// as End Devices, not Routers.
    /// 
    /// EMBER_CANNOT_JOIN_AS_ROUTER 
    CannotJoinAsRouter = 0x98,
    /// The local node ID has changed. The application can
    /// obtain the new node ID by calling
    /// emberGetNodeId().
    /// 
    /// EMBER_NODE_ID_CHANGED 
    NodeIDChanged = 0x99,
    /// The local PAN ID has changed. The application can
    /// obtain the new PAN ID by calling emberGetPanId().
    /// 
    /// EMBER_PAN_ID_CHANGED 
    PANIDChanged = 0x9A,
    /// The network has been opened for joining.
    /// 
    /// EMBER_NETWORK_OPENED 
    NetworkOpened = 0x9C,
    /// The network has been closed for joining.
    /// 
    /// EMBER_NETWORK_CLOSED 
    NetworkClosed = 0x9D,
    /// An attempt to join or rejoin the network failed
    /// because no router beacons could be heard by the
    /// joining node.
    /// 
    /// EMBER_NO_BEACONS 
    NoBeacons = 0xAB,

    /// An attempt was made to join a Secured Network
    /// using a pre-configured key, but the Trust Center sent
    /// back a Network Key in-the-clear when an encrypted
    /// Network Key was required.
    /// 
    /// EMBER_RECEIVED_KEY_IN_THE_CLEAR 
    ReceivedKeyInTheClear = 0xAC,
    /// An attempt was made to join a Secured Network, but
    /// the device did not receive a Network Key.
    /// 
    /// EMBER_NO_NETWORK_KEY_RECEIVED 
    NoNetworkKeyReceived = 0xAD,
    /// After a device joined a Secured Network, a Link Key
    /// was requested but no response was ever received.
    /// 
    /// EMBER_NO_LINK_KEY_RECEIVED 
    NoLinkKeyReceived = 0xAE,
    /// An attempt was made to join a Secured Network
    /// without a pre-configured key, but the Trust Center
    /// sent encrypted data using a pre-configured key.
    /// 
    /// EMBER_PRECONFIGURED_KEY_REQUIRED 
    PreconfiguredKeyRequired = 0xAF,

    /// The node has not joined a network.
    /// 
    /// EMBER_NOT_JOINED 
    NotJoined = 0x93,
    /// The chosen security level (the value of
    /// EMBER_SECURITY_LEVEL) is not supported by the
    /// stack.
    /// 
    /// EMBER_INVALID_SECURITY_LEVEL 
    InvalidSecurityLevel = 0x95,
    /// A message cannot be sent because the network is
    /// currently overloaded.
    /// 
    /// EMBER_NETWORK_BUSY 
    NetworkBusy = 0xA1,
    /// The application tried to send a message using an
    /// endpoint that it has not defined.
    /// 
    /// EMBER_INVALID_ENDPOINT 
    InvalidEndpoint = 0xA3,
    /// The application tried to use a binding that has been
    /// remotely modified and the change has not yet been
    /// reported to the application.
    /// 
    /// EMBER_BINDING_HAS_CHANGED 
    BindingHasChanged = 0xA4,
    /// An attempt to generate random bytes failed because
    /// of insufficient random data from the radio.
    /// 
    /// EMBER_INSUFFICIENT_RANDOM_DATA 
    InsufficientRandomData = 0xA5,
    /// There was an error in trying to encrypt at the APS
    /// Level. This could result from either an inability to
    /// determine the long address of the recipient from the
    /// short address (no entry in the binding table) or there
    /// is no link key entry in the table associated with the
    /// destination, or there was a failure to load the correct
    /// key into the encryption core.
    /// 
    /// EMBER_APS_ENCRYPTION_ERROR 
    APSEncrpytionError = 0xA6,
    /// There was an attempt to form or join a network with
    /// security without calling emberSetInitialSecurityState()
    /// first.
    /// 
    /// EMBER_SECURITY_STATE_NOT_SET 
    SecurityStateNotSet = 0xA8,
    /// There was an attempt to set an entry in the key table
    /// using an invalid long address. An entry cannot be set
    /// using either the local device's or Trust Center's IEEE
    /// address. Or an entry already exists in the table with
    /// the same IEEE address. An Address of all zeros or
    /// all F's are not valid addresses in 802.15.4.
    /// 
    /// EMBER_KEY_TABLE_INVALID_ADDRESS 
    KeyTableInvalidAddress = 0xB3,
    /// There was an attempt to set a security configuration
    /// that is not valid given the other security settings.
    /// 
    /// EMBER_SECURITY_CONFIGURATION_INVALID 
    SecurityConfiguraitonInvalid = 0xB7,
    /// There was an attempt to broadcast a key switch too
    /// quickly after broadcasting the next network key. The
    /// Trust Center must wait at least a period equal to the
    /// broadcast timeout so that all routers have a chance
    /// to receive the broadcast of the new network key.
    /// 
    /// EMBER_TOO_SOON_FOR_SWITCH_KEY 
    TooSoonForSwitchKey = 0xB8,
    /// The message could not be sent because the link key
    /// corresponding to the destination is not authorized for
    /// use in APS data messages. APS Commands (sent
    /// by the stack) are allowed. To use it for encryption of
    /// APS data messages it must be authorized using a
    /// key agreement protocol (such as CBKE).
    /// 
    /// EMBER_KEY_NOT_AUTHORIZED 
    KeyNotAuthotized = 0xBB,
    /// The security data provided was not valid, or an integrity check failed.
    /// 
    /// EMBER_SECURITY_DATA_INVALID 
    SecurityDataInvalid = 0xBD,
    /// A ZigBee route error command frame was received
    /// indicating that a source routed message from this
    /// node failed en route.
    /// 
    /// EMBER_SOURCE_ROUTE_FAILURE 
    SourceRouteFailure = 0xA9,
    /// A ZigBee route error command frame was received
    /// indicating that a message sent to this node along a
    /// many-to-one route failed en route. The route error
    /// frame was delivered by an ad-hoc search for a
    /// functioning route.
    /// 
    /// EMBER_MANY_TO_ONE_ROUTE_FAILURE 
    ManyToOneRouteFaulure = 0xAA,
    /// A critical and fatal error indicating that the version of
    /// the stack trying to run does not match with the chip it
    /// is running on. The software (stack) on the chip must
    /// be replaced with software that is compatible with the
    /// chip.
    /// 
    /// EMBER_STACK_AND_HARDWARE_MISMATCH 
    StackAndHardwareMismatch = 0xB0,
    /// An index was passed into the function that was
    /// larger than the valid range.
    /// 
    /// EMBER_INDEX_OUT_OF_RANGE 
    IndexOutOfRange = 0xB1,
    /// There are no empty entries left in the table.
    /// 
    /// EMBER_TABLE_FULL 
    TableFull = 0xB4,
    /// The requested table entry has been erased and
    /// contains no valid data.
    /// 
    /// EMBER_TABLE_ENTRY_ERASED 
    TableEntryErased = 0xB6,
    /// The requested function cannot be executed because
    /// the library that contains the necessary functionality is
    /// not present.
    /// 
    /// EMBER_LIBRARY_NOT_PRESENT 
    LibraryNotPresent = 0xB5,
    /// The stack accepted the command and is currently
    /// processing the request. The results will be returned
    /// via an appropriate handler.
    /// 
    /// EMBER_OPERATION_IN_PROGRESS 
    OperationInProgress = 0xBA,

    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_0 
    ApplicationError0 = 0xF0,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_1
    ApplicationError1 = 0xF1,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_2
    ApplicationError2 = 0xF2,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_3
    ApplicationError3 = 0xF3,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_4
    ApplicationError4 = 0xF4,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_5
    ApplicationError5 = 0xF5,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_6
    ApplicationError6 = 0xF6,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_7
    ApplicationError7 = 0xF7,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_8
    ApplicationError8 = 0xF8,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_9
    ApplicationError9 = 0xF9,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_10
    ApplicationError10 = 0xFA,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_11
    ApplicationError11 = 0xFB,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_12
    ApplicationError12 = 0xFC,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_13
    ApplicationError13 = 0xFD,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_14
    ApplicationError14 = 0xFE,
    /// This error is reserved for customer application use.
    /// This will never be returned from any portion of the
    /// network stack or HAL.
    /// 
    /// EMBER_APPLICATION_ERROR_15
    ApplicationError15 = 0xFF,
}

/// Either marks an event as inactive or specifies the units for the event execution time.
/// 
/// https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf (P.g. 38)
#[repr(u8)]
pub enum EventUnits {
    /// The event is not scheduled to run.
    Inactive = 0x00,
    /// The execution time is in approximate milliseconds.
    MSTime = 0x01,
    /// The execution time is in 'binary' quarter
    /// seconds (256 approximate milliseconds each).
    QSTime = 0x02,
    /// The execution time is in 'binary' minutes
    /// (65536 approximate milliseconds each).
    MinuteTime = 0x03,
}

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