//! # Big Luca
//!
//! This module implements the big luca bot

mod answer;
mod aphorism;
mod automatize;
mod commands;
mod config;
mod instagram;
mod parameters;
mod redis;
mod repository;
mod stickers;
mod youtube;

pub use self::redis::RedisRepository;
use answer::{Answer, AnswerBuilder};
use aphorism::AphorismJar;
use automatize::Automatizer;
use commands::Command;
pub use config::Config;
pub use parameters::Parameters;
use stickers::Stickers;

use once_cell::sync::OnceCell;
use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};
use url::Url;

pub static AUTOMATIZER: OnceCell<Automatizer> = OnceCell::new();
pub static PARAMETERS: OnceCell<Parameters> = OnceCell::new();

/// Big luca bot application
pub struct BigLuca {
    bot: AutoSend<Bot>,
}

impl BigLuca {
    /// Initialize big luca
    pub async fn init() -> anyhow::Result<Self> {
        // parse configuration
        info!("reading configuration");
        let config = Config::try_from_env()?;
        info!("reading parameters");
        let parameters = Parameters::try_from_path(&config.parameters_path)?;
        if PARAMETERS.set(parameters).is_err() {
            anyhow::bail!("failed to set parameters");
        }
        // sanitize aphorism jar
        info!("sanitizing aphorism jar...");
        AphorismJar::sanitize_aphorisms(PARAMETERS.get().unwrap().aphorisms.as_slice()).await?;
        info!("aphorism jar sanitized");
        info!("starting automatizer");
        let automatizer = Automatizer::start()
            .await
            .map_err(|e| anyhow::anyhow!("failed to start automatizer: {}", e))?;
        if AUTOMATIZER.set(automatizer).is_err() {
            anyhow::bail!("failed to set automatizer");
        };
        info!("automatizer started; starting bot");
        let bot = Bot::from_env().auto_send();
        Ok(Self { bot })
    }

    /// Run big luca bot
    pub async fn run(self) -> anyhow::Result<()> {
        // setup hooks
        let port = Self::get_heroku_port()?;
        if let Some(port) = port {
            Self::run_on_heroku(self, port).await
        } else {
            Self::run_simple(self).await
        }
    }

    /// run bot with heroku webhooks
    async fn run_on_heroku(self, port: u16) -> anyhow::Result<()> {
        info!("running bot with heroku listener (PORT: {})", port);
        let addr = ([0, 0, 0, 0], port).into();
        let token = self.bot.inner().token();
        let host = std::env::var("HOST").map_err(|_| anyhow::anyhow!("HOST is not SET"))?;
        let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
        debug!("configuring listener {}...", url);
        let listener = webhooks::axum(self.bot.clone(), webhooks::Options::new(addr, url))
            .await
            .map_err(|e| anyhow::anyhow!("could not configure listener: {}", e))?;
        // start bot
        teloxide::commands_repl_with_listener(self.bot, Self::answer, listener, Command::ty())
            .await;
        Ok(())
    }

    /// run bot without webhooks
    async fn run_simple(self) -> anyhow::Result<()> {
        info!("running bot without webhooks");
        teloxide::commands_repl(self.bot, Self::answer, Command::ty()).await;
        Ok(())
    }

    /// Answer handler for bot
    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("got command {:?}", command);
        let answer = match command {
            Command::Help => Answer::simple_text(Command::descriptions()),
            Command::Start => Self::start(),
            Command::BigCorsi => Self::active_courses(),
            Command::BigKatanga => Self::subscribe_to_automatizer(&message.chat.id).await,
            Command::BigNews => Self::get_latest_videos().await,
            Command::BigPerla => Self::aphorism(&message.chat.id).await,
            Command::BigPezzente => Self::unsubscribe_from_automatizer(&message.chat.id).await,
            Command::BigRelease => Self::get_release(),
            Command::BigSocial => Self::get_latest_instagram_post().await,
            Command::BigSito => Self::big_luca_website(),
            Command::BigVideo => Self::get_latest_video().await,
        };
        answer.send(&bot, message.chat.id).await
    }

    fn start() -> Answer {
        AnswerBuilder::default()
            .text("Chi ?? Big Luca? Genio indiscusso dell'Online Marketing. Dopo aver macinato milioni nel mercato americano (il pi?? agguerrito e competitivo al mondo) con guadagni superiori ai 100.000 dollari NETTI al mese a soli 25 anni, decise di approdare nel mercato europeo affermandosi subito come massima autorit?? dell'Online Marketing.Tanto che nel 2018 fu selezionato da Dan Kennedy in persona per prendere parte al suo esclusivo e leggendario Titanium Mastermind composto solo dai 16 migliori Marketer al mondo (con guadagni dimostrati superiori al milione di euro).")
            .text(r#"Definito come il ???Creatore di Milionari??? Negli anni, l'impatto che Big Luca ha avuto su diverse persone ?? stato straordinario e fuori dal comune.
 Dozzine di imprenditori partiti da zero, appartenenti alle nicchie pi?? disparate, hanno creato realt?? multi-milionarie grazie ai suoi corsi e alle sue consulenze private.           
 Ed in tanti casi solo dopo una singola consulenza sono riusciti a decuplicare i propri profitti.
D'altronde, nonostante i costi delle sue consulenze diventino sempre pi?? proibitivi, ogni mese imprenditori di ogni nicchia ne fanno richiesta.
Le persone fanno letteralmente la fila e viaggiano anche per 10.000 km per ricevere le sue consulenze private.
La lista di attesa pu?? durare mesi e solo in pochi dopo una rigida selezione riescono ad accedere direttamente al suo supporto one-to-one."#)
            .text("Allora? Sei pronto a diventare un allievo del Papi? Per cominciare entra in katanga con /bigkatanga per avere tutti gli aggiornamenti del Big o vedi tutti i comandi con /help")
            .sticker(Stickers::i_want_you())
            .finalize()
    }

    fn get_release() -> Answer {
        Answer::simple_text(format!(
            "big-luca-bot {}. Sviluppato da @veeso97. Contribuisci al progetto su Github https://github.com/veeso/big-luca-bot. Sostieni il mio progetto su Ko-Fi https://ko-fi.com/veeso",
            env!("CARGO_PKG_VERSION")
        ))
    }

    /// Send a random aphorism
    async fn aphorism(chat: &ChatId) -> Answer {
        let mut aphorism_jar =
            match AphorismJar::try_from(PARAMETERS.get().unwrap().aphorisms.as_slice()) {
                Ok(jar) => jar,
                Err(e) => return Self::error(e),
            };
        match aphorism_jar.get_next(chat).await {
            Ok(aphorism) => AnswerBuilder::default()
                .text(aphorism)
                .sticker(Stickers::random())
                .finalize(),
            Err(e) => Self::error(e),
        }
    }

    /// Get latest videos from papi
    async fn get_latest_videos() -> Answer {
        match youtube::Youtube::get_latest_videos().await {
            Ok(feed) => {
                let mut message = String::new();
                for video in feed.entries() {
                    message.push_str(
                        format!(
                            "??? {} ???? {}\n",
                            video.title.as_deref().unwrap_or_default(),
                            video.url
                        )
                        .as_str(),
                    );
                }
                Answer::simple_text(message)
            }
            Err(err) => Self::error(err),
        }
    }

    /// Get latest video from papi
    async fn get_latest_video() -> Answer {
        match youtube::Youtube::get_latest_video().await {
            Ok(video) => Answer::simple_text(format!(
                "???? {} ???? {}",
                video.title.unwrap_or_default(),
                video.url
            )),
            Err(err) => Self::error(err),
        }
    }

    /// Get latest instagram post
    async fn get_latest_instagram_post() -> Answer {
        match instagram::InstagramService::get_latest_post().await {
            Ok(post) => AnswerBuilder::default()
                .text(format!("???? {}", post.caption.unwrap_or_default()))
                .image(post.display_url)
                .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// Get big luca website
    fn big_luca_website() -> Answer {
        Answer::simple_text(
            "Vai sul sito della Big Luca International ???? https://www.biglucainternational.com/it",
        )
    }

    /// Get latest active courses
    fn active_courses() -> Answer {
        let mut builder = AnswerBuilder::default();
        for course in PARAMETERS.get().unwrap().courses.iter() {
            builder = builder.text(course);
        }
        builder.sticker(Stickers::lucro_time()).finalize()
    }

    /// Subscribe chat to the automatizer
    async fn subscribe_to_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().subscribe(chat_id).await {
            Ok(_) => AnswerBuilder::default()
            .text("sei ora iscritto alla piattaforma Katanga! ???????????? Da ora riceverai tutte le perle del papi e i suoi ultimi aggiornamenti automaticamente su questa chat! ????")
            .sticker(Stickers::like())
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    async fn unsubscribe_from_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().unsubscribe(chat_id).await {
            Ok(()) => AnswerBuilder::default()
                .text("ti sei disinscritto dalla piattaforma Katanga ????, ora torna pure dai tuoi amici sfigati a vendere ai poveri. ????")
                .sticker(Stickers::grrr())
                .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// The answer to return in case of an error
    fn error(err: impl ToString) -> Answer {
        AnswerBuilder::default()
            .text(err)
            .sticker(Stickers::despair())
            .finalize()
    }

    // get heroku port
    fn get_heroku_port() -> anyhow::Result<Option<u16>> {
        match std::env::var("PORT").map(|x| x.parse()) {
            Err(_) => Ok(None),
            Ok(Ok(p)) => Ok(Some(p)),
            Ok(Err(e)) => anyhow::bail!("could not parse PORT environment variable: {}", e),
        }
    }
}
