use chess_core::*;
use clap::Parser;
use leptos_bevy_canvas::prelude::*;
fn main() {
    let opt = Opt::parse();
    let duplexs = event_duplex::<ChessEvent>();
    init_chess(opt, duplexs.1).run();
}
