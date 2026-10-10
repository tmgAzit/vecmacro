#[macro_export]
macro_rules! avec {
    ($($element: expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut vs = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
            $(vs.push($element);)*
            vs
        }
    };

    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};

    ($element: expr; $count: expr) => {
        {
            let mut vs = Vec::new();
            vs.resize($count, $element);
            vs
        }
    };
}

#[macro_export]
#[doc(hidden)]

macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };

    (@SUBST; $($element:expr),*) => {
        () 
    };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![32];
    assert_eq!(x.len(), 1);
    assert!(!x.is_empty());
}

#[test]
fn double() {
    let x: Vec<u32> = avec![32, 33];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 32);
    assert_eq!(x[1], 33);
}

#[test]
fn trailing() {
    let x: Vec<u32> = avec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert!(!x.is_empty());
}
