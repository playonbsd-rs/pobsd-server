use libpobsd::{GameFilter, SearchType, Status};

use super::AppDb;

#[derive(Debug, Clone, Default)]
pub struct GameStats {
    pub engine_stats: Vec<(String, usize, String)>,
    pub runtime_stats: Vec<(String, usize, String)>,
    pub genre_stats: Vec<(String, usize, String)>,
    pub tag_stats: Vec<(String, usize, String)>,
    pub year_stats: Vec<(String, usize, String)>,
    pub dev_stats: Vec<(String, usize, String)>,
    pub publi_stats: Vec<(String, usize, String)>,
    pub status_stats: Vec<(String, usize, String)>,
    pub total_games: usize,
}

impl AppDb {
    pub fn update_stats(&mut self) {
        let st = SearchType::NotCaseSensitive;

        let mut status_stats: Vec<(String, usize, String)> = Vec::new();
        let mut filter = GameFilter::default();

        let ss = Status::Unknown;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Unknown".into(), nbr, ss.to_string()));

        let ss = Status::DoesNotRun;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Does not run".into(), nbr, ss.to_string()));

        let ss = Status::Launches;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Launches".into(), nbr, ss.to_string()));

        let ss = Status::MajorBugs;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Major bugs".into(), nbr, ss.to_string()));

        let ss = Status::MediumImpact;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Medium impact".into(), nbr, ss.to_string()));

        let ss = Status::MinorBugs;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Minor bugs".into(), nbr, ss.to_string()));

        let ss = Status::Completable;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Completable".into(), nbr, ss.to_string()));

        let ss = Status::Perfect;
        filter.set_status(&ss);
        let nbr = self.games.search_game_by_filter(&st, &filter).count;
        status_stats.push(("Perfect".into(), nbr, ss.to_string()));

        let mut engine_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_engines_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        engine_stats.sort_by(|a, b| a.1.cmp(&b.1));
        engine_stats.reverse();
        engine_stats.truncate(15);

        let mut runtime_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_runtimes_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        runtime_stats.sort_by(|a, b| a.1.cmp(&b.1));
        runtime_stats.reverse();
        runtime_stats.truncate(15);

        let mut genre_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_genres_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        genre_stats.sort_by(|a, b| a.1.cmp(&b.1));
        genre_stats.reverse();
        genre_stats.truncate(15);

        let mut tag_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_tags_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        tag_stats.sort_by(|a, b| a.1.cmp(&b.1));
        tag_stats.reverse();
        tag_stats.truncate(15);

        let mut year_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_years_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        year_stats.sort_by(|a, b| a.1.cmp(&b.1));
        year_stats.reverse();
        year_stats.truncate(15);
        year_stats.sort_by(|a, b| a.0.cmp(&b.0));
        year_stats.reverse();

        let mut dev_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_devs_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        dev_stats.sort_by(|a, b| a.1.cmp(&b.1));
        dev_stats.reverse();
        dev_stats.truncate(15);

        let mut publi_stats: Vec<(String, usize, String)> = self
            .games
            .get_all_publis_with_ids()
            .iter()
            .map(|x| (x.0.clone(), x.1.len(), x.0.clone()))
            .collect();
        publi_stats.sort_by(|a, b| a.1.cmp(&b.1));
        publi_stats.reverse();
        publi_stats.truncate(15);

        self.stats = GameStats {
            engine_stats,
            runtime_stats,
            genre_stats,
            tag_stats,
            year_stats,
            dev_stats,
            publi_stats,
            status_stats,
            total_games: self.games.get_all_games().count,
        }
    }
}
