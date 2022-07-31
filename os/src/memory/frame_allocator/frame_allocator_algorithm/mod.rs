/// 连续分配算法的接口
trait ContinuousStorageAllocationAlgorithm {
    fn new() -> Self;
    ///
    ///
    /// # 参数
    ///
    /// * `start`: 注意是Inclusive的
    /// * `end`: 注意是Exclusive的. 左闭右开。
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn init(&mut self, start: usize, end: usize);
    fn alloc(&mut self, count: usize) -> Option<usize>;
    fn dealloc(&mut self, frame: usize, count: usize);
    fn get_remain_frame_cnt(&mut self) -> usize;
}
// 选择具体的算法，具体的算法与页号无关，可以分配任何连续资源。
mod stack_allocator;
mod buddy_system_allocator;
mod first_fit_allocator;
mod best_fit_allocator;
mod worse_fit_allocator;
// type ContinuousStorageAllocationAlgorithmImpl = stack_allocator::StackAllocator;
type ContinuousStorageAllocationAlgorithmImpl = buddy_system_allocator::BuddySystemAllocator;
// type ContinuousStorageAllocationAlgorithmImpl = first_fit_allocator::FirstFitAllocator;
// type ContinuousStorageAllocationAlgorithmImpl = best_fit_allocator::BestFitAllocator;
// type ContinuousStorageAllocationAlgorithmImpl = worse_fit_allocator::WorseFitAllocator;

// 利用具体算法，实现物理页号的分配。
use super::{FrameAllocator, PhysPageNum};

/// 使用具体的连续分配算法实现物理帧分配算法。
pub struct FrameAllocatorImpl {
    algorithm: ContinuousStorageAllocationAlgorithmImpl,
}

impl FrameAllocator for FrameAllocatorImpl {
    fn new() -> Self {
        Self {
            algorithm: ContinuousStorageAllocationAlgorithmImpl::new(),
        }
    }
    fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.algorithm.init(l.0, r.0)
    }
    fn alloc(&mut self, count: usize) -> Option<PhysPageNum> {
        if let Some(frame) = self.algorithm.alloc(count.into()) {
            Some(frame.into())
        } else {
            None
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum, count: usize) {
        self.algorithm.dealloc(ppn.into(), count).into()
    }
    fn get_remain_frame_cnt(&mut self) -> usize {
        self.algorithm.get_remain_frame_cnt()
    }
}


