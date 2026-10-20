/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;


// 定义链表当中的节点
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>, // 使用 NonNull<T> 的方式，来替代裸指针！确保不会发生空引用的现象！又因为 Rust 的空指针优化，因此使用 Option 之后，不会引入额外开销
}

// 一个新的 Node 节点
impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}


// 链表结构体
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>, // 起始位置，可以理解为头指针
    end: Option<NonNull<Node<T>>>, // 末尾位置，可以理解为尾指针
}


// 默认实现
impl<T> Default for LinkedList<T> 
    where T:PartialOrd+Ord+Clone
{
    fn default() -> Self {
        Self::new()
    }
}


impl<T> LinkedList<T> 
    where T:PartialOrd+Ord+Clone
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
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) }); // 使用 Box 安全创建裸指针
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // 对于 NonNull 的操作为 unsafe 的操作
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 递归写法，一个一个往后找
    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 合并两个有序的单链表
    // 两个指针，分别遍历链表当中的元素，将小的那一个加入到新的链表当中
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self // 加上 trait 限制，否则无法比较数据大小
	{
        let mut ans=LinkedList::new();
        let a_len = list_a.length as i32;
        let b_len = list_b.length as i32; 
        
        // 两个指针，分别指向两个链表当中的节点
        let mut i:i32=0;
        let mut j:i32=0;

        // 当两个链表都没有遍历完的时候
        while i<a_len && j<b_len{ // Rust 当中类型必须强匹配
            
            let a = match list_a.get(i){
                Some(num)=>num,
                None=>break,
            };

            let b =match list_b.get(j){
                Some(num)=>num,
                None=>break,
            };

            if a<=b{
                ans.add(a.clone());
                i+=1;
            }else{
                ans.add(b.clone());
                j+=1;
            }

        }

        // 处理完了之后，收尾

        while i < a_len{
            let a =match list_a.get(i){
                Some(num)=>num,
                None=>break,
            };
            ans.add(a.clone());
            i+=1;
        }

        while j < b_len{
            let b =match list_b.get(j){
                Some(num)=>num,
                None=>break,
            };
            ans.add(b.clone());
            j+=1;
        }

        ans

	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
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
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}