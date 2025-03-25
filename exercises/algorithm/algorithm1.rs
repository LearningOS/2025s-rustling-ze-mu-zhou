use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T>
where
    T: PartialOrd + Clone, // 添加 Clone trait 约束
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T>
where
    T: PartialOrd + Clone, // 添加 Clone trait 约束
{
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        let mut current = self.start;
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                return Some(unsafe { &(*node.as_ptr()).val });
            }
            current = unsafe { (*node.as_ptr()).next };
            i += 1;
        }

        None
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        while let (Some(a_node), Some(b_node)) = (a_ptr, b_ptr) {
            let a_val = unsafe { (*a_node.as_ptr()).val.clone() }; // 直接克隆值
            let b_val = unsafe { (*b_node.as_ptr()).val.clone() }; // 直接克隆值

            if a_val <= b_val {
                merged_list.add(a_val); // 添加 T 类型的值
                a_ptr = unsafe { (*a_node.as_ptr()).next };
            } else {
                merged_list.add(b_val); // 添加 T 类型的值
                b_ptr = unsafe { (*b_node.as_ptr()).next };
            }
        }

        while let Some(a_node) = a_ptr {
            let a_val = unsafe { (*a_node.as_ptr()).val.clone() }; // 直接克隆值
            merged_list.add(a_val); // 添加 T 类型的值
            a_ptr = unsafe { (*a_node.as_ptr()).next };
        }

        while let Some(b_node) = b_ptr {
            let b_val = unsafe { (*b_node.as_ptr()).val.clone() }; // 直接克隆值
            merged_list.add(b_val); // 添加 T 类型的值
            b_ptr = unsafe { (*b_node.as_ptr()).next };
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", unsafe { &(*node.as_ptr()).val })?;
            first = false;
            current = unsafe { (*node.as_ptr()).next };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}