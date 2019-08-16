use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Artist,
    Album,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
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
}

impl TableViewItem<BasicColumn> for Track {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Artist => format!("{}", self.artist),
            BasicColumn::Album => format!("{}", self.album),
        }
    }

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

pub fn build_tracks_table(siv: &mut Cursive, items: Vec<Track>) {
    let mut table = TableView::<Track, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(20))
        .column(BasicColumn::Artist, "Artist", |c| c.align(HAlign::Center))
        .column(BasicColumn::Album, "Album", |c| {
            c.align(HAlign::Right).width_percent(20)
        });

    table.set_items(items);
    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_id("table", move |table: &mut TableView<Track, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row))
                .button("Close", move |s| {
                    s.call_on_id("table", |table: &mut TableView<Track, BasicColumn>| {
                        table.remove_item(index);
                    });
                    s.pop_layer();
                }),
        );
    });

    siv.add_layer(Dialog::around(table.with_id("table").min_size((50, 20))).title("Table View"));
}
