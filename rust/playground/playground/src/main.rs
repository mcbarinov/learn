struct ListItem<T> {
    data: T,
    next: Option<Box<ListItem<T>>>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem { data, next: None }
    }




}

struct LinkedList<T> {
    head: ListItem<T>,
}

fn main() {}
