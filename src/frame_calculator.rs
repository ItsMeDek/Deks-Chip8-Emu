pub struct FrameCalculator {
    total_fps: u64,
    fps_count_start: std::time::Instant,
    current_fps_result: u64,
}

impl FrameCalculator {
    pub fn new() -> Self {
        Self {
            total_fps: 0,
            fps_count_start: std::time::Instant::now(),
            current_fps_result: 0,
        }
    }

    pub fn tick(&mut self) {
        self.total_fps += 1;
        if self.fps_count_start.elapsed().as_secs_f64() >= 1.0 {
            self.current_fps_result = self.total_fps;
            self.total_fps = 0;
            self.fps_count_start = std::time::Instant::now();
        }
    }

    pub fn fps(&self) -> u64 {
        self.current_fps_result
    }
}