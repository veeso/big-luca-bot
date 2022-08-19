//! # Stickers
//!
//! A static getters for big luca stickers

use rand::Rng;
use teloxide::types::InputFile;

pub struct Stickers;

#[allow(dead_code)]
impl Stickers {
    pub fn i_want_you() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ2Fi_6Mw9h41pbftXuWRKqbUk93uewACGwwAAtz2cFLKudLnGc5aWSkE",
        )
    }

    pub fn surprised() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ2Vi_6P8BhQhnkt0MPAktbNUIAn01gACmA0AAvC9aVJ200Wqdts53ykE",
        )
    }

    pub fn thinking() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ2li_6QhGaIAAcxsFB_pMPkxKtOMPOUAAhYMAAK9D2lS3BjCgCNOSkwpBA",
        )
    }

    pub fn face() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ2ti_6RQLoIF68VbmdIFHkixutYs2gACDwsAAt6fcVLJzsofM5LAuykE",
        )
    }

    pub fn fiero() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ21i_6Roq1RgmpK8mllzLq1mETK3yQAC_QwAAkSCcVIrbcaiCisg_CkE",
        )
    }

    pub fn thinking_seated() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ29i_6SCZJOfG6421kUGULZqYZsrqAAClAwAAoWRaVJPffpBb3mtoikE",
        )
    }

    pub fn grrr() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ3Fi_6SvgR-prgwNPQ1PFiQ7M8h00AACfwsAAv8ycVIbq4KgRxJqfSkE",
        )
    }

    pub fn oh_no() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ3Ni_6TIYIBodFe_c-5UYCN0_IXYNAACzQsAAlNhcFJLgZZBl2vhoykE",
        )
    }

    pub fn got_it() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ3Zi_6Tb-7edgvh5Xx3IjKnjcpLMKQAC8w0AAjHAcVL8xbQqnS4EZykE",
        )
    }

    pub fn luna_e_stelle() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ3li_6T3mx2CKPwWRrMAAS44c78WsIMAAq4MAAIqR3BSA_RmBdoq2QwpBA",
        )
    }

    pub fn mildly_furious() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ3ti_6UKppdOakqDa9WYX41IaZlvZgACmAoAAl81cFKXhBZwvyB6eikE",
        )
    }

    pub fn despair() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ39i_6UeUK5hMmEsCuAoVIlAet-MZAACJgwAAg8baFIvkuPrcv9pAikE",
        )
    }

    pub fn omg() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXQ4Fi_6U6vTx0sz-ulT06w6lhyFFFVQACcgwAAowRcFKeAWRuKEj_kykE",
        )
    }

    pub fn random() -> InputFile {
        let mut rng = rand::thread_rng();
        STICKERS[rng.gen_range(0..STICKERS.len())]()
    }
}

const STICKERS: &[fn() -> InputFile] = &[
    Stickers::despair,
    Stickers::face,
    Stickers::fiero,
    Stickers::got_it,
    Stickers::grrr,
    Stickers::i_want_you,
    Stickers::luna_e_stelle,
    Stickers::mildly_furious,
    Stickers::oh_no,
    Stickers::omg,
    Stickers::surprised,
    Stickers::thinking,
    Stickers::thinking_seated,
];
