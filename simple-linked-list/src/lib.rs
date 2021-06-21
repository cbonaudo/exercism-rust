use std::iter::FromIterator;

mod list_item;
use list_item::ListItem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<ListItem<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        if let Some(head) = &self.head {
            let mut list_item = &head.next;
            let mut i = 1;

            while let Some(next) = list_item {
                list_item = &next.next;
                i += 1;
            }

            return i;
        } else {
            return 0;
        }
    }

    pub fn push(&mut self, element: T) {
        let item = ListItem::new(element);

        if let Some(head) = self.head.as_mut() {
            head.push(item);
        } else {
            self.head = Some(Box::new(item));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head_item) = self.head.as_mut() {
            if head_item.next.is_some() {
                head_item.pop()
            } else {
                let item = head_item.value.clone();
                self.head = None;
                Some(item)
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head_item) = self.head.as_ref() {
            if head_item.next.is_some() {
                head_item.peek()
            } else {
                Some(&head_item.value)
            }
        } else {
            None
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut my_vec = Vec::new();

        if let Some(head_item) = self.head.as_ref() {
            head_item.add_values_recursive(&mut my_vec);
        } 

        my_vec.iter().rev().cloned().collect()
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ll = Self::new();

        iter.into_iter().for_each(|item| ll.push(item));

        ll
    }
}

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();

        while !linked_list.is_empty() {
            if let Some(item) = linked_list.pop() {
                vec.push(item);
            }
        }
        
        vec.iter().rev().cloned().collect()
    }
}
