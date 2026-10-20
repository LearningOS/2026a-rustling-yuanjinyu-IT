/*
	heap
	This question requires you to implement a binary heap function
*/



use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 新元素放置在末尾，然后自底向上调整堆结构

        // 1. 新元素先放在数组末尾
        self.items.push(value);
        self.count+=1;

        // 2. 从新元素的位置开始，向上调整
        let mut idx = self.count;
        while idx > 1{
            let parent = self.parent_idx(idx);

            // 当前节点优先级没有父节点高，就不管
            if !((self.comparator)(&self.items[idx],&self.items[parent])){
                break;
            }

            // 否则的话，交换当前子节点和父节点
            self.items.swap(idx,parent);
            idx=parent;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right<=self.count && ((self.comparator)(&self.items[right],&self.items[left])) {
            right
        }else{
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count==0{
            return None;
        }

        // 先移除数组末尾的元素
        let last = self.items.pop().unwrap();
        self.count-=1;

        if self.count==0{
            return Some(last);
        }

        // 保存原来的堆顶，用最后一个元素填补根节点
        let root = std::mem::replace(&mut self.items[1], last);

        // 从根节点开始，向下调整堆结构
        let mut idx = 1;
        while self.children_present(idx) {
            // 找到应当交换的子节点
            let child = self.smallest_child_idx(idx);
            
            // 如果已经满足堆结构，直接结束
            if !((self.comparator)(&self.items[child],&self.items[idx])){
                break;
            }

            // 否则交换，并且继续向下调整
            self.items.swap(idx, child);
            idx=child;
        }

        Some(root)

    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}