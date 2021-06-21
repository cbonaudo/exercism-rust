#[derive(Clone)]
pub struct ListItem<T> {
    pub value: T,
    pub next: Option<Box<ListItem<T>>>,
}

impl<T: Clone> ListItem<T> {
    pub fn new(element: T) -> Self {
        Self {
            value: element,
            next: None,
        }
    }

    pub fn push(&mut self, item: ListItem<T>) {
        if let Some(next) = self.next.as_mut() {
            next.push(item);
        } else {
            self.next = Some(Box::new(item));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let next = self.next.as_mut().unwrap();

        if next.next.is_some() {
            next.pop()
        } else {
            let item = next.value.clone();
            self.next = None;
            Some(item)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let next = self.next.as_ref().unwrap();

        if next.next.is_some() {
            next.peek()
        } else {
            Some(&next.value)
        }
    }

    pub fn add_values_recursive(&self, mut my_vec: &mut Vec<T>) {
        my_vec.push(self.value.clone());

        if let Some(next_item) = self.next.as_ref() {
            next_item.add_values_recursive(&mut my_vec);
        } 
    }
}
