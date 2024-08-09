// Copyright 2024 Adam Burucs. MIT license.
use rand::prelude::*;

#[derive(PartialEq)]
enum VerbType {
    Present,
    Past,
}

#[warn(dead_code)]
struct Noun {
    singular: &'static str,
    plural: &'static str,
}

#[warn(dead_code)]
struct Verb {
    present: &'static str,
    past: &'static str,
}

const NOUNS: [Noun; 35] = [
    Noun {
        singular: "twirling gib",
        plural: "twirling gibs",
    },
    Noun {
        singular: "server protocol number",
        plural: "server protocol numbers",
    },
    Noun {
        singular: "internet code",
        plural: "internet codes",
    },
    Noun {
        singular: "crushing issue",
        plural: "crushing issues",
    },
    Noun {
        singular: "origin bug",
        plural: "origin bugs",
    },
    Noun {
        singular: "radius damage",
        plural: "radius damages",
    },
    Noun {
        singular: "pak file support",
        plural: "pak file support",
    },
    Noun {
        singular: "screen warp",
        plural: "screen warps",
    },
    Noun {
        singular: "clamp velocity",
        plural: "clamp velocities",
    },
    Noun {
        singular: "respawn rule",
        plural: "respawn rules",
    },
    Noun {
        singular: "direction protocol",
        plural: "direction protocols",
    },
    Noun {
        singular: "armor color flash",
        plural: "armor color flashes",
    },
    Noun {
        singular: "alias model",
        plural: "alias models",
    },
    Noun {
        singular: "level",
        plural: "levels",
    },
    Noun {
        singular: "enemy",
        plural: "enemies",
    },
    Noun {
        singular: "deathmatch",
        plural: "deathmatches",
    },
    Noun {
        singular: "player movement",
        plural: "player movements",
    },
    Noun {
        singular: "cheat code",
        plural: "cheat codes",
    },
    Noun {
        singular: "performance",
        plural: "performances",
    },
    Noun {
        singular: "buffer",
        plural: "buffers",
    },
    Noun {
        singular: "demo",
        plural: "demos",
    },
    Noun {
        singular: "release change",
        plural: "release changes",
    },
    Noun {
        singular: "server",
        plural: "servers",
    },
    Noun {
        singular: "network connection",
        plural: "network connections",
    },
    Noun {
        singular: "monster",
        plural: "monsters",
    },
    Noun {
        singular: "matrix",
        plural: "matrices",
    },
    Noun {
        singular: "zooming",
        plural: "zoomings",
    },
    Noun {
        singular: "scene",
        plural: "scenes",
    },
    Noun {
        singular: "space partitioning",
        plural: "space partitionings",
    },
    Noun {
        singular: "surface cache",
        plural: "surface caches",
    },
    Noun {
        singular: "config file",
        plural: "config files",
    },
    Noun {
        singular: "rendering",
        plural: "renderings",
    },
    Noun {
        singular: "texture",
        plural: "textures",
    },
    Noun {
        singular: "light",
        plural: "lights",
    },
    Noun {
        singular: "precache",
        plural: "precaches",
    },
];

const VERBS: [Verb; 9] = [
    Verb {
        present: "add",
        past: "added",
    },
    Verb {
        present: "fix",
        past: "fixed",
    },
    Verb {
        present: "remove",
        past: "removed",
    },
    Verb {
        present: "test",
        past: "tested",
    },
    Verb {
        present: "debug",
        past: "debugged",
    },
    Verb {
        present: "refactor",
        past: "refactored",
    },
    Verb {
        present: "design",
        past: "designed",
    },
    Verb {
        present: "improve",
        past: "improved",
    },
    Verb {
        present: "unify",
        past: "unified",
    },
];

const ADJECTIVES: [&str; 23] = [
    "negative",
    "extra",
    "absolute",
    "flickering",
    "weird",
    "referential",
    "close",
    "far",
    "top",
    "bottom",
    "solid",
    "opaque",
    "binary",
    "hexadecimal",
    "low-level",
    "high-level",
    "broken",
    "partial",
    "bright",
    "dark",
    "frozen",
    "stuck",
    "flying",
];

const MAX_VERBS: usize = VERBS.len();
const MAX_NOUNS: usize = NOUNS.len();
const MAX_ADJECTIVES: usize = ADJECTIVES.len();

fn generate_sentence(verb_type: VerbType) -> String {
    let mut rng = thread_rng();
    let rand_plural = rng.gen_range(0..1);
    let rand_verb = rng.gen_range(0..MAX_VERBS);
    let rand_adjective = rng.gen_range(0..MAX_ADJECTIVES);
    let rand_noun = rng.gen_range(0..MAX_NOUNS);

    let verb: &str = if verb_type == VerbType::Present {
        VERBS[rand_verb].present
    } else {
        VERBS[rand_verb].past
    };

    let noun: &str = if rand_plural == 0 {
        NOUNS[rand_noun].singular
    } else {
        NOUNS[rand_noun].plural
    };

    let sentence = format!("{} {} {}", verb, ADJECTIVES[rand_adjective], noun);
    sentence
}

fn main() {
    println!("John Carmack Planner");
    println!("Generate random plan files in John Carmack style.");
    println!();
    println!("Generating plan notes...");
    println!();
    for _i in 0..10 {
        let sentence = generate_sentence(VerbType::Past);
        println!("{sentence}");
    }
}
