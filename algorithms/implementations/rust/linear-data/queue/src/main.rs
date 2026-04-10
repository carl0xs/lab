use std::collections::VecDeque; 

fn main() {
    // starting an empty queue with a capacity for 10 elements of type int
   let mut deque: VecDeque<u32> = VecDeque::with_capacity(10); 
 
   //add value to deque
   deque.push_back(1);
   deque.push_back(2);
   deque.push_back(3);
   deque.push_back(4);

   //remove last value
   deque.pop_back();

   println!("{:?}", deque);
}





