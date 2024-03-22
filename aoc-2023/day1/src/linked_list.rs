pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Link::Empty }
    }
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    value: T,
    next: Link<T>
}
