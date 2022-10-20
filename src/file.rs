use std::env::current_dir;
use std::path::PathBuf;

pub fn relative(path: &str) -> Result<PathBuf, std::io::Error> {
    current_dir().map(|p| p.join(path))
}

pub fn relative_(path: &str) -> PathBuf {
    relative(path).unwrap()
}

pub fn current_dir_name_() -> String {
    let binding = current_dir().unwrap();
    binding
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

#[macro_export]
macro_rules! path {
    ($($tt:tt)+) => {
        $crate::__tokenize_path!([] [] $($tt)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tokenize_path {
    ([$(($($component:tt)+))*] [$($cur:tt)+] / $($rest:tt)+) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)] [] $($rest)+)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)*] $first:tt $($rest:tt)*) => {
        $crate::__tokenize_path!([$(($($component)+))*] [$($cur)* $first] $($rest)*)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)+]) => {
        $crate::__tokenize_path!([$(($($component)+))* ($($cur)+)])
    };

    ([$(($($component:tt)+))*]) => {{
        let mut path = ::std::path::PathBuf::new();
        $(
            path.push(&($($component)+));
        )*
        path
    }};
}
