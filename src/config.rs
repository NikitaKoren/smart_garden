use crate::definitions::{PeriodicTask, PeriodicTaskType};

#[derive(Debug, Clone)]
pub struct Config {
    pub scheduler_tick_ms: u64,
    pub water_pump_working_cycle_seconds: u64,
    pub messages_in_flight_limit: usize,
    pub periodic_tasks: Vec<PeriodicTask>
}

impl Config {
    pub fn new () -> Self {
        Config {
            scheduler_tick_ms: 1000,
            water_pump_working_cycle_seconds: 60,
            messages_in_flight_limit: 8,
            periodic_tasks: vec![
                PeriodicTask { disabled: false, task_type: PeriodicTaskType::WaterPlants, cron: "1/10 * * * * *".to_string() },
                PeriodicTask { disabled: true, task_type: PeriodicTaskType::ReadMoistureSensorsData, cron: "* * * * * *".to_string() }
            ]
        }
    }
}