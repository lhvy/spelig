use arrayvec::ArrayVec;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) static KEYS: Lazy<HashMap<char, ArrayVec<char, 6>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert('q', IntoIterator::into_iter(['1', 'a', 'w']).collect());
    m.insert(
        'w',
        IntoIterator::into_iter(['2', '3', 'q', 'e', 's']).collect(),
    );
    m.insert(
        'e',
        IntoIterator::into_iter(['3', '4', 'w', 's', 'd']).collect(),
    );
    m.insert(
        'r',
        IntoIterator::into_iter(['4', '5', 'e', 't', 'd', 'f']).collect(),
    );
    m.insert('t', IntoIterator::into_iter(['5', 'r', 'f', 'g']).collect());
    m.insert(
        'y',
        IntoIterator::into_iter(['t', '6', '7', 'g', 'h', 'u']).collect(),
    );
    m.insert(
        'u',
        IntoIterator::into_iter(['y', '7', '8', 'j', 'i']).collect(),
    );
    m.insert(
        'i',
        IntoIterator::into_iter(['u', '8', '9', 'j', 'k', 'o']).collect(),
    );
    m.insert(
        'o',
        IntoIterator::into_iter(['i', '9', '0', 'p', 'l']).collect(),
    );
    m.insert('p', IntoIterator::into_iter(['0', 'o', 'l']).collect());

    m.insert('a', IntoIterator::into_iter(['q', 'w', 's', 'z']).collect());
    m.insert(
        's',
        IntoIterator::into_iter(['a', 'w', 'd', 'x', 'z']).collect(),
    );
    m.insert(
        'd',
        IntoIterator::into_iter(['s', 'e', 'r', 'f', 'c', 'x']).collect(),
    );
    m.insert(
        'f',
        IntoIterator::into_iter(['d', 'r', 'g', 'v', 'c']).collect(),
    );
    m.insert(
        'g',
        IntoIterator::into_iter(['f', 't', 'h', 'b', 'v']).collect(),
    );
    m.insert(
        'h',
        IntoIterator::into_iter(['g', 'y', 'u', 'j', 'n', 'b']).collect(),
    );
    m.insert(
        'j',
        IntoIterator::into_iter(['h', 'u', 'i', 'k', 'm', 'n']).collect(),
    );
    m.insert(
        'k',
        IntoIterator::into_iter(['j', 'i', 'l', ',', 'm']).collect(),
    );
    m.insert(
        'l',
        IntoIterator::into_iter(['k', 'o', 'p', ';', '.', ',']).collect(),
    );

    m.insert('z', IntoIterator::into_iter(['a', 's', 'x']).collect());
    m.insert('x', IntoIterator::into_iter(['z', 's', 'd', 'c']).collect());
    m.insert('c', IntoIterator::into_iter(['x', 'd', 'f', 'v']).collect());
    m.insert('v', IntoIterator::into_iter(['c', 'f', 'g', 'b']).collect());
    m.insert('b', IntoIterator::into_iter(['v', 'g', 'h', 'n']).collect());
    m.insert('n', IntoIterator::into_iter(['b', 'h', 'j', 'm']).collect());
    m.insert('m', IntoIterator::into_iter(['n', 'j', 'k', ',']).collect());

    m.insert(';', IntoIterator::into_iter([':']).collect());
    m.insert(':', IntoIterator::into_iter([';']).collect());
    m.insert('\'', IntoIterator::into_iter(['"']).collect());
    m.insert('"', IntoIterator::into_iter(['\'']).collect());
    m.insert(',', IntoIterator::into_iter(['.']).collect());
    m.insert('.', IntoIterator::into_iter([',']).collect());

    m
});
