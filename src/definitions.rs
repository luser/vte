#[derive(Debug)]
pub enum State {
   Anywhere = 0,
   CsiEntry = 1,
   CsiIgnore = 2,
   CsiIntermediate = 3,
   CsiParam = 4,
   DcsEntry = 5,
   DcsIgnore = 6,
   DcsIntermediate = 7,
   DcsParam = 8,
   DcsPassthrough = 9,
   Escape = 10,
   EscapeIntermediate = 11,
   Ground = 12,
   OscString = 13,
   SosPmApcString = 14,
   Unused__ = 15,
}

#[derive(Debug)]
pub enum Action {
   None = 0,
   Clear = 1,
   Collect = 2,
   CsiDispatch = 3,
   EscDispatch = 4,
   Execute = 5,
   Hook = 6,
   Ignore = 7,
   OscEnd = 8,
   OscPut = 9,
   OscStart = 10,
   Param = 11,
   Print = 12,
   Put = 13,
   Unhook = 14,
   Unused__ = 15,
}

/// Unpack a u8 into a State and Action
///
/// The implementation of this assumes that there are *precisely* 16 variants for both Action and
/// State. Furthermore, it assumes that the enums are tag-only; that is, there is no data in any
/// variant.
///
/// Bad things will happen if those invariants are violated.
#[inline(always)]
fn unpack(delta: u8) -> (State, Action) {
    (
        // Action is stored in bottom 4 bits
        unsafe { ::std::mem::transmute(delta & 0x0f) },

        // State is stored in top 4 bits
        unsafe { ::std::mem::transmute(delta >> 4) },
    )
}

#[cfg(test)]
mod tests {
    use super::{State, Action, unpack};
    #[test]
    fn unpack_state_action() {
        match unpack(0xee) {
            (State::SosPmApcString, Action::Unhook) => (),
            _ => panic!("unpack failed"),
        }

        match unpack(0xff) {
            (State::Unused__, Action::Unused__) => (),
            _ => panic!("unpack failed"),
        }
    }
}