#![warn(
rust_2018_idioms,
unused,
rust_2021_compatibility,
nonstandard_style,
future_incompatible,
missing_copy_implementations,
missing_debug_implementations,
missing_docs,
clippy::all,
clippy::pedantic,
clippy::nursery,
clippy::cargo,
clippy::unwrap_used,
clippy::missing_assert_message,
clippy::todo,
clippy::allow_attributes_without_reason,
clippy::panic,
clippy::panicking_unwrap,
clippy::panic_in_result_fn
)]

use std::time::Duration;

use rodio::{Sample, Source};
use rodio::queue::{queue, SourcesQueueOutput};
use rodio::source::SineWave;
use rodio::source::Zero;

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // let notes_c4 = SineWave::new(261.626);
    // let notes_e4 = SineWave::new(329.628);
    // let notes_g4 = SineWave::new(391.995);

    // let source = source::from_iter([
    //         notes_c4.clone().take_duration(Duration::from_secs_f32_f32(1.0)),
    //         notes_e4.clone().take_duration(Duration::from_secs_f32_f32(1.0)),
    //         notes_g4.clone().take_duration(Duration::from_secs_f32_f32(1.0)),
    //     ]
    //         .into_iter(),);
    //
    // let chord_c4 = notes_c4.mix(notes_e4).mix(notes_g4).take_duration(Duration::from_secs_f32_f32(1.0));
    //
    // sink.append(source);
    // sink.append(chord_c4);

    let notes_a4 = SineWave::new(440.0);
    let notes_a7 = SineWave::new(3520.0);
    let notes_b4 = SineWave::new(493.883);
    let notes_b7 = SineWave::new(3951.07);
    let notes_c4 = SineWave::new(261.626);
    let notes_c5 = SineWave::new(523.251);
    let notes_c8 = SineWave::new(4186.01);
    let notes_d4 = SineWave::new(293.665);
    let notes_e4 = SineWave::new(329.628);
    let notes_e7 = SineWave::new(2637.02);
    let notes_e5 = SineWave::new(659.255);
    let notes_f4 = SineWave::new(349.228);
    let notes_f7 = SineWave::new(2793.83);
    let notes_f4_sharp = SineWave::new(369.994);
    let notes_g4 = SineWave::new(391.995);
    let notes_g4_sharp = SineWave::new(415.305);
    let notes_g7_sharp = SineWave::new(3322.44);


    let am = notes_a4.clone().mix(notes_c5.clone()).mix(notes_e5);
    let c = notes_c4.mix(notes_e4.clone()).mix(notes_g4);
    let d = notes_d4.clone().mix(notes_f4_sharp).mix(notes_a4.clone());
    let dm = notes_d4.mix(notes_f4.clone()).mix(notes_a4.clone());
    let e = notes_e4.mix(notes_g4_sharp).mix(notes_b4);
    let e7 = notes_e7.mix(notes_g7_sharp).mix(notes_b7);
    let f = notes_f4.mix(notes_a4).mix(notes_c5);
    let f7 = notes_f7.mix(notes_a7).mix(notes_c8);
    let cadence = 1.95;

    let (input, output) = queue(false);

    // intro
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // verse 1
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // verse 2
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // verse 3
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // organ solo
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence*2.0)).clone());
    // input.append(Zero::new(d.channels(), d.sample_rate()).take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    // c/e chord should be here
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // verse 4
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    //verse 5
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // verse 6
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(f.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(e7.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(c.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(d.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(f7.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);
    input.append(e7.clone().take_duration(Duration::from_secs_f32(cadence)).clone(),);

    // coda
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(dm.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(dm.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(dm.clone().take_duration(Duration::from_secs_f32(cadence)).clone());

    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(dm.clone().take_duration(Duration::from_secs_f32(cadence)).clone());
    input.append(am.clone().take_duration(Duration::from_secs_f32(cadence)).clone());


    // Decode that sound file into a source

    sink.append(output
       .buffered()
    );

    sink.sleep_until_end();
}
//
// struct InstrumentNote<A: Sample+Sync+Send+'static> {
//     note: SourcesQueueOutput<A>,
// }
//
// impl<A: Sample+Sync+Send+'static> InstrumentNote<A> {
//     fn new(note: impl Source<Item=A>+Clone+Sized+Send+'static, attack_duration: Duration, attack_amplitude: f32, decay_duration: Duration, sustain_duration: Duration, release_duration: Duration) -> Self {
//         let attack = note
//             .clone()
//             .amplify(attack_amplitude)
//             .fade_in(attack_duration)
//             .take_duration(attack_duration+decay_duration)
//             .buffered();
//         let decay = attack.take_crossfade_with(note.clone().skip_duration(attack_duration), decay_duration).buffered();
//         let sustain = note.clone().skip_duration(attack_duration+decay_duration).take_duration(sustain_duration);
//         let release = note.clone()
//             .skip_duration(attack_duration+decay_duration+sustain_duration)
//             .take_duration(release_duration)
//             .take_crossfade_with(Zero::new(note.channels(), note.sample_rate()), release_duration).buffered();
//
//         let (input, output) = queue(false);
//         input.append(attack);
//         input.append(decay);
//         input.append(sustain);
//         input.append(release);
//
//         Self {
//             note: output,
//         }
//     }
// }
//
// impl<A> Iterator for InstrumentNote<A> where A: Sample+Sync+Send {
//     type Item = A;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.note.next()
//     }
// }
//
// impl<A> Source for InstrumentNote<A>
//     where A: Sample+Sync+Send
// {
//     fn current_frame_len(&self) -> Option<usize> {
//         self.note.current_frame_len()
//     }
//
//     fn channels(&self) -> u16 {
//         self.note.channels()
//     }
//
//     fn sample_rate(&self) -> u32 {
//         self.note.sample_rate()
//     }
//
//     fn total_duration(&self) -> Option<Duration> {
//         self.note.total_duration()
//     }
// }

//
// fn c_major_inversion_e() -> Vec<SineWave> {
//     let notes_e4 = SineWave::new(329.628);
//     let notes_g4 = SineWave::new(391.995);
//     let notes_c5 = SineWave::new(523.251);
//
//     vec![notes_e4, notes_g4, notes_c5]
// }
//
// fn e7_major() -> Vec<SineWave> {
//     let notes_e4 = SineWave::new(329.628);
//     let notes_g4 = SineWave::new(391.995);
//     let notes_b4 = SineWave::new(493.883);
//
//     vec![notes_e4, notes_g4, notes_b4]
// }