// standard
use std::thread;
use std::time::Duration;
use std::thread::sleep;


// internal
use crate::config::{Config};
use crate::definitions::{WorkerChannelMessage, PeriodicTaskType};

// external
use crossbeam_channel::{Receiver};
use sysfs_gpio::{Direction, Pin};


const WATER_PUMP_PIN: u64 = 4;

fn handle_commands(cmd: WorkerChannelMessage, config: &Config) {
    match cmd {
        WorkerChannelMessage::PeriodicTask(task) => {
            match task.task_type {
                PeriodicTaskType::WaterPlants => {
                    let water_pump = Pin::new(WATER_PUMP_PIN);
                    let result = water_pump.with_exported(|| {
                        water_pump.set_direction(Direction::Out).unwrap();
                        water_pump.set_value(1).unwrap();
                        sleep(Duration::from_secs(config.water_pump_working_cycle_seconds));
                        water_pump.set_value(0).unwrap();
                        Ok(())
                    });

                    match result {
                        Err(_) => {
                            println!("Tried to water plants but failed to connect or turn on the water pump");
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
}

pub struct Worker {} impl Worker {
    pub fn run_threaded(
        config: &'static Config,
        worker_channel_receiver: Receiver<WorkerChannelMessage>
    ) {
        thread::spawn(move || {
            loop {
                if let Ok(cmd) = worker_channel_receiver.try_recv() {
                    handle_commands(cmd, &config);
                }
                std::thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}