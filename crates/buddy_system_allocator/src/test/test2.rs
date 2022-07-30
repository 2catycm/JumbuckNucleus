use std::mem::size_of;
use crate::Heap;

#[test]
/// 实验结果： ORDER表示堆最大有多大, 单位为Byte。
fn test_order_heap() {
    assert_eq!(size_of::<usize>(), 8);
    // let mut heap = Heap::<3>::new(); // panic
    let mut heap = Heap::<4>::new(); //2表示最大分配的内存空间为2的2次方个 Byte. 不是 多少个usize。
    let space: [usize; 1] = [0; 1]; // 1个 8byte
    unsafe{
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(1) as usize);
    }
}