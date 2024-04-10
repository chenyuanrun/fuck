use std::fmt::Debug;

use crate::{AddrOf, FuckTo};

pub struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

unsafe impl Sync for ListHead {}
unsafe impl Send for ListHead {}

impl Clone for ListHead {
    fn clone(&self) -> Self {
        ListHead {
            next: self.next,
            prev: self.prev,
        }
    }
}

impl Debug for ListHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("ListHead ({:#x})", self.addr_of() as usize))
            .field("next", &format!("{:#x}", self.next as usize))
            .field("prev", &format!("{:#x}", self.prev as usize))
            .finish()
    }
}

impl ListHead {
    pub const fn null() -> Self {
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }

    pub unsafe fn init(this: *mut ListHead) {
        this.fuck_to().next = this;
        this.fuck_to().prev = this;
    }

    pub unsafe fn iter(this: *mut ListHead) -> ListIter {
        ListIter {
            head: this,
            cur: this.fuck_to().next,
        }
    }

    pub unsafe fn add(head: *mut ListHead, new: *mut ListHead) {
        new.fuck_to().next = head.fuck_to().next;
        new.fuck_to().prev = head;

        head.fuck_to().next.fuck_to().prev = new;
        head.fuck_to().next = new;
    }

    pub unsafe fn add_tail(head: *mut ListHead, new: *mut ListHead) {
        new.fuck_to().next = head;
        new.fuck_to().prev = head.fuck_to().prev;

        head.fuck_to().prev.fuck_to().next = new;
        head.fuck_to().prev = new;
    }

    pub unsafe fn empty(head: *mut ListHead) -> bool {
        if head.fuck_to().next == head {
            true
        } else {
            false
        }
    }

    pub unsafe fn del(entry: *mut ListHead) {
        entry.fuck_to().prev.fuck_to().next = entry.fuck_to().next;
        entry.fuck_to().next.fuck_to().prev = entry.fuck_to().prev;

        ListHead::init(entry);
    }
}

pub struct ListIter {
    head: *mut ListHead,
    cur: *mut ListHead,
}

impl Iterator for ListIter {
    type Item = *mut ListHead;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.head {
            return None;
        }
        let res = self.cur;
        self.cur = unsafe { self.cur.fuck_to().next };
        Some(res)
    }
}

#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $container:ty, $($fields:expr)+ $(,)?) => {
        $ptr.byte_sub(std::mem::offset_of!($container, $($fields)+)) as *mut $container
    };
}

#[macro_export]
macro_rules! list_for_each_entry {
    ($container:ty, $head:expr, $($fields:expr)+, |$entry:ident| => $st:stmt) => {
        for _item in ListHead::iter($head) {
            let $entry = container_of!(_item, $container, $($fields)+);
            $st
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::AddrOfMut;

    #[test]
    fn test_linked_list() {
        let mut head: ListHead = ListHead::null();
        unsafe { ListHead::init(head.addr_of_mut()) };
        println!("{:?}", head);

        struct Foo {
            val: usize,
            list: ListHead,
        }

        let mut foo = Foo {
            val: 1024,
            list: ListHead::null(),
        };
        unsafe { ListHead::init(foo.list.addr_of_mut()) };
        println!("{:?}", foo.list);

        let foo_ptr = unsafe { container_of!((foo.list.addr_of_mut()), Foo, list) };
        assert_eq!(foo_ptr, (foo.addr_of_mut()));

        unsafe { ListHead::add(head.addr_of_mut(), foo.list.addr_of_mut()) };
        println!("head: {:?}", head);
        println!("foo.list: {:?}", foo.list);

        let mut item_count = 0;

        unsafe {
            list_for_each_entry!(Foo, head.addr_of_mut(), list, |item| => {
                assert_eq!(item.fuck_to().val, 1024);
                item_count += 1;
            });
        }

        assert_eq!(item_count, 1);
    }
}
