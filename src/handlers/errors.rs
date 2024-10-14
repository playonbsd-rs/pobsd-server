use askama::Template;

#[derive(Template)]
#[template(path = "500.html")]
pub struct InternalErrorTemplate {}

#[derive(Template)]
#[template(path = "404_no_game.html")]
pub struct ErrorNoGameTemplate {}

#[derive(Template)]
#[template(path = "404_no_games.html")]
pub struct ErrorNoGamesTemplate {}
