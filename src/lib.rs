//! mantra-miner is a library to make your software "recite" mantras while it runs.
//!
//! A spoof on crypto mining, this library spawns a thread and writes the specified mantras to a
//! buffer. The user can select the mantras as well as an optional preparation and conclusion
//! sections that mirror the format of traditional Buddhist ritual practices.
//!
//! The library was born for use with [Trane](https://github.com/trane-project/trane) as a way to
//! allow its users to contribute back to the maintainer in a symbolic and non-monetary way. In
//! Trane, the mantra of Tara Sarasvati - the manifestation of the Buddhist deity Tara associated
//! with wisdom, music, learning, and the arts - is recited as the users run the software to
//! acquire and practice complex skills.
//!  
//! Similar examples of using mantras in mediums other than the voice exist throughout Asia. Prayer
//! wheels contain written mantras that are said to generate the same merit as reciting the amount
//! of mantras inside every time the wheel completes a full rotation. With the use of microfilm, a
//! prayer wheel can contain millions or more mantras. Another example consists of carving mantras
//! in rock, which is common in the Himalayas and Tibet.
//!
//! For more information, check the project's README.

use anyhow::Result;
use parking_lot::Mutex;
use std::{
    io::{sink, BufWriter, Write},
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc,
    },
    thread,
    time::Duration,
};

/// A mantra to be "recited" by the miner. Since a computer can't actually recite a mantra, the term
/// refers to the process of writing the mantra syllable by syllable to an output buffer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mantra {
    /// The syllables of the mantra. The mantra will be recited syllable by syllable.
    pub syllables: Vec<String>,

    /// The number of times to repeat the mantra. If it's `None`, the mantra will be repeated once.
    pub repeats: Option<usize>,
}

impl Mantra {
    /// Writes the mantra syllable by syllable to the given buffer.
    fn recite<T>(&self, output: &mut BufWriter<T>, rate: Duration) -> Result<()>
    where
        T: Write,
    {
        let repeats = self.repeats.unwrap_or(1);
        for _ in 0..repeats {
            for syllable in &self.syllables {
                output.write_all(syllable.as_bytes())?;
                output.write_all("\n".as_bytes())?;
                thread::sleep(rate);
            }
        }
        Ok(())
    }
}

/// The options used to configure the mantra miner.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Options {
    /// Traditional Buddhist sadhanas, or ritual practices, consists of three parts. The first part,
    /// preparation, consists of taking refuge in the Three Jewels and arising bodhicitta, the
    /// desire to attain enlightenment for the benefit of all sentient beings.
    pub preparation: Option<String>,

    /// The number of times to repeat the preparation. If the value is `None`, it will be recited
    /// once.
    pub preparation_repeats: Option<usize>,

    /// The second part of the sadhana is the main body of the practice, which for the purposes of
    /// this mantra miner consists of reciting the given mantras.
    pub mantras: Vec<Mantra>,

    /// The third part of the sadhana is the conclusion, which traditionally consists of dedicating
    /// the merit of the practice to all sentient beings.
    pub conclusion: Option<String>,

    /// The number of times to repeat the conclusion. If the value is `None`, it will be recited
    /// once.
    pub conclusion_repeats: Option<usize>,

    /// The number of times to repeat the entire sadhana. If it's `None`, the sadhana will be
    /// repeated indefinitely until the miner is stopped or the program is terminated.
    pub repeats: Option<usize>,

    /// The number of nanoseconds to wait between each syllable of a mantra or character of the
    /// preparation or conclusion.
    pub rate_ns: u64,
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

/// A mantra miner that spawns a thread and "recites" mantras by writing them to an output buffer.
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
    fn recite_string<T>(
        input: &Option<String>,
        output: &mut BufWriter<T>,
        rate: Duration,
    ) -> Result<()>
    where
        T: Write,
    {
        match input {
            None => Ok(()),
            Some(input) => {
                for c in input.chars() {
                    let mut b = [0; 4];
                    output.write_all(c.encode_utf8(&mut b).as_bytes())?;
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
        let rate = Duration::from_nanos(options.rate_ns);

        while options.should_repeat(run_count) {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

            let preparation_repeats = options.preparation_repeats.unwrap_or(1);
            for _ in 0..preparation_repeats {
                Self::recite_string(&options.preparation, &mut output, rate)?;
            }

            for mantra in &options.mantras {
                mantra.recite(&mut output, rate)?;
            }

            let conclusion_repeats = options.conclusion_repeats.unwrap_or(1);
            for _ in 0..conclusion_repeats {
                Self::recite_string(&options.conclusion, &mut output, rate)?;
            }

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
        *self.count.lock()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{
        io::{BufWriter, Write},
        thread,
        time::Duration,
    };

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
    fn should_repeat() {
        let mut options = Options::default();

        options.repeats = Some(10);
        assert_eq!(options.should_repeat(5), true);
        assert_eq!(options.should_repeat(10), false);
        assert_eq!(options.should_repeat(50), false);

        options.repeats = None;
        assert_eq!(options.should_repeat(5), true);
        assert_eq!(options.should_repeat(10), true);
        assert_eq!(options.should_repeat(50), true);
    }

    #[test]
    fn recite_string() -> Result<()> {
        let rate = Duration::from_nanos(10);
        let buffer = Vec::with_capacity(100);
        let mut output = BufWriter::new(buffer);
        MantraMiner::recite_string(&Some(PREPARATION.to_string()), &mut output, rate)?;
        output.flush()?;
        assert_eq!(output.get_ref(), PREPARATION.as_bytes());
        Ok(())
    }

    #[test]
    fn recite_mantra() -> Result<()> {
        let mantra = simple_mantra();
        let rate = Duration::from_nanos(10);
        let buffer = Vec::with_capacity(100);
        let mut output = BufWriter::new(buffer);
        mantra.recite(&mut output, rate)?;
        output.flush()?;
        assert_eq!(output.get_ref(), "om\nma\nni\npad\nme\nhum\n".as_bytes());
        Ok(())
    }

    #[test]
    fn set_repeats() -> Result<()> {
        let options = Options {
            preparation: None,
            preparation_repeats: None,
            mantras: vec![simple_mantra()],
            conclusion: None,
            conclusion_repeats: None,
            rate_ns: 1000,
            repeats: Some(10),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(10));
        miner.stop()?;
        assert_eq!(miner.count(), 10);
        Ok(())
    }

    #[test]
    fn indefinite_repeats() -> Result<()> {
        let options = Options {
            preparation: None,
            preparation_repeats: None,
            mantras: vec![simple_mantra()],
            conclusion: None,
            conclusion_repeats: None,
            rate_ns: 1000,
            repeats: None,
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(10));
        miner.stop()?;
        assert!(miner.count() > 10);
        Ok(())
    }

    #[test]
    fn with_preparation_and_conclusion() -> Result<()> {
        let options = Options {
            preparation: Some(PREPARATION.to_string()),
            preparation_repeats: None,
            mantras: vec![simple_mantra()],
            conclusion: Some(DEDICATION.to_string()),
            conclusion_repeats: None,
            rate_ns: 1000,
            repeats: Some(3),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(100));
        miner.stop()?;
        assert_eq!(miner.count(), 3);
        Ok(())
    }

    #[test]
    fn with_repeated_preparation_and_conclusion() -> Result<()> {
        let options = Options {
            preparation: Some(PREPARATION.to_string()),
            preparation_repeats: Some(3),
            mantras: vec![simple_mantra()],
            conclusion: Some(DEDICATION.to_string()),
            conclusion_repeats: Some(3),
            rate_ns: 1000,
            repeats: Some(3),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(100));
        miner.stop()?;
        assert_eq!(miner.count(), 3);
        Ok(())
    }

    #[test]
    fn using_repeated_mantra() -> Result<()> {
        let options = Options {
            preparation: Some(PREPARATION.to_string()),
            preparation_repeats: None,
            mantras: vec![repeated_mantra()],
            conclusion: Some(DEDICATION.to_string()),
            conclusion_repeats: None,
            rate_ns: 1000,
            repeats: Some(3),
        };
        let mut miner = MantraMiner::new(options);
        miner.start()?;
        thread::sleep(Duration::from_millis(100));
        miner.stop()?;
        assert_eq!(miner.count(), 3);
        Ok(())
    }

    #[test]
    fn options() {
        let options = Options {
            preparation: None,
            preparation_repeats: None,
            mantras: vec![repeated_mantra()],
            conclusion: None,
            conclusion_repeats: None,
            rate_ns: 1000,
            repeats: Some(3),
        };
        let options_clone = options.clone();
        let miner = MantraMiner::new(options);
        assert_eq!(miner.options(), options_clone);
    }
}
