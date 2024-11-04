//! # Commands
//!
//! Big luca bot commands

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(description = "These commands are supported:")]
pub enum Command {
    #[command(
        rename = "bigcorsi",
        description = "visualizza gli attuali corsi disponibili del Papi"
    )]
    BigCorsi,
    #[command(
        rename = "bignews",
        description = "visualizza gli ultimi video del Papi"
    )]
    BigNews,
    #[command(rename = "bigperla", description = "il Papi ti lancia una perla")]
    BigPerla,
    #[command(
        rename = "bigkatanga",
        description = "imposta il papi per inviarti i messaggi automatici"
    )]
    BigKatanga,
    #[command(
        rename = "bigpezzente",
        description = "disattiva l'invio dei messaggi automatici del papi"
    )]
    BigPezzente,
    #[command(
        rename = "bigrelease",
        description = "visualizza la versione attuale del big-luca-bot"
    )]
    BigRelease,
    #[command(
        rename = "bigsocial",
        description = "visualizza l'ultimo post su instagram del papi"
    )]
    BigSocial,
    #[command(rename = "bigsito", description = "vai al sito del Papi")]
    BigSito,
    #[command(
        rename = "bigvideo",
        description = "visualizza l'ultimo video del Papi"
    )]
    BigVideo,
    #[command(rename = "help", description = "visualizza l'aiuto")]
    Help,
    #[command(rename = "start", description = "inizializza bot")]
    Start,
}
