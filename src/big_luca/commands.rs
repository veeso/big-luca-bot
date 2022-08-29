//! # Commands
//!
//! Big luca bot commands

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "visualizza gli attuali corsi disponibili del Papi")]
    BigCorsi,
    #[command(description = "visualizza gli ultimi video del Papi")]
    BigNews,
    #[command(description = "il Papi ti lancia una perla")]
    BigPerla,
    #[command(description = "imposta il papi per inviarti i messaggi automatici")]
    BigKatanga,
    #[command(description = "disattiva l'invio dei messaggi automatici del papi")]
    BigPezzente,
    #[command(description = "visualizza la versione attuale del big-luca-bot")]
    BigRelease,
    #[command(description = "visualizza l'ultimo post su instagram del papi")]
    BigSocial,
    #[command(description = "vai al sito del Papi")]
    BigSito,
    #[command(description = "visualizza l'ultimo video del Papi")]
    BigVideo,
    #[command(description = "visualizza l'aiuto")]
    Help,
    #[command(description = "inizializza bot")]
    Start,
}
