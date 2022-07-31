use super::ContinuousStorageAllocationAlgorithm;
// use alloc::collections::linked_list::LinkedList;
use alloc::collections::vec_deque::VecDeque;

pub struct FirstFitAllocator {
    captains: VecDeque<(usize, usize)>,
    // 一些统计数据
    allocated: usize,
    total: usize,
}

impl ContinuousStorageAllocationAlgorithm for FirstFitAllocator {
    fn new() -> Self {
        Self {
            captains: VecDeque::new(),
            allocated: 0,
            total: 0,
        }
    }

    fn init(&mut self, start: usize, end: usize) {
        assert!(start <= end);
        self.total = end - start;
        self.captains = VecDeque::with_capacity(self.total);
        self.captains.push_back((start, self.total));
        log::info!("敢为天下先分配器启动成功！当前空闲物理页帧的数量为{}", self.get_remain_frame_cnt())
    }

    fn alloc(&mut self, count: usize) -> Option<usize> {
        for i in 0..self.captains.len(){
            let (allocated_frame, troop_size) = self.captains[i];
            if troop_size > count{
                self.captains[i].0 += count;
                self.captains[i].1 -= count;
                return Some(allocated_frame);
            }else if troop_size==count{
                self.captains.remove(i);
                return Some(allocated_frame);
            }
        }
        None
    }

    fn dealloc(&mut self, frame: usize, count: usize) {
        let mut frame = frame;
        for i in 0..self.captains.len(){
            let (start, troop_size) = self.captains[i];
            assert_ne!(start, frame);
            if start>frame{ //前面一直都是比frame小的。现在比它大，所以插在前面。
                assert!(frame+count<=start); //不应当overlap
                //试图合并
                //可以不合并，但是算法就不完备。
                if frame+count==start{
                    self.captains[i].0 = frame;
                    self.captains[i].1 +=count;
                }else {
                    if self.captains[i-1].0!=frame{ //不是此前合并的情况。
                        self.captains.insert(i, (frame, count)); //新的队长。
                    }
                }
                return;
            }else{
                assert!(start+troop_size<=frame); //不应当overlap
                //可删除，但是算法就不完备。
                if start+troop_size==frame{ //前面可以合并
                    self.captains[i].1 +=count;
                    frame = self.captains[i].0; //下一次循环的时候，一定满足上面的if，可以试图和后面的也合并
                }
            }
        }
        //都比frame小，插入到最后。
        self.captains.push_back((frame, count));
    }

    fn get_remain_frame_cnt(&mut self) -> usize {
        self.total - self.allocated
    }
}