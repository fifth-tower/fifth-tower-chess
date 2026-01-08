pub enum Game {
    Chess,
}

impl Game {
    pub fn get_server_url(&self, room: String) -> String {
        let room = if room.len() > 0 {
            "_".to_string() + room.as_str()
        } else {
            room
        };
        match &self {
            Game::Chess => format!("ws://8.137.49.149/box/chess{}?next=2", room),
        }
    }
    pub fn get_invite_url(&self, room: String) -> String {
        let room = if room.len() > 0 {
            "/friend?rid=".to_string() + room.as_str()
        } else {
            room
        };
        match &self {
            Game::Chess => format!("/chess{}", room),
        }
    }

    pub fn server_url(game: String, room: String) -> String {
        match game.to_lowercase().as_str() {
            "chess" => Game::Chess.get_server_url(room),
            _ => "".to_string(),
        }
    }

    pub fn invite_url(game: String, room: String) -> String {
        match game.to_lowercase().as_str() {
            "chess" => Game::Chess.get_invite_url(room),
            _ => "".to_string(),
        }
    }
}
