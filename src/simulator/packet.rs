use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Debug, Default)]
pub struct Packet {
    /// Packet ID
    id: u64,
    /// Time t, the packet showed up.
    t_arrival: u64,
    /// Time t, the packet left the hardware
    t_departure: u64,
    /// Resources requested
    pub resource_req: Vec<f64>,
    /// Resources actually allocated, empty if none.
    resource_alloc: Vec<f64>,
    /// Number of ticks it needs to complete, given requested resources.
    service_time: f64,
    /// Actual service time it has gotten so far.
    adjusted_service_time: f64,
}

impl Packet {
    pub fn new(id: u64, t: u64, service_time: f64, resource_req: Vec<f64>) -> Packet {
        Packet {
            id: id,
            t_arrival: t,
            resource_req: resource_req,
            service_time: service_time,
            ..Default::default()
        }
    }

    pub fn step(&mut self, t: u64) -> bool {
        if !self.is_scheduled() {
            return false;
        }

        let ratio = self.resource_alloc[0] / self.resource_req[0];
        self.adjusted_service_time += ratio;

        let done = self.is_completed();
        if done {
            self.t_departure = t
        }
        return done;
    }

    pub fn is_completed(&mut self) -> bool {
        return self.adjusted_service_time > self.service_time;
    }
    pub fn is_scheduled(&mut self) -> bool {
        return self.resource_alloc.len() > 0;
    }

    /// Returns the number of ticks it actually took to service this packet.
    /// Make sure you check whether this packet is completed, using
    /// is_completed().
    pub fn latency(&mut self) -> u64 {
        if self.t_arrival < 0 {
            return 0;
        }
        return self.t_departure - self.t_arrival;
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Packet {}

impl Hash for Packet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
