use super::ContinuousStorageAllocationAlgorithm;
use alloc::vec::Vec;
/// an implementation for frame allocator
pub struct StackAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>, //存的是physical page number。 Vec是作为一个自动扩容栈来使用的。
}
impl ContinuousStorageAllocationAlgorithm for StackAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn init(&mut self, l: usize, r: usize) {
        self.current = l;
        self.end = r;
        log::info!("Stack 物理帧分配器启动，可用的物理页数：{}", self.get_remain_frame_cnt());
        assert_eq!(self.get_remain_frame_cnt(), self.end - self.current);
    }
    fn alloc(&mut self, count: usize) -> Option<usize> {
        if count != 1 {
            log::warn!("stack 物理帧分配器 不支持连续多个页面的分配！");
            None
        } else if let Some(ppn) = self.recycled.pop() {
            //如果最后一个元素可以解包
            Some(ppn) //返回最后一个页。
        } else if self.current == self.end {
            None //已经非法
        } else {
            self.current += 1;
            Some(self.current - 1) //向量为空。 此时需要搞个新的页号送给他。
        }
    }
    fn dealloc(&mut self, frame: usize, count: usize) {
        if count != 1 {
            log::warn!("试图在不支持多个连续页面分配的分配器上连续释放多个页面！");
        }
        // validity check
        if frame >= self.current || self.recycled.iter().any(|&v| v == frame) {
            panic!("Frame ppn={:#x} has not been allocated!", frame);
        }
        // recycle
        self.recycled.push(frame);
    }
    fn get_remain_frame_cnt(&mut self) -> usize {
        (self.end - self.current) + self.recycled.len()
    }
}
