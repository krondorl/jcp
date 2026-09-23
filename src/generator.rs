// Copyright 2024-present Adam Burucs. MIT license.

use crate::{
    adjectives::ADJECTIVES,
    nouns::NOUNS,
    verbs::{VerbType, VERBS},
};
use tinyrand::{RandRange, StdRand};

pub fn generate_sentence(verb_type: VerbType, rng: &mut StdRand) -> String {
    let plural = rng.next_range(0usize..2) == 1;
    let verb = &VERBS[rng.next_range(0usize..VERBS.len())];
    let adjective = ADJECTIVES[rng.next_range(0usize..ADJECTIVES.len())];
    let noun = &NOUNS[rng.next_range(0usize..NOUNS.len())];

    let verb = match verb_type {
        VerbType::Present => verb.present,
        VerbType::Past => verb.past,
    };
    let noun = if plural { noun.plural } else { noun.singular };

    let sentence = format!("{verb} {adjective} {noun}");
    sentence
}
