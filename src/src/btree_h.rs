use super::*;
use crate::src::btree::{Btree, Node};

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Value {
    pub(crate) value: *mut u8,
    pub(crate) len: u64,
}

pub(crate) type ValueT = Value;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtreeKey {
    pub(crate) key: *mut u8,
    pub(crate) len: u64,
}

pub(crate) type BtreeKeyT = BtreeKey;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Entry {
    pub(crate) key: BtreeKeyT,
    pub(crate) value: ValueT,
}

pub(crate) type EntryT = Entry;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct EntryList {
    pub(crate) entries: *mut EntryT,
    pub(crate) len: u64,
    pub(crate) cap: u64,
}

pub(crate) type EntryListT = EntryList;

pub(crate) type NodeT = Node;

pub(crate) type BtreeT = Btree;
