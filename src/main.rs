// modules
mod scheduler;
mod config;
mod definitions;
mod telegram_bot;
mod worker;
mod reply_text;

#[cfg(test)]
mod tests;

// internal
use config::{Config};
use definitions::{SchedulerChannelMessage, WorkerChannelMessage};
use scheduler::{Scheduler};
use worker::{Worker};
use telegram_bot::{TelegramBot};

//external
use dotenv::dotenv;
use crossbeam_channel::{bounded, Sender, Receiver};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = Config::new();
}

#[tokio::main]
async fn main() {
    // setup system signal handler
    ctrlc::set_handler(move || {
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    // setup env config
    dotenv().ok();

    // setup communication channels between all threads (SchedulerThread, WorkerThread, TelegramBotThread (main thread))
    let (scheduler_channel_sender, scheduler_channel_receiver): (Sender<SchedulerChannelMessage>, Receiver<SchedulerChannelMessage>) = bounded(CONFIG.messages_in_flight_limit);
    let (worker_channel_sender, worker_channel_receiver): (Sender<WorkerChannelMessage>, Receiver<WorkerChannelMessage>) = bounded(CONFIG.messages_in_flight_limit);

    // setup threads
    Scheduler::run_threaded(&CONFIG, scheduler_channel_receiver, &worker_channel_sender);
    Worker::run_threaded(&CONFIG, worker_channel_receiver);

    // setup telegram bot server and listen for incoming messages
    TelegramBot::run_async(&CONFIG, worker_channel_sender, scheduler_channel_sender).await;
}

