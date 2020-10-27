use super::handlers::common_key_events;
use crate::app::{App, UIViewWindow};
use crate::event::Key;
use crate::network::IoEvent;
use rspotify::model::{
  album::SavedAlbum,
  artist::FullArtist,
  page::{CursorBasedPage, Page},
  show::SimplifiedEpisode,
  track::SavedTrack,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

pub type SavedArtist = FullArtist;

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

impl Pageable for SavedAlbum {
  fn get_dispatch(next: Option<String>, offset: u32) -> Option<IoEvent> {
    if let Some(_next) = next {
      Some(IoEvent::GetCurrentUserSavedAlbums(Some(offset)))
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
#[derive(Default, Clone)]
pub struct NewScrollableResultPages<T> {
  pub items: Vec<T>,
  next: Option<String>,
  pub selected_index: usize,
  pub ui_view_height: Option<UIViewWindow>,
  pub fetching_page: Arc<AtomicBool>,
}

impl<T: Pageable + Clone> NewScrollableResultPages<T> {
  pub fn new() -> Self {
    NewScrollableResultPages {
      selected_index: 0,
      items: vec![],
      next: None,
      ui_view_height: None,
      fetching_page: Arc::new(AtomicBool::new(false)),
    }
  }

  pub fn dispatch(&self, app: &App) {
    if !(self.fetching_page.load(Ordering::Relaxed)) {
      self.fetching_page.store(true, Ordering::Relaxed);
      if let Some(event) = T::get_dispatch(self.next.clone(), self.items.len() as u32) {
        app.dispatch(event);
      }
    }
  }

  pub fn add_page(&mut self, page: &dyn PageAdapter<T>) {
    self.items.extend_from_slice(page.items());
    self.next = page.next();
  }

  pub fn get_selected_item(&self) -> Option<&T> {
    self.items.get(self.selected_index)
  }

  pub fn handle_list_navigation_event(&self, key: Key, app: &App) -> usize {
    match key {
      k if common_key_events::down_event(k) => {
        if !self.items.is_empty() {
          (self.selected_index + 1) % self.items.len()
        } else {
          0
        }
      }
      k if common_key_events::up_event(k) => self
        .selected_index
        .checked_sub(1)
        .unwrap_or(self.items.len() - 1),
      k if common_key_events::high_event(k) => self
        .ui_view_height
        .as_ref()
        .map(|v| v.start_index)
        .unwrap_or(self.selected_index),
      k if common_key_events::middle_event(k) => self
        .ui_view_height
        .as_ref()
        .map(|v| (v.start_index + v.height / 2))
        .unwrap_or(self.selected_index)
        .min(self.items.len() - 1),
      k if common_key_events::low_event(k) => self
        .ui_view_height
        .as_ref()
        .map(|v| (v.start_index + v.height))
        .unwrap_or(self.selected_index)
        .min(self.items.len() - 1),
      k if k == app.user_config.keys.next_page => {
        if let Some(window) = &self.ui_view_height {
          if window.height + self.selected_index < self.items.len() {
            window.height + self.selected_index
          } else {
            self.items.len() - 1
          }
        } else {
          self.selected_index
        }
      }
      k if k == app.user_config.keys.previous_page => {
        if let Some(window) = &self.ui_view_height {
          self.selected_index.saturating_sub(window.height)
        } else {
          self.selected_index
        }
      }
      k if common_key_events::list_end_event(k) => self.items.len() - 1,
      k if common_key_events::list_begin_event(k) => 0,
      k if common_key_events::is_list_navigation_key_event(k, app) => {
        unimplemented!("List navigation event {:?} needs an implementation", k)
      }
      _ => unreachable!("This function cannot handle non-navigation key events!"),
    }
  }
}
