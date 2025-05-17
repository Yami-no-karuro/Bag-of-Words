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
    let query: &str = "Lo stregone e l'anello.";
    let books: Vec<(&str, &str)> = Vec::from([
        ("book-01", "Il romanzo ha inizio con la festa del 111º compleanno di Bilbo e del 33° di suo nipote Frodo."),
        ("book-02", "Alla fine della festa, Bilbo comunica a tutti i presenti che intende lasciare la Contea per sempre e, dopo essersi infilato l'anello, sparisce."),
        ("book-03", "Lo stregone comincia a sospettare della natura dell'anello, perciò consiglia a Frodo di non adoperarlo mai e si allontana da Casa Baggins alla ricerca della verità."),
        ("book-04", "Diciassette anni dopo, Gandalf scopre che l'anello in possesso di Frodo è l'Unico Anello, forgiato molti anni prima dall'Oscuro Signore Sauron."),
        ("book-05", "Sauron tuttavia lo perse dopo essere stato sconfitto dal re elfico Gil-galad e dal re dei Dúnedain Elendil al culmine della battaglia dell'Ultima Alleanza tra Elfi e Uomini.")
    ]);

    for book in books {
        let input: String = String::from(book.1);
        let bow: BoW = BoW::build(input);
        bow.save(book.0);
    }
}
