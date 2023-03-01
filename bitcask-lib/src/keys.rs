use std::{collections, iter};

use crate::keydir_entry::KeydirEntry;

type KeydirEntryKeys<'a> = collections::hash_map::Keys<'a, String, KeydirEntry>;

pub struct Keys<'a>(KeydirEntryKeys<'a>);

impl<'a> Keys<'a> {
    pub fn new(keys: KeydirEntryKeys<'a>) -> Self {
        Self(keys)
    }
}

impl<'a> iter::Iterator for Keys<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
