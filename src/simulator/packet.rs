#[derive(Debug)]
pub struct Packet {
    /// Packet ID
    id: u64,
    /// Time t, the packet showed up.
    t_arrival: u64,
    /// Time t, the packet left the hardware
    t_departure: u64,
    /// Resources requested
    resource_req: Vec<f64>,
    /// Resources actually allocated, empty if none.
    resource_alloc: Vec<f64>,
    /// Number of ticks it needs to complete, given requested resources.
    expected_service_time: f64,
    /// Actual service time it has gotten so far.
    service_time: f64,
}

impl Packet {
    pub fn is_completed(&mut self) -> bool {
        return self.service_time > self.expected_service_time;
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
