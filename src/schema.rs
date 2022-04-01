table! {
    conference (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    player (id) {
        id -> Int4,
        steam_id -> Int4,
        name -> Varchar,
    }
}

table! {
    roster (id) {
        id -> Int4,
        season_id -> Int4,
        team_id -> Int4,
    }
}

table! {
    rosterplayer (player_id, roster_id) {
        player_id -> Int4,
        roster_id -> Int4,
    }
}

table! {
    season (id) {
        id -> Int4,
        conference_id -> Int4,
        name -> Varchar,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
    }
}

table! {
    team (id) {
        id -> Int4,
        conference_id -> Int4,
        name -> Varchar,
        flag_url -> Nullable<Varchar>,
    }
}

table! {
    timeframe (id) {
        id -> Int4,
        season_id -> Int4,
        open -> Bool,
        close_date -> Timestamptz,
        name -> Varchar,
    }
}

joinable!(roster -> season (season_id));
joinable!(roster -> team (team_id));
joinable!(rosterplayer -> player (player_id));
joinable!(rosterplayer -> roster (roster_id));
joinable!(season -> conference (conference_id));
joinable!(team -> conference (conference_id));
joinable!(timeframe -> season (season_id));

allow_tables_to_appear_in_same_query!(
    conference,
    player,
    roster,
    rosterplayer,
    season,
    team,
    timeframe,
);
