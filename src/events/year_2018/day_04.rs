use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use tracing::debug;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut events: Vec<Event> = input.into_iter().map(|s| s.parse().unwrap()).collect();
        events.sort_by(|a, b| a.time.cmp(&b.time));

        let mut guards = HashMap::new();
        let mut current_guard = &mut Guard::new(99999999999);
        let mut time = Time::default();

        for event in events {
            if current_guard.is_asleep {
                debug!(?current_guard.id, "sleep -> {} -- {}", time.minute, event.time.minute-1);
                for i in time.minute..(event.time.minute) {
                    current_guard.minutes_sleeping[i] += 1;
                }
            }
            time = event.time;

            match event.event {
                GuardEvent::BeginsShift(id) => {
                    debug!("guard {}", id);
                    current_guard = guards.entry(id).or_insert(Guard::new(id))
                }
                GuardEvent::Sleeps => {
                    debug!(?current_guard.id, "sleep");
                    current_guard.is_asleep = true;
                }
                GuardEvent::Wakes => {
                    debug!(?current_guard.id, "wake");
                    current_guard.is_asleep = false
                }
            }
        }

        let most_sleeing_guard = guards.into_values().max().unwrap();
        Ok(most_sleeing_guard.value().into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut events: Vec<Event> = input.into_iter().map(|s| s.parse().unwrap()).collect();
        events.sort_by(|a, b| a.time.cmp(&b.time));

        let mut guards = HashMap::new();
        let mut current_guard = &mut Guard::new(99999999999);
        let mut time = Time::default();

        for event in events {
            if current_guard.is_asleep {
                debug!(?current_guard.id, "sleep -> {} -- {}", time.minute, event.time.minute-1);
                for i in time.minute..(event.time.minute) {
                    current_guard.minutes_sleeping[i] += 1;
                }
            }
            time = event.time;

            match event.event {
                GuardEvent::BeginsShift(id) => {
                    debug!("guard {}", id);
                    current_guard = guards.entry(id).or_insert(Guard::new(id))
                }
                GuardEvent::Sleeps => {
                    debug!(?current_guard.id, "sleep");
                    current_guard.is_asleep = true;
                }
                GuardEvent::Wakes => {
                    debug!(?current_guard.id, "wake");
                    current_guard.is_asleep = false
                }
            }
        }

        let most_sleeing_guard = guards
            .into_values()
            .max_by(|a, b| a.max_times_sleeing().cmp(&b.max_times_sleeing()))
            .unwrap();
        Ok(most_sleeing_guard.value2().into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Guard {
    id: usize,
    is_asleep: bool,
    minutes_sleeping: [usize; 60],
}

impl Guard {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_asleep: false,
            minutes_sleeping: [0; 60],
        }
    }

    fn value(&self) -> usize {
        let minute = self.minutes_sleeping.iter().position_max().unwrap();
        self.id * minute
    }

    fn max_times_sleeing(&self) -> usize {
        *self.minutes_sleeping.iter().max().unwrap()
    }

    fn value2(&self) -> usize {
        let minute = self.minutes_sleeping.iter().position_max().unwrap();
        self.id * minute
    }
}

impl Ord for Guard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sleep: usize = self.minutes_sleeping.iter().sum();
        let other_sleep: usize = other.minutes_sleeping.iter().sum();

        sleep.cmp(&other_sleep)
    }
}

#[derive(Debug, Clone, Copy)]
enum GuardEvent {
    BeginsShift(usize),
    Sleeps,
    Wakes,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    time: Time,
    event: GuardEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
struct Time {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let time = s[1..17].parse().unwrap();

        let event = match s.as_bytes()[19] {
            b'G' => GuardEvent::BeginsShift(s.number_in_prefixed("#")),
            b'f' => GuardEvent::Sleeps,
            b'w' => GuardEvent::Wakes,
            _ => unreachable!(),
        };

        Ok(Event { time, event })
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(' ').unwrap();

        let (year, month, day) = match p1.split('-').collect::<Vec<_>>()[..] {
            [year, month, day] => (
                year.parse().unwrap(),
                month.parse().unwrap(),
                day.parse().unwrap(),
            ),
            ref _other => panic!("split wrong!"),
        };

        let (hour, minute) = match p2.split_once(':') {
            Some((h, m)) => (h.parse().unwrap(), m.parse().unwrap()),
            None => todo!(),
        };

        Ok(Time {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

/*
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
*/
