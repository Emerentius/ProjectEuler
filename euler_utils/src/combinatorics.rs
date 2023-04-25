// algorithms for permutations from
// https://alistairisrael.wordpress.com/2009/09/22/simple-efficient-pnk-algorithm/
pub struct PermutationsStreamIter<T: Ord + Copy> {
    data: Vec<T>,
    first: bool,
}

impl<T: Ord + Copy> PermutationsStreamIter<T> {
    pub fn new<S>(data: S) -> PermutationsStreamIter<T>
    where
        S: AsRef<[T]>,
    {
        PermutationsStreamIter {
            first: true,
            data: data.as_ref().to_vec(),
        }
    }

    pub fn reset(&mut self) {
        self.first = true;
        self.data.sort();
    }

    pub fn streaming_next(&mut self) -> Option<&[T]> {
        if self.first {
            self.first = false;
            return Some(&self.data[..]);
        }

        let slice = &mut self.data[..];

        for i in (0..slice.len().saturating_sub(1)).rev() {
            if slice[i + 1] > slice[i] {
                let swappee1 = slice[i];
                let min_greater: Option<(usize, T)>;
                {
                    min_greater = (&slice[i..])
                        .iter()
                        .cloned()
                        .enumerate()
                        .skip(1) // instead of slicing one later, so index is i+found_num
                        .filter(|&(_, el)| el > swappee1)
                        .min_by_key(|&(_, el)| el);
                }
                if let Some((idx_offset, _)) = min_greater {
                    slice[i] = slice[i + idx_offset];
                    slice[i + idx_offset] = swappee1;
                    (&mut slice[i + 1..]).reverse();
                }
                return Some(slice);
            }
        }
        None
    }
}

pub struct PartialPermutationsStreamIter<T: Ord + Copy> {
    data: Vec<T>,
    k: usize, // how many elements to pick
    first: bool,
}

impl<T: Ord + Copy> PartialPermutationsStreamIter<T> {
    pub fn new<S>(data: S, k: usize) -> PartialPermutationsStreamIter<T>
    where
        S: AsRef<[T]>,
    {
        if k > data.as_ref().len() {
            panic!("Cannot pick more elements than are supplied in collection")
        }
        PartialPermutationsStreamIter {
            first: true,
            k: k,
            data: data.as_ref().to_vec(),
        }
    }

    pub fn reset(&mut self) {
        self.first = true;
        self.data.sort();
    }

    pub fn set_k(&mut self, k: usize) {
        self.k = k;
        self.reset()
    }

    pub fn streaming_next(&mut self) -> Option<&[T]> {
        if self.first {
            self.first = false;
            return Some(&self.data[..self.k]);
        }

        let slice = &mut self.data[..];

        let n = self.k - 1;
        // assume edge to be at the cutoff
        let swappee_edge1 = slice[n];
        let min_greater_edge: Option<(usize, T)>;
        {
            min_greater_edge = (&slice[n..])
                .iter()
                .cloned()
                .enumerate()
                .skip(1) // instead of slicing one later, so index is n+found_num
                .filter(|&(_, el)| el > swappee_edge1)
                .min_by_key(|&(_, el)| el);
        }
        if let Some((idx_offset, _)) = min_greater_edge {
            //println!("first: {} <-> {}", swappee_edge1, el);
            slice[n] = slice[n + idx_offset]; // el
            slice[n + idx_offset] = swappee_edge1;
            return Some(&slice[..self.k]);
        } else {
            (&mut slice[n + 1..]).reverse();
        }

        // edge not at cutoff
        // rest almost exactly the same
        for i in (0..self.k.saturating_sub(1)).rev() {
            if slice[i + 1] > slice[i] {
                let swappee1 = slice[i];
                let min_greater: Option<(usize, T)>;
                {
                    min_greater = (&slice[i..])
                        .iter()
                        .cloned()
                        .enumerate()
                        .skip(1) // instead of slicing one later, so index is i+found_num
                        .filter(|&(_, el)| el > swappee1)
                        .min_by_key(|&(_, el)| el);
                }
                if let Some((idx_offset, _)) = min_greater {
                    slice[i] = slice[i + idx_offset];
                    slice[i + idx_offset] = swappee1;
                    (&mut slice[i + 1..]).reverse();
                }
                return Some(&slice[..self.k]);
            }
        }
        None
    }
}
/*
pub fn next_permutation<T: Ord + Copy>(slice: &mut [T]) -> bool {
    for i in (0..slice.len().saturating_sub(1)).rev() {
        if slice[i+1] > slice[i] {
            let swappee1 = slice[i];
            let min_greater: Option<(usize, T)>;
            {
                min_greater = (&slice[i..]).iter()
                    .cloned()
                    .enumerate()
                    .skip(1) // instead of slicing one later, so index is i+found_num
                    .filter(|&(_,el)| el > swappee1)
                    .min_by_key(|&(_,el)| el);
            }
            if let Some((idx_offset, _)) = min_greater {
                slice[i] = slice[i+idx_offset];
                slice[i+idx_offset] = swappee1;
                (&mut slice[i+1..]).reverse();
            }
            return true;
        }
    }
    false
}

pub fn next_partial_permutation<T: Ord + Copy + std::fmt::Display>(slice: &mut [T], n: usize) -> bool {
    if n > slice.len() { panic!("Cannot permute first {} elements in slice of length {}", n, slice.len())}

    let n = n-1;
    // assume edge to be at the cutoff
    let swappee_edge1 = slice[n];
    let min_greater_edge: Option<(usize, T)>;
    {
        min_greater_edge = (&slice[n..]).iter()
            .cloned()
            .enumerate()
            .skip(1) // instead of slicing one later, so index is n+found_num
            .filter(|&(_,el)| el > swappee_edge1)
            .min_by_key(|&(_,el)| el);
    }
    if let Some((idx_offset, _)) = min_greater_edge {
        //println!("first: {} <-> {}", swappee_edge1, el);
        slice[n] = slice[n+idx_offset]; // el
        slice[n+idx_offset] = swappee_edge1;
        return true;
    } else {
        (&mut slice[n+1..]).reverse();
    }

    // rest almost exactly the same
    for i in (0..1+n.saturating_sub(1)).rev() { // <-- sole change
        if slice[i+1] > slice[i] {
            let swappee1 = slice[i];
            let min_greater: Option<(usize, T)>;
            {
                min_greater = (&slice[i..]).iter()
                    .cloned()
                    .enumerate()
                    .skip(1) // instead of slicing one later, so index is i+found_num
                    .filter(|&(_,el)| el > swappee1)
                    .min_by_key(|&(_,el)| el);
            }
            if let Some((idx_offset, _)) = min_greater {
                slice[i] = slice[i+idx_offset];
                slice[i+idx_offset] = swappee1;
                (&mut slice[i+1..]).reverse();
            }
            return true;
        }
    }
    false
}
*/
