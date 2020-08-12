use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use std::collections::BTreeMap;

type MovieId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

    clients: RemoteData<BTreeMap<MovieId, Movie>>,
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

pub struct Movie {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    MoviesFetched(fetch::Result<BTreeMap<MovieId, Movie>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,
    
    AddMovie,
    DeleteMovie(MovieId),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::MoviesFetched(Ok(movies)) => {},
        Msg::MoviesFetched(Err(fetch_error)) => {},

        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},

        Msg::ClearErrors => {},

        // ------ Client ------

        Msg::AddMovie => {},
        Msg::DeleteMovie(movie_id) => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["Movies view"]
}
