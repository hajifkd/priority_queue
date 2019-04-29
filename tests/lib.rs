extern crate rand;

use priority_queue;

#[test]
fn test_sort_in_priority() {
    let mut pq = priority_queue::PriorityQueue::new();
    pq.insert(3, &7);
    pq.insert(0, &8);
    pq.insert(2, &10);
    pq.insert(4, &1);
    pq.insert(5, &9);

    assert_eq!(pq.pop_min(), Some(&8));
    assert_eq!(pq.pop_min(), Some(&10));
    assert_eq!(pq.pop_min(), Some(&7));
    assert_eq!(pq.pop_min(), Some(&1));
    assert_eq!(pq.pop_min(), Some(&9));
}

#[test]
fn test_sort_in_priority_ramdomly() {
    for _ in 0..100 {
        let mut pq = priority_queue::PriorityQueue::new();
        let data = (0..100).collect::<Vec<_>>();
        let mut order = vec![];
        for i in 0..100 {
            let priority: usize = rand::random();
            order.push((priority, i));
            pq.insert(priority, &data[i]); 
        }

        order.sort();

        for (_, d) in order.iter() {
            assert_eq!(pq.pop_min(), Some(d));
        }
    }
}
