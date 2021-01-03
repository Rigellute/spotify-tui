mod album_list;
mod album_tracks;
mod analysis;
mod artist;
mod artists;
mod basic_view;
mod common_key_events;
mod dialog;
mod empty;
mod episode_table;
mod error_screen;
mod help_menu;
mod home;
mod input;
mod library;
mod made_for_you;
mod playbar;
mod playlist;
mod podcasts;
mod recently_played;
mod search_results;
mod select_device;
mod track_table;

use super::app::{ActiveBlock, App, ArtistBlock, RouteId, SearchResultBlock};
use crate::event::Key;
use crate::network::IoEvent;
use rspotify::model::{context::CurrentlyPlaybackContext, PlayingItem};

pub use input::handler as input_handler;

// takes a Key struct and returns the matching key according to the keymap.
fn remap_key(key: Key, app: &mut App) -> Key {
  match key {
    _ if key == app.user_config.keymap.q => Key::Char('q'),
    _ if key == app.user_config.keymap.w => Key::Char('w'),
    _ if key == app.user_config.keymap.e => Key::Char('e'),
    _ if key == app.user_config.keymap.r => Key::Char('r'),
    _ if key == app.user_config.keymap.t => Key::Char('t'),
    _ if key == app.user_config.keymap.y => Key::Char('y'),
    _ if key == app.user_config.keymap.u => Key::Char('u'),
    _ if key == app.user_config.keymap.i => Key::Char('i'),
    _ if key == app.user_config.keymap.o => Key::Char('o'),
    _ if key == app.user_config.keymap.p => Key::Char('p'),
    _ if key == app.user_config.keymap.a => Key::Char('a'),
    _ if key == app.user_config.keymap.s => Key::Char('s'),
    _ if key == app.user_config.keymap.d => Key::Char('d'),
    _ if key == app.user_config.keymap.f => Key::Char('f'),
    _ if key == app.user_config.keymap.g => Key::Char('g'),
    _ if key == app.user_config.keymap.h => Key::Char('h'),
    _ if key == app.user_config.keymap.j => Key::Char('j'),
    _ if key == app.user_config.keymap.k => Key::Char('k'),
    _ if key == app.user_config.keymap.l => Key::Char('l'),
    _ if key == app.user_config.keymap.z => Key::Char('z'),
    _ if key == app.user_config.keymap.x => Key::Char('x'),
    _ if key == app.user_config.keymap.c => Key::Char('c'),
    _ if key == app.user_config.keymap.v => Key::Char('v'),
    _ if key == app.user_config.keymap.b => Key::Char('b'),
    _ if key == app.user_config.keymap.n => Key::Char('n'),
    _ if key == app.user_config.keymap.m => Key::Char('m'),
    _ if key == app.user_config.keymap.Q => Key::Char('Q'),
    _ if key == app.user_config.keymap.W => Key::Char('W'),
    _ if key == app.user_config.keymap.E => Key::Char('E'),
    _ if key == app.user_config.keymap.R => Key::Char('R'),
    _ if key == app.user_config.keymap.T => Key::Char('T'),
    _ if key == app.user_config.keymap.Y => Key::Char('Y'),
    _ if key == app.user_config.keymap.U => Key::Char('U'),
    _ if key == app.user_config.keymap.I => Key::Char('I'),
    _ if key == app.user_config.keymap.O => Key::Char('O'),
    _ if key == app.user_config.keymap.P => Key::Char('P'),
    _ if key == app.user_config.keymap.A => Key::Char('A'),
    _ if key == app.user_config.keymap.S => Key::Char('S'),
    _ if key == app.user_config.keymap.D => Key::Char('D'),
    _ if key == app.user_config.keymap.F => Key::Char('F'),
    _ if key == app.user_config.keymap.G => Key::Char('G'),
    _ if key == app.user_config.keymap.H => Key::Char('H'),
    _ if key == app.user_config.keymap.J => Key::Char('J'),
    _ if key == app.user_config.keymap.K => Key::Char('K'),
    _ if key == app.user_config.keymap.L => Key::Char('L'),
    _ if key == app.user_config.keymap.Z => Key::Char('Z'),
    _ if key == app.user_config.keymap.X => Key::Char('X'),
    _ if key == app.user_config.keymap.C => Key::Char('C'),
    _ if key == app.user_config.keymap.V => Key::Char('V'),
    _ if key == app.user_config.keymap.B => Key::Char('B'),
    _ if key == app.user_config.keymap.N => Key::Char('N'),
    _ if key == app.user_config.keymap.M => Key::Char('M'),
    _ => key,
  }
}

pub fn handle_app(key: Key, app: &mut App) {
  let remapped_key = remap_key(key, app);
  // First handle any global event and then move to block event
  match remapped_key {
    Key::Esc => {
      handle_escape(app);
    }
    _ if remapped_key == app.user_config.keys.jump_to_album => {
      handle_jump_to_album(app);
    }
    _ if remapped_key == app.user_config.keys.jump_to_artist_album => {
      handle_jump_to_artist_album(app);
    }
    _ if remapped_key == app.user_config.keys.jump_to_context => {
      handle_jump_to_context(app);
    }
    _ if remapped_key == app.user_config.keys.manage_devices => {
      app.dispatch(IoEvent::GetDevices);
    }
    _ if remapped_key == app.user_config.keys.decrease_volume => {
      app.decrease_volume();
    }
    _ if remapped_key == app.user_config.keys.increase_volume => {
      app.increase_volume();
    }
    // Press space to toggle playback
    _ if remapped_key == app.user_config.keys.toggle_playback => {
      app.toggle_playback();
    }
    _ if remapped_key == app.user_config.keys.seek_backwards => {
      app.seek_backwards();
    }
    _ if remapped_key == app.user_config.keys.seek_forwards => {
      app.seek_forwards();
    }
    _ if remapped_key == app.user_config.keys.next_track => {
      app.dispatch(IoEvent::NextTrack);
    }
    _ if remapped_key == app.user_config.keys.previous_track => {
      app.previous_track();
    }
    _ if remapped_key == app.user_config.keys.help => {
      app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
    }

    _ if remapped_key == app.user_config.keys.shuffle => {
      app.shuffle();
    }
    _ if remapped_key == app.user_config.keys.repeat => {
      app.repeat();
    }
    _ if remapped_key == app.user_config.keys.search => {
      app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
    }
    _ if remapped_key == app.user_config.keys.copy_song_url => {
      app.copy_song_url();
    }
    _ if remapped_key == app.user_config.keys.copy_album_url => {
      app.copy_album_url();
    }
    _ if remapped_key == app.user_config.keys.audio_analysis => {
      app.get_audio_analysis();
    }
    _ if remapped_key == app.user_config.keys.basic_view => {
      app.push_navigation_stack(RouteId::BasicView, ActiveBlock::BasicView);
    }
    _ => handle_block_events(remapped_key, app),
  }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
  let current_route = app.get_current_route();
  match current_route.active_block {
    ActiveBlock::Analysis => {
      analysis::handler(key, app);
    }
    ActiveBlock::ArtistBlock => {
      artist::handler(key, app);
    }
    ActiveBlock::Input => {
      input::handler(key, app);
    }
    ActiveBlock::MyPlaylists => {
      playlist::handler(key, app);
    }
    ActiveBlock::TrackTable => {
      track_table::handler(key, app);
    }
    ActiveBlock::EpisodeTable => {
      episode_table::handler(key, app);
    }
    ActiveBlock::HelpMenu => {
      help_menu::handler(key, app);
    }
    ActiveBlock::Error => {
      error_screen::handler(key, app);
    }
    ActiveBlock::SelectDevice => {
      select_device::handler(key, app);
    }
    ActiveBlock::SearchResultBlock => {
      search_results::handler(key, app);
    }
    ActiveBlock::Home => {
      home::handler(key, app);
    }
    ActiveBlock::AlbumList => {
      album_list::handler(key, app);
    }
    ActiveBlock::AlbumTracks => {
      album_tracks::handler(key, app);
    }
    ActiveBlock::Library => {
      library::handler(key, app);
    }
    ActiveBlock::Empty => {
      empty::handler(key, app);
    }
    ActiveBlock::RecentlyPlayed => {
      recently_played::handler(key, app);
    }
    ActiveBlock::Artists => {
      artists::handler(key, app);
    }
    ActiveBlock::MadeForYou => {
      made_for_you::handler(key, app);
    }
    ActiveBlock::Podcasts => {
      podcasts::handler(key, app);
    }
    ActiveBlock::PlayBar => {
      playbar::handler(key, app);
    }
    ActiveBlock::BasicView => {
      basic_view::handler(key, app);
    }
    ActiveBlock::Dialog(_) => {
      dialog::handler(key, app);
    }
  }
}

fn handle_escape(app: &mut App) {
  match app.get_current_route().active_block {
    ActiveBlock::SearchResultBlock => {
      app.search_results.selected_block = SearchResultBlock::Empty;
    }
    ActiveBlock::ArtistBlock => {
      if let Some(artist) = &mut app.artist {
        artist.artist_selected_block = ArtistBlock::Empty;
      }
    }
    ActiveBlock::Error => {
      app.pop_navigation_stack();
    }
    ActiveBlock::Dialog(_) => {
      app.pop_navigation_stack();
    }
    // These are global views that have no active/inactive distinction so do nothing
    ActiveBlock::SelectDevice | ActiveBlock::Analysis => {}
    _ => {
      app.set_current_route_state(Some(ActiveBlock::Empty), None);
    }
  }
}

fn handle_jump_to_context(app: &mut App) {
  if let Some(current_playback_context) = &app.current_playback_context {
    if let Some(play_context) = current_playback_context.context.clone() {
      match play_context._type {
        rspotify::senum::Type::Album => handle_jump_to_album(app),
        rspotify::senum::Type::Artist => handle_jump_to_artist_album(app),
        rspotify::senum::Type::Playlist => {
          app.dispatch(IoEvent::GetPlaylistTracks(play_context.uri, 0))
        }
        _ => {}
      }
    }
  }
}

fn handle_jump_to_album(app: &mut App) {
  if let Some(CurrentlyPlaybackContext {
    item: Some(item), ..
  }) = app.current_playback_context.to_owned()
  {
    match item {
      PlayingItem::Track(track) => {
        app.dispatch(IoEvent::GetAlbumTracks(Box::new(track.album)));
      }
      PlayingItem::Episode(_episode) => {
        // Do nothing for episode (yet!)
      }
    };
  }
}

// NOTE: this only finds the first artist of the song and jumps to their albums
fn handle_jump_to_artist_album(app: &mut App) {
  if let Some(CurrentlyPlaybackContext {
    item: Some(item), ..
  }) = app.current_playback_context.to_owned()
  {
    match item {
      PlayingItem::Track(track) => {
        if let Some(artist) = track.artists.first() {
          if let Some(artist_id) = artist.id.clone() {
            app.get_artist(artist_id, artist.name.clone());
            app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
          }
        }
      }
      PlayingItem::Episode(_episode) => {
        // Do nothing for episode (yet!)
      }
    }
  };
}
