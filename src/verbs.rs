// Copyright 2024-present Adam Burucs. MIT license.

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum VerbType {
    Present,
    Past,
}

pub struct Verb {
    pub present: &'static str,
    pub past: &'static str,
}

pub const VERBS: [Verb; 30] = [
    Verb {
        present: "update",
        past: "updated",
    },
    Verb {
        present: "allow",
        past: "allowed",
    },
    Verb {
        present: "combine",
        past: "combined",
    },
    Verb {
        present: "stop",
        past: "stopped",
    },
    Verb {
        present: "check",
        past: "checked",
    },
    Verb {
        present: "move",
        past: "moved",
    },
    Verb {
        present: "work on",
        past: "worked on",
    },
    Verb {
        present: "rebuild",
        past: "rebuilt",
    },
    Verb {
        present: "clean up",
        past: "cleaned up",
    },
    Verb {
        present: "fix",
        past: "fixed",
    },
    Verb {
        present: "change",
        past: "changed",
    },
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
    Verb {
        present: "spawn",
        past: "spawned",
    },
    Verb {
        present: "optimize",
        past: "optimized",
    },
    Verb {
        present: "make",
        past: "made",
    },
    Verb {
        present: "cache",
        past: "cached",
    },
    Verb {
        present: "uncache",
        past: "uncache",
    },
    Verb {
        present: "do",
        past: "did",
    },
    Verb {
        present: "begin",
        past: "began",
    },
    Verb {
        present: "finish",
        past: "finished",
    },
    Verb {
        present: "kill",
        past: "killed",
    },
    Verb {
        present: "fade",
        past: "fade",
    },
];
