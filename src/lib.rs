//! Mantra Miner

use anyhow::Result;
use parking_lot::Mutex;
use std::{
    io::{sink, BufWriter, Sink, Write},
    sync::Arc,
    thread,
    time::Duration,
};

/// A mantra to be "recited" by the miner. Since a computer can't actually recite a mantra, the term
/// refers to the process of writing the mantra syllable by syllable to an input buffer.
#[derive(Clone, Debug)]
pub struct Mantra {
    /// The syllables of the mantra. The mantra will be recited syllable by syllable.
    pub syllables: Vec<String>,

    /// The number of times to repeat the mantra. If it's `None`, the mantra will be repeated once.
    pub repeats: Option<usize>,
}

impl Mantra {
    fn recite(&self, output: &mut BufWriter<Sink>, rate: Duration) -> Result<()> {
        let repeats = self.repeats.unwrap_or(1);
        for _ in 0..repeats {
            for syllable in &self.syllables {
                output.write(syllable.as_bytes())?;
                thread::sleep(rate);
            }
        }
        Ok(())
    }
}

/// The options used to configure the mantra miner.
#[derive(Clone, Debug)]
pub struct Options {
    /// Traditional Buddhist sadhanas, or ritual practices, consists of three parts. The first part,
    /// preparation, consists of taking refuge in the Three Jewels and arising bodhicitta, the
    /// desire to attain enlightenment for the benefit of all sentient beings.
    pub preparation: Option<String>,

    /// The second part of the sadhana is the main body of the practice, which for the purposes of
    /// this mantra miner consists of reciting the given mantras.
    pub mantras: Vec<Mantra>,

    /// The third part of the sadhana is the conclusion, which consists of dedicating the merit of
    /// the practice to all sentient beings.
    pub conclusion: Option<String>,

    /// The number of times to repeat the entire sadhana. If it's `None`, the sadhana will be
    /// repeated indefinitely until the miner is stopped or the program is terminated.
    pub repeats: Option<usize>,

    /// The number of milliseconds to wait between each syllable of a mantra or character of the
    /// preparation or conclusion.
    pub rate_ms: u64,
}

impl Options {
    /// Returns whether the mantra miner should perform another iteration.
    fn should_repeat(&self, count: usize) -> bool {
        match self.repeats {
            Some(repeats) => count < repeats,
            None => true,
        }
    }
}

/// The mantra miner.
pub struct MantraMiner {
    /// The options used to configure the mantra miner.
    options: Options,

    /// The number of times the mantra miner has completed a recitation of the entire sadhana.
    count: Arc<Mutex<usize>>,

    /// Whether the mantra miner is currently running.
    running: bool,

    /// The handle to the thread running the mantra miner.
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl MantraMiner {
    /// Returns a new instance of `MantraMiner` with the given options.
    pub fn new(options: Options) -> MantraMiner {
        MantraMiner {
            options,
            count: Arc::new(Mutex::new(0)),
            running: false,
            thread_handle: None,
        }
    }

    /// Recites the optional string. Used to recite the preparation and conclusion.
    fn recite_string(
        input: &Option<String>,
        output: &mut BufWriter<Sink>,
        rate: Duration,
    ) -> Result<()> {
        match input {
            None => Ok(()),
            Some(input) => {
                for c in input.chars() {
                    let mut b = [0; 4];
                    output.write(c.encode_utf8(&mut b).as_bytes())?;
                    thread::sleep(rate);
                }
                Ok(())
            }
        }
    }

    /// Runs the mantra miner.
    fn run(options: Options, total_count: Arc<Mutex<usize>>) -> Result<()> {
        let mut run_count = 0;
        let mut output = BufWriter::new(sink());
        let rate = Duration::from_millis(options.rate_ms);

        while options.should_repeat(run_count) {
            Self::recite_string(&options.preparation, &mut output, rate)?;

            for mantra in &options.mantras {
                mantra.recite(&mut output, rate)?;
            }

            Self::recite_string(&options.conclusion, &mut output, rate)?;

            *total_count.lock() += 1;
            run_count += 1;
        }
        Ok(())
    }

    /// Spawns a new thread to run the mantra miner.
    pub fn start(&mut self) {
        let cloned_options = self.options.clone();
        let cloned_count = self.count.clone();
        self.thread_handle = Some(thread::spawn(move || {
            let _ = MantraMiner::run(cloned_options, cloned_count);
        }));
        self.running = true;
    }

    /// Stops the thread running the mantra miner.
    pub fn stop(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {}
    }

    /// Returns the options used to configure this mantra miner.
    pub fn options(&self) -> Options {
        self.options.clone()
    }

    /// Returns the count of the mantra miner.
    pub fn count(&self) -> usize {
        self.count.lock().clone()
    }

    /// Returns whether the mantra miner is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {}
