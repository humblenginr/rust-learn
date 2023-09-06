// we use associated types, opposed to generic trait parameters, if we know that there can only be one implementation of the trait for a particular type
// the argument(not very strong) is that if we use generic trait parameters, the compiler needs to do some extra
// work to figure out the right implementation
// trait Iterator {
//     // an associated type
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// same trait using generic trait parameters
// trait Iterator<Item> {
//     fn next(&mut self) -> Option<Item>;
// }
//

// pub fn flatten<O>(iter: O) -> Flatten<O> {
//     Flatten::new(iter)
// }

// pub struct Flatten<O> {
//     outer: O,
// }

// impl<O> Flatten<O> {
//     pub fn new(outer: O) -> Self {
//         Self { outer }
//     }
// }

// impl<O> Iterator for Flatten<O>
// where
//     O: Iterator,
//     O::Item: IntoIterator,
// {
//     type Item = <O::Item as IntoIterator>::Item;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.outer.next().and_then(|inner| inner.into_iter().next())
//     }
// }

struct Counter {
    vec: Vec<usize>,
}

impl IntoIterator for Counter {
    type Item = usize;
    type IntoIter = Vec<usize>;
    fn into_iter(self) -> Self::IntoIter {}
}

fn main() {
    let ctr = Counter { count: 0 };
    // for v in vs {
    //     // consumes v and gives owned value
    // }
    // for v in vs.iter() {
    //     // borrows vs, gives & to v
    // }
    // for v in &vs {
    //     // borrows vs, gives & to v
    // }
}
