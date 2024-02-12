use std::io::BufReader;
use std::io::stdin;

use std::thread;
use std::time::Duration;

// This is to change the amount of time I have to wait when its time to debug.
const DBG_MODE: bool = true;

fn main() {

    let mut timer_start_decision = String::new();
    let mut add_tasks_decision = String::new();

    loop {

    }
    // number of seconds of work time, 5 seconds at debug, 15 mins at performance
    enter_loop();
}

fn enter_loop() {
    let mut seconds = if DBG_MODE {
        Duration::from_secs(5)
    } else {
        Duration::from_secs(900)
    };
    // This is to alternate between work and stand time.
    let mut alternator = true;

    loop {
        thread::sleep(seconds.clone());

        // if its time to stand up, alternator should be true.
        // if it isn't alternator should be false. That means its time to get back to work.
        if alternator {
            println!("Time to Stand Up!");
        } else {
            println!("Time to Get Back to Work!");
        }

        // plays audio every iteration. since the thread sleeps at the beginning of
        // the loop for 'seconds' seconds, it only plays every time its either time to
        // stand up or go back to work.
        play_audio();

        // seconds alternates between dbg -> 2 and 5 seconds, perf -> 15 seconds and 15 mins.
        seconds = if alternator {
            if DBG_MODE {
                Duration::from_secs(2)
            } else {
                Duration::from_secs(15)
            }
        } else {
            if DBG_MODE {
                Duration::from_secs(5)
            } else {
                Duration::from_secs(900)
            }
        };

        alternator = !alternator;
    }
}

// function that plays the audio
fn play_audio() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // Since I'm running this with PATH=$PATH to run it from anywhere, I can't have this be relative
    // Otherwise it can't find the audio file.
    let file = std::fs::File::open(
        "/Users/justinlee/Desktop/rust-tutorial-projects/standup_timer/assets/notifSound.mp3",
    )
    .unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    // Thread sleeps until the audio finishes
    sink.sleep_until_end();
}

/*
use std::io::BufReader;
use std::thread;
use std::time::Duration;

use std::io::stdin;

fn main() {
    // This is to change the amount of time I have to wait when its time to debug.
    let debug_mode: bool = true;

    // These will either be y or n.
    let mut timer_start_decision = String::new();
    let mut add_tasks = String::new();

    loop {
        println!("Add Tasks? (y/n)");

        stdin()
            .read_line(&mut add_tasks)
            .expect("Failed to read line - 24");

        if add_tasks == "y" {
            todo!(" Adding tasks ");
        } else {
            println!("Start work? (y/n)");
            stdin()
                .read_line(&mut timer_start_decision)
                .expect("Failed to read line - 32");

            if timer_start_decision == "y" {
                enter_loop(debug_mode);
                break;
            } else {
                continue;
            }
        }
    }
}

fn enter_loop(DBG_MODE: bool) {
    // After 20 iterations ( 10 work times ) of the loop, I will get a 30 min break.
    // This is 30 mins of break every 2.5 hours of work.
    let mut until_long_break: i8 = 20;

    // This is to alternate between work and stand time.
    let mut alternator = true;

    // number of seconds of work time, 5 seconds at debug, 15 mins at performance
    let mut seconds = if DBG_MODE {
        Duration::from_secs(5)
    } else {
        Duration::from_secs(900)
    };

    loop {
        thread::sleep(seconds.clone());

        // if its time to stand up, alternator should be true.
        // if it isn't alternator should be false. That means its time to get back to work.
        if alternator {
            println!("Time to Stand Up!");
        } else {
            // If 'until_long_break is greater than 0, I still have to work 15 min periods.
            // otherwise, A different string is printed, saying that its time for me to take
            // a long break.
            if until_long_break > 0 {
                println!("Time to Get Back to Work!");
                until_long_break -= 1;
            } else {
                println!("Time to take a long break!");
            }
        }

        // plays audio every iteration. since the thread sleeps at the beginning of
        // the loop for 'seconds' seconds, it only plays every time its either time to
        // stand up or go back to work.
        play_audio();

        // seconds alternates between dbg -> 2 and 5 seconds, perf -> 15 seconds and 15 mins.
        seconds = if alternator {
            if DBG_MODE {
                Duration::from_secs(2)
            } else {
                Duration::from_secs(15)
            }
        } else {
            if DBG_MODE {
                Duration::from_secs(5)
            } else {
                Duration::from_secs(900)
            }
        };

        // if until long break is less than or equal to 0, it overrides the seconds to make it
        // 10 secs at debug and 30 mins at performance.
        if until_long_break <= 0 {
            until_long_break = 20;
            seconds = if DBG_MODE {
                Duration::from_secs(10)
            } else {
                Duration::from_secs(1800)
            };
        }

        alternator = !alternator;
    }
}

// function that plays the audio
fn play_audio() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // Since I'm running this with PATH=$PATH to run it from anywhere, I can't have this be relative
    // Otherwise it can't find the audio file.
    let file = std::fs::File::open(
        "/Users/justinlee/Desktop/rust-tutorial-projects/standup_timer/assets/notifSound.mp3",
    )
    .unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    // Thread sleeps until the audio finishes
    sink.sleep_until_end();
}
*/
