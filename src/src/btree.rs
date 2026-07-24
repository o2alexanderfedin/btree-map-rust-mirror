use super::*;
use crate::{
    __builtin___memcpy_chk, __builtin___memset_chk, __builtin_object_size,
    free, malloc, memcmp,
};
use crate::src::btree_h::{BtreeT, EntryListT, EntryT, NodeT, ValueT};

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Node {
    pub(crate) key_hash: u32,
    pub(crate) p_key: [u8; 10],
    pub(crate) key_len: u64,
    pub(crate) value: ValueT,
    pub(crate) child_left: *mut NodeT,
    pub(crate) child_right: *mut NodeT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Btree {
    pub(crate) node: *mut NodeT,
}

pub(crate) extern "C" fn btree_malloc(size: u64) -> *mut () {
    let ptr: *mut () = unsafe { malloc(size) };
    unsafe {
        __builtin___memset_chk(ptr, 0, size,
            unsafe { __builtin_object_size(ptr as *const (), 0) })
    };
    return ptr;
}

pub(crate) extern "C" fn new_btree() -> *mut BtreeT {
    let bt: *mut BtreeT =
        btree_malloc(core::mem::size_of::<BtreeT>() as u64) as *mut BtreeT;
    unsafe { (*bt).node = 0 as *mut () as *mut NodeT };
    return bt;
}

pub(crate) extern "C" fn min_size(a: u64, b: u64) -> u64 {
    if a < b { return a; }
    return b;
}

pub(crate) extern "C" fn calc_key_hash(key: *mut (), key_len: u64) -> u32 {
    let mut key_sum: u32 = 0 as u32;
    let byte_key: *const u8 = key as *mut u8 as *const u8;
    {
        let mut i: u64 = 0 as u64;
        '__b0: loop {
            if !(i < key_len) { break '__b0; }
            '__c0: loop {
                key_sum = key_sum % 4294967295u32;
                key_sum =
                    key_sum.wrapping_add(((unsafe { *byte_key.add(i as usize) }
                                        as u64).wrapping_mul(i.wrapping_add(1 as u64)) %
                                4294967295u32 as u64) as u32);
                break '__c0;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    return key_sum;
}

pub(crate) extern "C" fn new_node(key: *mut (), key_len: u64, value: *mut (),
    value_len: u64) -> *mut NodeT {
    let node: *mut NodeT =
        btree_malloc(core::mem::size_of::<NodeT>() as u64) as *mut NodeT;
    unsafe { (*node).key_len = min_size(10 as u64, key_len) };
    unsafe {
        __builtin___memcpy_chk(unsafe { &raw mut (*node).p_key[0 as usize] }
                    as *mut u8 as *mut (), key as *const (),
            unsafe { (*node).key_len },
            unsafe {
                __builtin_object_size(unsafe {
                                &raw mut (*node).p_key[0 as usize]
                            } as *mut u8 as *const (), 0)
            })
    };
    unsafe { (*node).value.value = btree_malloc(value_len) as *mut u8 };
    unsafe { (*node).value.len = value_len };
    unsafe {
        __builtin___memcpy_chk(unsafe { (*node).value.value } as *mut (),
            value as *const (), unsafe { (*node).value.len },
            unsafe {
                __builtin_object_size(unsafe { (*node).value.value } as
                        *const (), 0)
            })
    };
    unsafe {
        (*node).key_hash =
            calc_key_hash(unsafe { &raw mut (*node).p_key[0 as usize] } as
                        *mut u8 as *mut (), unsafe { (*node).key_len })
    };
    return node;
}

pub(crate) extern "C" fn find_value(self__1: &mut NodeT, key_hash: u32,
    key: *mut (), mut key_len: u64) -> *mut ValueT {
    key_len = min_size(10 as u64, key_len);
    if (*self__1).key_hash == key_hash {
        if unsafe {
                    memcmp(&raw mut (*self__1).p_key[0 as usize] as *mut u8 as
                            *const (), key as *const (), key_len)
                } == 0 {
            return &mut (*self__1).value;
        }
    }
    if key_hash > (*self__1).key_hash {
        if (*self__1).child_right as *mut () == 0 as *mut () {
            return 0 as *mut () as *mut ValueT;
        }
        return find_value(unsafe { &mut *(*self__1).child_right }, key_hash,
                key, key_len);
    }
    if (*self__1).child_left as *mut () == 0 as *mut () {
        return 0 as *mut () as *mut ValueT;
    }
    return find_value(unsafe { &mut *(*self__1).child_left }, key_hash, key,
            key_len);
}

pub(crate) extern "C" fn find_entry(self_: &BtreeT, key: *mut (),
    mut key_len: u64) -> *mut ValueT {
    if (*self_).node as *mut () == 0 as *mut () {
        return 0 as *mut () as *mut ValueT;
    }
    key_len = min_size(10 as u64, key_len);
    let key_hash: u32 = calc_key_hash(key, key_len);
    return find_value(unsafe { &mut *(*self_).node }, key_hash, key, key_len);
}

pub(crate) extern "C" fn btree_free(mut ptr: *mut ()) -> () {
    unsafe { free(ptr) };
    ptr = 0 as *mut ();
}

pub(crate) extern "C" fn free_node(self__1: *mut NodeT) -> () {
    if self__1 as *mut () == 0 as *mut () { return; }
    btree_free(unsafe { (*self__1).value.value } as *mut ());
    free_node(unsafe { (*self__1).child_left });
    free_node(unsafe { (*self__1).child_right });
    btree_free(self__1 as *mut ());
}

pub(crate) extern "C" fn add_node(self__1: &mut NodeT, n_node: *mut NodeT)
    -> () {
    if unsafe { (*n_node).key_hash } > (*self__1).key_hash {
        if (*self__1).child_right as *mut () == 0 as *mut () {
            (*self__1).child_right = n_node;
            return;
        }
        add_node(unsafe { &mut *(*self__1).child_right }, n_node);
        return;
    }
    if unsafe { (*n_node).key_hash } == (*self__1).key_hash {
        if unsafe {
                    memcmp(&raw mut (*self__1).p_key[0 as usize] as *mut u8 as
                            *const (),
                        unsafe { &raw mut (*n_node).p_key[0 as usize] } as *mut u8
                            as *const (), unsafe { (*n_node).key_len })
                } == 0 {
            (*self__1).value.len = unsafe { (*n_node).value.len };
            unsafe {
                __builtin___memcpy_chk((*self__1).value.value as *mut (),
                    unsafe { (*n_node).value.value } as *const (),
                    unsafe { (*n_node).value.len },
                    unsafe {
                        __builtin_object_size((*self__1).value.value as *const (),
                            0)
                    })
            };
            free_node(n_node);
            return;
        }
    }
    if (*self__1).child_left as *mut () == 0 as *mut () {
        (*self__1).child_left = n_node;
        return;
    }
    add_node(unsafe { &mut *(*self__1).child_left }, n_node);
    return;
}

pub(crate) extern "C" fn add_entry(self_: &mut BtreeT, key: *mut (),
    key_len: u64, value: *mut (), value_len: u64) -> () {
    let n_node: *mut NodeT = new_node(key, key_len, value, value_len);
    if (*self_).node as *mut () == 0 as *mut () {
        (*self_).node = n_node;
        return;
    }
    add_node(unsafe { &mut *(*self_).node }, n_node);
}

pub(crate) extern "C" fn free_tree(self_: &mut *mut BtreeT) -> () {
    free_node(unsafe { (**self_).node });
    btree_free(*self_ as *mut ());
    *self_ = 0 as *mut () as *mut BtreeT;
    return;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn delete_node(root: *mut NodeT, key_hash: u32,
    key: *mut (), mut key_len: u64) -> *mut NodeT {
    if root as *mut () == 0 as *mut () { return 0 as *mut () as *mut NodeT; }
    key_len = min_size(10 as u64, key_len);
    if key_hash < unsafe { (*root).key_hash } {
        unsafe {
            (*root).child_left =
                delete_node(unsafe { (*root).child_left }, key_hash, key,
                    key_len)
        };
    } else if key_hash > unsafe { (*root).key_hash } {
        unsafe {
            (*root).child_right =
                delete_node(unsafe { (*root).child_right }, key_hash, key,
                    key_len)
        };
    } else {
        if unsafe {
                    memcmp(unsafe { &raw mut (*root).p_key[0 as usize] } as
                                *mut u8 as *const (), key as *const (), key_len)
                } == 0 {
            if unsafe { (*root).child_left } as *mut () == 0 as *mut () {
                let mut temp: *mut NodeT = unsafe { (*root).child_right };
                btree_free(root as *mut ());
                return temp;
            } else if unsafe { (*root).child_right } as *mut () ==
                    0 as *mut () {
                let mut temp: *mut NodeT = unsafe { (*root).child_left };
                btree_free(root as *mut ());
                return temp;
            }
            /// Node with two children
            let mut temp: *mut NodeT = unsafe { (*root).child_right };
            while !(temp).is_null() &&
                    unsafe { (*temp).child_left } as *mut () != 0 as *mut () {
                temp = unsafe { (*temp).child_left };
            }

            /// Copy the inorder successor's content to this node
            unsafe { (*root).key_hash = unsafe { (*temp).key_hash } };
            unsafe {
                __builtin___memcpy_chk(unsafe {
                                &raw mut (*root).p_key[0 as usize]
                            } as *mut u8 as *mut (),
                    unsafe { &raw mut (*temp).p_key[0 as usize] } as *mut u8 as
                        *const (), unsafe { (*temp).key_len },
                    unsafe {
                        __builtin_object_size(unsafe {
                                        &raw mut (*root).p_key[0 as usize]
                                    } as *mut u8 as *const (), 0)
                    })
            };
            unsafe { (*root).key_len = unsafe { (*temp).key_len } };
            unsafe {
                __builtin___memcpy_chk(unsafe { (*root).value.value } as
                        *mut (), unsafe { (*temp).value.value } as *const (),
                    unsafe { (*temp).value.len },
                    unsafe {
                        __builtin_object_size(unsafe { (*root).value.value } as
                                *const (), 0)
                    })
            };
            unsafe { (*root).value.len = unsafe { (*temp).value.len } };

            /// Delete the inorder successor
            unsafe {
                (*root).child_right =
                    delete_node(unsafe { (*root).child_right },
                        unsafe { (*temp).key_hash },
                        unsafe { &raw mut (*temp).p_key[0 as usize] } as *mut u8 as
                            *mut (), unsafe { (*temp).key_len })
            };
        }
    }
    return root;
}

pub(crate) extern "C" fn remove_entry(self_: *mut BtreeT, key: *mut (),
    mut key_len: u64) -> () {
    if self_ as *mut () == 0 as *mut () { return; }
    key_len = min_size(10 as u64, key_len);
    let key_hash: u32 = calc_key_hash(key, key_len);
    unsafe {
        (*self_).node =
            delete_node(unsafe { (*self_).node }, key_hash, key, key_len)
    };
}

pub(crate) extern "C" fn get_node_count(self__1: *mut NodeT) -> u64 {
    if self__1 as *mut () == 0 as *mut () { return 0 as u64; }
    return (1 as
                    u64).wrapping_add(get_node_count(unsafe {
                        (*self__1).child_left
                    })).wrapping_add(get_node_count(unsafe {
                    (*self__1).child_right
                }));
}

pub(crate) extern "C" fn get_entry_count(self_: *mut BtreeT) -> u64 {
    if self_ as *mut () == 0 as *mut () { return 0 as u64; }
    return get_node_count(unsafe { (*self_).node });
}

pub(crate) extern "C" fn list_node_entries(self__1: *mut NodeT,
    list: *mut EntryListT) -> () {
    if self__1 as *mut () == 0 as *mut () { return; }
    list_node_entries(unsafe { (*self__1).child_left }, list);
    if unsafe { (*list).len } >= unsafe { (*list).cap } { return; }
    let entry: *mut EntryT =
        btree_malloc(core::mem::size_of::<EntryT>() as u64) as *mut EntryT;
    unsafe {
        (*entry).key.key =
            btree_malloc(unsafe { (*self__1).key_len }) as *mut u8
    };
    unsafe { (*entry).key.len = unsafe { (*self__1).key_len } };
    unsafe {
        __builtin___memcpy_chk(unsafe { (*entry).key.key } as *mut (),
            unsafe { &raw mut (*self__1).p_key[0 as usize] } as *mut u8 as
                *const (), unsafe { (*self__1).key_len },
            unsafe {
                __builtin_object_size(unsafe { (*entry).key.key } as
                        *const (), 0)
            })
    };
    unsafe {
        (*entry).value.value =
            btree_malloc(unsafe { (*self__1).value.len }) as *mut u8
    };
    unsafe { (*entry).value.len = unsafe { (*self__1).value.len } };
    unsafe {
        __builtin___memcpy_chk(unsafe { (*entry).value.value } as *mut (),
            unsafe { (*self__1).value.value } as *const (),
            unsafe { (*self__1).value.len },
            unsafe {
                __builtin_object_size(unsafe { (*entry).value.value } as
                        *const (), 0)
            })
    };
    unsafe {
        *unsafe { (*list).entries.add(unsafe { (*list).len } as usize) } =
            unsafe { core::ptr::read(entry) }
    };
    unsafe { (*list).len += 1 as u64 };
    list_node_entries(unsafe { (*self__1).child_right }, list);
}

pub(crate) extern "C" fn list_entries(self_: *mut BtreeT) -> *mut EntryListT {
    if self_ as *mut () == 0 as *mut () {
        return 0 as *mut () as *mut EntryListT;
    }
    let list: *mut EntryListT =
        btree_malloc(core::mem::size_of::<EntryListT>() as u64) as
            *mut EntryListT;
    unsafe { (*list).len = 0 as u64 };
    unsafe { (*list).cap = get_entry_count(self_) };
    unsafe {
        (*list).entries =
            btree_malloc((core::mem::size_of::<EntryT>() as
                            u64).wrapping_mul(unsafe { (*list).cap })) as *mut EntryT
    };
    list_node_entries(unsafe { (*self_).node }, list);
    return list;
}

pub(crate) extern "C" fn free_entry_list(list: &mut *mut EntryListT) -> () {
    {
        let mut i: u64 = 0 as u64;
        '__b2: loop {
            if !(i < unsafe { (**list).len }) { break '__b2; }
            '__c2: loop {
                btree_free(unsafe {
                            (*unsafe { (**list).entries.add(i as usize) }).key.key
                        } as *mut ());
                btree_free(unsafe {
                            (*unsafe { (**list).entries.add(i as usize) }).value.value
                        } as *mut ());
                break '__c2;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    btree_free(unsafe { (**list).entries } as *mut ());
    btree_free(*list as *mut ());
    *list = 0 as *mut () as *mut EntryListT;
}
