pub mod ezsp;
pub mod ember;

use bit_struct::*; 

enums! {
    pub SleepModes {
        Idle,
        DeepSleep,
        PowerDown,
        Reserved
    }

    pub OverflowStatus {
        NoMemoryShortage,
        NCPOutOfMemory,   
    }

    pub TruncatedStatus {
        NoTruncated,
        Truncated
    }

    pub CallbackPending {
        NoPendingCallbacks,
        CallbackPending
    }

    pub CallbackTypes {
        None,
        Synchronous,
        Asynchronous,
        Reserved
    }
}


bit_struct! {
    /// Defines the low byte of the two-byte frame control header.
    /// 
    /// The low and high/extended frame control bytes and defined
    /// seperately to support older versions of the protocol that
    /// used a single low byte frame control field.
    pub struct CommandFrameControl(u8) {
        /// Set to 0 when sending a command, should
        /// be 1 when reciving a command.
        response_indicator: bool,

        network_index: u2,
        
        /// Only popualted for response, 00 for command.
        callback_type: CallbackTypes,

        callback_pending: bool,

        sleep_mode: SleepModes
    }
}


bit_struct! {
    pub struct ResponseFrameControl(u8) {
        /// Set to 0 when sending a command, should
        /// be 1 when reciving a command.
        response_indicator: bool,

        network_index: u2,
        
        /// Only popualted for response, 00 for command.
        callback_type: CallbackTypes,

        callback_pending: CallbackPending,

        transcated: TruncatedStatus,

        overflow: OverflowStatus
    }
}

enums! {
    pub SecurityEnabled {
        Disabled,
        Enabled
    }

    pub PaddingEnabled {
        Disabled,
        Enabled
    }

    pub FrameFormatVersion {
        Version0,
        Version1,
        Reserved2,
        Reserved3
    }
}

bit_struct! {
    /// Defines the high byte of the two-byte frame control header.
    /// 
    /// The low and high/extended frame control bytes and defined
    /// seperately to support older versions of the protocol that
    /// used a single low byte frame control field.
    /// 
    /// The extended high frame control byte has the same fields for
    /// command and response frames.
    pub struct ExtendedFrameControl(u8) {
        security_enabled: SecurityEnabled,
        padding_enabled: PaddingEnabled,
        reserved:  u4,
        frame_format_version: FrameFormatVersion
    }
}

/// Defines a EZSP frame.
pub struct Frame {
    /// An incrementing sequence number for the frame
    /// used for corrolation with response frames.
    sequence: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_modes () {
        assert!(SleepModes::Idle.inner_raw() == 0b00);
        assert!(SleepModes::DeepSleep.inner_raw() == 0b01);
        assert!(SleepModes::PowerDown.inner_raw() == 0b10);
        assert!(SleepModes::Reserved.inner_raw() == 0b11);
    }

    #[test]
    fn callback_types () {
        assert!(CallbackTypes::None.inner_raw() == 0b00);
        assert!(CallbackTypes::Synchronous.inner_raw() == 0b01);
        assert!(CallbackTypes::Asynchronous.inner_raw() == 0b10);
        assert!(CallbackTypes::Reserved.inner_raw() == 0b11);
    }

    #[test]
    fn frame_format_version () {
        assert!(FrameFormatVersion::Version0.inner_raw() == 0b00);
        assert!(FrameFormatVersion::Version1.inner_raw() == 0b01);
        assert!(FrameFormatVersion::Reserved2.inner_raw() == 0b10);
        assert!(FrameFormatVersion::Reserved3.inner_raw() == 0b11);
    }
}
