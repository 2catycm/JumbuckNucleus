//! Implementation of [`FrameAllocator`] which
//! controls all the frames in the operating system.
use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;
/// 物理页帧的分配器接口。
trait FrameAllocator {
    fn new() -> Self;
    fn init(&mut self, l: PhysPageNum, r: PhysPageNum);
    fn alloc(&mut self, count: usize) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum, count: usize);
    fn get_remain_frame_cnt(&mut self) -> usize;
}
mod frame_allocator_algorithm;
use frame_allocator_algorithm::FrameAllocatorImpl;
lazy_static! {
    /// 按需初始化的全局物理页帧分配器。
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}
/// initiate the frame allocator using `end_kernel` and `MEMORY_END`
pub fn init_frame_allocator() {
    extern "C" {
        fn end_kernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(end_kernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
}
//2. 对外暴露管理frame的安全接口
/// manage a frame which has the same lifecycle as the tracker
pub struct FrameBlockTracker {
    pub(super)ppn: PhysPageNum,
    pub(super)count: usize,
}

impl FrameBlockTracker {
    ///Create an empty `FrameTracker`
    pub fn new(ppn: PhysPageNum, count: usize) -> Self {
        // 清理页的内容，全部变为0.
        for i in 0..count{
            PhysPageNum::from(usize::from(ppn)+i).clear();
        }
        Self { ppn,  count}
    }
}

impl Debug for FrameBlockTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("物理页面块跟踪器<起始物理页号={:#x}， 物理页数={}>", self.ppn.0, self.count))
    }
}

impl Drop for FrameBlockTracker {
    fn drop(&mut self) {
        dealloc_frames(self.ppn, self.count);
    }
}
/// allocate a frame
/// 接口暴露：本模块外的接口允许调用alloc方法，但是不允许调用dealloc。因为使用[`FrameBlockTracker`]来管理生命周期。
#[allow(unused)]
pub fn alloc_one_frame() -> Option<FrameBlockTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc(1)
        .map(|ppn| FrameBlockTracker::new(ppn, 1))
}

/// 分配一系列连续页面。
/// 接口暴露：本模块外的接口允许调用alloc方法，但是不允许调用dealloc。因为使用[`FrameBlockTracker`]来管理生命周期。
#[allow(unused)]
pub fn alloc_frames(count:usize)->Option<FrameBlockTracker>{
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc(count)
        .map(|ppn| FrameBlockTracker::new(ppn, count))
}
fn dealloc_frames(ppn: PhysPageNum, count:usize){
    FRAME_ALLOCATOR
        .exclusive_access().dealloc(ppn, count);
}

/// 返回当前剩下的物理页帧数量。
pub fn get_remain_frame_cnt() -> usize {
    FRAME_ALLOCATOR.exclusive_access().get_remain_frame_cnt()
}


#[allow(unused)]
/// 物理页帧 分配器 性能测试。
pub fn test_frame_allocator() {
    let mut v: Vec<FrameBlockTracker> = Vec::new();
    for i in 0..5 {
        let frame = alloc_frames(12).unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = alloc_frames(8).unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    log::info!("物理页帧 分配器 性能测试通过。");
}
