
struct Node<T> {
  grammar: Some,
  ctor_name: String,
  match_length: i32,
  children: Option<Vec<T>>,
}

impl Node {
  fn new(grammar: Some, ctor_name: String, match_length: i32) -> Node {
    Node { grammar, ctor_name, match_length, children: None }
  }

  fn num_children(self) -> i32 {
    self.children.map_or(0, |children| children.count())
  }

  fn child_at(self, idx: u32) -> Option<T> {
    self.children.map(|children| children.get(idx))
  }

  fn index_of_child(self, child: &T) -> Option<u32> {
    self.children.map(|children| children.find_index(child))
  }

  fn has_children(self) -> bool {
    self.num_children() > 1
  }

  fn has_no_children(self) -> bool {
    !self.has_children()
  }

  fn first_child(self) -> Option<T> {
    self.children.map(|children| children.first())
  }

  fn last_child(self) -> Option<T> {
    self.children.map(|children| children.last())
  }
}
