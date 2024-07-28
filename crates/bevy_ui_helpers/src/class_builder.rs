use bevy::utils::all_tuples;

pub trait ClassBuilder<T> {
    fn apply(&self, bundle: &mut T);
}

impl<T> ClassBuilder<T> for () {
    fn apply(&self, _: &mut T) {}
}

impl<F, T> ClassBuilder<T> for F
where
    F: Fn(&mut T),
{
    fn apply(&self, bundle: &mut T) {
        self(bundle)
    }
}

// impl<F1, F2, T> ClassBuilder<T> for (F1, F2)
// where
//     F1: ClassBuilder<T>,
//     F2: ClassBuilder<T>,
// {
//     fn apply(&self, bundle: &mut T) {
//         self.0.apply(bundle);
//         self.1.apply(bundle);
//     }
// }
macro_rules! tuple_impl {
    ($($name: ident),*) => {
        #[allow(non_snake_case)]
        impl<T, $($name: ClassBuilder<T>),*> ClassBuilder<T> for ($($name,)*)
        {
            fn apply(&self, bundle: &mut T) {
                let ($($name,)+) = self;
                ($($name.apply(bundle),)*);
            }
        }
    };
}

all_tuples!(tuple_impl, 1, 15, B);
// This generates tuple implementations:
// tuple_impl!(F1, F2);
// tuple_impl!(F1, F2, F3, ...);
