//! Mantra Miner

use anyhow::Result;
use parking_lot::Mutex;
use std::{
    io::{sink, BufWriter, Sink, Write},
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc,
    },
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

    /// The channel used to signal the thread to stop.
    stop_channel: Option<Sender<()>>,
}

impl MantraMiner {
    /// Returns a new instance of `MantraMiner` with the given options.
    pub fn new(options: Options) -> MantraMiner {
        MantraMiner {
            options,
            count: Arc::new(Mutex::new(0)),
            stop_channel: None,
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
    fn run(options: Options, total_count: Arc<Mutex<usize>>, rx: Receiver<()>) -> Result<()> {
        let mut run_count = 0;
        let mut output = BufWriter::new(sink());
        let rate = Duration::from_millis(options.rate_ms);

        while options.should_repeat(run_count) {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

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
    pub fn start(&mut self) -> Result<()> {
        // Stop any existing thread.
        self.stop()?;

        let cloned_options = self.options.clone();
        let cloned_count = self.count.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = MantraMiner::run(cloned_options, cloned_count, rx);
        });
        self.stop_channel = Some(tx);
        Ok(())
    }

    /// Stops the thread running the mantra miner.
    pub fn stop(&mut self) -> Result<()> {
        if let Some(tx) = self.stop_channel.take() {
            let _ = tx.send(());
        }
        Ok(())
    }

    /// Returns the options used to configure this mantra miner.
    pub fn options(&self) -> Options {
        self.options.clone()
    }

    /// Returns the count of the mantra miner.
    pub fn count(&self) -> usize {
        self.count.lock().clone()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{thread, time::Duration};

    use crate::{Mantra, MantraMiner, Options};

    const PREPARATION: &str = "I take refuge in the Three Jewels and arise bodhicitta.";
    const DEDICATION: &str = "I dedicate the merit of this practice to all sentient beings.";

    fn simple_mantra() -> Mantra {
        Mantra {
            syllables: vec![
                "om".to_string(),
                "ma".to_string(),
                "ni".to_string(),
                "pad".to_string(),
                "me".to_string(),
                "hum".to_string(),
            ],
            repeats: None,
        }
    }

    fn repeated_mantra() -> Mantra {
        Mantra {
            syllables: vec!["hri".to_string()],
            repeats: Some(108),
        }
    }

    #[test]
    fn set_repeats() -> Result<()> {
        let options = Options {
            preparation: None,
            mantras: vec![simple_mantra()],
            conclusion: None,
            rate_ms: 1,
            repeats: Some(10),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(1000));
        miner.stop()?;
        assert_eq!(miner.count(), 10);
        Ok(())
    }

    #[test]
    fn indefinite_repeats() -> Result<()> {
        let options = Options {
            preparation: None,
            mantras: vec![simple_mantra()],
            conclusion: None,
            rate_ms: 1,
            repeats: None,
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(1000));
        miner.stop()?;
        assert!(miner.count() > 10);
        Ok(())
    }

    #[test]
    fn with_preparation_and_dedication() -> Result<()> {
        let options = Options {
            preparation: Some(PREPARATION.to_string()),
            mantras: vec![simple_mantra()],
            conclusion: Some(DEDICATION.to_string()),
            rate_ms: 1,
            repeats: Some(3),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(1000));
        miner.stop()?;
        assert_eq!(miner.count(), 3);
        Ok(())
    }

    #[test]
    fn using_repeated_mantra() -> Result<()> {
        let options = Options {
            preparation: None,
            mantras: vec![repeated_mantra()],
            conclusion: None,
            rate_ms: 1,
            repeats: Some(3),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(1000));
        miner.stop()?;
        assert_eq!(miner.count(), 3);
        Ok(())
    }
}
