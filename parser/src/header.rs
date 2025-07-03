struct Header {
    player: Player,
}

/// Defines the play side.
#[derive(FromRepr, Debug, PartialEq, Clone)]
#[repr(u8)]
enum Player {
    One,   // SP
    Two,   // Couple play
    Three, // DP
    Four,  // Battle Play. This is very, very rare
}

impl Default for Player {
    fn default() -> Self {
        Self::One
    }
}
