use bitmask::bitmask;

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
#[repr(u8)]
pub enum NodeType {
    /// Device is not joined.
    /// 
    /// EMBER_UNKNOWN_DEVICE 
    UnknownDevice = 0x00,
    /// Will relay messages and can act as a parent to other nodes.
    /// 
    /// EMBER_COORDINATOR 
    Coordinator = 0x01,
    /// Will relay messages and can act as a parent to other nodes.
    /// 
    /// EMBER_ROUTER 
    Router = 002,
    /// Communicates only with its parent and will not relay messages.
    /// 
    /// EMBER_END_DEVICE 
    EndDevice = 0x03,
    /// An end device whose radio can be turned off to save power.
    /// 
    /// The application must poll to receive messages.
    /// 
    /// EMBER_SLEEPY_END_DEVICE 
    SleepyEndDevice = 0x04,
}

/// The possible join states for a node.
/// 
/// EmberNetworkStatus
#[repr(u8)]
pub enum NetworkStatus {
    /// The node is not associated with a network in any way.
    /// 
    /// EMBER_NO_NETWORK 
    NoNetwork = 0x00,
    /// The node is currently attempting to join a network.
    /// 
    /// EMBER_JOINING_NETWORK 
    JoiningNetwork = 0x01,
    /// The node is joined to a network.
    /// 
    /// EMBER_JOINED_NETWORK 
    JoinedNetwork = 0x02,
    /// The node is an end device joined to a network
    /// but its parent is not responding.
    /// 
    /// EMBER_JOINED_NETWORK_NO_PARENT 
    JoinedNetworkNoParent = 0x03,
    /// The node is in the process of leaving its current network.
    /// 
    /// EMBER_LEAVING_NETWORK 
    LeavingNetowrk = 004,
}

/// Incoming message types.
/// 
/// EmberIncomingMessageType 
#[repr(u8)]
pub enum IncomingMessageType  {
    /// Unicast.
    /// 
    /// EMBER_INCOMING_UNICAST 
    Unicast = 0x00,
    /// Unicast reply.
    /// 
    /// EMBER_INCOMING_UNICAST_REPLY 
    UnicastReply = 0x01,
    /// Multicast.
    /// 
    /// EMBER_INCOMING_MULTICAST 
    Multicast = 0x02,
    /// Multicast sent by the local device.
    /// 
    /// EMBER_INCOMING_MULTICAST_LOOPBACK 
    MulticastLoopback = 0x03,
    /// Broadcast.
    /// 
    /// EMBER_INCOMING_BROADCAST 
    Broadcast = 0x04,
    /// Broadcast sent by the local device.
    /// 
    /// EMBER_INCOMING_BROADCAST_LOOPBACK 
    BroadcastLoopback = 0x05,
    /// Many to one route request.
    /// 
    /// EMBER_INCOMING_MANY_TO_ONE_ROUTE_REQUEST 
    ManyToOneRouteRequest = 0x06
}

/// Outgoing message types.
/// 
/// EmberOutgoingMessageType 
#[repr(u8)]
pub enum OutgoingMessageType {
    /// Unicast sent directly to an EmberNodeId.
    /// 
    /// EMBER_OUTGOING_DIRECT 
    Direct = 0x00,
    /// Unicast sent using an entry in the address table.
    /// 
    /// EMBER_OUTGOING_VIA_ADDRESS_TABLE 
    ViaAddressTable = 001,
    /// Unicast sent using an entry in the binding table.
    /// 
    /// EMBER_OUTGOING_VIA_BINDING 
    ViaBinding = 0x02,
    /// Multicast message.
    /// 
    /// This value is passed to emberMessageSentHandler() only.
    /// 
    /// It may not be passed to emberSendUnicast().
    /// 
    /// EMBER_OUTGOING_MULTICAST 
    Multicast = 0x03,
    /// Broadcast message.
    /// 
    /// This value is passed to emberMessageSentHandler() only.
    /// 
    /// It may not be passed to emberSendUnicast().
    /// 
    /// EMBER_OUTGOING_BROADCAST 
    Broadcast = 0x04,
}

/// MAC passthrough message type flags.
/// 
/// EmberMacPassthroughType 
#[repr(u8)]
pub enum MacPassthroughType  {
    /// No MAC passthrough messages.
    /// 
    /// EMBER_MAC_PASSTHROUGH_NONE 
    None = 0x00,
    /// SE InterPAN messages.
    /// 
    /// EMBER_MAC_PASSTHROUGH_SE_INTERPAN 
    SEInterPAN = 0x01,
    /// Legacy EmberNet messages.
    /// 
    /// EMBER_MAC_PASSTHROUGH_EMBERNET 
    EmberNet = 0x02,
    /// Legacy EmberNet messages filtered by their source address.
    /// 
    /// EMBER_MAC_PASSTHROUGH_EMBERNET_SOURCE 
    EmberNetSource = 0x04
}

/// Binding types.
/// 
/// EmberBindingType 
#[repr(u8)]
pub enum BindingType {
    /// A binding that is currently not in use.
    /// 
    /// EMBER_UNUSED_BINDING 
    Unused = 0x00,
    /// A unicast binding whose 64-bit identifier is the destination EUI64.
    /// 
    /// EMBER_UNICAST_BINDING 
    Unicast = 0x01,
    /// A unicast binding whose 64-bit identifier is the aggregator EUI64.
    /// 
    /// EMBER_MANY_TO_ONE_BINDING 
    ManyToOne = 0x02,
    /// A multicast binding whose 64-bit identifier is the group address.
    /// 
    /// A multicast binding can be used to send messages to the group and
    /// to receive messages sent to the group.
    /// 
    /// EMBER_MULTICAST_BINDING 
    Multicast = 0x03
}

bitmask! {
    /// Options to use when sending a message.
    mask ApsOption: u16 where
    /// Options to use when sending a message.
    flags ApsOptionFlags {
        /// No options.
        /// 
        /// EMBER_APS_OPTION_NONE 
        None = 0x0000,
        /// Send the message using APS Encryption, using the Link
        /// Key shared with the destination node to encrypt the data
        /// at the APS Level.
        /// 
        /// EMBER_APS_OPTION_ENCRYPTION 
        Encryption = 0x0020,
        /// Resend the message using the APS retry mechanism.
        /// 
        /// EMBER_APS_OPTION_RETRY 
        Retry = 0x0040,
        /// Causes a route discovery to be initiated
        /// if no route to the destination is known.
        /// 
        /// EMBER_APS_OPTION_ENABLE_ROUTE_DISCOVERY 
        EnableRouteDiscovery = 0x0100,
        /// Causes a route discovery to be initiated even if one is known.
        /// 
        /// EMBER_APS_OPTION_FORCE_ROUTE_DISCOVERY 
        ForceRouteDiscovery = 0x0200,
        /// Include the source EUI64 in the network frame.
        /// 
        /// EMBER_APS_OPTION_SOURCE_EUI64 
        SourceEUI64 = 0x0800,
        /// Send a ZDO request to discover the node ID of the
        /// destination, if it is not already known.
        /// 
        /// EMBER_APS_OPTION_ENABLE_ADDRESS_DISCOVERY 
        EnableAddressDiscovery = 0x1000,
        /// Reserved.
        /// 
        /// EMBER_APS_OPTION_POLL_RESPONSE 
        PollResponse = 0x2000,
        /// This incoming message is a ZDO request not handled by
        /// the EmberZNet stack, and the application is responsible
        /// for sending a ZDO response.
        ///
        /// This flag is used only when  the ZDO is configured to have
        /// requests handled by the application.
        /// 
        /// See the EZSP_CONFIG_APPLICATION_ZDO_FLAGS configuration
        /// parameter for more information.
        /// 
        /// EMBER_APS_OPTION_ZDO_RESPONSE_REQUIRED 
        ZDOResponseRequired = 0x4000,
        /// This message is part of a fragmented message.
        /// 
        /// This option may only be set for unicasts.
        /// 
        /// The groupId field gives the index of this fragment
        /// in the low-order byte.
        /// 
        /// If the low-order byte is zero this is the first
        /// fragment and the high-order byte contains the number
        /// of fragments in the message.
        /// 
        /// EMBER_APS_OPTION_FRAGMENT 
        Fragment = 0x8000
    }
}

/// Decision made by the trust center when a node attempts to join.
/// 
/// EmberJoinDecision
#[repr(u8)]
pub enum JoinDecision {
    /// Allow the node to join.
    /// 
    /// The joining node should have a pre-configured key.
    /// 
    /// The security data sent to it will be encrypted with that key.
    /// 
    /// EMBER_USE_PRECONFIGURED_KEY 
    UsePreconfiguredKey = 0x00,
    /// Allow the node to join.
    /// 
    /// Send the network key in-the-clear to the joining device.
    /// 
    /// EMBER_SEND_KEY_IN_THE_CLEAR 
    SendKeyInTheClear = 0x01,
    /// Deny join.
    /// 
    /// EMBER_DENY_JOIN 
    DenyJoin = 0x02,
    /// Take no action.
    /// 
    /// EMBER_NO_ACTION 
    NoAction = 0x03
}

bitmask! {
    /// This is the Initial Security Bitmask that controls
    /// the use of various security features.
    /// 
    /// EmberInitialSecurityBitmask 
    mask InitialSecurityBitmask: u16 where
    /// This is the Initial Security Bitmask that controls
    /// the use of various security features.
    /// 
    /// EmberInitialSecurityBitmask 
    flags InitialSecurityBitmaskFlags {
        /// This enables ZigBee Standard Security on the node.
        /// 
        /// EMBER_STANDARD_SECURITY_MODE 
        StandardSecurityMode = 0x0000,
        /// This enables Distributed Trust Center Mode for the device
        /// forming the network.
        /// 
        /// (Previously known as EMBER_NO_TRUST_CENTER_MODE)
        /// 
        /// EMBER_DISTRIBUTED_TRUST_CENTER_MODE 
        DistributedTrustCenterMode = 0x0002,
        /// This enables a Global Link Key for the Trust Center.
        /// 
        /// All nodes will share the same Trust Center Link Key.
        /// 
        /// EMBER_TRUST_CENTER_GLOBAL_LINK_KEY 
        TrustCenterGlobalLinkKey = 0x0004,
        /// This enables devices that perform MAC Association with a
        /// pre-configured Network Key to join the network.
        /// 
        /// It is only set on the Trust Center.
        /// 
        /// EMBER_PRECONFIGURED_NETWORK_KEY_MODE 
        PreconfiguredNetworkKeyMode = 0x0008,
        /// This denotes that the preconfiguredKey is not the actual
        /// Link Key but a Secret Key known only to the Trust Center.
        /// 
        /// It is hashed with the IEEE Address of the destination
        /// device in order to create the actual Link Key used in
        /// encryption.
        /// 
        /// This is bit is only used by the Trust Center.
        /// 
        /// The joining device need not set this.
        /// 
        /// EMBER_TRUST_CENTER_USES_HASHED_LINK_KEY 
        TrustCenterUsesHashedLinkKey = 0x0084,
        /// This denotes that the preconfiguredKey element has valid
        /// data that should be used to configure the initial security
        /// state.
        /// 
        /// EMBER_HAVE_PRECONFIGURED_KEY 
        HavePreconfiguredKey = 0x0100,
        /// This denotes that the networkKey element has valid data
        /// that should be used to configure the initial security state.
        /// 
        /// EMBER_HAVE_NETWORK_KEY 
        HaveNetworkKey = 0x0200,
        /// This denotes to a joining node that it should attempt to
        /// acquire a Trust Center Link Key during joining. This is only
        /// necessary if the device does not have a pre-configured
        /// key.
        /// 
        /// EMBER_GET_LINK_KEY_WHEN_JOINING 
        GetLinkKeyWhenJoining = 0x0400,
        /// This denotes that a joining device should only accept an
        /// encrypted network key from the Trust Center (using its pre-
        /// configured key).
        /// 
        /// A key sent in-the-clear by the Trust Center will be rejected
        /// and the join will fail.
        /// 
        /// This option is only valid when utilizing a pre-configured key.
        /// 
        /// EMBER_REQUIRE_ENCRYPTED_KEY 
        RequireEncryptedKey = 0x0800,
        /// This denotes whether the device should NOT reset its
        /// outgoing frame counters (both NWK and APS) when
        /// ::emberSetInitialSecurityState() is called.
        /// 
        /// Normally it is advised to reset the frame counter before
        /// joining a new network.
        /// 
        /// However in cases where a device is joining to the same
        /// network again (but not using ::emberRejoinNetwork()) it
        /// should keep the NWK and APS frame counters stored in its tokens.
        /// 
        /// EMBER_NO_FRAME_COUNTER_RESET 
        NoFrameConterReset = 0x1000,
        /// This denotes that the device should obtain its
        /// preconfigured key from an installation code stored in the
        /// manufacturing token.
        /// 
        /// The token contains a value that will be hashed to obtain
        /// the actual preconfigured key. If that token is not valid,
        /// then the call to emberSetInitialSecurityState() will fail.
        /// 
        /// EMBER_GET_PRECONFIGURED_KEY_FROM_INSTALL_CODE
        GetPreconfiguredKeyFromInstallCode = 0x2000,
        /// This denotes that the EmberInitialSecurityState::preconfiguredTrustCenterEui64
        /// has a value in it containing the trust center EUI64.
        /// 
        /// The device will only join a network and accept commands from
        /// a trust center with that EUI64.
        /// 
        /// Normally this bit is NOT set, and the EUI64 of the trust center
        /// is learned during the join process.
        /// 
        /// When commissioning a device to join onto an existing network,
        /// which is using a trust center, and without sending any messages,
        /// this bit must be set and the field EmberInitialSecurityState::preconfiguredTrustCenterEui64
        /// must be populated with the appropriate EUI64.
        /// 
        /// EMBER_HAVE_TRUST_CENTER_EUI64 
        HaveTrustCenterEUI64 = 0x0040
    }
}

bitmask! {
    /// This is the Current Security Bitmask that details
    /// the use of various security features.
    /// 
    /// EmberCurrentSecurityBitmask 
    mask CurrentSecurityBitmask: u16 where
    /// This is the Current Security Bitmask that details
    /// the use of various security features.
    /// 
    /// EmberCurrentSecurityBitmask 
    flags CurrentSecurityBitmaskFlags {
        /// This denotes that the device is running in
        /// a network with ZigBee Standard Security.
        /// 
        /// EMBER_STANDARD_SECURITY_MODE 
        StandardSecurityMode = 0x0000,
        /// This denotes that the device is running in a
        /// network without a centralized Trust Center.
        /// 
        /// EMBER_DISTRIBUTED_TRUST_CENTER_MODE 
        DistributedTrustCenterMode = 0x0002,
        /// This denotes that the device has a Global Link Key.
        /// 
        /// The Trust Center Link Key is the same across multiple nodes.
        /// 
        /// EMBER_GLOBAL_LINK_KEY 
        GlobalLinkKey = 0x0004,
        /// This denotes that the node has a Trust Center Link Key.
        /// 
        /// EMBER_HAVE_TRUST_CENTER_LINK_KEY 
        HaveTrustCenterLinkKey = 0x0010,
        /// This denotes that the Trust Center is using a Hashed Link Key.
        /// 
        /// EMBER_TRUST_CENTER_USES_HASHED_LINK_KEY 
        TrustCenterUsesHashedLinkKey = 0x0084,
    }
}

/// Describes the type of ZigBee security key.
/// 
/// EmberKeyType 
#[repr(u8)]
pub enum KeyType {
    /// A shared key between the Trust Center and a device.
    /// 
    /// EMBER_TRUST_CENTER_LINK_KEY 
    TrustCenterLink = 0x01,
    /// The current active Network Key used by all devices in the network.
    /// 
    /// EMBER_CURRENT_NETWORK_KEY 
    CurrentNetwork = 0x03,
    /// The alternate Network Key that was previously in use,
    /// or the newer key that will be switched to.
    /// 
    /// EMBER_NEXT_NETWORK_KEY 
    NextNetwork = 0x04,
    /// An Application Link Key shared with another (non-Trust Center) device.
    /// 
    /// EMBER_APPLICATION_LINK_KEY 
    ApplicationLink = 0x05
}

bitmask! {
    /// Describes the presence of valid data within
    /// the EmberKeyStruct structure.
    /// 
    /// EmberKeyStructBitmask 
    mask KeyStructBitmask: u16 where
    /// Describes the presence of valid data within
    /// the EmberKeyStruct structure.
    /// 
    /// EmberKeyStructBitmask 
    flags KeyStructBitmaskFlags {
        /// The key has a sequence number associated with it.
        /// 
        /// EMBER_KEY_HAS_SEQUENCE_NUMBER 
        HasSequenceNumber = 0x0001,
        /// The key has an outgoing frame counter associated with it.
        /// 
        /// EMBER_KEY_HAS_OUTGOING_FRAME_COUNTER 
        HasOutgoingFrmaeCounter = 0x0002,
        /// The key has an incoming frame counter associated with it.
        /// 
        /// EMBER_KEY_HAS_INCOMING_FRAME_COUNTER 
        HasIncomingFrameCounter = 0x0004,
        /// The key has a Partner IEEE address associated with it.
        /// 
        /// EMBER_KEY_HAS_PARTNER_EUI64 
        HasPartnerEUI64 = 0x0008,
    }
}

/// The status of the device update.
/// 
/// EmberDeviceUpdate
#[repr(u8)]
pub enum DeviceUpdate {
    StandardSecuritySecuredRejoin = 0x0,
    StandardSecurityUnsecuredJoin = 0x1,
    DeviceLeft = 0x2,
    StanardSecurityUnsecuredRejoin = 0x3,
}

/// The status of the attempt to establish a key.
/// 
/// EmberKeyStatus 
#[repr(u8)]
pub enum KeyStatus {
    /// EMBER_APP_LINK_KEY_ESTABLISHED 
    AppLinkKeyEstablished = 0x01,
    /// EMBER_TRUST_CENTER_LINK_KEY_ESTABLISHED 
    TrustCenterLinkKeyEstablished = 0x03,
    /// EMBER_KEY_ESTABLISHMENT_TIMEOUT 
    KeyEstablishmentTimeout = 0x04,
    /// EMBER_KEY_TABLE_FULL 
    KeyTableFull = 0x05,
    /// EMBER_TC_RESPONDED_TO_KEY_REQUEST 
    RespondedToKeyRequest = 0x06,
    /// EMBER_TC_APP_KEY_SENT_TO_REQUESTER 
    AppKeySentToRequester = 0x07,
    /// EMBER_TC_RESPONSE_TO_KEY_REQUEST_FAILED 
    ResponseToKeyRequestFailed = 0x08,
    /// EMBER_TC_REQUEST_KEY_TYPE_NOT_SUPPORTED 
    RequestKeyTypeNotSupported = 0x09,
    /// EMBER_TC_NO_LINK_KEY_FOR_REQUESTER 
    NoLinkKeyForRequested = 0x0A,
    /// EMBER_TC_REQUESTER_EUI64_UNKNOWN 
    RequestedEUI64Unknown = 0x0B,
    /// EMBER_TC_RECEIVED_FIRST_APP_KEY_REQUEST 
    ReceivedFirstAppKeyRequest = 0x0C,
    /// EMBER_TC_TIMEOUT_WAITING_FOR_SECOND_APP_KEY_REQUEST 
    TimeoutWaitingForSecondAppKeyRequest = 0x0D,
    /// EMBER_TC_NON_MATCHING_APP_KEY_REQUEST_RECEIVED 
    NonMatchingAppKeyRequestReceived = 0x0E,
    /// EMBER_TC_FAILED_TO_SEND_APP_KEYS 
    FailedToSendAppKeys = 0x0F,
    /// EMBER_TC_FAILED_TO_STORE_APP_KEY_REQUEST 
    FailtedtoStoreAppKeyRequest = 0x10,
    /// EMBER_TC_REJECTED_APP_KEY_REQUEST 
    RejectedAppKeyRequest = 0x11,
}

/// Defines the events reported to the application by the readAndClearCounters command.
#[repr(u8)]
pub enum CounterType {
    /// The MAC received a broadcast.
    /// 
    /// EMBER_COUNTER_MAC_RX_BROADCAST 
    MACRXBroadccast = 0,
    /// The MAC transmitted a broadcast.
    /// 
    /// EMBER_COUNTER_MAC_TX_BROADCAST 
    MACTXBroadcast = 1,
    /// The MAC received a unicast
    /// 
    /// EMBER_COUNTER_MAC_RX_UNICAST 
    MACRXUnicast = 2,
    /// The MAC successfully transmitted a unicast.
    /// 
    /// EMBER_COUNTER_MAC_TX_UNICAST_SUCCESS 
    MACTXUnicastSuccess = 3,
    /// The MAC retried a unicast.
    /// 
    /// EMBER_COUNTER_MAC_TX_UNICAST_RETRY 
    MACTXUnicastRetry = 4,
    /// The MAC unsuccessfully transmitted a unicast.
    /// 
    /// EMBER_COUNTER_MAC_TX_UNICAST_FAILED 
    MACTXUnicastFailed = 5,
    /// The APS layer received a data broadcast.
    /// 
    /// EMBER_COUNTER_APS_DATA_RX_BROADCAST 
    APSDataRXBroadcast = 6,
    /// The APS layer transmitted a data broadcast.
    /// 
    /// EMBER_COUNTER_APS_DATA_TX_BROADCAST 
    APSDataTXBroadcast = 7,
    /// The APS layer received a data unicast.
    /// 
    /// EMBER_COUNTER_APS_DATA_RX_UNICAST 
    APSDataRXUnicast = 8,
    /// The APS layer successfully transmitted a data unicast.
    /// 
    /// EMBER_COUNTER_APS_DATA_TX_UNICAST_SUCCESS 
    APSDATATXUnicastSuccess = 9,
    /// The APS layer retried a data unicast.
    /// 
    /// EMBER_COUNTER_APS_DATA_TX_UNICAST_RETRY 
    APSDATATXUnicastRetry = 10,
    /// The APS layer unsuccessfully transmitted a data unicast.
    /// 
    /// EMBER_COUNTER_APS_DATA_TX_UNICAST_FAILED 
    APSDataTXUnicastFailed = 11,
    /// The network layer successfully submitted a new route discovery to the MAC.
    /// 
    /// EMBER_COUNTER_ROUTE_DISCOVERY_INITIATED 
    RouteDiscoveryInitialted = 12,
    /// An entry was added to the neighbor table.
    /// 
    /// EMBER_COUNTER_NEIGHBOR_ADDED 
    NeighborAdded = 13,
    /// An entry was removed from the neighbor table.
    /// 
    /// EMBER_COUNTER_NEIGHBOR_REMOVED 
    NeighborRemoved = 14,
    /// A neighbor table entry became stale because it had not been heard from.
    /// 
    /// EMBER_COUNTER_NEIGHBOR_STALE 
    NeighborStale = 15,
    /// A node joined or rejoined to the network via this node.
    /// 
    /// EMBER_COUNTER_JOIN_INDICATION 
    JoinIndication = 16,
    /// An entry was removed from the child table.
    /// 
    /// EMBER_COUNTER_CHILD_REMOVED 
    ChildRemobed = 17,
    /// EZSP-UART only. An overflow error occurred in the UART.
    /// 
    /// EMBER_COUNTER_ASH_OVERFLOW_ERROR 
    ASHOverflowError = 18,
    /// EZSP-UART only. A framing error occurred in the UART.
    /// 
    /// EMBER_COUNTER_ASH_FRAMING_ERROR 
    ASHFramingError = 19,
    /// EZSP-UART only. An overrun error occurred in the UART.
    /// 
    /// EMBER_COUNTER_ASH_OVERRUN_ERROR 
    ASHOverrunError = 20,
    /// A message was dropped at the network layer
    /// because the NWK frame counter was not higher
    /// than the last message seen from that source.
    /// 
    /// EMBER_COUNTER_NWK_FRAME_COUNTER_FAILURE 
    NWKFramecounterFailure = 21,
    /// A message was dropped at the APS layer because
    /// the APS frame counter was not higher than the last
    /// message seen from that source.
    /// 
    /// EMBER_COUNTER_APS_FRAME_COUNTER_FAILURE 
    APSFrameCounterFailure = 22,
    /// Utility counter for general debugging use.
    /// 
    /// EMBER_COUNTER_UTILITY 
    Utility = 23,
    /// A message was dropped at the APS layer because
    /// it had APS encryption but the key associated with
    /// the sender has not been authenticated, and thus the
    /// key is not authorized for use in APS data messages.
    /// 
    /// EMBER_COUNTER_APS_LINK_KEY_NOT_AUTHORIZED 
    APSLinkKeyNotAuthorized = 24,
    /// An NWK-encrypted message was received but
    /// dropped because decryption failed.
    /// 
    /// EMBER_COUNTER_NWK_DECRYPTION_FAILURE 
    NWKDecryptionDFailure = 25,
    /// An APS encrypted message was received but
    /// dropped because decryption failed.
    /// 
    /// EMBER_COUNTER_APS_DECRYPTION_FAILURE 
    APSDecryptionFailure = 26,
    /// The number of times we failed to allocate a set of
    /// linked packet buffers.
    /// 
    /// This doesn't necessarily mean that the packet buffer
    /// count was 0 at the time, but that the number requested
    /// was greater than the number free.
    /// 
    /// EMBER_COUNTER_ALLOCATE_PACKET_BUFFER_FAILURE 
    AllocatePacketBufferFailure = 27,
    /// The number of relayed unicast packets.
    /// 
    /// EMBER_COUNTER_RELAYED_UNICAST 
    RelayedUnicast = 28,
    /// The number of times we dropped a packet due to
    /// reaching the preset PHY to MAC queue limit
    /// (sli_802154mac_max_phy_to_mac_queue_length).
    /// 
    /// EMBER_COUNTER_PHY_TO_MAC_QUEUE_LIMIT_REACHED 
    PHYToMACQueueLimitReached = 29,
    /// The number of times we dropped a packet due to
    /// the packet-validate library checking a packet and
    /// rejecting it due to length or other formatting
    /// problems.
    /// 
    /// EMBER_COUNTER_PACKET_VALIDATE_LIBRARY_DROPPED_COUNT
    PacketValidateLibraryDroppedCount = 30,
    /// The number of times the NWK retry queue is full
    /// and a new message failed to be added.
    /// 
    /// EMBER_COUNTER_TYPE_NWK_RETRY_OVERFLOW 
    NWKRetryOverflow = 31,
    /// The number of times the PHY layer was unable to
    /// transmit due to a failed CCA.
    /// 
    /// EMBER_COUNTER_PHY_CCA_FAIL_COUNT 
    PHYCCAFailCount = 32,
    /// The number of times an NWK broadcast was
    /// dropped because the broadcast table was full.
    /// 
    /// EMBER_COUNTER_BROADCAST_TABLE_FULL 
    BroadcastTableFull = 33,
    /// The number of low priority packet traffic arbitration requests.
    /// 
    /// EMBER_COUNTER_PTA_LO_PRI_REQUESTED 
    PTXLOPRIRequested = 34,
    /// The number of high priority packet traffic arbitration requests.
    /// 
    /// EMBER_COUNTER_PTA_HI_PRI_REQUESTED 
    PTXHIPRIRequested = 35,
    /// The number of low priority packet traffic arbitration requests denied.
    /// 
    /// EMBER_COUNTER_PTA_LO_PRI_DENIED 
    PTXLOPRIDenied = 36,
    /// The number of high priority packet traffic arbitration requests denied.
    /// 
    /// EMBER_COUNTER_PTA_HI_PRI_DENIED 
    PTXHIPRIDenied = 37,
    /// The number of aborted low priority packet traffic arbitration transmissions.
    /// 
    /// EMBER_COUNTER_PTA_LO_PRI_TX_ABORTED 
    PTXLOPRITXAborted = 38,
    /// The number of aborted high priority packet traffic arbitration transmissions.
    /// 
    /// EMBER_COUNTER_PTA_HI_PRI_TX_ABORTED 
    PTXHIPRITXAborted = 39,
    /// A placeholder giving the number of Ember counter types.
    /// 
    /// EMBER_COUNTER_TYPE_COUNT 
    TypeCount = 40,
}

/// The type of method used for joining.
/// 
/// EmberJoinMethod 
#[repr(u8)]
pub enum JoinMethod {
    /// Normally devices use MAC Association to join a network, which
    /// respects the "permit joining" flag in the MAC Beacon.
    /// 
    /// This value should be used by default.
    /// 
    /// EMBER_USE_MAC_ASSOCIATION 
    MACAssociation = 0x0,
    /// For those networks where the "permit joining" flag is never turned
    /// on, they will need to use a ZigBee NWK Rejoin.
    /// 
    /// This value causes the rejoin to be sent without NWK security and the
    /// Trust Center will be asked to send the NWK key to the device.
    /// 
    /// The NWK key sent to the device can be encrypted with the device's 
    /// corresponding Trust Center link key.
    /// 
    /// That is determined by the ::EmberJoinDecision on the Trust Center
    /// returned by the ::emberTrustCenterJoinHandler().
    /// 
    /// EMBER_USE_NWK_REJOIN 
    NWKRejoin = 0x1,
    /// For those networks where the "permit joining" flag is never turned
    /// on, they will need to use an NWK Rejoin.
    /// 
    /// If those devices have been preconfigured with the NWK key (including
    /// sequence number) they can use a secured rejoin.
    /// 
    /// This is only necessary for end devices since they need a parent.
    /// 
    /// Routers can simply use the ::EMBER_USE_CONFIGURED_NWK_STATE join method
    /// below.
    /// 
    /// EMBER_USE_NWK_REJOIN_HAVE_NWK_KEY 
    NWKRejoinHaveNWKKey = 0x2,
    /// For those networks where all network and security information is
    /// known ahead of time, a router device may be commissioned such
    /// that it does not need to send any messages to begin
    /// communicating on the network.
    /// 
    /// EMBER_USE_CONFIGURED_NWK_STATE 
    ConfiguredNWKState = 0x3,
}

/// Flags for controlling which incoming ZDO requests
/// are passed to the application.
/// 
/// EmberZdoConfigurationFlags 
// TODO: is bitmask?
#[repr(u8)]
pub enum ZdoConfigurationFlags {
    /// Set this flag in order to receive supported ZDO request
    /// messages via the incomingMessageHandler callback.
    /// 
    /// A supported ZDO request is one that is handled by the
    /// EmberZNet stack.
    /// 
    /// The stack will continue to handle the request and send
    /// the appropriate ZDO response even if this configuration
    /// option is enabled.
    /// 
    /// EMBER_APP_RECEIVES_SUPPORTED_ZDO_REQUESTS 
    AppReceivedSuportedZDOequests = 0x01,
    /// Set this flag in order to receive unsupported ZDO
    /// request messages via the incomingMessageHandler callback.
    /// 
    /// An unsupported ZDO request is one that is not handled
    /// by the EmberZNet stack, other than to send a 'not supported'
    /// ZDO response.
    /// 
    /// If this configuration option is enabled, the stack will no
    /// longer send any ZDO response, and it is the application's
    /// responsibility to do so.
    /// 
    /// EMBER_APP_HANDLES_UNSUPPORTED_ZDO_REQUESTS 
    AppHandlesUnsupportedZDORequests = 0x02,
    /// Set this flag in order to receive the following ZDO
    /// request messages via the incomingMessageHandler callback:
    /// - SIMPLE_DESCRIPTOR_REQUEST
    /// - MATCH_DESCRIPTORS_REQUEST
    /// - ACTIVE_ENDPOINTS_REQUEST
    /// 
    /// If this configuration option is enabled, the stack will no
    /// longer send any ZDO response for these requests, and it is
    /// the application's responsibility to do so.
    /// 
    /// EMBER_APP_HANDLES_ZDO_ENDPOINT_REQUESTS 
    AppHandlesZDOEndpointRequers = 0x04,
    /// Set this flag in order to receive the following ZDO
    /// request messages via the incomingMessageHandler callback:
    /// - BINDING_TABLE_REQUEST
    /// - BIND_REQUEST
    /// - UNBIND_REQUEST
    /// 
    /// If this configuration option is enabled, the stack will no
    /// longer send any ZDO response for these requests, and it is the
    /// application's responsibility to do so.
    /// 
    /// EMBER_APP_HANDLES_ZDO_BINDING_REQUESTS 
    AppHandlesZDOBindingRequeset = 0x08,
}

/// Type of concentrator.
/// 
/// EmberConcentratorType 
#[repr(u16)]
pub enum ConcentratorType {
    /// A concentrator with insufficient memory to store source
    /// routes for the entire network. Route records are sent to the
    /// concentrator prior to every inbound APS unicast.
    /// 
    /// EMBER_LOW_RAM_CONCENTRATOR 
    LowRAM = 0xFFF8,

    /// A concentrator with sufficient memory to store source
    /// routes for the entire network. Remote nodes stop sending
    /// route records once the concentrator has successfully
    /// received one.
    /// 
    /// EMBER_HIGH_RAM_CONCENTRATOR 
    HighRAM = 0xFFF9
}

/// ZLL device state identifier.
/// 
/// EmberZllState 
// TODO: is bitmask?
#[repr(u16)]
pub enum ZLLState {
    /// No state.
    /// 
    /// EMBER_ZLL_STATE_NONE 
    None = 0x0000,
    /// The device is factory new.
    /// 
    /// EMBER_ZLL_STATE_FACTORY_NEW 
    FactoryNew = 0x0001,
    /// The device is capable of assigning addresses to other devices.
    /// 
    /// EMBER_ZLL_STATE_ADDRESS_ASSIGNMENT_CAPABLE 
    AddressAssignmentCapable = 0x0002,
    /// The device is initiating a link operation.
    /// 
    /// EMBER_ZLL_STATE_LINK_INITIATOR 
    LinkInitiator = 0x0010,
    /// The device is requesting link priority.
    /// 
    /// EMBER_ZLL_STATE_LINK_PRIORITY_REQUEST 
    LinkPriorityRequest = 0x0020,
    /// The device is on a non-ZLL network.
    /// 
    /// EMBER_ZLL_STATE_NON_ZLL_NETWORK 
    NonZLLNetwork = 0x0100
}

/// ZLL key encryption algorithm enumeration.
/// 
/// EmberZllKeyIndex
#[repr(u8)]
pub enum ZLLKeyIndex {
    /// Key encryption algorithm for use during development.
    /// 
    /// EMBER_ZLL_KEY_INDEX_DEVELOPMENT 
    Development = 0x00,
    /// Key encryption algorithm shared by all certified devices.
    /// 
    /// EMBER_ZLL_KEY_INDEX_MASTER 
    Master = 004,
    /// Key encryption algorithm for use during development and certification.
    /// 
    /// EMBER_ZLL_KEY_INDEX_CERTIFICATION 
    Certification = 0x0F
}

bitmask! {
    /// Bitmask options for emberNetworkInit().
    /// 
    /// EmberNetworkInitBitmask 
    mask NetworkInitBitmask: u16 where
    /// Bitmask options for emberNetworkInit().
    /// 
    /// EmberNetworkInitBitmask 
    flags NetworkInitBitmaskFlags {
        /// No options for Network Init
        /// 
        /// EMBER_NETWORK_INIT_NO_OPTIONS 
        NoOptions = 0x0000,
        /// Save parent info (node ID and EUI64) in a token
        /// during joining/rejoin, and restore on reboot.
        /// 
        /// EMBER_NETWORK_INIT_PARENT_INFO_IN_TOKEN 
        ParentInfoInToken = 0x0001,
        /// Send a rejoin request as an end device on
        /// reboot if parent information is persisted.
        /// 
        /// EMBER_NETWORK_INIT_END_DEVICE_REJOIN_ON_REBOOT 
        EndDeviceRejoinOnReboot = 0x0002
    }
}

/// Network configuration for the desired
/// radio interface for multi-phy network.
/// 
/// EmberMultiPhyNwkConfig
#[repr(u8)]
pub enum MultiPHYNWKConfig {
    /// Enable broadcast support on Routers
    /// 
    /// EMBER_BROADCAST_SUPPORT 
    BroadcastSupport = 0x01,
}

/// Duty cycle states.
/// 
/// EmberDutyCycleState
#[repr(u8)]
pub enum DutyCycleState {
    /// No Duty cycle tracking or metrics are taking place.
    /// 
    /// EMBER_DUTY_CYCLE_TRACKING_OFF 
    TrackingOff = 0,
    /// Duty Cycle is tracked and has not exceeded any thresholds.
    /// 
    /// EMBER_DUTY_CYCLE_LBT_NORMAL 
    LBTNormal = 1,
    /// We have exceeded the limited threshold
    /// of our total duty cycle allotment.
    /// 
    /// EMBER_DUTY_CYCLE_LBT_LIMITED_THRESHOLD_REACHED 
    LBTLimitedThresholdTreached = 2,
    /// We have exceeded the critical threshold
    /// of our total duty cycle allotment.
    /// 
    /// EMBER_DUTY_CYCLE_LBT_CRITICAL_THRESHOLD_REACHED 
    LBTCriticalTrhesholdReached = 3,
    /// We have reached the suspend limit and are
    /// blocking all outbound transmissions.
    /// 
    /// EMBER_DUTY_CYCLE_LBT_SUSPEND_LIMIT_REACHED 
    LBTSuspendLimitReached = 4
}

/// Radio power modes.
/// 
/// EmberRadioPowerMode
#[repr(u8)]
pub enum RadioPowerMode {
    /// The radio receiver is switched on.
    /// 
    /// EMBER_RADIO_POWER_MODE_RX_ON 
    RXOn = 0,
    /// The radio receiver is switched off.
    /// 
    /// EMBER_RADIO_POWER_MODE_OFF 
    Off = 1
}

/// Entropy sources.
/// 
/// EmberEntropySource
#[repr(u8)]
pub enum EntropySource {
    /// Entropy source error.
    /// 
    /// EMBER_ENTROPY_SOURCE_ERROR 
    Error = 0,
    /// Entropy source is the radio.
    /// 
    /// EMBER_ENTROPY_SOURCE_RADIO 
    Radio = 1,
    /// Entropy source is the TRNG powered by mbed TLS.
    /// 
    /// EMBER_ENTROPY_SOURCE_MBEDTLS_TRNG 
    MbedTLSTRND = 2,
    /// Entropy source is powered by mbed TLS, the source is not TRNG.
    /// 
    /// EMBER_ENTROPY_SOURCE_MBEDTLS 
    MbedTLS = 3
}

// TODO: sl_zb_sec_man_key_type_t

// TODO: sl_zb_sec_man_derived_key_type_t

// TODO: sl_zigbee_sec_man_flags_t

// 16-bit ID of a node in the network.
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
pub type LibraryID  = u8;

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
    join_method: JoinMethod,

    /// The ID of the network manager in the current network.
    /// 
    /// This may only be set at joining when using
    /// EMBER_USE_CONFIGURED_NWK_STATE as the join method.
    network_manager_id: NodeId,

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
    options: ApsOption,

    /// The group ID for this message, if it is multicast mode.
    group_id: u16,

    /// The sequence number.
    sequence: u8,
}

pub struct BindingTableEntry {
    /// The type of binding.
    binding_type: BindingType,

    /// The endpoint on the local node.
    local: u8,

    /// A cluster ID that matches one from the local endpoint's simple descriptor. 
    cluster_id: u16,

    /// The endpoint on the remote node (specified by identifier).
    remote: u8,

    /// A 64-bit identifier. This is either the destination EUI64
    /// (for unicasts) or the 64-bit group address (for multicasts).
    identifier: EUI64,

    /// The index of the network the binding belongs to.
    network_index: u8,
}

pub struct MulticastTableEntry  {
    /// The multicast group ID.
    multicast_id: MulticastId,

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
    pan_id: PanId,

    /// The extended PAN ID of the received beacon.
    extended_pan_id: [u8; 8],

    /// The sender of the received beacon.
    sender: NodeId,

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
    long_id: EUI64
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
    bitmask : InitialSecurityBitmask,

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
    preconfigured_trust_center_eui: EUI64
} 

/// The security options and information currently used by the stack.
pub struct CurrentSecurityState  {
    /// A bitmask indicating the security options currently in
    /// use by a device joined in the network.
    bitmask: CurrentSecurityBitmask,

    /// The IEEE Address of the Trust Center device.
    trust_center_long_address: EUI64,
}

/// A structure containing a key and its associated data.
pub struct KeyStruct {
    /// A bitmask indicating the presence of data within the 
    /// various fields in the structure.
    bitmask : KeyStructBitmask,

    /// The type of the key.
    key_type: KeyType,

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
    partner_eui64: EUI64
}

/// Network Initialization parameters.
pub struct NetworkInitStruct {
    /// Configuration options for network init.
    bitmask: NetworkInitBitmask,
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
    eui64: EUI64,

    /// The node id.
    node_id: NodeId,

    /// The ZLL state.
    state: ZLLState,

    /// The node type.
    node_type : NodeType,

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
    key_index: ZLLKeyIndex,

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
    ieee_address: EUI64,
    
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
    node_id : NodeId,

    /// Minimum free node id.
    free_node_id_min: NodeId,

    /// Maximum free node id.
    free_node_id_max: NodeId,

    /// Minimum group id.
    group_id_min: MulticastId,

    /// Maximum group id.
    group_id_max: MulticastId,

    /// Minimum free group id.
    free_group_id_min: MulticastId,

    /// Maximum free group id.
    free_group_id_max: MulticastId,
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
    node_id: NodeId,

    /// Amount of overall duty cycle consumed (up to suspend limit).
    duty_cycle_consumed: DutyCycleHectoPct,
}

/// The transient key data structure.
pub struct TransientKeyData {
    /// The IEEE address paired with the transient link key.
    eui64: EUI64,

    /// The key data structure matching the transient key.
    key_data: KeyData,

    /// This bitmask indicates whether various fields
    /// in the structure contain valid data.
    bitmask: KeyStructBitmask,

    /// The number of seconds remaining before the key is
    /// automatically timed out of the transient key table.
    remaining_time_seconds: u16,

    /// The network index indicates which NWK uses this key.
    network_index: u8,
}

/// A structure containing a child node's data.
pub struct ChildData {
    /// The EUI64 of the child.
    eui64: EUI64,

    /// The node type of the child.
    node_type : NodeType,

    /// The short address of the child.
    id: NodeId,

    /// The phy of the child.
    phy: u8,

    /// The power of the child.
    power: u8,

    /// The timeout of the child.
    timeout: u8,
    
    /// The GPD's EUI64.
    gpd_ieee_address: EUI64,

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
    status: GpProxyTableEntryStatus,

    /// The tunneling options (this contains both options
    /// and extendedOptions from the spec).
    options: u32,

    /// The addressing info of the GPD.
    gpd: GpAddress,
    
    /// The assigned alias for the GPD.
    assigned_alias: NodeId,

    /// The security options field.
    security_options: u8,

    /// The security frame counter of the GPD.
    gpd_security_frame_counter: GpSecurityFrameCounter,

    /// The key to use for GPD.
    gpd_key: KeyData,

    /// The list of sinks (hardcoded to 2 which is the spec minimum).
    sink_list: [GpSinkListEntry; GP_SINK_LIST_ENTRIES],

    /// The groupcast radius.
    groupcast_radius: u8,

    /// The search counter.
    search_counter: u8,
}

/// The internal representation of a sink table entry.
pub struct GpSinkTableEntry {
    /// Internal status of the sink table entry.
    status: GpSinkTableEntryStatus,

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
    assigned_alias: NodeId,

    /// The groupcast radius.
    groupcast_radius: u8,

    /// The security options field.
    security_options: u8,

    /// The security frame counter of the GPD.
    gpd_security_frame_counter: GpSecurityFrameCounter,

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
