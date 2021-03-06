#![feature(use_extern_macros, nll)]

extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, BaseAudioContext, AudioNode, AudioScheduledSourceNode, OscillatorType};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,


}

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FmOsc {
        // TODO, how to throw from a constructor?

        let ctx = web_sys::AudioContext::new().unwrap();
        let base: &BaseAudioContext = ctx.as_ref();

        // create our web audio objects
        let primary = base.create_oscillator().unwrap();
        let fm_osc = base.create_oscillator().unwrap();
        let gain = base.create_gain().unwrap();
        let fm_gain = base.create_gain().unwrap();

        // some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);


        // Create base class references:
        let primary_node: &AudioNode = primary.as_ref();
        let gain_node: &AudioNode = gain.as_ref();
        let fm_osc_node: &AudioNode = fm_osc.as_ref();
        let fm_gain_node: &AudioNode = fm_gain.as_ref();
        let destination = base.destination();
        let destination_node: &AudioNode = destination.as_ref();


        // connect them up:

        // The primary oscillator is routed through the gain node, so that it can control the overall output volume
        primary_node.connect_with_destination_and_output_and_input_using_destination(gain.as_ref());
        // Then connect the gain node to the AudioContext destination (aka your speakers)
        gain_node.connect_with_destination_and_output_and_input_using_destination(destination_node);

        // the FM oscillator is connected to its own gain node, so it can control the amount of modulation
        fm_osc_node.connect_with_destination_and_output_and_input_using_destination(fm_gain.as_ref());

        // Connect the FM oscillator to the frequency parameter of the main oscillator, so that the
        // FM node can modulate its frequency
        fm_gain_node.connect_with_destination_and_output_using_destination(&primary.frequency());


        // start the oscillators!
        AsRef::<AudioScheduledSourceNode>::as_ref(&primary).start();
        AsRef::<AudioScheduledSourceNode>::as_ref(&fm_osc).start();

        FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        }

    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 { gain = 1.0; }
        if gain < 0.0 { gain = 0.0; }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the primary oscillator, so
        // we update the frequency of both in this method
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);

    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain.gain().set_value(self.fm_gain_ratio * self.primary.frequency().value());

    }

    /// This should be between 0 and 1, though higher values are accepted
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }


}