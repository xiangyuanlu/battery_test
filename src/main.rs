use tracing::info;

pub mod biz;
pub mod log;
pub mod serial_port;
pub mod webserver;
pub mod xutil;
use crate::webserver::start_web;

//nums = [5,7,7,8,8,10], target = 8
//nums = [5,7,7,8,8,10], target = 6
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    pub fn search_pos(nums: &Vec<i32>, target: i32, dir_left: bool) -> i32 {
        let mut ans: i32 = nums.len() as i32;
        let (mut l, mut r) = (0 as i32, nums.len() as i32 - 1);
        while l <= r {
            let m = (l + r) / 2;
            if nums[m as usize] > target || (dir_left && (nums[m as usize] >= target)) {
                r = m - 1;
                ans = m as i32;
            } else {
                l = m + 1;
            }
        }

        ans
    }

    let l = search_pos(&nums, target, true);
    let r = search_pos(&nums, target, false) - 1;
    let mut ans = Vec::new();
    if l <= r && r < nums.len() as i32 && nums[l as usize] == target && nums[r as usize] == target {
        ans.push(l);
        ans.push(r);
    } else {
        ans.push(-1);
        ans.push(-1);
    }
    ans
}

#[tokio::main]
async fn main() {
    let v = [1];
    let t = 1;
    let ans = search_range(v.to_vec(), t);
    println!("{:?}", ans);

    // log::init_log();
    // start_web().await;
    // info!("Starting");
}
