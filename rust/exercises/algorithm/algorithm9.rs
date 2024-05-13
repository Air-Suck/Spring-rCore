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
            //这里先塞进去了一个元素，相当于是把0占掉了
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
        //实际上就是先将元素插入，然后再重新构建就好了
        self.items.push(value);
        self.count+=1;
        //首先需要找到最后一个非叶子节点（实际上就是最后一个节点的父节点）
        let mut parentIndex =self.parent_idx(self.count);
        let mut childIndex:usize;
        //如果是小根堆返回true的话就表示当前父节点比子节点小，也就是符合小根堆条件（大根堆也是同理）
        while parentIndex>=1{
            childIndex=self.smallest_child_idx(parentIndex);
            if (self.comparator)(&self.items[parentIndex],&self.items[childIndex]){
                parentIndex-=1;
            }else{
                //这个时候就说明当前是不满足条件的，需要将该子结点上移
                self.items.swap(parentIndex,childIndex);
            }
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
		//就当作是返回当前节点的孩子节点的大小吧，小根堆返回小的，大根堆返回大的
        if self.children_present(idx){
            let left =&self.items[self.left_child_idx(idx)];
            //注意这里可能右孩子不存在
            if self.right_child_idx(idx)>self.count{
                return self.left_child_idx(idx);
            }else{
                let right=&self.items[self.right_child_idx(idx)];
                //如果这个函数返回的是true，不论是大根堆和还是小根堆都是返回left（因为大根堆在left大的时候才会返回true，小根堆在left小的时候才会返回true）
                if (self.comparator)(left,right){
                    return self.left_child_idx(idx);
                }else{
                    return self.right_child_idx(idx);
                }
            }
        }else{
            //说明当前节点是没有孩子的，直接返回0吧
            return 0;
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
        //TODO
        if self.is_empty(){
            None
        }else{
            //将根元素根尾部元素互换
            self.items.swap(1,self.count);
            let temp=self.items.pop();
            self.count-=1;
            //重新调整堆
            //首先需要找到最后一个非叶子节点（实际上就是最后一个节点的父节点）
            let mut parentIndex =self.parent_idx(self.count);
            let mut childIndex:usize;
            //如果是小根堆返回true的话就表示当前父节点比子节点小，也就是符合小根堆条件（大根堆也是同理）
            while parentIndex>=1{
                childIndex=self.smallest_child_idx(parentIndex);
                if (self.comparator)(&self.items[parentIndex],&self.items[childIndex]){
                    parentIndex-=1;
                }else{
                    //这个时候就说明当前是不满足条件的，需要将该子结点上移
                    self.items.swap(parentIndex,childIndex);
                }
            }
            //调整结束将temp封装成Option返回
            temp
        }
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