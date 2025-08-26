//! Implements CPU Scheduling algorithms

struct Job {
    id: u32
    name: Option<String>,
    priority: Option<u32>,
    arrival: u32,
    burst: Option<u32>
}

/// Maybe we'll use it later for making a Gantt Chart
/*
struct Execution {
    id: u32,
    name: Option<String>,
    start: u32,
    end: u32
}
*/


enum Policy {
    FCFS,
    SJF,
    SRTF,
    PS, // Priority Scheduling
    PPS, // Preemptive Priority Scheduling
    RR(u32) // Round Robin
}


fn scheduler(jobs: &mut Vec<Job>, policy: Policy) {
    match policy {
        Policy::FCFS => fcfs(jobs),
        Policy::SJF => sjf(jobs),
        Policy::SRTF => srtf(jobs),
        Policy::PS => ps(jobs),
        Policy::PPS => pps(jobs),
        Policy::RR(q) => rr(jobs, q)
    }
}

fn fcfs(jobs: &mut Vec<Job>) {
    jobs.sort_by_key(|j| j.arrival);
}

// TODO
fn sjf(jobs: &mut Vec<Job>) {}
fn srtf(jobs: &mut Vec<Job>) {}
fn ps(jobs: &mut Vec<Job>) {}
fn pps(jobs: &mut Vec<Job>) {}
fn rr(jobs: &mut Vec<Job>, quantum: u32) {}
