pub mod linked_list {
#[derive(Debug)]
    /// A linked list
    /// It is either empty or has a head and a tail
    /// Box is A type for heap allocation.
    pub enum List<T> {
        Nil,
        Cons(T, Box<List<T>>),
    }

    use crate::linked_list::List::{Nil, Cons};

    /// Clone lists
    impl<T : Clone> Clone for List<T> {
        fn clone(self : &List<T>) -> List<T> {
            match self {
                Nil => Nil,
                Cons(x, xs) => Cons(x.clone(), xs.clone()),
            }
        }
    }

    /// foldl :: (b -> a -> b) -> b -> [a] -> b
    ///
    /// Use a reference `xs`.
    pub fn foldl<A : Copy, B : Copy>(folder : fn(B, A) -> B, seed : B, xs : &List<A>) -> B {
        match xs {
            Nil => seed,
            Cons(x, xss) => foldl(folder, folder(seed, *x), xss),
        }
    }

    /// map :: (a -> b) -> [a] -> [b]
    pub fn map<A, B, F>(f : F, xs : &List<A>) -> List<B> where
        F : Fn(&A) -> B {
            match xs {
                Nil => Nil,
                Cons(x, xss) => Cons(f(x), Box::new(map(f, xss))),
            }
        }

    /// concat, This version reuses the lists.
    pub fn concat<A>(xs : List<A>, ys : List<A>) -> List<A> {
        match xs {
            Nil => ys,
            Cons(x, xss) => Cons(x, Box::new(concat(*xss, ys))),
        }
    }

    /// concat_map :: (a -> [b]) -> [a] -> [b]
    ///
    /// Creates a new list.
    pub fn concat_map<A, B, F>(f : F, xs : &List<A>) -> List<B> where
        F : Fn(&A) -> List<B> {
            match xs {
                Nil => Nil,
                Cons(x, xss) => concat(f(x), concat_map(f, xss)),
            }
        }

    /// tails :: [x] -> [[x]]
    /// tails [1,2] = [[1,2],[2],[]]
    ///
    /// Creates new lists
    pub fn tails<T : Clone>(xs : &List<T>) -> List<List<T>> {
        match xs {
            Nil => Nil,
            Cons(x, xss) => Cons(Cons(x.clone(), xss.clone()), Box::new(tails(xss))),
        }
    }

    /// zip_with :: (a -> b -> c) -> [a] -> [b] -> [c]
    pub fn zip_with<A, B, C, F>(f : F, xs : &List<A>, ys : &List<B>) -> List<C> where
        F : Fn(&A, &B) -> C {
            match xs {
                Nil => Nil,
                Cons(x, xss) => match ys {
                    Nil => Nil,
                    Cons(y, yss) => Cons(f(x, y), Box::new(zip_with(f, xss, yss))),
                }
            }
        }

    /// elem :: Eq a => a -> [a] -> Bool
    pub fn elem<T : Eq>(value : T, xs : &List<T>) -> bool {
        match xs {
            Nil => false,
            Cons(x, tail) => *x == value || elem(value, &tail),
        }
    }

    /// Create a list with `length` elements.
    pub fn range(length : i32) -> List<i32> {
        if length == 0 {
            return Nil;
        } else {
            return Cons(length, Box::new(range(length - 1)));
        }
    }

}
