use std::fmt;
use std::str::FromStr;

use teloxide_core::types::ChatId;

#[derive(Debug, Clone)]
pub enum PeriodicTaskType {
    WaterPlants,
    ReadMoistureSensorsData
}

#[derive(Debug, Clone)]
pub enum SchedulerChannelMessage {
    UpdatePeriodicTaskScheduleCommand(PeriodicTask),
    ListPeriodicTasks(ChatId),
    RemovePeriodicTask(PeriodicTaskType),
    ResetTasks
}

#[derive(Debug, Clone)]
pub enum WorkerChannelMessage {
    PeriodicTask(PeriodicTask),
    WaterPlants(u64),
}

impl fmt::Display for PeriodicTaskType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeriodicTaskType::WaterPlants => write!(f, "WaterPlants"),
            PeriodicTaskType::ReadMoistureSensorsData => write!(f, "ReadMoistureSensorsData"),
        }
    }
}

impl FromStr for PeriodicTaskType {
    type Err = ();

    fn from_str(input: &str) -> Result<PeriodicTaskType, Self::Err> {
        match input {
            "WaterPlants"  => Ok(PeriodicTaskType::WaterPlants),
            "ReadMoistureSensorsData"  => Ok(PeriodicTaskType::ReadMoistureSensorsData),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeriodicTask {
    pub task_type: PeriodicTaskType,
    pub cron: String,
    pub disabled: bool
}

impl fmt::Display for PeriodicTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task: '{}' \nSchedule: '{}' \nDisabled: {}", self.task_type, self.cron, self.disabled)
    }
}
