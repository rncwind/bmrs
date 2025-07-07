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
#[derive(FromRepr, Debug, PartialEq, Clone)]
#[repr(u8)]
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
    /// The string represents the identifier (In the example "cc" or "aa")
    /// and the f32 represents the rank as a percentage of rank 2.
    ///
    /// # Example
    /// To steal from hitkey as an example.
    /// ```
    /// #RANK 2
    /// #EXRANKaa 48
    /// #EXRANKcc 100
    /// #114a0:aa0000cc
    /// ```
    /// Would result in the timing window for measure 114 changing to DEFEXRANK 48,
    /// and then going back to DEFEXRANK 100 at the end of the measure.
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
///
/// Negative BPMs are allowed, and in general are expected to make the chart scroll _backwards_.
///
/// For more info, see https://hitkey.nekokan.dyndns.info/exbpm-object.htm
///
/// In parsing, we expect to parse the identifier to the string, and the bpm to the float.
///
/// # Example
/// ```
/// #BPMAA 256
/// #BPMBB 155.5
/// #00108:AABBAABB
/// ```
///
/// This defines the BPM AA to be 256, the BPM BB to be 155.5 and says where to use it
/// in the chart itself.
pub struct ExBPM(String, f32);

/// Represent the multiple types of BPM as enum variants.
pub enum BPM {
    Constant(ConstantBPM),
    Extended(ExBPM),
}

/// `#STOP[01-ZZ] n`, DDR Style stops!
///
/// Operates on Channel #xxx09
///
/// A STOP sequence is an object which stops scrolling of the chart for a defined
/// period of time.
/// Once the time is elapsed, the chart will resume normal scrolling.
///
/// We should treat a `#STOPxx 1` as defining a STOP for 1/192nd of a note in 4/4.
/// It's also important to note that we use the BPM _at time of stop_.
///
/// Implementing this in game is a bit tricky, due to the #xxx02 command which adjusts
/// how long a measure is.
///
/// Decimal stops have their fractional part dropped.
///
/// A negative STOP duration is weirdly defined. For LR2 it applies "-1" and causes it
/// to skip some amount of notes.
/// Generally, it's ignored.
///
/// # Example
///
/// ```
/// #STOP11 96
/// #STOP22 96
/// #00109:0011
/// #00202:0.5
/// #00209:0022
/// ```
///
/// Reading BMS is awkward, so think about each line here being split like so
/// [AAA][BB]:[CC]
/// Where AAA is the measure, BB is the command we're executing on that measure
/// and CC is the "operand" to that command.
///
/// Defines STOP 11 as having a length of 192nd note * 96, and so does STOP 22.
/// #00109:0011 says to do STOP 11 on measure 1
/// #00209:0022 says to do STOP 22 on measure 2
///
/// An example of a 1 second stop
/// ```
/// #BPM 60 // Set BPM to 60
/// #STOP33 48 // 48 * 1/192 stop length
/// #00109:33
/// ```
///
/// If a STOP and a note are on the same time, the note plays first then timing changes.
/// If a STOP and a BPM change are on the same time, then BPM changes then STOP is applied.
///
/// For some more examples, please refer to
/// https://hitkey.bms.ms/cmds.htm#STOP
pub struct Stop(String, u32);

/// `#LNTYPE[0-3]`. Long Note type
///
/// LNType is a field kept for backwards compatibility, as it's no longer needed
/// with the advent of LNOBJ.
///
/// LnType 1 is what we hope we see. We should probably just parse error and
/// tell the user we're ignoring their chart if it's LNType 2 or 3.
///
/// This is omissble.
pub struct LNType(u8);

/// `#LNOBJ xx`
///
/// This is RDM type LNs. They have sounds on keyup and they're annoying.
///
/// TODO: Explain how this works better rather than handwaving it.
pub struct LNObj(u32);

/// `#WAV[00-ZZ] filename`
///
/// One of the most common commands! This defines the sound files that we actually
/// play!
///
/// A single file is assignable to two or more indexes. This is used for polyphony.
///
/// "Alternate search" is expected now. If we cant find example.wav we should search
/// for example.ogg, example.mp3 etc
///
/// Certain channels are assigned, to certain things.
///
/// - `#xxx01` is assigned to the BGM.
/// - `#xxx11-19` is assigned to P1 notes
/// - `#xxx21-29` is assigned to P2 notes
/// - `#xxx31-39` is assigned to P1 _invisible_ notes
/// - `#xxx41-49` is assigned to P2 _invisible_ notes
/// - `#xxx51-59` are assigned to P1 LNs.
/// - `#xxx61-69` are assigned to P2 LNs.
/// - `#xxxD1-D9` are assigned to P1 landmines.
/// - `#xxxE1-E9` are assigned to P2 landmines.
///
/// For more info see https://hitkey.bms.ms/cmds.htm#WAVXX
/// as this is one of the most complex commands we encounter
pub struct Wav(u32, String);

/// `#BMP[00-ZZ] filename`
///
/// Image resources. And Also video!
///
/// Interesringly, certain channels are assigned to certain images.
///
/// - `#xxx04` is the base bga
/// - `#xxx06` is the POOR image
/// - `#xxx07` is the "BGA Layer"
/// - `#xxx0A` is the "BGA Layer 2"
///
/// Videos can be used in any of these indexs. In general, we should expect video playback
/// to only be possible on #xxx04.
///
/// When a video is being played, we should apply layers/miss animation etc overtop.
///
/// If nothing is found in channel `#xxx04/06/07` we should display just a black image.
///
/// We should support as many image formats as is reasonable.
/// PNG, JPG, GIF, TGA, DDS are all common.
///
/// Like with #WAV we should support alternate search. So try PNG then JPEG then GIF etc.
pub struct Bmp(u32, String);
