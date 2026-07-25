use super::*;
use crate::src::btree::{
    add_entry, find_entry, free_entry_list, free_tree, list_entries, new_btree, remove_entry,
};
use crate::src::btree_h::{BtreeT, EntryListT, ValueT};
use crate::{__assert_rtn, memcmp, printf};

pub(crate) extern "C" fn test_add_entry() -> () {
    let mut btree: *mut BtreeT = new_btree();
    add_entry(
        unsafe { &mut *btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let value: *mut ValueT = find_entry(
        unsafe { &*btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_add_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                12,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_add_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                13,
                c"memcmp(value->value, \"value_1\", sizeof(\"value_1\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
}

pub(crate) extern "C" fn test_entry_list() -> () {
    let mut btree: *mut BtreeT = new_btree();
    add_entry(
        unsafe { &mut *btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_2".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_2".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_4".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_4".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_5".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_5".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut list: *mut EntryListT = list_entries(btree);
    if !(unsafe { (*list).len } == 5 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                25,
                c"list->len == 5".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(0 as isize) }).key.key } as *const (),
            c"entry_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                26,
                c"memcmp(list->entries[0].key.key, \"entry_1\", sizeof(\"entry_1\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(0 as isize) }).value.value } as *const (),
            c"value_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                27,
                c"memcmp(list->entries[0].value.value, \"value_1\", sizeof(\"value_1\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(1 as isize) }).key.key } as *const (),
            c"entry_2".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                28,
                c"memcmp(list->entries[1].key.key, \"entry_2\", sizeof(\"entry_2\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(1 as isize) }).value.value } as *const (),
            c"value_2".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                29,
                c"memcmp(list->entries[1].value.value, \"value_2\", sizeof(\"value_2\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(2 as isize) }).key.key } as *const (),
            c"entry_3".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                30,
                c"memcmp(list->entries[2].key.key, \"entry_3\", sizeof(\"entry_3\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(2 as isize) }).value.value } as *const (),
            c"value_3".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                31,
                c"memcmp(list->entries[2].value.value, \"value_3\", sizeof(\"value_3\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(3 as isize) }).key.key } as *const (),
            c"entry_4".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                32,
                c"memcmp(list->entries[3].key.key, \"entry_4\", sizeof(\"entry_4\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(3 as isize) }).value.value } as *const (),
            c"value_4".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                33,
                c"memcmp(list->entries[3].value.value, \"value_4\", sizeof(\"value_4\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(4 as isize) }).key.key } as *const (),
            c"entry_5".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                34,
                c"memcmp(list->entries[4].key.key, \"entry_5\", sizeof(\"entry_5\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(4 as isize) }).value.value } as *const (),
            c"value_5".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_entry_list".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                35,
                c"memcmp(list->entries[4].value.value, \"value_5\", sizeof(\"value_5\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
    free_entry_list(&mut list);
}

pub(crate) extern "C" fn test_remove_entry() -> () {
    let mut btree: *mut BtreeT = new_btree();
    add_entry(
        unsafe { &mut *btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_2".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_2".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_4".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_4".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    add_entry(
        unsafe { &mut *btree },
        c"entry_5".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_5".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    remove_entry(
        btree,
        c"entry_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut list: *mut EntryListT = list_entries(btree);
    if !(unsafe { (*list).len } == 4 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                49,
                c"list->len == 4".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(0 as isize) }).key.key } as *const (),
            c"entry_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                50,
                c"memcmp(list->entries[0].key.key, \"entry_1\", sizeof(\"entry_1\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(0 as isize) }).value.value } as *const (),
            c"value_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                51,
                c"memcmp(list->entries[0].value.value, \"value_1\", sizeof(\"value_1\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(1 as isize) }).key.key } as *const (),
            c"entry_2".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                52,
                c"memcmp(list->entries[1].key.key, \"entry_2\", sizeof(\"entry_2\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(1 as isize) }).value.value } as *const (),
            c"value_2".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                53,
                c"memcmp(list->entries[1].value.value, \"value_2\", sizeof(\"value_2\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(2 as isize) }).key.key } as *const (),
            c"entry_4".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                54,
                c"memcmp(list->entries[2].key.key, \"entry_4\", sizeof(\"entry_4\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(2 as isize) }).value.value } as *const (),
            c"value_4".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                55,
                c"memcmp(list->entries[2].value.value, \"value_4\", sizeof(\"value_4\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(3 as isize) }).key.key } as *const (),
            c"entry_5".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                56,
                c"memcmp(list->entries[3].key.key, \"entry_5\", sizeof(\"entry_5\")) == 0".as_ptr()
                    as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*unsafe { (*list).entries.offset(3 as isize) }).value.value } as *const (),
            c"value_5".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_remove_entry".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                57,
                c"memcmp(list->entries[3].value.value, \"value_5\", sizeof(\"value_5\")) == 0"
                    .as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
    free_entry_list(&mut list);
}

pub(crate) extern "C" fn test_multiple_key_types() -> () {
    let mut btree: *mut BtreeT = new_btree();
    add_entry(
        unsafe { &mut *btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
        c"value_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut int_key: u32 = 1 as u32;
    add_entry(
        unsafe { &mut *btree },
        &raw mut int_key as *mut (),
        core::mem::size_of::<u32>() as u64,
        c"value_2".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut long_key: u64 = 10 as u64;
    add_entry(
        unsafe { &mut *btree },
        &raw mut long_key as *mut (),
        core::mem::size_of::<u64>() as u64,
        c"value_3".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut byte_key: u8 = 9 as u8;
    add_entry(
        unsafe { &mut *btree },
        &raw mut byte_key as *mut (),
        core::mem::size_of::<u8>() as u64,
        c"value_4".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    let mut value: *mut ValueT = find_entry(
        unsafe { &*btree },
        c"entry_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                73,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                74,
                c"memcmp(value->value, \"value_1\", sizeof(\"value_1\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    value = find_entry(
        unsafe { &*btree },
        &raw mut int_key as *mut (),
        core::mem::size_of::<u32>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                77,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_2".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                78,
                c"memcmp(value->value, \"value_2\", sizeof(\"value_2\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    value = find_entry(
        unsafe { &*btree },
        &raw mut long_key as *mut (),
        core::mem::size_of::<u64>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                81,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_3".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                82,
                c"memcmp(value->value, \"value_3\", sizeof(\"value_3\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    value = find_entry(
        unsafe { &*btree },
        &raw mut byte_key as *mut (),
        core::mem::size_of::<u8>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                85,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_4".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_multiple_key_types".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                86,
                c"memcmp(value->value, \"value_4\", sizeof(\"value_4\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn test_add_custom_struct() -> () {
    let mut btree: *mut BtreeT = new_btree();
    let mut key: CustomKeyN10customKey = CustomKeyN10customKey {
        key: 1 as u32,
        key2: 2 as u32,
    };
    add_entry(
        unsafe { &mut *btree },
        &raw mut key as *mut (),
        core::mem::size_of::<CustomKeyT>() as u64,
        c"value_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 8]>() as u64,
    );
    /// Navigate down the tree to find the node to delete
    let value: *mut ValueT = find_entry(
        unsafe { &*btree },
        &raw mut key as *mut (),
        core::mem::size_of::<CustomKeyT>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_add_custom_struct".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                101,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            c"value_1".as_ptr() as *mut i8 as *const (),
            core::mem::size_of::<[i8; 8]>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_add_custom_struct".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                102,
                c"memcmp(value->value, \"value_1\", sizeof(\"value_1\")) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn test_add_custom_struct_to_value() -> () {
    let mut btree: *mut BtreeT = new_btree();
    /// Found the node to delete
    let mut c_value: CustomValueN12customValue = CustomValueN12customValue {
        value: 1 as u32,
        value2: 2 as u32,
    };
    add_entry(
        unsafe { &mut *btree },
        c"key_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 6]>() as u64,
        &raw mut c_value as *mut (),
        core::mem::size_of::<CustomValueT>() as u64,
    );
    let value: *mut ValueT = find_entry(
        unsafe { &*btree },
        c"key_1".as_ptr() as *mut i8 as *mut (),
        core::mem::size_of::<[i8; 6]>() as u64,
    );
    if !(value as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"test_add_custom_struct_to_value".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                116,
                c"value != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(unsafe {
        memcmp(
            unsafe { (*value).value } as *const (),
            &raw mut c_value as *const (),
            core::mem::size_of::<CustomValueT>() as u64,
        )
    } == 0) as i32 as i64
        != 0
    {
        unsafe {
            __assert_rtn(
                c"test_add_custom_struct_to_value".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                117,
                c"memcmp(value->value, &c_value, sizeof(c_value)) == 0".as_ptr() as *mut i8
                    as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    free_tree(&mut btree);
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn __main_inner() -> i32 {
    test_add_entry();
    test_entry_list();
    test_remove_entry();
    test_multiple_key_types();
    test_add_custom_struct();

    /// Node with two children
    test_add_custom_struct_to_value();
    unsafe { printf(c"All tests passed!\n".as_ptr() as *mut i8 as *const i8) };
    return 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CustomKeyN10customKey {
    pub(crate) key: u32,
    pub(crate) key2: u32,
}

pub(crate) type CustomKeyT = CustomKeyN10customKey;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CustomValueN12customValue {
    pub(crate) value: u32,
    pub(crate) value2: u32,
}

pub(crate) type CustomValueT = CustomValueN12customValue;
