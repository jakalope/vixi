use mode_map::MapErr;
use op::InsertOp;
use op::NormalOp;
use state::State;
use std::marker::PhantomData;

pub trait Transition<K> where K: Ord, K: Copy {
    fn name(&self) -> &'static str;
    fn transition(&self, state: &mut State<K>) -> Mode<K>;
}

#[derive(Clone, Copy, Debug)]
pub struct NormalMode<K> { t: PhantomData<K>  }

#[derive(Clone, Copy, Debug)]
pub struct InsertMode<K> { t: PhantomData<K> }

pub enum Mode<K> {
    Normal(NormalMode<K>),
    Insert(InsertMode<K>),
}

pub fn normal<K>() -> Mode<K> {
    Mode::Normal(NormalMode::<K>{ t: PhantomData::<K>{} })
}

pub fn insert<K>() -> Mode<K> {
    Mode::Insert(InsertMode::<K>{ t: PhantomData::<K>{} })
}

impl<K> Transition<K> for Mode<K> where K: Ord, K: Copy {
    fn name(&self) -> &'static str {
        match *self {
            Mode::Normal(x) => x.name(),
            Mode::Insert(x) => x.name(),
        }
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match *self {
            Mode::Normal(x) => x.transition(state),
            Mode::Insert(x) => x.transition(state),
        }
    }
}

