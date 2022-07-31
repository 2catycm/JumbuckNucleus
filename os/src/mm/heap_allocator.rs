//! The global allocator
use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("绵羊核心堆内存申请异常，可能存在内存泄露！{:?}", layout);
}
/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE]; //数组
/// initiate heap allocator
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
/// 测试堆分配器是否正确工作
#[allow(unused)]
pub fn test_heap() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        assert_eq!(*val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    log::info!("堆测试通过！");
}

///通过堆上使用过多内存，让系统奔溃
#[allow(unused)]
pub fn test_panic_when_heap_space_not_enough() {
    log::warn!("正在执行崩溃测试，接下来操作系统应当崩溃并且关机，而不是死机。");
    {
        for i in 0..100000000 {
            use alloc::boxed::Box;
            let t = Box::new(3);
            if i % 100000 == 0 {
                log::info!("正在访问物理地址 {:p}", t.as_ref());
            }
            core::mem::forget(t);
        }
    }
}