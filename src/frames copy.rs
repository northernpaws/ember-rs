
/// Typing for the frame codes used to identify a frame type.
pub type FrameID = u16;

pub trait Frame {
    fn frame_id () -> FrameID;
    fn frame_name () -> &'static str;
}

macro_rules! frame {
    // `()` indicates that the macro takes no argument.
    (
        $(#[$frame_attr: meta])* $frame_name: ident : $frame_id: tt where
        $(#[$en_attr: meta])* parameters { $($token: tt)+ }
    ) => {
        #[allow(dead_code)]
        $(#[$frame_attr])*
        pub struct $frame_name {}

        impl $frame_name {}

        impl Frame for $frame_name {
            fn frame_id () -> FrameID {
                $frame_id
            }

            fn frame_name () -> &'static str{
                "$frame_name"
            }
        }
    };
}

frame! {
    /// Test
    Version : 0x0000 where
    parameters {
        Test1
    }
}
