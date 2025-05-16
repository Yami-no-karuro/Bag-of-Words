#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bow;
mod tokenizer;

use bow::BoW;
use tokenizer::{
    Token, 
    Tokenizer
};

fn main() {
    let titles: Vec<&str> = Vec::from([
        "Il romanzo ha inizio con la festa del 111º compleanno di Bilbo e del 33° di suo nipote Frodo.",
        "Alla fine della festa, Bilbo comunica a tutti i presenti che intende lasciare la Contea per sempre e, dopo essersi infilato l'anello, sparisce.",
        "Lo stregone comincia a sospettare della natura dell'anello, perciò consiglia a Frodo di non adoperarlo mai e si allontana da Casa Baggins alla ricerca della verità.",
        "Diciassette anni dopo, Gandalf scopre che l'anello in possesso di Frodo è l'Unico Anello, forgiato molti anni prima dall'Oscuro Signore Sauron.",
        "Sauron tuttavia lo perse dopo essere stato sconfitto dal re elfico Gil-galad e dal re dei Dúnedain Elendil al culmine della battaglia dell'Ultima Alleanza tra Elfi e Uomini."
    ]);

    for title in titles {
        let input: String = String::from(title);
        let bow: BoW = BoW::build(input);
        dbg!(bow);
    }
}
