use std::cmp::Ordering;

pub struct ArrayTree<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    comparator: C,
    data: Vec<Vec<T>>,
    capacity: u16,
    num_elements: usize,
}

impl<T, C> ArrayTree<T, C>
where
    C: Fn(&T, &T) -> Ordering,
    T: Clone + std::fmt::Debug,
{
    pub fn new(comparator: C, capacity: u16) -> ArrayTree<T, C> {
        let data = Vec::with_capacity(capacity as usize);
        ArrayTree {
            comparator,
            data,
            capacity,
            num_elements: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.num_elements
    }

    pub fn insert(&mut self, t: T) -> bool {
        // println!("\nInserting: {:?}", t);
        if self.data.len() == 0 {
            self.data.push(self.new_block(t));
            return true;
        }

        // Binary search for block index
        let (idx_block, equals) = binary_search_by(
            &self.data,
            |block| (self.comparator)(&block[0], &t),
        );
        if equals {
            return false;
        }

        // Convert from "first larger" to "last smaller" index semantics
        let mut idx_block = if idx_block > 0 {
            idx_block - 1
        } else {
            0
        };
        // println!("idx_block: {}    block_len: {}", idx_block, self.data[idx_block].len());

        // Split block if necessary
        if self.data[idx_block].len() >= self.capacity as usize {
            let tail_from = (self.capacity / 2) as usize;
            let tail_upto = self.capacity as usize;
            let block_tail = self.data[idx_block][tail_from .. tail_upto].to_vec();

            self.data[idx_block].truncate(tail_from);
            self.data.insert(idx_block + 1, block_tail);

            // println!("block l: {:?}", self.data[idx_block]);
            // println!("block r: {:?}", self.data[idx_block + 1]);
            // Determine into which of the two split blocks the new value goes.
            // FIXME: Can we miss an "equals" case here if we go into block than doesn't have the equal element?
            if (self.comparator)(&t, &self.data[idx_block + 1][0]) == Ordering::Greater {
                idx_block += 1;
            }
            // println!("idx_block: {}", idx_block);
        }

        // Binary search for value index
        let (idx_value, equals) = binary_search_by(
            &self.data[idx_block],
            |x| (self.comparator)(&x, &t),
        );
        if equals {
            return false;
        }
        // println!("idx_value: {}", idx_value);

        // Value insert
        let block_len = self.data[idx_block].len();
        if idx_value < block_len {
            // println!("block: {:?}", self.data[idx_block]);
            self.data[idx_block].insert(idx_value, t);
            // println!("block: {:?}", self.data[idx_block]);
        } else {
            self.data[idx_block].push(t);
        }

        self.num_elements += 1;
        true
    }

    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        let mut i = 0;
        for block in &self.data {
            for x in block {
                f(i, x);
                i += 1;
            }
        }
    }

    pub fn collect(&self) -> Vec<T> {
        let mut data = Vec::with_capacity(self.num_elements);
        self.traverse(|_, x| data.push(x.clone()));
        data
    }

    pub fn debug(&self) {
        println!("{:?}", self.data);
    }

    fn new_block(&self, t: T) -> Vec<T> {
        let mut block = Vec::with_capacity(self.capacity as usize);
        block.push(t);
        block
    }
}


pub fn binary_search_by<T, F>(data: &[T], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    if data.len() == 0 {
        return (data.len(), false);
    }
    let mut l: usize = 0;
    let mut r: usize = data.len();

    while r > l {
        let mid = l + (r - l) / 2;

        let mid_el = &data[mid];
        // println!("{} {} {} {:?}", l, r, mid, mid_el);

        let cmp = f(mid_el);
        match cmp {
            Ordering::Greater => {
                r = mid;
            }
            Ordering::Equal => {
                return (mid, true)
            }
            Ordering::Less => {
                l = mid + 1;
            }
        }
    }

    (r, false)
}


/*
pub fn find_last_block_smaller<T, F>(data: &[Vec<T>], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    (0, false)
}

pub fn find_insert_index<T, F>(data: &[Vec<T>], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    (0, false)
}
*/

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;
    use pretty_assertions::assert_eq;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    fn int_comparator(a: &i32, b: &i32) -> Ordering {
        a.cmp(b)
    }

    macro_rules! new_array {
        ($capacity:expr, $data:expr) => {{
            let data: Vec<Vec<i32>> = $data;
            let num_elements = data.iter().map(|block| block.len()).sum();
            ArrayTree {
                comparator: int_comparator,
                capacity: $capacity,
                data: $data,
                num_elements,
            }
        }};
    }
    macro_rules! insert_many {
        ($at:expr, $data:expr) => {
            for x in $data.iter() {
                $at.insert(x.clone());
            }
        };
    }

    #[test]
    fn test_array_tree_prefers_push() {
        let mut at = new_array!(16, vec![vec![1, 2], vec![4, 5]]);
        assert_eq!(at.num_elements, 4);
        at.insert(3);
        assert_eq!(at.data, [vec![1, 2, 3], vec![4, 5]]);
        assert_eq!(at.num_elements, 5);
    }

    #[test]
    fn test_array_no_index_hiccup() {
        let mut at = new_array!(8, vec![vec![2], vec![4], vec![6, 8]]);
        at.insert(7);
        assert_eq!(at.data, [vec![2], vec![4], vec![6, 7, 8]]);
    }

    #[test]
    fn test_array_tree_split() {
        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(1);
        assert_eq!(at.data, [vec![1, 2], vec![4], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(3);
        assert_eq!(at.data, [vec![2, 3], vec![4], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(5);
        assert_eq!(at.data, [vec![2], vec![4, 5], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(7);
        assert_eq!(at.data, [vec![2, 4], vec![6, 7], vec![8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(9);
        assert_eq!(at.data, [vec![2, 4], vec![6], vec![8, 9]]);
        assert_eq!(at.num_elements, 5);
    }

    #[test]
    fn test_array_tree_collect() {
        for cap in vec![2, 3, 4, 5] {
            let mut at = ArrayTree::new(int_comparator, cap as u16);
            insert_many!(at, [1, 2, 3, 4]);
            assert_eq!(at.collect(), [1, 2, 3, 4]);

            let mut at = ArrayTree::new(int_comparator, cap as u16);
            insert_many!(at, [1, 2, 3, 4]);
            assert_eq!(at.collect(), [1, 2, 3, 4]);
        }
    }

    //#[ignore]
    #[test]
    fn test_failing() {
        let mut at = ArrayTree::new(|a: &f64, b: &f64| a.partial_cmp(b).unwrap(), 16);
        let vals = vec![0.6994135560499647, 0.15138991083383901, 0.17989509662598502, 0.22855960374503625, 0.7394173591733456, 0.8606810583068278, 0.025843624735059523, 0.1416162372765526, 0.9789425643425963, 0.6312677864630949, 0.34678659888024466, 0.7876614416763924, 0.6260871506068197, 0.34733559592131624, 0.5722923635764159, 0.14416998787798063, 0.839158671060864, 0.2621428817535354, 0.9334439919690996, 0.016414089291711065, 0.8795903741012259, 0.051958655798298614, 0.8313985552845266, 0.026928982020677505, 0.779969564116276, 0.6437306675337413, 0.03822809941255523, 0.777911020749552, 0.4639770428538855, 0.7039388191038694, 0.31363729764551374, 0.8111651227165783, 0.5174339383176408, 0.49384841003283086, 0.5214549475595969, 0.0823716635367353, 0.7310183483079477, 0.6196297749276181, 0.6226877845880779, 0.8987550167723078, 0.9536731852226494, 0.2719858776118911, 0.837006810218081, 0.7570466272336563, 0.9649096907962248, 0.09547804495341239, 0.26299769639555115, 0.6883529379785718, 0.23545125345269502, 0.5611223421257663, 0.81145380876482, 0.7821846165410649, 0.8385374221326543, 0.2287909449815878, 0.9938012642875733, 0.30515950398348823, 0.021945251189301795, 0.7456118789178752, 0.24917873250483202, 0.19461925257672297, 0.08596890658908873, 0.8208413553993631, 0.2799020116906893, 0.622583855342935, 0.3406868767224045, 0.7125811318179431, 0.8171813899535424, 0.9875530622413784, 0.8124194427320398, 0.27890169087536465, 0.4582999489551358, 0.8170130026270258, 0.1116683852975886, 0.9523649049789342, 0.1626401579175366, 0.7006463636943299, 0.5396656897339597, 0.73824000529768, 0.8975902131523751, 0.3138666758196337, 0.959190654990596, 0.6786382471256971, 0.8807317907186307, 0.9923109213923168, 0.7704353170122445, 0.20331717853087872, 0.9191784945915048, 0.3458975102965529, 0.44567705127366397, 0.08758863415076357, 0.8940937525362007, 0.2046747373689708, 0.1540080303289173, 0.8088614347095653, 0.09821866105193844, 0.050284880746519045, 0.9585396829998039, 0.35100273069739263, 0.8263845327940142, 0.6305932414080216];
        for (i, x) in vals.iter().enumerate() {
            at.insert(*x);
            let mut expected = vals[0 .. i + 1].to_vec();
            expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert_eq!(at.collect(), expected);
        }


    }
}