use std::cell::RefCell;
use std::cell::RefMut;

pub struct Library<T> {
    pub items: Vec<RefCell<T>>
}

impl<T> Library<T> {
    pub fn new(items: Vec<T>) -> Library<T> {
        Library {
            items: items.into_iter().map(|i| RefCell::new(i)).collect()
        }
    }

    pub fn iter(&self) -> SubLibrary<T> 
    {
        SubLibrary::new(self)
    }

    pub fn get(&self, i: usize) -> Option<RefMut<T>> {
        self.items.get(i).map(|b| b.borrow_mut())
    }

    pub fn get_rc(&self, i: usize) -> Option<&RefCell<T>> {
        self.items.get(i)
    }

    pub fn push(&mut self, item: T) {
        self.items.push(RefCell::new(item));
    }
}

pub struct SubLibrary<'a, T: 'a> {
    pub items: Vec<&'a RefCell<T>>,
    index: usize
}

impl<'a, T> SubLibrary<'a, T> {
    pub fn new(l: &'a Library<T>) -> SubLibrary<'a, T> {
        SubLibrary {
            items: l.items.iter().map(|rc| rc).collect(),
            index: 0
        }
    }

    // pub fn from_item(item: &'a RefCell<T>) -> SubLibrary<'a, T> {
    //     SubLibrary {
    //         items: vec![item],
    //         index: 0
    //     }
    // }
}


impl<'a, T> Iterator for SubLibrary<'a, T> {
    type Item = RefMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.index).map(|r| r.borrow_mut());
        self.index += 1;
        item
    }
}
