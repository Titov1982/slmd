pub struct SerializeObj {
    pub tick: usize,
    pub cpu_usage: f32,
    pub mem_used: u64,
}

impl SerializeObj {
    pub fn new(tick: usize, cpu_usage: f32, mem_used: u64,) -> Self {
        Self {
            tick,
            cpu_usage,
            mem_used,
        }
    }
}