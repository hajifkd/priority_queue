use std::collections::HashMap;
use std::hash::Hash;

pub struct PriorityQueue<'a, T: Ord, S: Eq + Hash> {
    heap: Vec<(T, &'a S)>,
    indices: HashMap<&'a S, usize>,
}

impl<'a, T: Ord, S: Eq + Hash> PriorityQueue<'a, T, S> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: vec![],
            indices: HashMap::new(),
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.indices.insert(self.heap[i].1, j);
        self.indices.insert(self.heap[j].1, i);
        self.heap.swap(i, j);
    }

    fn heapify(&mut self, i: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left >= self.len() {
            return;
        } else if right >= self.len() {
            if self.heap[i].0 > self.heap[left].0 {
                self.swap(i, left);
            }
        } else {
            let mut i_smallest = i;

            if self.heap[i_smallest].0 > self.heap[left].0 {
                i_smallest = left;
            }

            if self.heap[i_smallest].0 > self.heap[right].0 {
                i_smallest = right;
            }

            if i_smallest != i {
                self.swap(i, i_smallest);
                self.heapify(i_smallest);
            }
        }
    }

    fn heapify_up(&mut self, i: usize) {
        if i == 0 {
            return;
        }

        let parent = (i - 1) / 2;

        if self.heap[i].0 < self.heap[parent].0 {
            self.swap(i, parent);
            self.heapify_up(parent);
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.heap.len(), self.indices.len());
        self.heap.len()
    }

    pub fn pop_min(&mut self) -> Option<&S> {
        if self.len() == 0 {
            None
        } else {
            self.swap(0, self.len() - 1);
            // must not None
            let result = self.heap.pop().unwrap();
            self.indices.remove(&result.1);

            self.heapify(0);

            Some(result.1)
        }
    }

    pub fn insert(&mut self, priority: T, value: &'a S) {
        // First, add a new element
        self.heap.push((priority, value));
        self.indices.insert(value, self.heap.len() - 1);

        // Then, heapify it.
        self.heapify_up(self.len() - 1);
    }

    pub fn decrease_priority(&mut self, value: &'a S, priority: T) {
        let i = self.indices[&value];
        self.heap[i] = (priority, self.heap[i].1);
        self.heapify_up(i);
    }

    pub fn increase_priority(&mut self, value: &'a S, priority: T) {
        let i = self.indices[&value];
        self.heap[i] = (priority, self.heap[i].1);
        self.heapify(i);
    }
}
