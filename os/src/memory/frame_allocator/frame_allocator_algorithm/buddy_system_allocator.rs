use super::ContinuousStorageAllocationAlgorithm;
use alloc::collections::btree_set::BTreeSet;
use core::cmp::min;
use core::mem::size_of;

/// # 兄弟齐心系统分配器
/// 一个使用伙伴系统(buddy system)策略的动态连续存储资源分配器(dynamic continuous storage resource allocator)。
/// 常用于操作系统(作为硬件资源的管理器)管理启动堆内存、物理内存、虚拟内存的连续存储分配。
pub struct BuddySystemAllocator {
    // 32个平衡二叉树有序集。保存的是32种不同大小的页面的32棵树表示的空闲列表。
    free_list: [BTreeSet<usize>; 32],
    // 一些统计数据
    allocated: usize,
    total: usize,
}

impl ContinuousStorageAllocationAlgorithm for BuddySystemAllocator {
    /// 使用默认构造函数初始化数组。
    fn new() -> Self {
        Self {
            free_list: Default::default(),
            allocated: 0,
            total: 0,
        }
    }

    fn init(&mut self, start: usize, end: usize) {
        assert!(start<=end);
        let mut total = 0; //一共成功获得了多少个页面。
        let mut current_start = start;
        while current_start < end {
            let low_bit = if current_start > 0 {
                current_start & (!current_start + 1) //树状数组中应当管辖的数量。就是取得了自己第一个low_bit 的大小。比如 low_bit(8) = 1000 low_bit(6) = 10
            } else {
                32
            };
            let size = min(low_bit, prev_power_of_two(end - current_start));
            total += size;
            // trailing_zeros()是结尾有多少个0.
            self.free_list[size.trailing_zeros() as usize].insert(current_start);
            current_start += size;
        }
        self.total += total;
        log::info!("兄弟齐心系统分配器启动成功！当前空闲物理页帧的数量为{}", self.get_remain_frame_cnt())
    }

    fn alloc(&mut self, count: usize) -> Option<usize> {
        let size = count.next_power_of_two();
        let class = size.trailing_zeros() as usize;
        for i in class..self.free_list.len() {
            // Find the first non-empty size class
            if !self.free_list[i].is_empty() {
                // Split buffers
                for j in (class + 1..i + 1).rev() {
                    if let Some(block_ref) = self.free_list[j].iter().next() {
                        let block = *block_ref;
                        self.free_list[j - 1].insert(block + (1 << (j - 1)));
                        self.free_list[j - 1].insert(block);
                        self.free_list[j].remove(&block);
                    } else {
                        return None;
                    }
                }

                let result = self.free_list[class].iter().next().clone();
                if let Some(result_ref) = result {
                    let result = *result_ref;
                    self.free_list[class].remove(&result);
                    self.allocated += size;
                    return Some(result);
                } else {
                    return None;
                }
            }
        }
        None
    }

    fn dealloc(&mut self, frame: usize, count: usize) {
        let size = count.next_power_of_two();
        let class = size.trailing_zeros() as usize;

        // Merge free buddy lists
        let mut current_ptr = frame;
        let mut current_class = class;
        while current_class < self.free_list.len() {
            let buddy = current_ptr ^ (1 << current_class);
            if self.free_list[current_class].remove(&buddy) == true {
                // Free buddy found
                current_ptr = min(current_ptr, buddy);
                current_class += 1;
            } else {
                self.free_list[current_class].insert(current_ptr);
                break;
            }
        }

        self.allocated -= size;
    }

    fn get_remain_frame_cnt(&mut self) -> usize {
        self.total-self.allocated
    }
}

///
fn prev_power_of_two(num: usize) -> usize {
    // leading_zeros是说这个数字前面有多少个0.
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}