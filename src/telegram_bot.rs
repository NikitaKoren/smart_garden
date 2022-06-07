// standard
use std::error::Error;
use std::process;

// internal
use crate::config::{Config};
use crate::definitions::{WorkerChannelMessage, SchedulerChannelMessage, PeriodicTaskType, PeriodicTask};
use crate::reply_text::{get_confirmation_phrase, get_fact};

// external
use teloxide::{prelude::*, utils::command::BotCommands};
use job_scheduler::{Schedule};
use crossbeam_channel::{Sender};


#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Help")]
    Help,
    #[command(description = "Water Plants")]
    WaterPlants(String),
    #[command(description = "List Periodic Tasks")]
    ListTasks,
    #[command(description = "Removes Periodic Task")]
    RemoveTask(String),
    #[command(description = "Update Periodic Task Schedule")]
    UpdateTask(String),
    #[command(description = "Reset all tasks to default config")]
    ResetTasks,
    #[command(description = "Shutdowns the system. Please note that you will need to restart it manually.")]
    Shutdown
}

async fn reply_helper(bot: AutoSend<Bot>, message: Message) {
    let phrase = get_confirmation_phrase();
    let fact = get_fact();
    match bot.send_message(message.chat.id, format!("Beep Boop Bop... {phrase} Also did you know that {fact}.")).await {
        Err(_) => { 
            println!("Failed to send a reply."); 
        }
        _ => {}
    }
}

async fn handle_commands(bot: AutoSend<Bot>, message: Message, command: Command, worker_channel_sender: Sender<WorkerChannelMessage>, scheduler_channel_sender: Sender<SchedulerChannelMessage>, config: &Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::WaterPlants(watering_cycle_seconds) => {
            let max_time_sec = 60 * 5;
            let min_time_sec = 10;
            let water_pump_working_cycle_seconds: u64 = watering_cycle_seconds.trim().to_string().parse().unwrap_or(config.water_pump_working_cycle_seconds);

            if water_pump_working_cycle_seconds < min_time_sec || water_pump_working_cycle_seconds > max_time_sec {
                bot.send_message(message.chat.id, format!("Wrong input... cycle to water plants is {water_pump_working_cycle_seconds} but should be a numder between {max_time_sec} and {min_time_sec} seconds")).await?;
                return Ok(());
            }

            worker_channel_sender.send(WorkerChannelMessage::WaterPlants(water_pump_working_cycle_seconds)).unwrap();
            reply_helper(bot, message).await;
        },
        Command::ListTasks => {
            scheduler_channel_sender.send(SchedulerChannelMessage::ListPeriodicTasks(message.chat.id)).unwrap();
        }
        Command::RemoveTask(task_type) => {

            let task = match task_type.as_str() {
                "WaterPlants" => Ok(PeriodicTaskType::WaterPlants),
                "ReadMoistureSensorsData" => Ok(PeriodicTaskType::ReadMoistureSensorsData),
                _ => Err(())
            };

            match task {
                Ok(t) => {
                    scheduler_channel_sender.send(SchedulerChannelMessage::RemovePeriodicTask(t)).unwrap();
                    reply_helper(bot, message).await
                },
                Err(_) => {
                    bot.send_message(message.chat.id, format!("Wrong input... {task_type} is not a valid task. Use /listperiodictasks command to see the valid list of tasks")).await?;
                }
            }
        },
        Command::UpdateTask(input) => {
            let inputs: Vec<_> = input.split(" | ").collect();
            if inputs.len() != 2 {
                bot.send_message(message.chat.id, format!("Wrong input... Couldn't parse task_type or/and cron expression")).await?;
                return Ok(())
            }
            let task_type = inputs[0];
            let schedule = inputs[1];

            let schedule_: Result<Schedule, _> = schedule.parse();
            let task_type_ = match task_type {
                "WaterPlants" => Ok(PeriodicTaskType::WaterPlants),
                "ReadMoistureSensorsData" => Ok(PeriodicTaskType::ReadMoistureSensorsData),
                _ => Err(())
            };
  

            if schedule_.is_err() || task_type_.is_err() {
                bot.send_message(message.chat.id, format!("Wrong input... {task_type} or {schedule} is not valid. Please double check the syntax and try again.")).await?;
            } else {
                let task = PeriodicTask {
                    task_type: task_type_.unwrap(),
                    cron: schedule.to_string(),
                    disabled: false
                };
                scheduler_channel_sender.send(SchedulerChannelMessage::UpdatePeriodicTaskScheduleCommand(task)).unwrap();
                reply_helper(bot, message).await
            }
        },
        Command::ResetTasks => {
            scheduler_channel_sender.send(SchedulerChannelMessage::ResetTasks).unwrap();
            reply_helper(bot, message).await
        },
        Command::Shutdown => {
            bot.send_message(message.chat.id, format!("Shutting down... I won't be able to process any commands until you restart me.")).await?;
            process::exit(1);
        }
    }
    Ok(())
}

pub struct TelegramBot {} impl TelegramBot {
     pub async fn run_async(
        config: &'static Config,
        worker_channel_sender: Sender<WorkerChannelMessage>,
        scheduler_channel_sender: Sender<SchedulerChannelMessage>
    ) {
        let teloxide_bot = Bot::from_env().auto_send();

        let handler = move |bot: AutoSend<Bot>, message: Message, command: Command| {  
            let worker_channel_sender_ = worker_channel_sender.clone();
            let scheduler_channel_sender_ = scheduler_channel_sender.clone();

            async move {
                match handle_commands(bot, message, command, worker_channel_sender_, scheduler_channel_sender_, config).await {
                    Err(error) => {
                        println!("Failed to handle a command {:?}", error);
                    },
                    _ => {},
                };
                respond(())
            }
        };

        teloxide::commands_repl(teloxide_bot, handler, Command::ty()).await;
    }
}

