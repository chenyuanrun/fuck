pub mod utils;

pub trait FuckTo<T> {
    unsafe fn fuck_to(self) -> T;
}

impl<'a, T> FuckTo<&'a T> for *const T {
    unsafe fn fuck_to(self) -> &'a T {
        &*self
    }
}

impl<'a, T> FuckTo<&'a mut T> for *mut T {
    unsafe fn fuck_to(self) -> &'a mut T {
        &mut *self
    }
}

pub trait AddrOf {
    fn addr_of(&self) -> *const Self {
        std::ptr::addr_of!(*self)
    }
}

impl<T> AddrOf for T {}

pub trait AddrOfMut {
    fn addr_of_mut(&mut self) -> *mut Self {
        std::ptr::addr_of_mut!(*self)
    }
}

impl<T> AddrOfMut for T {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuck_to() {
        struct FooData {
            val: i32,
        }
        let data = FooData { val: 999 };
        let mut data_mut = FooData { val: 888 };

        unsafe {
            assert_eq!(data.addr_of().fuck_to().val, 999);
            assert_eq!(data_mut.addr_of_mut().fuck_to().val, 888);
        }
    }
}
