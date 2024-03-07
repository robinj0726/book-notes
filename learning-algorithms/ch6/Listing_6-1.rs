struct Node<T> {
  value: T,
  // 1. the value of next could be null, so we use Option
  // 2. since the size of the struct Node is not known at compile time, we use Box to store the value of next
  //    then the size of a pointer is unambiguous at compile time
  next: Option<Box<Node<T>>>,
}

fn sum_iterative<T>(mut n: Option<Box<Node<T>>>) -> T
where
  T: std::ops::Add<Output = T> + Default + Copy,
{
  let mut total = T::default();
  while let Some(node) = n {
      total = total + node.value;
      n = node.next;
  }
  total
}

fn sum_list<T>(n: Option<Box<Node<T>>>) -> T
where
  T: std::ops::Add<Output = T> + Default + Copy,
{
  match n {
      None => T::default(),
      Some(node) => node.value + sum_list(node.next),
  }
}