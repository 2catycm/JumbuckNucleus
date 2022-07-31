trait FrameAllocatorAlgorithm {
    fn new() -> Self;
    fn init(&mut self, start: usize, end: usize);
    fn alloc(&mut self, count: usize) -> Option<usize>;
    fn dealloc(&mut self, frame: usize, count: usize);
    fn get_remain_frame_cnt(&mut self) -> usize;
}

// 利用具体算法，实现物理页号的分配。
use super::{FrameAllocator, PhysPageNum};

/// 使用具体的连续分配算法实现物理帧分配算法。
pub struct FrameAllocatorImpl {
    algorithm: ContinuousStorageAllocationAlgorithm,
}

impl FrameAllocator for FrameAllocatorImpl {
    fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.algorithm.init(l.0, r.0)
    }
    fn new() -> Self {
        Self {
            algorithm: ContinuousStorageAllocationAlgorithm::new(),
        }
    }
    fn get_remain_frame_cnt(&mut self) -> usize {
        self.algorithm.get_remain_frame_cnt()
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
}

// 选择具体的算法，此时与页号无关了。
mod stack_allocator;

type ContinuousStorageAllocationAlgorithm = stack_allocator::StackAllocator;
