pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    if lst.len() == 0 {
        return res;
    }
    for i in lst.iter() {
        res.push(*i);
    }
    let rem = res.remove(0);
    res.push(rem);
    res
}

fn squared_sum(lst: &Vec<i32>) -> i32 {
    lst.iter().map(|x| x*x).fold(0,|a,h| a + h)    
}

fn cubed_sum(lst: Vec<i32>) -> i32 {
    lst.iter().map(|x| x*x*x).fold(0,|a,h| a + h)    
}

pub fn squared_cubed_sum(lst: Vec<i32>) -> i32 {
    let sq = squared_sum(&lst);
    let cube = cubed_sum(lst);
    sq + cube
}

pub fn partial_sum(i: usize, is: &[i32]) -> i32 {
    let mut sum = 0;
    for (a,val) in is.iter().enumerate() {
        if a==i {
            sum = sum + val;
            break;
        } else {
            sum = sum + val;
        }
    }
    sum
}

pub fn ack(m: i32, n: i32) -> i32 {
    if m == 0 {n+1}
    else if m > 0 && n == 0 {ack(m-1,1)}
    else {ack(m-1,ack(m,n-1))}
}

pub fn mean(lst: &[f64]) -> Option<f64> {
    if lst.len() == 0 {
        None
    } else {
        let sum = lst.iter().fold(0.0, |acc, x| acc + x);
        let len = lst.len();
        let new_len = len as f64;
        let mean = sum/new_len;
        Some(mean)
    }
}

pub fn bsearch<T: Eq + PartialOrd> (lst: &[T], ele: &T) -> Option<usize> {
    let mut last_ele = lst.len()-1;
    let mut first_ele = 0;
    while first_ele <= last_ele {
        let search_ele = (first_ele+last_ele)/2;
        if lst[search_ele] == *ele {
            return Some(search_ele);
        } else if lst[search_ele] < *ele {
            first_ele = search_ele + 1;
        } else {
            last_ele = search_ele - 1;
        }
    }
    return None;
}

pub fn consecutive_1s(arr: &[i32]) -> i32 {
    let mut best = 0;
    let mut curr = 0;
    for i in arr.iter() {
        if (*i == 1) {
            curr = curr + 1;
        } else {
            if (curr > best) {
                best = curr;
                curr = 0;
            }
        } 
    }
    return best;
}

fn main() {
    let nums = [1,2,3,4,5,6,7,8,9];
    let nums2 = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    let ret = partial_sum(13, &nums);
    println!("{}", ret);
    
    let ret2 = ack(1,2);
    println!("{}", ret2);
    
    let ret3 = mean(&nums2);
    println!("{:?}", ret3);
    
    println!("{}", nums[nums.len()-1]);
    
    let ret4 = bsearch(&nums, &1);
    println!("{:?}", ret4);
    
    let test = "ENGL,330,1123,MWM
PHYS,444,1123,WF
CMSC,330,1123,MWM";
use std::collections::HashMap;
    let mut dept: HashMap<&str, usize> = HashMap::new();
    let mut room: HashMap<&str, usize> = HashMap::new();
    let mut time: HashMap<&str, usize> = HashMap::new();
    for line in test.lines() {
        let v: Vec<&str> = line.split(',').collect();
        for (i,key) in v.iter().enumerate() {
            if i == 0 {
                if dept.contains_key(key) {
                    *dept.get_mut(key).unwrap() += 1;
                } else {
                    dept.insert(key, 1);
                }
            } else if i == 2 {
                if room.contains_key(key) {
                     *room.get_mut(key).unwrap() += 1;
                } else {
                    room.insert(key, 1);
                }
            } else if i == 3 {
                if time.contains_key(key) {
                     *time.get_mut(key).unwrap() += 1;
                } else {
                    time.insert(key, 1);
                }
            }
        }
    }
    for (key, value) in dept {
        println!("{} / {}", key, value);
    }
    println!("///////");
    for (key, value) in room {
        println!("{} / {}", key, value);
    }
    println!("///////");
    for (key, value) in time {
        println!("{} / {}", key, value);
    }
    
    
    println!("[][][][]");
    let  rot = vec![];
    let ret = rotate(&rot);
    for x in ret.iter() {
        println!("{}", x);
    }
    
    println!("[][][][]");
    let summ = vec![1,2,3];
    let ans = squared_cubed_sum(summ);
    println!("{}", ans);
    
    println!("----------");
    let mut q = Vec::new();
    println!("{:?}", q.peek());
    println!("{:?}",q.poll());
    q.enqueue(5);
    q.enqueue(1);
    q.enqueue(3);
    q.enqueue(4);
    println!("{:?}",q.poll());
    println!("----------");
    for x in q.iter() {
        println!("{}", x);
    }
    println!("----------");
    println!("{:?}", q.peek());
    
     println!("AAAAAAAAAAA");
     println!("{}",consecutive_1s(&[1, 1, 1, 0, 1, 1]));
}

pub trait Queue<T> {
    // Add an element to the back of the queue
    fn enqueue(&mut self, ele: T) -> ();
    // Get the element from the front of the queue without removing it.  If the queue is empty, return None
    fn peek(&self) -> Option<&T>;
    // Get the element from the front of the queue and removes it.  If the queue is empty, return None
    fn poll(&mut self) -> Option<T>;
}



impl <T> Queue<T> for Vec<T> {

    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
    }

    fn peek(&self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        } else {
            return Some(&self[0]);
        }
    }

    fn poll(&mut self) -> Option<T> {
         if self.len() == 0 {
            return None;
        } else {
            return Some(self.remove(0));
        }
    }
}
