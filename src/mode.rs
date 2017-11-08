use mode_map::MapErr;
use state::State;
use std::marker::PhantomData;

pub trait Transition<K>
where
    K: Ord,
    K: Copy,
{
    fn name(&self) -> &'static str;
    fn transition(&self, state: &mut State<K>) -> Mode<K>;
}

#[derive(Clone, Copy, Debug)]
pub struct NormalMode<K> {
    t: PhantomData<K>,
}

/// Used by `PendingMode` to remember what mode to transition to next.
#[derive(Clone, Copy, Debug)]
pub enum NextMode {
    Normal,
    Insert,
}

#[derive(Clone, Copy, Debug)]
pub struct PendingMode<K> {
    t: PhantomData<K>,
    pub next_mode: NextMode, // Mode to return to after motion or text object.
}

#[derive(Clone, Copy, Debug)]
pub struct InsertMode<K> {
    t: PhantomData<K>,
}

#[derive(Clone, Copy, Debug)]
pub enum Mode<K> {
    Normal(NormalMode<K>),
    Pending(PendingMode<K>),
    Insert(InsertMode<K>),
}

pub fn normal<K>() -> Mode<K> {
    Mode::Normal(NormalMode::<K> { t: PhantomData::<K> {} })
}

pub fn pending<K>(next_mode: NextMode) -> Mode<K> {
    Mode::Pending(PendingMode::<K> {
        t: PhantomData::<K> {},
        next_mode: next_mode,
    })
}

pub fn insert<K>() -> Mode<K> {
    Mode::Insert(InsertMode::<K> { t: PhantomData::<K> {} })
}

impl<K> Transition<K> for Mode<K>
where
    K: Ord,
    K: Copy,
{
    fn name(&self) -> &'static str {
        match *self {
            Mode::Normal(x) => x.name(),
            Mode::Pending(x) => x.name(),
            Mode::Insert(x) => x.name(),
        }
    }

    fn transition(&self, state: &mut State<K>) -> Mode<K> {
        match *self {
            Mode::Normal(x) => x.transition(state),
            Mode::Pending(x) => x.transition(state),
            Mode::Insert(x) => x.transition(state),
        }
    }
}
