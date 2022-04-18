use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self, null_mut},
};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    // [null, null]
    // push_back X
    // [ptr, ptr] -> (X, null)
    // push_back Y
    // [ptr, ptr] -> (X, ptr) -> (Y, null)
    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let new_tail = alloc(layout) as Link<T>;
            ptr::write(
                new_tail,
                Node {
                    elem,
                    next: ptr::null_mut(),
                },
            );

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    // [null, null]
    // push_front X
    // [ptr, ptr] -> (X, null)
    // push_front Y
    // [ptr, ptr] -> (Y, ptr) -> (X, null)
    // push_front Z
    // [ptr, ptr] -> (Z, ptr) -> (Y, ptr) -> (X, null)
    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let new_head = alloc(layout) as Link<T>;

            *new_head = Node {
                elem,
                next: self.head,
            };

            if self.head.is_null() {
                self.tail = new_head;
            }

            self.head = new_head
        }
    }

    // [ptr, ptr] -> (X, ptr) -> (Y, null)
    // pop_front
    // [ptr, ptr] -> (Y, null)
    // pop_front
    // [null, null]
    // pop_front
    // nothing
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let result;

            if self.head.is_null() {
                result = None
            } else {
                let head = ptr::read(self.head);

                result = Some(head.elem);

                dealloc(self.head as *mut u8, layout);

                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
            }

            result
        }
    }

    // [ptr, ptr] -> (X, ptr) -> (Y, ptr) -> (Z, null)
    // pop_back
    // [ptr, ptr] -> (X, ptr) -> (Y, null)
    // pop_back
    // [ptr, ptr] -> (X, null)
    // pop_back
    // [null, null]
    // pop_back
    // nothing
    //
    // This is O(n) for Singly Linked List
    // To make it O(1) we need the scary Doubly Linked List
    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let result;

            if self.tail.is_null() {
                result = None
            } else {
                let mut p = self.head;

                if (*p).next.is_null() {
                    result = Some(ptr::read(p).elem);
                    dealloc(p as *mut u8, layout);

                    self.head = ptr::null_mut();
                    self.tail = ptr::null_mut();
                } else {
                    while !(*(*p).next).next.is_null() {
                        p = (*p).next;
                    }

                    (*p).next = ptr::null_mut();

                    result = Some(ptr::read(self.tail).elem);
                    dealloc(self.tail as *mut u8, layout);

                    self.tail = p;
                }
            }

            result
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.elem) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn pop_back() {
        let mut list = List::new();

        list.push_front('Z');
        list.push_front('Y');
        list.push_front('X');
        assert_eq!(list.pop_back(), Some('Z'));
        assert_eq!(list.pop_back(), Some('Y'));
        assert_eq!(list.pop_back(), Some('X'));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_front(4);

        assert_eq!(list.pop_front(), Some(4));

        // Check normal removal
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);

        // Check the exhaustion case fixed the pointer right
        list.push_back(6);
        list.push_back(7);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert!(list.pop_front() == Some(1));
        list.push_back(4);
        assert!(list.pop_front() == Some(2));
        list.push_back(5);

        assert!(list.front() == Some(&3));
        list.push_back(6);
        list.front_mut().map(|x| *x *= 10);
        assert!(list.front() == Some(&30));
        assert!(list.pop_front() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop_front() == Some(400));
        list.front_mut().map(|x| *x *= 10);
        assert!(list.front() == Some(&5000));
        list.push_back(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
