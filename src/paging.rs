use super::handlers::common_key_events;
use crate::app::App;
use crate::event::Key;
use crate::network::IoEvent;
use rspotify::model::{
  artist::FullArtist,
  page::{CursorBasedPage, Page},
  show::SimplifiedEpisode,
  track::SavedTrack,
};

pub trait PageAdapter<T: Clone> {
  fn next(&self) -> Option<String>;

  fn items(&self) -> &[T];
}

impl<T: Clone> PageAdapter<T> for Page<T> {
  fn next(&self) -> Option<String> {
    self.next.clone()
  }

  fn items(&self) -> &[T] {
    &self.items
  }
}

impl<T: Clone> PageAdapter<T> for CursorBasedPage<T> {
  fn next(&self) -> Option<String> {
    self.cursors.after.clone()
  }

  fn items(&self) -> &[T] {
    &self.items
  }
}

pub trait Pageable {
  fn get_dispatch(next: Option<String>, offset: u32) -> Option<IoEvent>;
}

type SavedArtist = FullArtist;

impl Pageable for SavedTrack {
  fn get_dispatch(next: Option<String>, offset: u32) -> Option<IoEvent> {
    if let Some(_next_uri) = next {
      Some(IoEvent::GetCurrentSavedTracks(Some(offset)))
    } else {
      None
    }
  }
}

impl Pageable for SavedArtist {
  fn get_dispatch(after: Option<String>, _offset: u32) -> Option<IoEvent> {
    if let Some(after) = after {
      Some(IoEvent::GetFollowedArtists(Some(after)))
    } else {
      None
    }
  }
}

impl Pageable for SimplifiedEpisode {
  fn get_dispatch(next: Option<String>, offset: u32) -> Option<IoEvent> {
    if let Some(_next) = next {
      Some(IoEvent::GetShowEpisodes(None, offset))
    } else {
      None
    }
  }
}

/// This struct will hold paged results from the Spotify API. The idea is to collect
#[derive(Default)]
pub struct NewScrollableResultPages<T> {
  pub items: Vec<T>,
  next: Option<String>,
  pub selected_index: usize,
}

impl<T: Pageable + Clone> NewScrollableResultPages<T> {
  pub fn new() -> Self {
    NewScrollableResultPages {
      selected_index: 0,
      items: vec![],
      next: None,
    }
  }

  pub fn dispatch(&self, app: &mut App) {
    if let Some(event) = T::get_dispatch(self.next.clone(), self.items.len() as u32) {
      app.dispatch(event);
    }
  }

  pub fn add_page(&mut self, page: &dyn PageAdapter<T>) {
    self.items.extend_from_slice(page.items());
    self.next = page.next();
  }

  pub fn handle_common_key_event(&mut self, key: Key) {
    match key {
      k if common_key_events::down_event(k) => {
        // TODO: move this into a function to handle getting new pages when needed
        eprintln!("{} {}", self.selected_index, self.items.len());
        self.selected_index = if self.items.len() > 0 {
          (self.selected_index + 1) % self.items.len()
        } else {
          0
        }
      }
      k if common_key_events::up_event(k) => {
        // TODO: Move this into a function to handle getting new pages when needed
        self.selected_index = self
          .selected_index
          .checked_sub(1)
          .unwrap_or(self.items.len() - 1)
      }
      k if common_key_events::high_event(k) => {
        // TODO: jump to the top of the ui view, not the top of the list
        self.selected_index = 0;
      }
      k if common_key_events::middle_event(k) => {
        // TODO: jump to the center of the ui view, not the center of the list
        let num_items = self.items.len();
        self.selected_index = num_items / 2;
        if num_items % 2 == 0 {
            self.selected_index -= 1;
        }
      }
      k if common_key_events::low_event(k) => {
        // TODO: jump to the bottom of the ui view, not the list
        self.selected_index = self.items.len() - 1;
      }
      _ => {}
    }
  }
}
