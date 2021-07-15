//! 
//! A dynamic synth type.
//!

use instrument::{mode, note_freq};
use synth;

pub use instrument::mode::Dynamic as Mode;
pub use instrument::note_freq::Dynamic as NoteFreqGenerator;

pub use self::oscillator::{Oscillator, Waveform, Amplitude, Frequency, FreqWarp};
pub use self::oscillator::new as new_oscillator;


pub mod oscillator {
    use oscillator::Oscillator as Osc;
    pub use oscillator::waveform::Dynamic as Waveform;
    pub use oscillator::amplitude::Dynamic as Amplitude;
    pub use oscillator::frequency::Dynamic as Frequency;
    pub use oscillator::freq_warp::Dynamic as FreqWarp;

    /// An alias for a totally dynamic Oscillator.
    pub type Oscillator = Osc<Waveform, Amplitude, Frequency, FreqWarp>;

    /// Construct a new dynamic oscillator.
    pub fn new() -> Oscillator {
        use pitch::{LetterOctave, Letter};
        Oscillator::new(Waveform::Sine,
                        Amplitude::Constant(0.7),
                        Frequency::Hz(LetterOctave(Letter::C, 2).hz() as f64),
                        FreqWarp::None)
    }
}


/// An alias for a completely dynamic synth.
pub type Synth = synth::Synth<mode::Dynamic,
                              note_freq::DynamicGenerator,
                              oscillator::Waveform,
                              oscillator::Amplitude,
                              oscillator::Frequency,
                              oscillator::FreqWarp>;

impl Synth {

    /// Construct an entirely dynamic `Synth`.
    pub fn dynamic(dynamic_mode: mode::Dynamic) -> Self {
        synth::Synth::new(dynamic_mode, note_freq::DynamicGenerator::Constant)
    }

    pub fn dynamic_retrigger() -> Self {
        Self::dynamic(mode::Dynamic::retrigger())
    }

    pub fn dynamic_legato() -> Self {
        Self::dynamic(mode::Dynamic::legato())
    }

    pub fn dynamic_poly() -> Self {
        Self::dynamic(mode::Dynamic::poly())
    }

    /// Set the mode of the synth.
    pub fn set_mode(&mut self, mode: mode::Dynamic) {
        self.instrument.mode = mode;
    }

    /// Set the note frequency generator to be used by the synth.
    pub fn set_note_freq_gen(&mut self, note_freq_gen: note_freq::DynamicGenerator) {
        self.instrument.note_freq_gen = note_freq_gen;
    }
    
    /// Set the attack in milliseconds.
    pub fn set_attack<Attack>(&mut self, attack: Attack)
        where Attack: Into<time::Ms>,
    {
        self.instrument.attack_ms = attack.into();
    }
    
    /// Set the release in milliseconds.
    pub fn set_release<Release>(&mut self, release: Release)
        where Release: Into<time::Ms>,
    {
        self.instrument.attack_ms = release.into();
    }

    #[inline]
    /// Set the Synth's detune amount.
    pub fn set_detune(&mut self, detune: f32) {
        self.instrument.detune = detune;
    }
    #[inline]
    /// Set the Synth's spread amount.
    pub fn set_spread(&mut self, spread: f32) {
        self.spread = spread;
    }
    #[inline]
    /// Set the Synth's volume.
    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol;
    }


}
