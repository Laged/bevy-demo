# Crates-Rs-Bevy - Audio

**Pages:** 12

---

## Struct Pitch Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.Pitch.html

**Contents:**
- Struct Pitch Copy item path
- Fields§
- Implementations§
  - impl Pitch
    - pub fn new(frequency: f32, duration: Duration) -> Pitch
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for Pitch
    - fn clone(&self) -> Pitch
    - fn clone_from(&mut self, source: &Self)

A source of sine wave sound

Frequency at which sound will be played

Duration for which sound will be played

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct Pitch {
    pub frequency: f32,
    pub duration: Duration,
}
```

Example 2 (unknown):
```unknown
25fn play_pitch(
26    mut pitch_assets: ResMut<Assets<Pitch>>,
27    frequency: Res<PitchFrequency>,
28    mut play_pitch_reader: MessageReader<PlayPitch>,
29    mut commands: Commands,
30) {
31    for _ in play_pitch_reader.read() {
32        info!("playing pitch with frequency: {}", frequency.0);
33        commands.spawn((
34            AudioPlayer(pitch_assets.add(Pitch::new(frequency.0, Duration::new(1, 0)))),
35            PlaybackSettings::DESPAWN,
36        ));
37        info!("number of pitch assets: {}", pitch_assets.len());
38    }
39}
```

---

## Struct SpatialScale Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.SpatialScale.html

**Contents:**
- Struct SpatialScale Copy item path
- Tuple Fields§
- Implementations§
  - impl SpatialScale
    - pub const fn new(scale: f32) -> SpatialScale
    - pub const fn new_2d(scale: f32) -> SpatialScale
      - Examples found in repository?
- Trait Implementations§
  - impl Clone for SpatialScale
    - fn clone(&self) -> SpatialScale

A scale factor applied to the positions of audio sources and listeners for spatial audio.

Default is Vec3::ONE.

Create a new SpatialScale with the same value for all 3 dimensions.

Create a new SpatialScale with the same value for x and y, and 0.0 for z.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct SpatialScale(pub Vec3);
```

Example 2 (unknown):
```unknown
14fn main() {
15    App::new()
16        .add_plugins(DefaultPlugins.set(AudioPlugin {
17            default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
18            ..default()
19        }))
20        .add_systems(Startup, setup)
21        .add_systems(Update, update_emitters)
22        .add_systems(Update, update_listener)
23        .run();
24}
```

---

## Enum Volume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/enum.Volume.html

**Contents:**
- Enum Volume Copy item path
- Variants§
  - Linear(f32)
    - §Examples
  - Decibels(f32)
    - §Examples
- Implementations§
  - impl Volume
    - pub const SILENT: Volume
    - pub fn to_linear(&self) -> f32

A Volume represents an audio source’s volume level.

To create a new Volume from a linear scale value, use Volume::Linear.

To create a new Volume from decibels, use Volume::Decibels.

Create a new Volume from the given volume in the linear scale.

In a linear scale, the value 1.0 represents the “normal” volume, meaning the audio is played at its original level. Values greater than 1.0 increase the volume, while values between 0.0 and 1.0 decrease the volume. A value of 0.0 effectively mutes the audio.

Create a new Volume from the given volume in decibels.

In a decibel scale, the value 0.0 represents the “normal” volume, meaning the audio is played at its original level. Values greater than 0.0 increase the volume, while values less than 0.0 decrease the volume. A value of f32::NEG_INFINITY decibels effectively mutes the audio.

The silent volume. Also known as “off” or “muted”.

Returns the volume in linear scale as a float.

Returns the volume in decibels as a float.

If the volume is silent / off / muted, i.e., its underlying linear scale is 0.0, this method returns negative infinity.

Increases the volume by the specified percentage.

This method works in the linear domain, where a 100% increase means doubling the volume (equivalent to +6.02dB).

Decreases the volume by the specified percentage.

This method works in the linear domain, where a 50% decrease means halving the volume (equivalent to -6.02dB).

Scales the volume to a specific linear factor relative to the current volume.

This is different from adjust_by_linear as it sets the volume to be exactly the factor times the original volume, rather than applying the factor to the current volume.

Creates a fade effect by interpolating between current volume and target volume.

This method performs linear interpolation in the linear domain, which provides a more natural-sounding fade effect.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub enum Volume {
    Linear(f32),
    Decibels(f32),
}
```

Example 2 (javascript):
```javascript
let volume = Volume::Linear(0.5);
assert_eq!(volume.to_linear(), 0.5);
assert!(ops::abs(volume.to_decibels() - -6.0206) < EPSILON);

let volume = Volume::Linear(0.0);
assert_eq!(volume.to_linear(), 0.0);
assert_eq!(volume.to_decibels(), f32::NEG_INFINITY);

let volume = Volume::Linear(1.0);
assert_eq!(volume.to_linear(), 1.0);
assert!(ops::abs(volume.to_decibels() - 0.0) < EPSILON);
```

Example 3 (javascript):
```javascript
let volume = Volume::Decibels(-5.998);
assert!(ops::abs(volume.to_linear() - 0.5) < EPSILON);

let volume = Volume::Decibels(f32::NEG_INFINITY);
assert_eq!(volume.to_linear(), 0.0);

let volume = Volume::Decibels(0.0);
assert_eq!(volume.to_linear(), 1.0);

let volume = Volume::Decibels(20.0);
assert_eq!(volume.to_linear(), 10.0);
```

Example 4 (javascript):
```javascript
111fn fade_in(
112    mut commands: Commands,
113    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
114    time: Res<Time>,
115) {
116    for (mut audio, entity) in audio_sink.iter_mut() {
117        let current_volume = audio.volume();
118        audio.set_volume(
119            current_volume.fade_towards(Volume::Linear(1.0), time.delta_secs() / FADE_TIME),
120        );
121        if audio.volume().to_linear() >= 1.0 {
122            audio.set_volume(Volume::Linear(1.0));
123            commands.entity(entity).remove::<FadeIn>();
124        }
125    }
126}
127
128// Fades ou
...
```

---

## Enum SeekError Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/enum.SeekError.html

**Contents:**
- Enum SeekError Copy item path
- Variants (Non-exhaustive)§
  - NotSupported
    - Fields
  - SymphoniaDecoder(SeekError)
  - HoundDecoder(Error)
  - Other(Box<dyn Error + Send>)
- Implementations§
  - impl SeekError
    - pub fn source_intact(&self) -> bool

Occurs when try_seek fails because the underlying decoder has an error or does not support seeking.

One of the underlying sources does not support seeking

The source that did not support seek

The symphonia decoder ran into an issue

The hound (wav) decoder ran into an issue

Any other error probably in a custom Source

Will the source remain playing at its position before the seek or is it broken?

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
#[non_exhaustive]pub enum SeekError {
    NotSupported {
        underlying_source: &'static str,
    },
    SymphoniaDecoder(SeekError),
    HoundDecoder(Error),
    Other(Box<dyn Error + Send>),
}
```

---

## Trait CpalSample Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.CpalSample.html

**Contents:**
- Trait CpalSample Copy item path
- §Example
- Required Associated Constants§
    - const EQUILIBRIUM: Self
      - §Example
- Provided Associated Constants§
    - const IDENTITY: Self::Float = <Self::Float as FloatSample>::IDENTITY
      - §Example
- Required Associated Types§
    - type Signed: SignedSample + Duplex<Self>

A trait for working generically across different Sample format types.

Provides methods for converting to and from any type that implements the FromSample trait and provides methods for performing signal amplitude addition and multiplication.

The equilibrium value for the wave that this Sample type represents. This is normally the value that is equal distance from both the min and max ranges of the sample.

Note: This will likely be changed to an “associated const” if the feature lands.

The multiplicative identity of the signal.

In other words: A value which when used to scale/multiply the amplitude or frequency of a signal, returns the same signal.

This is useful as a default, non-affecting amplitude or frequency multiplier.

When summing two samples of a signal together, it is necessary for both samples to be represented in some signed format. This associated Addition type represents the format to which Self should be converted for optimal Addition performance.

For example, u32’s optimal Addition type would be i32, u8’s would be i8, f32’s would be f32, etc.

Specifying this as an associated type allows us to automatically determine the optimal, lossless Addition format type for summing any two unique Sample types together.

As a user of the sample crate, you will never need to be concerned with this type unless you are defining your own unique Sample type(s).

When multiplying two samples of a signal together, it is necessary for both samples to be represented in some signed, floating-point format. This associated Multiplication type represents the format to which Self should be converted for optimal Multiplication performance.

For example, u32’s optimal Multiplication type would be f32, u64’s would be f64, i8’s would be f32, etc.

Specifying this as an associated type allows us to automatically determine the optimal, lossless Multiplication format type for multiplying any two unique Sample types together.

As a user of the sample crate, you will never need to be concerned with this type unless you are defining your own unique Sample type(s).

Convert self to any type that implements FromSample<Self>.

Find more details on type-specific conversion ranges and caveats in the conv module.

Create a Self from any type that implements ToSample<Self>.

Find more details on type-specific conversion ranges and caveats in the conv module.

Converts self to the equivalent Sample in the associated Signed format.

This is a simple wrapper around Sample::to_sam

*[Content truncated]*

**Examples:**

Example 1 (javascript):
```javascript
pub trait CpalSample:
    Copy
    + Clone
    + PartialOrd
    + PartialEq {
    type Signed: SignedSample + Duplex<Self>;
    type Float: FloatSample + Duplex<Self>;

    const EQUILIBRIUM: Self;
    const IDENTITY: Self::Float = <Self::Float as FloatSample>::IDENTITY;

    // Provided methods
    fn to_sample<S>(self) -> S
       where Self: ToSample<S> { ... }
    fn from_sample<S>(s: S) -> Self
       where Self: FromSample<S> { ... }
    fn to_signed_sample(self) -> Self::Signed { ... }
    fn to_float_sample(self) -> Self::Float { ... }
    fn add_amp(self, amp: Self::Signed) -> Self { 
...
```

Example 2 (unknown):
```unknown
use dasp_sample::{I24, Sample};

fn main() {
    assert_eq!((-1.0).to_sample::<u8>(), 0);
    assert_eq!(0.0.to_sample::<u8>(), 128);
    assert_eq!(0i32.to_sample::<u32>(), 2_147_483_648);
    assert_eq!(I24::new(0).unwrap(), Sample::from_sample(0.0));
    assert_eq!(0.0, Sample::EQUILIBRIUM);
}
```

Example 3 (unknown):
```unknown
use dasp_sample::Sample;

fn main() {
    assert_eq!(0.0, f32::EQUILIBRIUM);
    assert_eq!(0, i32::EQUILIBRIUM);
    assert_eq!(128, u8::EQUILIBRIUM);
    assert_eq!(32_768_u16, Sample::EQUILIBRIUM);
}
```

Example 4 (unknown):
```unknown
use dasp_sample::{Sample, U48};

fn main() {
    assert_eq!(1.0, f32::IDENTITY);
    assert_eq!(1.0, i8::IDENTITY);
    assert_eq!(1.0, u8::IDENTITY);
    assert_eq!(1.0, U48::IDENTITY);
}
```

---

## Trait Decodable Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.Decodable.html

**Contents:**
- Trait Decodable Copy item path
- Required Associated Types§
    - type DecoderItem: Sample + Send + Sync
    - type Decoder: Source<Item = Self::DecoderItem> + Send + Iterator
- Required Methods§
    - fn decoder(&self) -> Self::Decoder
- Implementors§
  - impl Decodable for AudioSource
    - type DecoderItem = <Decoder<Cursor<AudioSource>> as Iterator>::Item
    - type Decoder = Decoder<Cursor<AudioSource>>

A type implementing this trait can be converted to a rodio::Source type.

It must be Send and Sync in order to be registered. Types that implement this trait usually contain raw sound data that can be converted into an iterator of samples. This trait is implemented for AudioSource. Check the example decodable for how to implement this trait on a custom type.

The type of the audio samples. Usually a u16, i16 or f32, as those implement rodio::Sample. Other types can implement the rodio::Sample trait as well.

The type of the iterator of the audio samples, which iterates over samples of type Self::DecoderItem. Must be a rodio::Source so that it can provide information on the audio it is iterating over.

Build and return a Self::Decoder of the implementing type

**Examples:**

Example 1 (unknown):
```unknown
pub trait Decodable:
    Send
    + Sync
    + 'static {
    type DecoderItem: Sample + Send + Sync;
    type Decoder: Source<Item = Self::DecoderItem> + Send + Iterator;

    // Required method
    fn decoder(&self) -> Self::Decoder;
}
```

---

## Struct AudioSource Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.AudioSource.html

**Contents:**
- Struct AudioSource Copy item path
- Fields§
- Trait Implementations§
  - impl AsRef<[u8]> for AudioSource
    - fn as_ref(&self) -> &[u8] ⓘ
  - impl Clone for AudioSource
    - fn clone(&self) -> AudioSource
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for AudioSource
    - fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

A source of audio data

Raw data of the audio source.

The data must be one of the file formats supported by Bevy (wav, ogg, flac, or mp3). However, support for these file formats is not part of Bevy’s default feature set. In order to be able to use these file formats, you will have to enable the appropriate optional features.

It is decoded using rodio::decoder::Decoder. The decoder has conditionally compiled methods depending on the features enabled. If the format used is not enabled, then this will panic with an UnrecognizedFormat error.

Returns the argument unchanged.

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct AudioSource {
    pub bytes: Arc<[u8]>,
}
```

---

## Trait Sample Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.Sample.html

**Contents:**
- Trait Sample Copy item path
- Required Methods§
    - fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self
    - fn amplify(self, value: f32) -> Self
    - fn to_f32(self) -> f32
    - fn saturating_add(self, other: Self) -> Self
    - fn zero_value() -> Self
- Dyn Compatibility§
- Implementations on Foreign Types§
  - impl Sample for f32

Represents a value of a single sample.

This trait is implemented by default on three types: i16, u16 and f32.

You can implement this trait on your own type as well if you wish so.

Linear interpolation between two samples.

The result should be equal to first * numerator / denominator + second * (1 - numerator / denominator).

Multiplies the value of this sample by the given amount.

Converts the sample to an f32 value.

Calls saturating_add on the sample.

Returns the value corresponding to the absence of sound.

This trait is not dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

**Examples:**

Example 1 (unknown):
```unknown
pub trait Sample: Sample {
    // Required methods
    fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self;
    fn amplify(self, value: f32) -> Self;
    fn to_f32(self) -> f32;
    fn saturating_add(self, other: Self) -> Self;
    fn zero_value() -> Self;
}
```

---

## Trait AudioSinkPlayback Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.AudioSinkPlayback.html

**Contents:**
- Trait AudioSinkPlayback Copy item path
- Required Methods§
    - fn volume(&self) -> Volume
    - fn set_volume(&mut self, volume: Volume)
    - fn speed(&self) -> f32
    - fn set_speed(&self, speed: f32)
    - fn play(&self)
    - fn position(&self) -> Duration
    - fn try_seek(&self, pos: Duration) -> Result<(), SeekError>
      - §Errors

Common interactions with an audio sink.

Gets the volume of the sound as a Volume.

If the sink is muted, this returns the managed volume rather than the sink’s actual volume. This allows you to use the returned volume as if the sink were not muted, because a muted sink has a physical volume of 0.

Changes the volume of the sound to the given Volume.

If the sink is muted, changing the volume won’t unmute it, i.e. the sink’s volume will remain “off” / “muted”. However, the sink will remember the volume change and it will be used when unmute is called. This allows you to control the volume even when the sink is muted.

Gets the speed of the sound.

The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.

Changes the speed of the sound.

The value 1.0 is the “normal” speed (unfiltered input). Any value other than 1.0 will change the play speed of the sound.

Resumes playback of a paused sink.

No effect if not paused.

Returns the position of the sound that’s being played.

This takes into account any speedup or delay applied.

Example: if you set_speed(2.0) and position() returns 5s, then the position in the recording is 10s from its start.

Attempts to seek to a given position in the current source.

This blocks between 0 and ~5 milliseconds.

As long as the duration of the source is known, seek is guaranteed to saturate at the end of the source. For example given a source that reports a total duration of 42 seconds calling try_seek() with 60 seconds as argument will seek to 42 seconds.

This function will return SeekError::NotSupported if one of the underlying sources does not support seeking.

It will return an error if an implementation ran into one during the seek.

When seeking beyond the end of a source, this function might return an error if the duration of the source is not known.

Pauses playback of this sink.

No effect if already paused. A paused sink can be resumed with play.

Returns true if the sink is paused.

Sinks can be paused and resumed using pause and play.

It won’t be possible to restart it afterwards.

Returns true if this sink has no more sounds to play.

Returns true if the sink is muted.

Muting a sink sets the volume to 0. Use unmute to unmute the sink and restore the original volume.

Restores the volume to the value it was before it was muted.

Toggles playback of the sink.

If the sink is paused, toggling playback resumes it. If the sink is playing, toggling pl

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait AudioSinkPlayback {
Show 16 methods    // Required methods
    fn volume(&self) -> Volume;
    fn set_volume(&mut self, volume: Volume);
    fn speed(&self) -> f32;
    fn set_speed(&self, speed: f32);
    fn play(&self);
    fn position(&self) -> Duration;
    fn try_seek(&self, pos: Duration) -> Result<(), SeekError>;
    fn pause(&self);
    fn is_paused(&self) -> bool;
    fn stop(&self);
    fn empty(&self) -> bool;
    fn is_muted(&self) -> bool;
    fn mute(&mut self);
    fn unmute(&mut self);

    // Provided methods
    fn toggle_playback(&self) { ... }
    fn toggle_mute(&
...
```

Example 2 (javascript):
```javascript
72fn pause(
73    keyboard_input: Res<ButtonInput<KeyCode>>,
74    music_controller: Query<&AudioSink, With<MyMusic>>,
75) {
76    let Ok(sink) = music_controller.single() else {
77        return;
78    };
79
80    if keyboard_input.just_pressed(KeyCode::Space) {
81        sink.toggle_playback();
82    }
83}
```

Example 3 (unknown):
```unknown
133fn mute(keyboard_input: Res<ButtonInput<KeyCode>>, mut sinks: Query<&mut SpatialAudioSink>) {
134    if keyboard_input.just_pressed(KeyCode::KeyM) {
135        for mut sink in sinks.iter_mut() {
136            sink.toggle_mute();
137        }
138    }
139}
```

Example 4 (javascript):
```javascript
85fn mute(
86    keyboard_input: Res<ButtonInput<KeyCode>>,
87    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
88) {
89    let Ok(mut sink) = music_controller.single_mut() else {
90        return;
91    };
92
93    if keyboard_input.just_pressed(KeyCode::KeyM) {
94        sink.toggle_mute();
95    }
96}
```

---

## Crate audio Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/index.html

**Contents:**
- Crate audio Copy item path
- Modules§
- Structs§
- Enums§
- Traits§

Audio support for the game engine Bevy

**Examples:**

Example 1 (unknown):
```unknown
fn main() {
   App::new()
        .add_plugins((MinimalPlugins, AssetPlugin::default(), AudioPlugin::default()))
        .add_systems(Startup, play_background_audio)
        .run();
}

fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("background_audio.ogg")),
        PlaybackSettings::LOOP,
    ));
}
```

---

## Trait Source Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/trait.Source.html

**Contents:**
- Trait Source Copy item path
- §A quick lesson about sounds
  - §Sampling
  - §Channels
  - §The Source trait
- §Frames
- Required Methods§
    - fn current_frame_len(&self) -> Option<usize>
    - fn channels(&self) -> u16
    - fn sample_rate(&self) -> u32

A sound is a vibration that propagates through air and reaches your ears. This vibration can be represented as an analog signal.

In order to store this signal in the computer’s memory or on the disk, we perform what is called sampling. The consists in choosing an interval of time (for example 20µs) and reading the amplitude of the signal at each interval (for example, if the interval is 20µs we read the amplitude every 20µs). By doing so we obtain a list of numerical values, each value being called a sample.

Therefore a sound can be represented in memory by a frequency and a list of samples. The frequency is expressed in hertz and corresponds to the number of samples that have been read per second. For example if we read one sample every 20µs, the frequency would be 50000 Hz. In reality, common values for the frequency are 44100, 48000 and 96000.

But a frequency and a list of values only represent one signal. When you listen to a sound, your left and right ears don’t receive exactly the same signal. In order to handle this, we usually record not one but two different signals: one for the left ear and one for the right ear. We say that such a sound has two channels.

Sometimes sounds even have five or six channels, each corresponding to a location around the head of the listener.

The standard in audio manipulation is to interleave the multiple channels. In other words, in a sound with two channels the list of samples contains the first sample of the first channel, then the first sample of the second channel, then the second sample of the first channel, then the second sample of the second channel, and so on. The same applies if you have more than two channels. The rodio library only supports this schema.

Therefore in order to represent a sound in memory in fact we need three characteristics: the frequency, the number of channels, and the list of samples.

A Rust object that represents a sound should implement the Source trait.

The three characteristics that describe a sound are provided through this trait:

The samples rate and number of channels of some sound sources can change by itself from time to time.

Note: As a basic example, if you play two audio files one after the other and treat the whole as a single source, then the channels and samples rate of that source may change at the transition between the two files.

However, for optimization purposes rodio supposes that the number of channels and the frequency stay the same for long periods of ti

*[Content truncated]*

**Examples:**

Example 1 (unknown):
```unknown
pub trait Source: Iteratorwhere
    Self::Item: Sample,{
Show 29 methods    // Required methods
    fn current_frame_len(&self) -> Option<usize>;
    fn channels(&self) -> u16;
    fn sample_rate(&self) -> u32;
    fn total_duration(&self) -> Option<Duration>;

    // Provided methods
    fn buffered(self) -> Buffered<Self> ⓘ
       where Self: Sized { ... }
    fn mix<S>(self, other: S) -> Mix<Self, S> ⓘ
       where Self: Sized,
             Self::Item: FromSample<<S as Iterator>::Item>,
             S: Source,
             <S as Iterator>::Item: Sample { ... }
    fn repeat_infinite(self) -
...
```

Example 2 (javascript):
```javascript
// Apply Automatic Gain Control to the source (AGC is on by default)
let agc_source = source.automatic_gain_control(1.0, 4.0, 0.005, 5.0);

// Get a handle to control the AGC's enabled state (optional)
let agc_control = agc_source.get_agc_control();

// You can toggle AGC on/off at any time (optional)
agc_control.store(false, std::sync::atomic::Ordering::Relaxed);

// Add the AGC-controlled source to the sink
sink.append(agc_source);

// Note: Using agc_control is optional. If you don't need to toggle AGC,
// you can simply use the agc_source directly without getting agc_control.
```

Example 3 (javascript):
```javascript
use std::time::Duration;

let source = source.buffered().reverb(Duration::from_millis(100), 0.7);
```

---

## Struct GlobalVolume Copy item path

**URL:** https://docs.rs/bevy/0.17.2/bevy/audio/struct.GlobalVolume.html

**Contents:**
- Struct GlobalVolume Copy item path
- Fields§
- Implementations§
  - impl GlobalVolume
    - pub fn new(volume: Volume) -> GlobalVolume
- Trait Implementations§
  - impl Clone for GlobalVolume
    - fn clone(&self) -> GlobalVolume
    - fn clone_from(&mut self, source: &Self)
  - impl Debug for GlobalVolume

Use this Resource to control the global volume of all audio.

Note: Changing GlobalVolume does not affect already playing audio.

The global volume of all audio.

Create a new GlobalVolume with the given volume.

Returns the argument unchanged.

Creates Self using default().

That is, this conversion is whatever the implementation of From<T> for U chooses to do.

**Examples:**

Example 1 (unknown):
```unknown
pub struct GlobalVolume {
    pub volume: Volume,
}
```

---
