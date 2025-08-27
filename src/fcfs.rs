//! Implements CPU Scheduling algorithms


use rand::Rng;
use std::collections::{HashMap, VecDeque};


/// Process received by the Scheduler
#[derive(Debug, Clone)]
pub struct Process {
    pub id: u32,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub priority: u32,
}

impl Process {
    pub fn new(arrival_time: u32, burst_time: u32, priority: Option<u32>) -> Self {
        let mut rng = rand::rng();

        let id: u32 = rng.random();
        
        Process {
            id,
            arrival_time,
            burst_time,
            priority: priority.unwrap_or(0)
        }
    }
}


/// Process which has been executed
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub start_time: u32,
    pub duration: u32,
}

impl Job {
    fn new(process: Process, start_time: u32) -> Self {
        Job {
            id: process.id,
            start_time,
            duration: process.burst_time
        }
    }
}


/// All the algorithms implemented or to be implemented
#[derive(Debug, Clone, Copy)]
pub enum SchedulingAlgorithm {
    FCFS,           // First Come First Serve

    // TODO: Add other algorithms later
    // SJF,            // Shortest Job First
    // SRTF,           // Shortest Remaining Time First (Preemptive SJF)
    // Priority,       // Priority Scheduling (Non-preemptive)
    // PriorityPreemptive, // Priority Scheduling (Preemptive)
    // RoundRobin(u32), // Round Robin with time quantum
}


/// Stats for a Process
#[derive(Debug)]
pub struct ProcessStats {
    pub process_id: u32,
    pub arrival_time: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
//  pub response_time: u32,    // first_start_time - arrival_time
}

impl ProcessStats {
    pub fn new(process: &Process, completion_time: u32) -> Self {
        let turnaround_time = completion_time - process.arrival_time;
        let waiting_time = turnaround_time - process.burst_time;

        ProcessStats {
            process_id: process.id,
            arrival_time: process.arrival_time,
            completion_time,
            turnaround_time,
            waiting_time,
//          response_time,
        }
    }
}


#[derive(Debug)]
pub struct SchedulingStats {
    pub stats: HashMap<u32, ProcessStats>, // process_id -> ProcessStats
}

impl SchedulingStats {
    pub fn new() -> Self {
        SchedulingStats { stats: HashMap::new() }
    }

    pub fn add_process_stats(&mut self, stats: ProcessStats) {
        self.stats.insert(stats.process_id, stats);
    }

    pub fn get_process_stats(&self, process_id: u32) -> Option<&ProcessStats> {
        self.stats.get(&process_id)
    }

    pub fn average_waiting_time(&self) -> f64 {
        let sum: u32 = self.stats.values().map(|s| s.waiting_time).sum();
        sum as f64 / self.stats.len() as f64
    }

    pub fn average_turnaround_time(&self) -> f64 {
        let sum: u32 = self.stats.values().map(|s| s.turnaround_time).sum();
        sum as f64 / self.stats.len() as f64
    }
}


// ===============================================================

// FCFS Scheduler
pub struct FCFSScheduler {
    pub ready_queue: VecDeque<Process>,
    pub jobs: Vec<Job>,
    pub stats: SchedulingStats,
}

impl FCFSScheduler {
    pub fn new() -> Self {
        FCFSScheduler {
            ready_queue: VecDeque::<Process>::new(),
            jobs: Vec::<Job>::new(),
            stats: SchedulingStats::new()
        }
    }

    pub fn schedule(&mut self) {
        let mut current_time: u32 = 0;

        while let Some(p) = self.ready_queue.pop_front() {
            if p.arrival_time > current_time {
                current_time = p.arrival_time;
            }

            let start_time: u32 = current_time;
            let duration: u32 = current_time + p.burst_time;
            let completion_time: u32 = start_time + duration;

            self.jobs.push(Job { id: p.id, start_time, duration });

            let stats: ProcessStats = ProcessStats::new(&p, completion_time);
            self.stats.add_process_stats(stats);

            current_time = completion_time;

        }
    }
}
