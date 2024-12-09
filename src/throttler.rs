pub struct Throttler {
    timer: f32,    // Accumulated time in seconds
    interval: f32, // Interval threshold in seconds
}

impl Throttler {
    // Create a new Throttler with a specified interval
    pub fn new(interval: f32) -> Self {
        Throttler {
            timer: 0.0,
            interval,
        }
    }

    // Update the timer
    pub fn update(&mut self, delta: f32) {
        self.timer += delta;
    }

    // Check if the action should be performed
    pub fn run_action<F>(&mut self, mut action: F)
    where
        F: FnMut(),
    {
        if self.timer >= self.interval {
            action();
            self.timer = 0.0;
        }
    }
}
