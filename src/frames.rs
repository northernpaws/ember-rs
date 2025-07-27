
/// Typing for the frame codes used to identify a frame type.
pub type FrameID = u16;

pub trait Frame {
    fn frame_id () -> FrameID;
    fn frame_name () -> &'static str;
}

macro_rules! frame {
    // 1. Parse struct meta attributes, its name and its type.
    (
        $(#[$frame_attr: meta])* $frame_name: ident : $T: tt
        $(#[$parameters_attr: meta])* parameters { $($parameters_token: tt)+ }
    ) => {
        frame! {
            frame_meta: [ $(#[$frame_attr])* ],
            frame_name: $frame_name,
            frame_id: $T,
            parameters_meta: [ $(#[$parameters_attr])* ],
            flags: [
                []
            ],
            $($parameters_token)+
        }
    };

    // 2. Parse flag meta attributes.
    (
        frame_meta: [ $(#[$frame_attr: meta])* ],
        frame_name: $frame_name: ident,
        frame_id: $T: tt,
        parameters_meta: [ $(#[$parameters_attr: meta])* ],
        flags: [
            $(
                meta: [ $(#[$flag_attr: meta])* ]
                flag: $flag_name: ident = $flag_value: tt;
            )*
            [ $(#[$prev_attr: meta])* ]
        ],
        #[$next_attr: meta] $($parameters_token: tt)*
    ) => {
        frame! {
            frame_meta: [ $(#[$frame_attr])* ],
            frame_name: $frame_name,
            frame_id: $T,
            parameters_meta: [ $(#[$parameters_attr])* ],
            flags: [
                $(
                    meta: [ $(#[$flag_attr])* ]
                    flag: $flag_name = $flag_value;
                )*
                [ $(#[$prev_attr])* #[$next_attr] ]
            ],
            $($parameters_token)*
        }
    };

    // 3.A Parse the flag itself.
    //
    // Handles the case with trailing comma.
    (
        frame_meta: [ $(#[$frame_attr: meta])* ],
        frame_name: $frame_name: ident,
        frame_id: $T: tt,
        parameters_meta: [ $(#[$parameters_attr: meta])* ],
        flags: [
            $(
                meta: [ $(#[$flag_attr: meta])* ]
                flag: $flag_name: ident = $flag_value: tt;
            )*
            [ $(#[$next_attr: meta])* ]
        ],
        $next_name: ident = $next_value: tt, $($parameters_token: tt)*
    ) => {
        frame! {
            frame_meta: [ $(#[$frame_attr])* ],
            frame_name: $frame_name,
            frame_id: $T,
            parameters_meta: [ $(#[$parameters_attr])* ],
            flags: [
                $(
                    meta: [ $(#[$flag_attr])* ]
                    flag: $flag_name = $flag_value;
                )*
                meta: [ $(#[$next_attr])* ]
                flag: $next_name = $next_value;
                []
            ],
            $($parameters_token)*
        }
    };

    // 3.B Parse the last flag if missing trailing comma.
    (
        frame_meta: [ $(#[$frame_attr: meta])* ],
        frame_name: $frame_name: ident,
        frame_id: $T: tt,
        parameters_meta: [ $(#[$parameters_attr: meta])* ],
        en_name: $en_name: ident,
        flags: [
            $(
                meta: [ $(#[$flag_attr: meta])* ]
                flag: $flag_name: ident = $flag_value: tt;
            )*
            [ $(#[$next_attr: meta])* ]
        ],
        $next_name: ident = $next_value: tt
    ) => {
        bitmask! {
            frame_meta: [ $(#[$frame_attr])* ],
            frame_name: $frame_name,
            frame_id: $T,
            parameters_meta: [ $(#[$parameters_attr])* ],
            flags: [
                $(
                    meta: [ $(#[$flag_attr])* ]
                    flag: $flag_name = $flag_value;
                )*
                meta: [ $(#[$next_attr])* ]
                flag: $next_name = $next_value;
                []
            ],
        }
    };

    // 4. End of the line. Time to declare the struct and enum.
    (
        frame_meta: [ $(#[$frame_attr: meta])* ],
        frame_name: $frame_name: ident,
        frame_id: $T: tt,
        parameters_meta: [ $(#[$parameters_attr: meta])* ],
        flags: [
            $(
                meta: [ $(#[$flag_attr: meta])* ]
                flag: $flag_name: ident = $flag_value: tt;
            )+
            []
        ],
    ) => {
        // #[repr($T)]
        // #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        // #[cfg_attr(feature = "std", derive(Debug, Hash))]
        // #[allow(dead_code)]
        // $(#[$parameters_attr])*
        // enum $en_name {
        //     $(
        //         $(#[$flag_attr])*
        //         $flag_name = $flag_value
        //     ),+
        // }
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "std", derive(Debug, Hash))]
        #[allow(dead_code)]
        $(#[$frame_attr])*
        struct $frame_name {
            
        }
        // frame!(@IMPL $frame_name $T $en_name, {
        //     $($flag_name = $flag_value),+
        // });
    };
}

frame! {
    /// This command allows the Host to specify the desired EZSP
    /// version and must be sent before any other command.
    /// 
    /// The response provides information about the firmware running on the NCP.
    Version: 0x0000

    /// Supplies parameters for the version command frame.
    parameters {
        /// The EZSP version the Host wishes to use. To
        /// successfully set the version and allow other
        /// commands, this must be same as
        /// EZSP_PROTOCOL_VERSION.
        desired_protocol_version = u8,
    }
}
