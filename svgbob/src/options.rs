
bitflags! {
    pub struct Options: u32 {
        /// merge fragments that can be merged
        const MERGE_FRAGMENTS = 1 << 0;
        /// group fragments that are touching together
        const GROUP_FRAGMENTS = 1 << 1;
        /// try to make a group of fragments to rect, when applicable
        const ENDORSE_FRAGMENTS = 1 << 2;
        /// allow arc even they didn't form a complete circle
        const QUARTERLY_ARCS = 1 << 3;
        /// allow oval shapes
        const ENABLE_OVAL = 1 << 4;
        ///  enable enhancement of lines
        ///  that can almost connect such as: _|_ /_  -|-
        const AGGRESSIVE_ENHANCE = 1 << 5;
    }
}
