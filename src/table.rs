use cursive::align::HAlign;
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Name,
    Artist,
    Album,
}

impl BasicColumn {
    fn _as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Artist => "Artist",
            BasicColumn::Album => "Album",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Track {
    pub artist: String,
    pub name: String,
    pub album: String,
    pub uri: String,
}

impl TableViewItem<BasicColumn> for Track {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Artist => self.artist.to_string(),
            BasicColumn::Album => self.album.to_string(),
        }
    }

    // Allow the user to change the order of the table
    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Artist => self.artist.cmp(&other.artist),
            BasicColumn::Album => self.album.cmp(&other.album),
        }
    }
}

pub fn build_tracks_table() -> TableView<Track, BasicColumn> {
    TableView::<Track, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(30))
        .column(BasicColumn::Artist, "Artist", |c| c.align(HAlign::Center))
        .column(BasicColumn::Album, "Album", |c| {
            c.align(HAlign::Right).width_percent(30)
        })
}
