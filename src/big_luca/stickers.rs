//! # Stickers
//!
//! A static getters for big luca stickers

use rand::Rng;
use teloxide::types::InputFile;

pub struct Stickers;

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

    pub fn shock() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeuVjCLj8QFEenMC7IzLd1o6FLk5yNwACVA0AAlulSVAzJC66LKV1EikE",
        )
    }

    pub fn wtf() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeutjCLkDhY7t5KV6GkYdw9YYAefsUgAC9hIAAiVTSFCzgU9v4oYpjykE",
        )
    }

    pub fn thinking2() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeuljCLkBVXJPIh_QzsDTwpLmBEcMVgACtw4AAlhUQFC4LAds8GbTWCkE",
        )
    }

    pub fn like() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeuFjCLj4zR8sXoOnKtvXLI3yxEc9ugACPQ4AAu4IQFC27Rt-CssQtCkE",
        )
    }

    pub fn pointing_down() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeuNjCLj6rTNxR-uBaVQsRHNKZHd7cQACQQ0AAsFeQVBdgJmxyHMScSkE",
        )
    }

    pub fn shrug() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXeudjCLj-lDwAAawLmNyJ5WKNJ49Jl0IAAjIMAAIipkhQCYimCOKXSzgpBA",
        )
    }

    pub fn double_pointing_down() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXkyRjDNkeidUyEOw3YR-z_cfi0RFyoQACjAwAAhBYYFCxYqFW11Pb2SkE",
        )
    }

    pub fn standing() -> InputFile {
        InputFile::file_id("CAACAgQAAxkBAAEXkydjDNkgwxlYYQozvHLBh4PS7XQKiAACkhUAAjNTaVDLs5YJ")
    }

    pub fn smiling() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXkypjDNkhodSIonwEWYfP8vbGl_zuHgACuRIAAhFXaFAP4JJlGloLrCkE",
        )
    }

    pub fn seated() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXpARjD3R-PoAOMpfY_oQ3P0RnLYB9AAPfDQACBuKBUPqjneaTxA--KQQ",
        )
    }

    pub fn conference() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXpAZjD3SADih-z-oEcAg38NDcdspUMwACqA0AAnY3gVAs5hA27uFFUikE",
        )
    }

    pub fn christmas() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXqS5jEGXaFYb7ClHLonV12r6Ah0O4AQAC_A8AAlRlgVA10GBRJpErxCkE",
        )
    }

    pub fn big_face() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXqTBjEGXcDj7JxnTywAELWoT6V3glfAACMw4AAmkviVAvqQmNvTJPwykE",
        )
    }

    pub fn beach() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXqSxjEGXYh72H7O5O1DLL4CWtAAFGUEoAAgUOAALsBIlQZYthJA6wqy8pBA",
        )
    }

    pub fn lucro_time() -> InputFile {
        InputFile::file_id(
            "CAACAgQAAxkBAAEXxhpjFe4m19MfIFbKbQYTXrmcacvIzgACtAwAAibNiFDGiXMGXPYL3CkE",
        )
    }

    pub fn random() -> InputFile {
        let mut rng = rand::thread_rng();
        STICKERS[rng.gen_range(0..STICKERS.len())]()
    }
}

const STICKERS: &[fn() -> InputFile] = &[
    Stickers::beach,
    Stickers::big_face,
    Stickers::christmas,
    Stickers::conference,
    Stickers::despair,
    Stickers::double_pointing_down,
    Stickers::face,
    Stickers::fiero,
    Stickers::got_it,
    Stickers::grrr,
    Stickers::i_want_you,
    Stickers::like,
    Stickers::lucro_time,
    Stickers::luna_e_stelle,
    Stickers::mildly_furious,
    Stickers::oh_no,
    Stickers::omg,
    Stickers::pointing_down,
    Stickers::seated,
    Stickers::shock,
    Stickers::shrug,
    Stickers::smiling,
    Stickers::standing,
    Stickers::surprised,
    Stickers::thinking_seated,
    Stickers::thinking,
    Stickers::thinking2,
    Stickers::wtf,
];
