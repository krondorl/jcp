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

pub const VERBS: [Verb; 326] = [
    Verb {
        present: "accelerate",
        past: "accelerated",
    },
    Verb {
        present: "accept",
        past: "accepted",
    },
    Verb {
        present: "activate",
        past: "activated",
    },
    Verb {
        present: "add",
        past: "added",
    },
    Verb {
        present: "adjust",
        past: "adjusted",
    },
    Verb {
        present: "advance",
        past: "advanced",
    },
    Verb {
        present: "aim",
        past: "aimed",
    },
    Verb {
        present: "allocate",
        past: "allocated",
    },
    Verb {
        present: "allow",
        past: "allowed",
    },
    Verb {
        present: "alternate",
        past: "alternated",
    },
    Verb {
        present: "anger",
        past: "angered",
    },
    Verb {
        present: "animate",
        past: "animated",
    },
    Verb {
        present: "appear",
        past: "appeared",
    },
    Verb {
        present: "apply",
        past: "applied",
    },
    Verb {
        present: "ask",
        past: "asked",
    },
    Verb {
        present: "attack",
        past: "attacked",
    },
    Verb {
        present: "attempt",
        past: "attempted",
    },
    Verb {
        present: "back",
        past: "backed",
    },
    Verb {
        present: "balance",
        past: "balanced",
    },
    Verb {
        present: "ban",
        past: "banned",
    },
    Verb {
        present: "bear",
        past: "bore",
    },
    Verb {
        present: "beat",
        past: "beat",
    },
    Verb {
        present: "become",
        past: "became",
    },
    Verb {
        present: "beep",
        past: "beeped",
    },
    Verb {
        present: "begin",
        past: "began",
    },
    Verb {
        present: "believe",
        past: "believed",
    },
    Verb {
        present: "bind",
        past: "bound",
    },
    Verb {
        present: "blend",
        past: "blended",
    },
    Verb {
        present: "blind",
        past: "blinded",
    },
    Verb {
        present: "blink",
        past: "blinked",
    },
    Verb {
        present: "block",
        past: "blocked",
    },
    Verb {
        present: "bob",
        past: "bobbed",
    },
    Verb {
        present: "bounce",
        past: "bounced",
    },
    Verb {
        present: "bound",
        past: "bounded",
    },
    Verb {
        present: "break",
        past: "broke",
    },
    Verb {
        present: "bring",
        past: "brought",
    },
    Verb {
        present: "buffer",
        past: "buffered",
    },
    Verb {
        present: "build",
        past: "built",
    },
    Verb {
        present: "burn",
        past: "burned",
    },
    Verb {
        present: "bypass",
        past: "bypassed",
    },
    Verb {
        present: "cache",
        past: "cached",
    },
    Verb {
        present: "calculate",
        past: "calculated",
    },
    Verb {
        present: "call",
        past: "called",
    },
    Verb {
        present: "cap",
        past: "capped",
    },
    Verb {
        present: "carry",
        past: "carried",
    },
    Verb {
        present: "cast",
        past: "cast",
    },
    Verb {
        present: "cause",
        past: "caused",
    },
    Verb {
        present: "center",
        past: "centered",
    },
    Verb {
        present: "change",
        past: "changed",
    },
    Verb {
        present: "charge",
        past: "charged",
    },
    Verb {
        present: "check",
        past: "checked",
    },
    Verb {
        present: "choose",
        past: "chose",
    },
    Verb {
        present: "clamp",
        past: "clamped",
    },
    Verb {
        present: "clean",
        past: "cleaned",
    },
    Verb {
        present: "clear",
        past: "cleared",
    },
    Verb {
        present: "click",
        past: "clicked",
    },
    Verb {
        present: "climb",
        past: "climbed",
    },
    Verb {
        present: "clip",
        past: "clipped",
    },
    Verb {
        present: "clog",
        past: "clogged",
    },
    Verb {
        present: "close",
        past: "closed",
    },
    Verb {
        present: "combine",
        past: "combined",
    },
    Verb {
        present: "come",
        past: "came",
    },
    Verb {
        present: "compile",
        past: "compiled",
    },
    Verb {
        present: "complete",
        past: "completed",
    },
    Verb {
        present: "configure",
        past: "configured",
    },
    Verb {
        present: "confuse",
        past: "confused",
    },
    Verb {
        present: "connect",
        past: "connected",
    },
    Verb {
        present: "consider",
        past: "considered",
    },
    Verb {
        present: "consolidate",
        past: "consolidated",
    },
    Verb {
        present: "constrain",
        past: "constrained",
    },
    Verb {
        present: "contain",
        past: "contained",
    },
    Verb {
        present: "continue",
        past: "continued",
    },
    Verb {
        present: "control",
        past: "controlled",
    },
    Verb {
        present: "convert",
        past: "converted",
    },
    Verb {
        present: "copy",
        past: "copied",
    },
    Verb {
        present: "correct",
        past: "corrected",
    },
    Verb {
        present: "count",
        past: "counted",
    },
    Verb {
        present: "cover",
        past: "covered",
    },
    Verb {
        present: "crash",
        past: "crashed",
    },
    Verb {
        present: "crawl",
        past: "crawled",
    },
    Verb {
        present: "create",
        past: "created",
    },
    Verb {
        present: "crush",
        past: "crushed",
    },
    Verb {
        present: "cull",
        past: "culled",
    },
    Verb {
        present: "cut",
        past: "cut",
    },
    Verb {
        present: "cycle",
        past: "cycled",
    },
    Verb {
        present: "damage",
        past: "damaged",
    },
    Verb {
        present: "darken",
        past: "darkened",
    },
    Verb {
        present: "deal",
        past: "dealt",
    },
    Verb {
        present: "debounce",
        past: "debounced",
    },
    Verb {
        present: "debug",
        past: "debugged",
    },
    Verb {
        present: "decouple",
        past: "decoupled",
    },
    Verb {
        present: "define",
        past: "defined",
    },
    Verb {
        present: "delay",
        past: "delayed",
    },
    Verb {
        present: "depend",
        past: "depended",
    },
    Verb {
        present: "design",
        past: "designed",
    },
    Verb {
        present: "detect",
        past: "detected",
    },
    Verb {
        present: "determine",
        past: "determined",
    },
    Verb {
        present: "develop",
        past: "developed",
    },
    Verb {
        present: "die",
        past: "died",
    },
    Verb {
        present: "dim",
        past: "dimmed",
    },
    Verb {
        present: "disable",
        past: "disabled",
    },
    Verb {
        present: "disconnect",
        past: "disconnected",
    },
    Verb {
        present: "display",
        past: "displayed",
    },
    Verb {
        present: "distribute",
        past: "distributed",
    },
    Verb {
        present: "do",
        past: "did",
    },
    Verb {
        present: "double",
        past: "doubled",
    },
    Verb {
        present: "draw",
        past: "drew",
    },
    Verb {
        present: "drift",
        past: "drifted",
    },
    Verb {
        present: "drive",
        past: "drove",
    },
    Verb {
        present: "drop",
        past: "dropped",
    },
    Verb {
        present: "drown",
        past: "drowned",
    },
    Verb {
        present: "dump",
        past: "dumped",
    },
    Verb {
        present: "duplicate",
        past: "duplicated",
    },
    Verb {
        present: "eat",
        past: "ate",
    },
    Verb {
        present: "echo",
        past: "echoed",
    },
    Verb {
        present: "eliminate",
        past: "eliminated",
    },
    Verb {
        present: "emit",
        past: "emitted",
    },
    Verb {
        present: "enable",
        past: "enabled",
    },
    Verb {
        present: "end",
        past: "ended",
    },
    Verb {
        present: "enter",
        past: "entered",
    },
    Verb {
        present: "error",
        past: "errored",
    },
    Verb {
        present: "establish",
        past: "established",
    },
    Verb {
        present: "execute",
        past: "executed",
    },
    Verb {
        present: "exit",
        past: "exited",
    },
    Verb {
        present: "expand",
        past: "expanded",
    },
    Verb {
        present: "expect",
        past: "expected",
    },
    Verb {
        present: "explode",
        past: "exploded",
    },
    Verb {
        present: "export",
        past: "exported",
    },
    Verb {
        present: "face",
        past: "faced",
    },
    Verb {
        present: "fade",
        past: "faded",
    },
    Verb {
        present: "fail",
        past: "failed",
    },
    Verb {
        present: "fake",
        past: "faked",
    },
    Verb {
        present: "fall",
        past: "fell",
    },
    Verb {
        present: "fault",
        past: "faulted",
    },
    Verb {
        present: "feel",
        past: "felt",
    },
    Verb {
        present: "fight",
        past: "fought",
    },
    Verb {
        present: "fill",
        past: "filled",
    },
    Verb {
        present: "filter",
        past: "filtered",
    },
    Verb {
        present: "find",
        past: "found",
    },
    Verb {
        present: "finish",
        past: "finished",
    },
    Verb {
        present: "fire",
        past: "fired",
    },
    Verb {
        present: "fix",
        past: "fixed",
    },
    Verb {
        present: "flash",
        past: "flashed",
    },
    Verb {
        present: "flicker",
        past: "flickered",
    },
    Verb {
        present: "float",
        past: "floated",
    },
    Verb {
        present: "flush",
        past: "flushed",
    },
    Verb {
        present: "fly",
        past: "flew",
    },
    Verb {
        present: "focus",
        past: "focused",
    },
    Verb {
        present: "follow",
        past: "followed",
    },
    Verb {
        present: "force",
        past: "forced",
    },
    Verb {
        present: "fork",
        past: "forked",
    },
    Verb {
        present: "fragment",
        past: "fragmented",
    },
    Verb {
        present: "freeze",
        past: "froze",
    },
    Verb {
        present: "generate",
        past: "generated",
    },
    Verb {
        present: "get",
        past: "got",
    },
    Verb {
        present: "gib",
        past: "gibbed",
    },
    Verb {
        present: "give",
        past: "gave",
    },
    Verb {
        present: "go",
        past: "went",
    },
    Verb {
        present: "grab",
        past: "grabbed",
    },
    Verb {
        present: "ground",
        past: "grounded",
    },
    Verb {
        present: "hack",
        past: "hacked",
    },
    Verb {
        present: "halve",
        past: "halved",
    },
    Verb {
        present: "handle",
        past: "handled",
    },
    Verb {
        present: "hang",
        past: "hung",
    },
    Verb {
        present: "have",
        past: "had",
    },
    Verb {
        present: "hear",
        past: "heard",
    },
    Verb {
        present: "help",
        past: "helped",
    },
    Verb {
        present: "hide",
        past: "hid",
    },
    Verb {
        present: "highlight",
        past: "highlighted",
    },
    Verb {
        present: "hit",
        past: "hit",
    },
    Verb {
        present: "hold",
        past: "held",
    },
    Verb {
        present: "hope",
        past: "hoped",
    },
    Verb {
        present: "hurt",
        past: "hurt",
    },
    Verb {
        present: "identify",
        past: "identified",
    },
    Verb {
        present: "ignore",
        past: "ignored",
    },
    Verb {
        present: "implement",
        past: "implemented",
    },
    Verb {
        present: "improve",
        past: "improved",
    },
    Verb {
        present: "include",
        past: "included",
    },
    Verb {
        present: "increase",
        past: "increased",
    },
    Verb {
        present: "increment",
        past: "incremented",
    },
    Verb {
        present: "induce",
        past: "induced",
    },
    Verb {
        present: "inform",
        past: "informed",
    },
    Verb {
        present: "integrate",
        past: "integrated",
    },
    Verb {
        present: "intermix",
        past: "intermixed",
    },
    Verb {
        present: "interpolate",
        past: "interpolated",
    },
    Verb {
        present: "introduce",
        past: "introduced",
    },
    Verb {
        present: "issue",
        past: "issued",
    },
    Verb {
        present: "jump",
        past: "jumped",
    },
    Verb {
        present: "keep",
        past: "kept",
    },
    Verb {
        present: "kick",
        past: "kicked",
    },
    Verb {
        present: "kill",
        past: "killed",
    },
    Verb {
        present: "land",
        past: "landed",
    },
    Verb {
        present: "leap",
        past: "leaped",
    },
    Verb {
        present: "learn",
        past: "learned",
    },
    Verb {
        present: "leave",
        past: "left",
    },
    Verb {
        present: "let",
        past: "let",
    },
    Verb {
        present: "license",
        past: "licensed",
    },
    Verb {
        present: "light",
        past: "lit",
    },
    Verb {
        present: "limit",
        past: "limited",
    },
    Verb {
        present: "link",
        past: "linked",
    },
    Verb {
        present: "load",
        past: "loaded",
    },
    Verb {
        present: "lock",
        past: "locked",
    },
    Verb {
        present: "log",
        past: "logged",
    },
    Verb {
        present: "look",
        past: "looked",
    },
    Verb {
        present: "loop",
        past: "looped",
    },
    Verb {
        present: "lose",
        past: "lost",
    },
    Verb {
        present: "lower",
        past: "lowered",
    },
    Verb {
        present: "make",
        past: "made",
    },
    Verb {
        present: "map",
        past: "mapped",
    },
    Verb {
        present: "match",
        past: "matched",
    },
    Verb {
        present: "merge",
        past: "merged",
    },
    Verb {
        present: "miss",
        past: "missed",
    },
    Verb {
        present: "mix",
        past: "mixed",
    },
    Verb {
        present: "modify",
        past: "modified",
    },
    Verb {
        present: "move",
        past: "moved",
    },
    Verb {
        present: "need",
        past: "needed",
    },
    Verb {
        present: "notice",
        past: "noticed",
    },
    Verb {
        present: "open",
        past: "opened",
    },
    Verb {
        present: "optimize",
        past: "optimized",
    },
    Verb {
        present: "overflow",
        past: "overflowed",
    },
    Verb {
        present: "override",
        past: "overrode",
    },
    Verb {
        present: "parse",
        past: "parsed",
    },
    Verb {
        present: "pass",
        past: "passed",
    },
    Verb {
        present: "pause",
        past: "paused",
    },
    Verb {
        present: "pick",
        past: "picked",
    },
    Verb {
        present: "play",
        past: "played",
    },
    Verb {
        present: "point",
        past: "pointed",
    },
    Verb {
        present: "port",
        past: "ported",
    },
    Verb {
        present: "position",
        past: "positioned",
    },
    Verb {
        present: "precache",
        past: "precached",
    },
    Verb {
        present: "press",
        past: "pressed",
    },
    Verb {
        present: "prevent",
        past: "prevented",
    },
    Verb {
        present: "print",
        past: "printed",
    },
    Verb {
        present: "process",
        past: "processed",
    },
    Verb {
        present: "program",
        past: "programmed",
    },
    Verb {
        present: "protect",
        past: "protected",
    },
    Verb {
        present: "pull",
        past: "pulled",
    },
    Verb {
        present: "push",
        past: "pushed",
    },
    Verb {
        present: "put",
        past: "put",
    },
    Verb {
        present: "raise",
        past: "raised",
    },
    Verb {
        present: "rank",
        past: "ranked",
    },
    Verb {
        present: "read",
        past: "read",
    },
    Verb {
        present: "rebuild",
        past: "rebuilt",
    },
    Verb {
        present: "reduce",
        past: "reduced",
    },
    Verb {
        present: "release",
        past: "released",
    },
    Verb {
        present: "relink",
        past: "relinked",
    },
    Verb {
        present: "reload",
        past: "reloaded",
    },
    Verb {
        present: "remain",
        past: "remained",
    },
    Verb {
        present: "remove",
        past: "removed",
    },
    Verb {
        present: "rename",
        past: "renamed",
    },
    Verb {
        present: "render",
        past: "rendered",
    },
    Verb {
        present: "repeat",
        past: "repeated",
    },
    Verb {
        present: "replace",
        past: "replaced",
    },
    Verb {
        present: "report",
        past: "reported",
    },
    Verb {
        present: "reset",
        past: "reset",
    },
    Verb {
        present: "resize",
        past: "resized",
    },
    Verb {
        present: "respawn",
        past: "respawned",
    },
    Verb {
        present: "restart",
        past: "restarted",
    },
    Verb {
        present: "retain",
        past: "retained",
    },
    Verb {
        present: "return",
        past: "returned",
    },
    Verb {
        present: "reuse",
        past: "reused",
    },
    Verb {
        present: "reverse",
        past: "reversed",
    },
    Verb {
        present: "rewrite",
        past: "rewrote",
    },
    Verb {
        present: "ride",
        past: "rode",
    },
    Verb {
        present: "rotate",
        past: "rotated",
    },
    Verb {
        present: "run",
        past: "ran",
    },
    Verb {
        present: "save",
        past: "saved",
    },
    Verb {
        present: "scale",
        past: "scaled",
    },
    Verb {
        present: "scroll",
        past: "scrolled",
    },
    Verb {
        present: "see",
        past: "saw",
    },
    Verb {
        present: "send",
        past: "sent",
    },
    Verb {
        present: "set",
        past: "set",
    },
    Verb {
        present: "shift",
        past: "shifted",
    },
    Verb {
        present: "ship",
        past: "shipped",
    },
    Verb {
        present: "shoot",
        past: "shot",
    },
    Verb {
        present: "show",
        past: "showed",
    },
    Verb {
        present: "shrink",
        past: "shrank",
    },
    Verb {
        present: "simulate",
        past: "simulated",
    },
    Verb {
        present: "skip",
        past: "skipped",
    },
    Verb {
        present: "slide",
        past: "slid",
    },
    Verb {
        present: "slow",
        past: "slowed",
    },
    Verb {
        present: "snap",
        past: "snapped",
    },
    Verb {
        present: "spawn",
        past: "spawned",
    },
    Verb {
        present: "specify",
        past: "specified",
    },
    Verb {
        present: "spin",
        past: "spun",
    },
    Verb {
        present: "split",
        past: "split",
    },
    Verb {
        present: "spread",
        past: "spread",
    },
    Verb {
        present: "start",
        past: "started",
    },
    Verb {
        present: "stay",
        past: "stayed",
    },
    Verb {
        present: "step",
        past: "stepped",
    },
    Verb {
        present: "stick",
        past: "stuck",
    },
    Verb {
        present: "stop",
        past: "stopped",
    },
    Verb {
        present: "store",
        past: "stored",
    },
    Verb {
        present: "stream",
        past: "streamed",
    },
    Verb {
        present: "stretch",
        past: "stretched",
    },
    Verb {
        present: "support",
        past: "supported",
    },
    Verb {
        present: "sweat",
        past: "sweated",
    },
    Verb {
        present: "switch",
        past: "switched",
    },
    Verb {
        present: "sync",
        past: "synced",
    },
    Verb {
        present: "take",
        past: "took",
    },
    Verb {
        present: "target",
        past: "targeted",
    },
    Verb {
        present: "test",
        past: "tested",
    },
    Verb {
        present: "texture",
        past: "textured",
    },
    Verb {
        present: "think",
        past: "thought",
    },
    Verb {
        present: "throw",
        past: "threw",
    },
    Verb {
        present: "tilt",
        past: "tilted",
    },
    Verb {
        present: "time",
        past: "timed",
    },
    Verb {
        present: "toggle",
        past: "toggled",
    },
    Verb {
        present: "touch",
        past: "touched",
    },
    Verb {
        present: "track",
        past: "tracked",
    },
    Verb {
        present: "transfer",
        past: "transferred",
    },
    Verb {
        present: "trigger",
        past: "triggered",
    },
    Verb {
        present: "try",
        past: "tried",
    },
    Verb {
        present: "turn",
        past: "turned",
    },
    Verb {
        present: "tweak",
        past: "tweaked",
    },
    Verb {
        present: "update",
        past: "updated",
    },
    Verb {
        present: "use",
        past: "used",
    },
    Verb {
        present: "view",
        past: "viewed",
    },
    Verb {
        present: "wait",
        past: "waited",
    },
    Verb {
        present: "wake",
        past: "woke",
    },
    Verb {
        present: "walk",
        past: "walked",
    },
    Verb {
        present: "warp",
        past: "warped",
    },
    Verb {
        present: "work",
        past: "worked",
    },
    Verb {
        present: "worry",
        past: "worried",
    },
    Verb {
        present: "wrap",
        past: "wrapped",
    },
    Verb {
        present: "write",
        past: "wrote",
    },
];
