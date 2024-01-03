use std::collections::{HashMap, VecDeque};

use anyhow::{Result, Ok};
use tracing::{info, debug};

use crate::utils::*;

use super::super::AocDay;


pub struct Day {

}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {

        let mut modules: HashMap<_, _> = input.iter().map(|line| Module::parse(line)).map(|module| (module.name.clone(), module)).collect();
        align_conjunctions(&mut modules);

        let mut low_counts = 0;
        let mut high_counts = 0;

        for _ in 0..1000 {
            let (high_count, low_count, _) = press_button(&mut modules, None);

            high_counts += high_count;
            low_counts += low_count;
        }


        info!("{} -- {}", high_counts, low_counts);

        let result = high_counts * low_counts;
        Ok(result.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let modules: HashMap<_, _> = input.iter().map(|line| Module::parse(line)).map(|module| (module.name.clone(), module)).collect();
        
        // This is actually just 4 binary counters, where each counter has a different cycle time.
        // Ergo we can use LCM to get when they all cycle at the same time.
        
        let mut graph = VecDeque::new();
        let mut counter_cycles = vec![];

        for node in &modules["broadcaster"].destinations {
            graph.push_back((node, 0, 1));
        }

        while let Some((name, mut value, bit)) = graph.pop_front() {
            let dependends = &modules[name].destinations;

            // Find next flipflop child. Conjectures are just used as a reset button here. ergo 0000 -> 0001 -> 0010 -> 0100 -> 1000 etc...
            if let Some(next) = dependends.iter().find(|&item| modules.get(item).map_or(false, |x| !x.is_conjecture())) {

                if dependends.len() == 2 {
                    value |= bit; // if len is 2, then we always have a conjecture (addr), and a flipflop.
                }
                // move to next input in chain
                graph.push_back((next, value, bit << 1));
            }
            else {
                // we reached end of cycle
                counter_cycles.push(value | bit);
            }
        }

        let result: usize = math_utils::lcm_multiple(counter_cycles.as_slice());
        Ok(result.into())
    }
}

fn press_button(modules: &mut HashMap<String, Module>, change_watch: Option<&str>) -> (usize, usize, bool) {
    let mut queue = VecDeque::new();

    let mut low_counts = 0;
    let mut high_counts = 0;
    let mut has_change = false;

    queue.push_back(Pulse::Low(String::from("button"), String::from("broadcaster")));

    while let Some(pulse) = queue.pop_front() {
        debug!("{:?}", &pulse);

        let receiver = match &pulse {
            Pulse::High(_, receviver) => {
                high_counts += 1;
                receviver
            },
            Pulse::Low(_, recevier) => {
                low_counts += 1;
                recevier
            },
        };

        has_change |= match change_watch {
            Some(s) if s == receiver.as_str() => true,
            _ => false,
        };

        if let Some(module) = modules.get_mut(receiver) {
            module.handle(pulse, &mut queue);
        }
    }

    (high_counts, low_counts, has_change)
}

fn align_conjunctions(modules: &mut HashMap<String, Module>) {
    let all_conjunctions_keys: Vec<_> = modules.iter()
        .filter_map(|(k, v)| match v.module_type {
            ModuleType::Conjunction(_) => Some(k.clone()),
            _ => None
        })
        .collect();
    
    for key in all_conjunctions_keys.iter() {
        let sources: Vec<_> = modules
            .values()
            .filter_map(|module| match module.destinations.contains(&key) {
                true => Some(module.name.clone()),
                _ => None
            } )
            .collect();

        let conj = modules.get_mut(key).unwrap();
        conj.module_type = ModuleType::Conjunction(sources.into_iter().map(|src| (src, Pulse::Low("".into(), "".into()))).collect());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High(String, String),
    Low(String, String),
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast, 
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>)
}
/*
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
 */

 #[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn parse(line: &str) -> Self {
        let (p1, deps) = line.split_once("->").unwrap();

        let module_type = match p1.as_bytes()[0] {
            b'&' => ModuleType::Conjunction(Default::default()),
            b'%' => ModuleType::FlipFlop(false),
            b'b' => ModuleType::Broadcast,
            _ => unreachable!()
        };

        let name = p1.trim_matches(&['&', '%', ' ']).into();

        let destinations: Vec<String> = deps.trim().split(',').map(|part| part.trim().into()).collect();

        Self { name, module_type, destinations }
    }

    fn is_conjecture(&self) -> bool {
        matches!(self.module_type, ModuleType::Conjunction(_))
    }

    fn handle(&mut self, pulse: Pulse, queue: &mut VecDeque<Pulse>) {
        match &mut self.module_type {
            ModuleType::Broadcast => {
                for dep in self.destinations.iter() {
                    match pulse {
                        Pulse::High(_, _) => queue.push_back(Pulse::High(self.name.clone(), dep.clone())),
                        Pulse::Low(_, _) => queue.push_back(Pulse::Low(self.name.clone(), dep.clone())),
                    };
                }
            },
            ModuleType::FlipFlop(ref mut state) => {
                if matches!(&pulse, Pulse::High(_, _)) {
                    return;
                }

                for dep in self.destinations.iter() {
                    let pulse = match state {
                        true => Pulse::Low(self.name.clone(), dep.clone()),
                        false => Pulse::High(self.name.clone(), dep.clone()),
                    };
                    queue.push_back(pulse);
                }

                *state = !*state;
            }
            ModuleType::Conjunction(ref mut state) => {

                let sender = match &pulse {
                    Pulse::High(sender, _) => sender,
                    Pulse::Low(sender, _) => sender,
                };

                state.insert(sender.into(), pulse.clone());
                if state.values().all(|input| matches!(input, Pulse::High(_, _))) {
                    for dep in self.destinations.iter() {
                        queue.push_back(Pulse::Low(self.name.clone(), dep.clone()));
                    }                
                }
                else {
                    for dep in self.destinations.iter() {
                        queue.push_back(Pulse::High(self.name.clone(), dep.clone()));
                    }       
                }
            }
        }
    }
}