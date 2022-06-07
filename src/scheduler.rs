// standard
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

// internal
use crate::config::{Config};
use crate::definitions::{SchedulerChannelMessage, WorkerChannelMessage, PeriodicTask, PeriodicTaskType};

// external
extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job, Uuid};
use crossbeam_channel::{Sender, Receiver};
use teloxide::{prelude::*};
use futures::future::join_all;
use tokio::runtime::Runtime;


#[derive(Debug, Clone)]
pub struct TaskMetaData {
    pub job_id: Uuid,
    pub task: PeriodicTask
}


pub struct Scheduler<'a> {
    config: &'a Config,
    scheduler_tick_ms: u64,
    cron: JobScheduler<'a>,
    scheduler_channel_receiver: &'a Receiver<SchedulerChannelMessage>,
    worker_channel_pub: &'a Sender<WorkerChannelMessage>,
    tasks: HashMap<String, TaskMetaData>,
    bot_notifier: AutoSend<Bot>
}


impl<'a> Scheduler<'a> {

    pub fn new(
        config: &'a Config,
        scheduler_channel_receiver: &'a Receiver<SchedulerChannelMessage>,
        worker_channel_pub: &'a Sender<WorkerChannelMessage>,
    ) -> Self {
        Scheduler {
            config: config,
            scheduler_tick_ms: config.scheduler_tick_ms,
            cron: JobScheduler::new(),
            scheduler_channel_receiver: scheduler_channel_receiver,
            worker_channel_pub: worker_channel_pub,
            tasks: HashMap::new(),
            bot_notifier: Bot::from_env().auto_send()
        }
     }

    pub fn run_threaded(
        config: &'a Config,
        scheduler_channel_receiver: Receiver<SchedulerChannelMessage>,
        worker_channel_pub: &'a Sender<WorkerChannelMessage>
    ) {

        let worker_channel_pub = worker_channel_pub.clone();

        let config = config.clone();

        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let mut scheduler = Scheduler::new(&config, &scheduler_channel_receiver, &worker_channel_pub);
            scheduler.add_tasks(&config.periodic_tasks);

            runtime.block_on(async move {
                loop {
                    println!("Scheduler tick");
                    scheduler.tick().await;
                }
            });

        });
    }

    /// The `tick` method increments time for the JobScheduler, executes
    /// any pending jobs and listens for the data on the feedback channel.
    /// Feedback channel can be used to adjust scheduler settings.
    pub async fn tick(&mut self) {
        // tick
        self.cron.tick();
        std::thread::sleep(Duration::from_millis(self.scheduler_tick_ms));

        // process feedback channel and adjust settings if needed
        self.process_feedback_channel().await;
    }

    pub fn add_task(&mut self, task: PeriodicTask) {

        let worker_channel_pub = self.worker_channel_pub.clone();
        let task_type = task.task_type.clone();
        let meta = task.clone();
        
        if let Ok(expression) = task.cron.parse() {
            let job_id = self.cron.add(Job::new(expression, move || {
                if !task.disabled {
                    worker_channel_pub.send(WorkerChannelMessage::PeriodicTask(task.clone())).unwrap_or_default();
                }
            }));

            self.tasks.insert(task_type.to_string(), TaskMetaData { job_id: job_id, task: meta });
        }
    }

    pub fn add_tasks(&mut self, tasks: &Vec<PeriodicTask>) {
        for task in tasks {
            self.add_task(task.clone());
        }
    }

    pub fn update_task(&mut self, task: PeriodicTask) {
        let task_id = task.task_type.to_string();

        match self.tasks.get(&task_id) {
            Some(task_metadata) => {
                self.cron.remove(task_metadata.job_id);
                self.add_task(task);
            },
            None => {
                println!("Could not find any tasks with the provided id {}.", task_id);
            }
        }
    }

    pub fn remove_task(&mut self, task_type: PeriodicTaskType) {
        let task_id = task_type.to_string();

        match self.tasks.get(&task_id) {
            Some(task_metadata) => {
                self.cron.remove(task_metadata.job_id);
                self.tasks.remove(&task_id);
            },
            None => {
                println!("Could not find any tasks with the provided id {}.", task_id);
            }
        }
    }

    async fn process_feedback_channel(&mut self) {
        if let Ok(message) = self.scheduler_channel_receiver.try_recv() {
            println!("Got feedback processing...");

            match message {
                SchedulerChannelMessage::UpdatePeriodicTaskScheduleCommand(task) => {
                    self.update_task(task);
                }
                SchedulerChannelMessage::ListPeriodicTasks(chat_id) => {
                    let tasks: Vec<_> = self.tasks.clone()
                        .into_values()
                        .map(|t| self.bot_notifier.send_message(chat_id, format!("{} \n{}", t.job_id, t.task)))
                        .collect();
                    join_all(tasks).await;
                },
                SchedulerChannelMessage::RemovePeriodicTask(task_type) => {
                    self.remove_task(task_type);
                }
                SchedulerChannelMessage::ResetTasks => {
                    self.tasks.clone().into_values().for_each(|t| self.remove_task(t.task.task_type.clone()));
                    self.add_tasks(&self.config.periodic_tasks);
                }
            }
        }
    }
}