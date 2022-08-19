//! # Commands
//!
//! Big luca bot commands

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "visualizza gli attuali corsi disponibili del Papi")]
    BigCorsi,
    #[command(description = "visualizza gli ultimi video del Papi")]
    BigNews,
    #[command(description = "il Papi ti lancia una perla")]
    BigPerla,
    #[command(description = "vai al sito del Papi")]
    BigSito,
    #[command(description = "visualizza l'ultimo video del Papi")]
    BigVideo,
    #[command(description = "visualizza l'aiuto")]
    Help,
}
