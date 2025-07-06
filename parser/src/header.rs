use strum_macros::FromRepr;

pub struct Header {
    player: Player,
}

/// `#PLAYER [1-4]`. Defines the play side.
#[derive(FromRepr, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Player {
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

/// `#RANK [0-3]`. Defines the judge difficulty.
///
/// We follow LR2 convention here, so Rank is 0,1,2,3
pub enum Rank {
    VeryHard, // RANK 0, +-8ms
    Hard,     // RANK 1, +- 15ms
    Normal,   // RANK 2, +- 18ms
    Easy,     // RANK 3, +- 21ms
}

// LR2 Convention is to apply Normal when unspecified.
impl Default for Rank {
    fn default() -> Self {
        Self::Normal
    }
}

pub enum JudgeRankType {
    /// `#RANK [0-3]` Normal rank system.
    ///
    /// This is what you see 99% of the time.
    Rank,

    /// `#DEFEXRANK n`. Percentage judge.
    /// Defexrank is very strange. It specifies judge difficulty as a percentage of RANK 2.
    /// This means that 100 is equal to RANK 2. If a DefExRank of 199.97 is sepcified then
    /// the rank is 199.97% of RANK 2.
    ///
    /// TODO: Implement Defexrank
    Defexrank(f32),
    /// `#EXRANK[01-ZZ] n`. In-chart adjustable rank.
    /// Exrank is also weird. It allows the timing window of the chart to be changed
    /// during play.
    ///
    /// To steal from hitkey as an example.
    /// ```
    /// #RANK 2
    /// #EXRANKaa 48
    /// #EXRANKcc 100
    /// #114a0:aa0000cc
    /// ```
    /// Would result in the timing window for measure 114 changing to DEFEXRANK 48,
    /// and then going back to DEFEXRANK 100 at the end of the measure.
    ///
    /// The string represents the identifier (In the example "cc" or "aa")
    /// and the f32 represents the rank as a percentage of rank 2.
    Exrank(String, f32),
}

#[cfg_attr(doc, katexit::katexit)]
/// `#TOTAL n`. Rate of Gague Recovery ™
///
/// Total is a bit tricky to wrap your head around.
/// As far as the player is concerned, TOTAL is what defines the rate of gauge recovery.
///
/// Let n be the value of #TOTAL n, and y the total amount of objects in the chart
/// the rate of gauge recovery on a PGREAT is
/// $\frac{n}{y}$
///
/// If there are 400 objects to hit, and TOTAL is 200, we get $\frac{200}{400} = 0.5$
///
/// If this field is omitted, the spec does not specify what the default value should be.
/// - LR2 uses 160
/// - jbmsparser for beatoraja uses 100.
/// - We will use 160
pub struct Total(f64);

impl Default for Total {
    fn default() -> Total {
        Self(160.0)
    }
}

/// `#VOLWAV n`. Flat volume multiplier.
///
/// Defaults to 100.
///
/// In general, this gets ignored.
///
/// #VOLWAV 250 would be playing at 250% volume.
///
/// #VOLWAV 25 would be playing at 25% volume.
pub struct Volwav(i32);

/// `#STAGEFILE imagefilename`. Splash screen.
///
/// This command is omissible. When omitted it is expected that the default splashscreen
/// will be used.
pub struct Stagefile(String);

/// `#BANNER imagefilename`. Song select banner image.
pub struct Banner(String);

/// `#BACKBMP imagefilename`. Static "movie" background.
///
/// If we chose to follow the OverActive style, then this is a pre-movie splash
/// like the song title, genre and such in IIDX.
/// https://right-stick.sub.jp/backbmp/index.html
pub struct BackBmp(String);

/// `#PLAYLEVEL n`. Song difficulty.
///
/// Reported difficulty level. This will usually be [1-12] IIDX style.
///
/// #PLAYLEVEL 0 is a strange case. This is usually for gimmick charts which
/// use commands like `#RANDOM` or `#SWITCH`
pub struct PlayLevel(u16);

/// For whatever reason, BM98 used #PLAYLEVEL 3 as it's default if this was
/// omitted. Apparently many followed this, even through it's not spec.
impl Default for PlayLevel {
    fn default() -> Self {
        PlayLevel(3)
    }
}

/// `#DIFFICULTY [1-5]`. Difficulty. Normal/Hyper etc
///
/// We follow an adjusted IIDX naming convention in this enum.
/// Beginner, Normal, Hyper, Another are from IIDX, but we take INSANE from bms.
/// There's a lot of naiming conventions, these are disambiguated in code comments
///
/// This command is omissable, and anything which doesn't have it is expected to be
/// unsortable and unfilterable by this metric.
#[derive(FromRepr, Debug, PartialEq, Clone)]
#[repr(u8)]
enum Difficulty {
    Beginner, // Easy/Beginner/Light
    Normal,   // Normal/Standard
    Hyper,    // Hard
    Another,  // EX, Maximum
    Insane,   // Kusofumen, 糞譜面, INSANE, 発狂, hakkyou, SUPER-CRAZY
}

/// `#TITLE string` Title of the track.
///
/// Unsurprisingly, defines the title of the track.
///
/// When omitted, it's not actually defined what will happen.
///
/// There's no limit on the length of the title.
///
/// LR2 trims whitespace on the left and right of the title.
///
/// Encoding and decoding can be hellish, because of how old BMS as a format is.
/// Don't expect UTF-8 everywhere, expect a good chunk of UTF-16, SHIFT-JIS,
/// IEC 2022 and even EUC.
///
/// Implicit Subtitles exist as a form of in-band signalling.
/// The subtitle is delimited by any of the following
/// - `#TITLE main-sub-`
/// - `#TITLE main～sub～`
/// - `#TITLE main(sub)`
/// - `#TITLE main[sub]`
/// - `#TITLE main<sub>`
/// - `#TITLE main␣␣sub` (Two+ halfwidth spaces)
/// - `#TITLE main"sub"` (Straight double quotes)
///
/// Because of how varied these are, we will make a choice as to which we will support.
/// This is fine since there's now a `#SUBTITLE` command.
///
/// We will support full width tilde and quote marks only.
pub struct Title(String);

/// `#SUBTITLE string` Subtitle of the track
///
/// Due to the limitations of the implicit subtitle, an actual SUBTITLE field was
/// defined.
///
/// Omissible. LR2 will only check for a implicit subtitle if `#SUBTITLE` doesn't exist.
pub struct Subtitle(String);

/// `#ARTIST string`
///
/// Definition of the track artist. Interestingly Artist isn't actually defined
/// in the spec.
pub struct Artist(String);

/// `#SUBARTIST string`
///
/// Added by LR2. This is used usually to define things like BGA artists,
/// noters and other such co-artists.
pub struct Subartist(String);

/// `#MAKER string`
///
/// Sometimes supported.
///
/// Used to denote when a composer differs from the chart maker. In this case
/// it is used to store the chart makers name.
pub struct Maker(String);

/// `#GENRE string`
///
/// Denotes the genre of music for the chart.
/// By default it will be empty if not set.
///
/// Supported by basically every client.
pub struct Genre(String);

// TODO: Landmine
// It's in WAV00

/// `#BPM n`
///
/// Defines the BPM of the music. Defines the scroll speed etc.
/// The BMS spec defines 130 as the default value.
///
/// it is expected that fractional BPMs are supported, as such we will repr
/// this as a float.
pub struct ConstantBPM(f32);

// Standard defined default.
// TODO. Implement BPM changes
impl Default for ConstantBPM {
    fn default() -> Self {
        Self(130.0)
    }
}

/// `#BPMxx n` OR `#EXBPM[01-ZZ] n`
///
/// Hitkey refers to this as exBPM or "Extended BPM Change Command".
/// This operates on Channel #xxx08.
///
/// This command allows the chart designer to describe BPM changes.
/// For example,
///
/// ```
/// #BPMAA 256
/// #BPMBB 155.5
/// #00108:AABBAABB
/// ```
///
/// This defines the BPM AA to be 256, the BPM BB to be 155.5 and says where to use it
/// in the chart itself.
///
/// Negative BPMs are allowed, and in general are expected to make the chart scroll _backwards_.
///
/// For more info, see https://hitkey.nekokan.dyndns.info/exbpm-object.htm
///
/// In parsing, we expect to parse the identifier to the string, and the bpm to the float.
pub struct ExBPM(String, f32);

/// Represent the multiple types of BPM as enum variants.
pub enum BPM {
    Constant(ConstantBPM),
    Extended(ExBPM),
}

/// `#STOP[01-ZZ] n`
pub struct Stop(String, u32);
