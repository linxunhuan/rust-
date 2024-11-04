// 参与区块链合并的元素的最少个数，否则采用插入排序
const MIN_MERGE: usize = 64;

// 排序状态体
struct SortState<'a>{
    list:&'a mut [i32],
    runs:Vec<Run>, // 保存各个区块
    pos:usize,
}

// 定义Run 实体，保存run在list 中的起始下标和区块大小
#[derive(Debug,Copy,Clone)]
struct Run{
    pos:usize,
    len:usize,
}

// merge_lo 排序状态体，用于归并排序区块A和B
struct MergeLo<'a>{
    list_len:usize,     // 待排序集合的大小
    first_pos:usize,    // run1 的起始位置
    first_len: usize,   // run1 的大小
    second_pos: usize,  // run2 的起始位置
    dest_pos: usize,    // 排序结果的下标位置
    list:&'a mut [i32], // 待排序结合的部分区间
    temp:Vec<i32>,      // 将临时栈的大小设置为run1 和run2 中的较小者
}

// merge_hi 排序状态体，用于归并排序区块B和C
struct MergeHi<'a>{
    first_pos: isize,
    second_pos: isize,
    dest_pos: usize,
    list:&'a mut [i32],
    temp:Vec<i32>,      // 将临时栈的大小设置为run1 和run2 中的较小者
}

// 计算minrun，实际的取值区间wei[32,64]
fn calc_minrun(len: usize) -> usize{
    // 如果len的低位中有任何一位为1，r就会被设置为1
    let mut r = 0;
    let mut new_len = len;
    while new_len >= MIN_MERGR{
        r |= new_len & 1;
        new_len >>= 1;
    }
    new_len + r;
}

// 计算run（run表示区块）的起始下标，并将逆序的区块调整为正序
fn count_run(list:& mut [i32]) -> usize{
    let (ord, pos) = find_run(list);

    if ord{ // 逆序转正序
        list.split_at_mut(pos).0.reverse();
    }
    pos
}

// 计算run（run表示区块）的起始下标，并将逆序的区块调整为正序
fn find_run(list:&[i32]) -> (bool, usize){
    let len = list.len();
    if len < 2{
        return (false, len);
    }

    let mut pos = 1;
    if list[0] > list[1]{
        // 降序，list[i+1] < list[i]
        while pos < len - 1 && list[pos + 1] < list[pos - 1]{
            pos += 1;
        }
        return (true, pos + 1);
    }else{
        // 升序，list[i+1] >= list[i]
        while pos < len - 1 && list[pos + 1] >= list[pos]{
            pos += 1;
        }
        return (false, pos + 1);
    }
}

// 下面为SortState 实现构造函数和排序函数。
// 当区块的大小不满足规则时，需要通过归并排序来实现区块的合并

impl<'a> SortState<'a> {
    fn new(list:&'a mut [i32]) -> Self{
        SortState { 
            list: list,
            runs: Vec::new(),
            pos: 0,
        }
    }

    fn sort(&mut self){
        let len = self.list.len();

        // 计算minrun
        let minrun = calc_minrun(len);

        while self.pos < len {
            let pos = self.pos;
            let mut run_len = count_run(self.list.split_at_mut(pos).1);

            // 判断剩下元素的个数是否下雨minrun
            // 如果小于minrun，则设置run_minlen = len - pos
            let run_minlen = if minrun > len - pos {
                len - pos
            }else{
                minrun
            };

            // 如果run 很小，则扩充其大小至run_minlen
            // 同时，扩充后的run是有序的，因此可以采用二分插入排序
            if run_len < run_minlen{
                run_len = run_minlen;
                let left = self.list
                    .split_at_mut(pos).1
                    .split_at_mut(run_len).0;
                binary_insertion_sort(left);
            }

            // 将run入栈，各个run的大小不同
            self.runs.push(Run{
                pos: pos,
                len: run_len,
            });

            // 找到下一个run 的位置
            self.pos += run_len;

            // run的大小各不相同，合并不满足条件
            // A > B + C 和 B > C 的run
            self.merge_collapse();
        }

        // 不管合并规则如何，强制从栈顶开始合并剩下的所以run
        // 直到只剩下一个run，结束tim排序过程
        self.merge_collapse();
    }

    // 合并 run，使得A > B + C 和 B > C
    // 如果A <= B + C ,则区块B与区块A和C中较小的那个合并
    // 如果只有区块A 和 B ，那么当A <= B时，合并区块A和B
    fn merge_collapse(&mut self){
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;

            // 判断区块A、B、C、D之间的关系，区块D的存在是为了预防特殊情况
            // A <= B + C || D <= A + B
            if (n >= 1 && runs[n - 1].len()
                <= runs[n].len() + runs[n + 1].len())
                || (n >= 2 && runs[n - 2].len()
                    <= runs[n ].len() + runs[n - 1].len())
                    {
                        // 判断三个连续区块（区块A、B、C）的大小关系并合并
                        // n - 1对应区块B、n + 1对应区块C
                        let (pos1, pos2) = if runs[n - 1].len() < runs[n + 1].len() {
                            (n - 1, n) // 区块A 和 B 合并
                        }else{
                            (n ,n + 1) // 区块B 和 C合并
                        };

                        // 取出待合并的run1 和 run2
                        let (run1, run2) = (runs[pos1], runs[pos2]);
                        debug_assert_eq!(run1.pos + run1.len(), run2.pos);

                        // 合并run到run1，即更新run1 并删除run2 
                        // run1的下标不变，但大小变为run1和run2的大小之和
                        runs.remove(pos2);
                        runs[pos1] = Run{
                            pos: run1.pos,
                            len: run1.len() + run2.len(),
                        };
                        // 取出合并后的run1并进行归并排序
                        let new_list = self.list
                            .split_at_mut(run1.pos).1
                            .split_at_mut(run1.len + run2.len).0;
                        merge_sort(new_list,run1.len,run2.len);
                    }else{
                        break;
                    }
        }
    }

    // 在所有的run都处理完毕后，强制合并剩余的run，直至只剩下一个run
    fn merge_force_collapse(&mut self){
        let runs = &mut self.runs;
        while runs.len() > 1{
            let n = runs.len() - 2;
            // 判断三个连续区块（区块A、B、C）的大小关系并合并
            // n - 1对应区块A、n对应区块B、n + 1对应区块C
            let (pos1,pos2) = if n > 0
                && runs[n - 1].len < runs[n + 1].len{
                    (n - 1,n)
                }else{
                    (n,n + 1)
                };
        }

        // 取出待合并的区块run1 和 run2
        let (run1, run2) = (runs[pos1], runs[pos2]);
        debug_assert_eq!(run1.len(), run2.pos);

        // 合并run到run1，即更新run1 并删除run2
        // run1的下标不变，但大小变为run1和run2的大小之和
        runs.remove(pos2);
        runs[pos1] = Run{
            pos: run1.pos,
            len: run1.len() + run2.len(),
        };

        // 取出合并后的run1并进行归并排序
        let new_list = self.list
            .split_at_mut(run1.pos).1
            .split_at_mut(run1.len + run2.len).0;
        merge_sort(new_list,run1.len, run2.len);
    }
}

/*
根据分区的6种情况，有可能需要合并区块A 和 B或合并区块B和C
由于区块A、B、C、D在内存中是挨着的
因此可以利用位置关系分别实现合并区块A 和B的merge_lo 函数以及合并区块B和C的merge_hi 函数
*/ 

// 对区块A、B、C进行归并排序
fn merge_sort(list:&mut [i32], first_len: usize, second_len: usize){
    if 0 == first_len || 0 == second_len{
        return;
    }

    if first_len <= second_len{
        merge_lo(list, first_len);
    }else{
        merge_hi(list,first_len, second_len);
    }
}

// 合并区块A和B为一个区块
fn merge_lo(list:&mut [i32], len: usize){
    unsafe{
        let mut state  = MergeLo::new(list,first_len);
        state.merge();
    }
}

impl<'a> MergeLo<'a> {
    fn drop(&mut self) {
        unsafe{
            // 将temp中剩余的值放到list的高位
            if self.first_pos < self.first_len {
                for i in 0..(self.first_len - self.first_pos) {
                    self.list[self.dest_pos + i] = self.temp[self.first_pos + i];
                }
            }
            // 将临时栈的大小设置为0
            self.temp.set_len(0);
        }
    }
}

// 合并区块B和C为一个区块
fn merge_hi(list:&mut [i32], first_len: usize, second_len: usize){
    unsafe{
        let mut state  = MergeHi::new(list,first_len, second_len);
        state.merge();
    }
}

impl<'a> MergeHi<'a> {
    unsafe fn new(
        list: &mut [i32],
        first_len: usize,
        second_len: usize,
    ) -> Self {
        let mut ret_val = MergeHi{
            first_pos:first_len as isize - 1,
            second_pos:second_len as isize - 1,
            dest_pos:list.len() as isize - 1,// 从末尾开始排序
            list: list,
            temp: Vec::with_capacity(second_len),
        };

        // 把run2 复制到temp中
        ret_val.temp.set_len(second_len);
        for i in 0..second_len {
            ret_val.temp[i] = ret_val.list[i + second_len];
        }
        ret_val
    }


    // 进行归并排序
    fn merge(&mut self){
        while self.first_pos < self.dest_pos
            && self.first_pos >= 0{
                debug_assert!(self.first_pos + self.second_pos + 1 == self.dest_pos);
                if self.temp[self.second_pos as usize] >= self.list[self.first_pos as usize]{
                    self.list[self.dest_pos as usize] = self.temp[self.second_pos as usize];
                    self.second_pos -= 1;
                }else{
                    self.deset_pos -= 1;
                }
            }
    }
}

// 清理临时栈
impl<'a> Drop for MergeHi<'a> {
    fn drop(&mut self) {
        unsafe{
            // 将temp中剩余的值放到list的高位
            if self.first_pos < elf.first_len{
                for i in 0..(self.first_len - self.first_pos) {
                    self.list[self.dest_pos + i] = self.temp[self.first_pos + i];
                }
            }
            // 将临时栈的大小设置为0
            self.temp.set_len(0);
        }
    }
}

// 合并区块B和C为一个区块
fn merge_hi(list:&mut [i32], first_len: usize, second_len: usize){
    unsafe{
        let mut state  = MergeHi::new(list,first_len, second_len);
        state.merge();
    }
}

impl<'a>MergeHi<'a>{
    unsafe fn new(
        list: &mut [i32],
        first_len: usize,
        second_len: usize,
    ) -> Self {
        let mut ret_val = MergeHi{
            first_pos:first_len as isize - 1,
            second_pos: second_len as isize - 1,
            dest_pos: list.len() as isize - 1,// 从末尾开始排序
            list: list,
            temp: Vec::with_capacity(second_len),
        };

        // 把run2 复制到temp中
        ret_val.temp.set_len(second_len);
        for i in 0..second_len {
            ret_val.temp[i] = ret_val.list[i + first_len];
        }
        ret_val
    }

    // 进行归并排序
    fn merge(&mut self){
        while self.first_pos < self.dest_pos && self.first_pos >= 0{
            debug_assert!(self.first_pos + self.second_pos + 1 == self.dest_pos);
            if self.temp[self.first_pos as usize]{
                self.list[self.dest_pos as usize] = self.temp[self.second_pos as usize];
                self.second_pos -= 1;
            }else{
                self.list[self.dest_pos as usize] = self.temp[self.second_pos as usize];
                self.first_pos -= 1;
            }
            self.dest_pos -= 1;
        }
    }
}

// 清理临时栈
impl<'a> Drop for MergeHi<'a>{
    fn drop(&mut self) {
        unsafe{
            // 将temp中剩余的值放到list的低位
            if self.second_pos >= 0{
                let size = self.second_pos + 1;
                let src = 0;
                let dest = self.dest_pos - size;
                for i in 0..size{
                    self.list[(dest + i) as usize] = self.list[(src + i) as usize];
                }
            }
            // 将临时栈的大小设置为0
            self.temp.set_len(0);
        }
    }
}
