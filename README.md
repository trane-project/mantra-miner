# Mantra Miner

[![Github Checks Status](https://img.shields.io/github/checks-status/trane-project/mantra-miner/master)](https://github.com/trane-project/mantra-miner/actions?query=branch%3Amaster)
[![Coverage Status](https://img.shields.io/coveralls/github/trane-project/mantra-miner)](https://coveralls.io/github/trane-project/mantra-miner?branch=master)
[![docs.rs](https://img.shields.io/docsrs/mantra-miner)](https://docs.rs/mantra-miner)
[![Latest Version](https://img.shields.io/crates/v/mantra-miner)](https://crates.io/crates/mantra-miner)
[![Stars](https://img.shields.io/github/stars/trane-project/mantra-miner?style=social)](https://github.com/trane-project/mantra-miner/stargazers)

A spoof on crypto mining, this library spawns a thread and writes the specified mantras to a buffer.
The user can select the mantras as well as an optional preparation and conclusion sections that
mirror the format of traditional Buddhist ritual practices.

The library was born for use with [Trane](https://github.com/trane-project/trane) as a way to allow
its users to contribute back to the maintainer in a symbolic and non-monetary way. In Trane, the
mantra of Tara Sarasvati - the manifestation of the Buddhist deity Tara associated with wisdom,
music, learning, and the arts - is recited as the users run the software to acquire and practice
complex skills.
 
Similar examples of using mantras in mediums other than the voice exist throughout Asia. Prayer
wheels contain written mantras that are said to generate the same merit as reciting the amount of
mantras inside every time the wheel completes a full rotation. With the use of microfilm, a prayer
wheel can contain millions or more mantras. Another example consists of carving mantras in rock,
which is common in the Himalayas and Tibet.

## Questions and Answers

### Why?

The short answer is that it's fun and maintaining my own projects allows me to do things that would
be otherwise frowned upon in a day job or in a commercial software offering.

The long answer is that I want to commit to the pledge of keeping Trane and all other software and
content offered by the Trane Project free (both as in beer and as in freedom). In practice, this
means that most of the people who ever benefit from Trane will never contribute back to the project,
neither with money nor code nor educational materials.

As a symbolic mean to allow users to contribute back to the project, I settled on the idea of having
Trane "recite" mantras while it runs. The mantra chosen for Trane is the mantra of Tara Sarasvati,
the manifestation of Tara most closely associated with music and learning. Given that Trane was
created to help me practice music, I think the mantra is quite appropriate.

### How are the mantras recited?

The library must act without drawing any attention from the user, so the mantras cannot be recited
by having them be outputted to the speakers or to the terminal. Instead, a separate thread is
spawned to write the mantras syllable by syllable to a buffer at regular intervals.

### Do mantras work?

While I am unable to make a definitive statement that would be accepted by staunch materialists, I
thought best to include a couple examples of remarkable people who believed in the power of mantras.

- **Thích Quảng Đức** was a Vietnamese Buddhist monk who burned himself to death without moving a
muscle in protest of the repressive religious policies implemented by the pro-Catholic government of
South Vietnam. He believed in mantras enough to pick one (the call to Amitābha Buddha in Vietnamese,
"Nam mô A Di Đà Phật") to recite before striking the match that would set him on fire.

- **Garchen Rinpoche** spent twenty years in a labor camp due to his status as a high Lama before
  the Chinese invasion of Tibet, practicing in secret and eventually achieving enough freedom from
  his mental obscurations that he came to see the labor camp as a land of jewels. During his time in
  prison he fell into a frozen water reservoir in the middle of winter and called out to White Tara
  using her mantra.

Personally, I have found mantras to have a powerful effect on my mind, to the point that I credit
Avalokiteshvara's mantra of compassion ("om mani padme hum") with helping me overcome what in
western medical terms would be diagnosed as a severe case of depression, but in actuality was simply
a case of a mistaken and bleak view of reality. The recitation of the mantra helped the view
represented by it to become embodied and not simply an intellectual construct.

### Who else should use this library?

Any maintainer of an open source project that wants their users to contribute back to their project
symbolically can do so by using this library to have their binaries "recite" mantras of their
choosing. There's no restriction on the type of mantras that can be used, so prayers of other faiths
can be used as well.
