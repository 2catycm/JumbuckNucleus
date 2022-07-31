use super::ContinuousStorageAllocationAlgorithm;
// use alloc::collections::linked_list::LinkedList;
use alloc::collections::vec_deque::VecDeque;

pub struct BestFitAllocator {
    captains: VecDeque<(usize, usize)>,
    // 一些统计数据
    allocated: usize,
    total: usize,
}

impl ContinuousStorageAllocationAlgorithm for BestFitAllocator {
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
        log::info!("最佳匹配分配器启动成功！当前空闲物理页帧的数量为{}", self.get_remain_frame_cnt())
    }

    fn alloc(&mut self, count: usize) -> Option<usize> {
        let mut min_diff = usize::MAX;
        let mut arg_min:Option<usize> = None;
        for i in 0..self.captains.len(){
            let (allocated_frame, troop_size) = self.captains[i];
            if troop_size > count{
                log::debug!("{:?} 队长是申请{}空间的一个选择", self.captains[i], count);
                let diff = troop_size-count;
                if diff<min_diff{
                    min_diff = diff;
                    arg_min = Some(i);
                }
            }else if troop_size==count{
                //提前结束，这就是最好的。
                log::debug!("{:?} 队长接受了 {} 的请求", self.captains[i], count);
                self.captains.remove(i);
                log::debug!("队伍消失");
                return Some(allocated_frame);
            }
        }
        if let Some(arg_min) = arg_min{
            log::debug!("{:?} 队长是申请{}空间的 best fit 选择", self.captains[arg_min], count);
            let allocated_frame = self.captains[arg_min].0;
            self.captains[arg_min].0 += count;
            self.captains[arg_min].1 -= count;
            log::debug!("队伍状态变更为{:?}", self.captains[arg_min]);
            return Some(allocated_frame);
        }else {
            log::warn!("无法找到合适的连续空间！");
            None
        }
    }

    fn dealloc(&mut self, frame: usize, count: usize) {

        for i in 0..self.captains.len(){
            let (start, troop_size) = self.captains[i];
            assert_ne!(start, frame);
            if start>frame{ //前面一直都是比frame小的。现在比它大，所以插在前面。
                log::debug!("{:?} 队长的左边可以释放({}, {})", self.captains[i], frame, count);
                assert!(frame+count<=start); //不应当overlap
                //试图合并
                //可以不合并，但是算法就不完备。
                if frame+count==start{
                    self.captains[i].0 = frame;
                    self.captains[i].1 +=count;
                    log::debug!("可以合并， 队伍状态变更为{:?}", self.captains[i]);
                }else {
                    self.captains.insert(i, (frame, count)); //新的队长。
                    log::debug!("右边不可以合并， 已经插入{:?}", self.captains[i]);
                }
                //顺便要看看左边能不能合并，因为这个情况之前没有考虑。
                if i!=0 {
                    let (start, troop_size) = self.captains[i-1];
                    if start+troop_size==frame {
                        self.captains[i].0 = start;
                        self.captains[i].1 += troop_size;
                        self.captains.remove(i-1);
                        log::debug!("左边可以合并，删除左边并且队伍状态变更为{:?}", self.captains[i-1]);
                    }
                }
                return;
            }else{
                assert!(start+troop_size<=frame); //不应当overlap
            }
        }
        //都比frame小，插入到最后。
        log::debug!("在所有队长之后，释放并产生新的队长({}, {})", frame, count);
        self.captains.push_back((frame, count));
    }

    fn get_remain_frame_cnt(&mut self) -> usize {
        self.total - self.allocated
    }
}