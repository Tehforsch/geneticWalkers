use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

pub struct Library<T> {
    pub items: Vec<Rc<RefCell<T>>>
}

impl<T> Library<T> {
    pub fn new(items: Vec<T>) -> Library<T> {
        Library {
            items: items.into_iter().map(|i| Rc::new(RefCell::new(i))).collect()
        }
    }

    pub fn iter(&self) -> SubLibrary<T> 
    {
        SubLibrary::new(self)
    }

    pub fn get(&self, i: usize) -> Option<RefMut<T>> {
        self.items.get(i).map(|b| b.borrow_mut())
    }

    pub fn get_rc(&self, i: usize) -> Option<Rc<RefCell<T>>> {
        self.items.get(i).map(|rc| rc.clone())
    }

    pub fn push(&mut self, item: T) {
        self.items.push(Rc::new(RefCell::new(item)));
    }
}

pub struct SubLibrary<'a, T: 'a> {
    pub items: Vec<&'a Rc<RefCell<T>>>,
    index: usize
}

impl<'a, T> SubLibrary<'a, T> {
    pub fn new(l: &'a Library<T>) -> SubLibrary<'a, T> {
        SubLibrary {
            items: l.items.iter().map(|rc| rc).collect(),
            index: 0
        }
    }
}


impl<'a, T> Iterator for SubLibrary<'a, T> {
    type Item = RefMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.index).map(|r| r.borrow_mut());
        self.index += 1;
        item
    }
}
