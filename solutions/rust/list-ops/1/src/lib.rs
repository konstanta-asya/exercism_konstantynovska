pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut current: Option<I::Item> = None;
    std::iter::from_fn(move || loop {
        match current.as_mut().and_then(|it| it.next()) {
            Some(item) => return Some(item),
            None => match nested_iter.next() {
                Some(next_iter) => current = Some(next_iter),
                None => return None,
            },
        }
    })
}

pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    std::iter::from_fn(move || {
        while let Some(item) = iter.next() {
            if predicate(&item) {
                return Some(item);
            }
        }
        None
    })
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut count = 0;
    while iter.next().is_some() {
        count += 1;
    }
    count
}

pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || iter.next().map(&function))
}

pub fn foldl<I, F, U>(mut iter: I, mut acc: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next() {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, mut acc: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next_back() {
        acc = function(acc, item);
    }
    acc
}

pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}
