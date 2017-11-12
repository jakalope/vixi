use mode_map::MapErr;
use state::State;
use std::marker::PhantomData;
use typeahead::Numeric;

pub trait Transition<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
{
    fn name(&self) -> &'static str;
    fn transition(&self, state: &mut State<K>) -> Mode<K>;
}

#[derive(Clone, Copy, Debug)]
pub struct NormalMode<K> {
    t: PhantomData<K>,
    pub count: i32,
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
    pub count: i32,
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

pub fn normal<K>(count: i32) -> Mode<K> {
    Mode::Normal(NormalMode::<K> {
        t: PhantomData::<K> {},
        count: count,
    })
}

pub fn recast_normal<K>(orig: &NormalMode<K>) -> Mode<K> {
    Mode::Normal(NormalMode::<K> {
        t: PhantomData::<K> {},
        count: orig.count,
    })
}

pub fn pending<K>(next_mode: NextMode, count: i32) -> Mode<K> {
    Mode::Pending(PendingMode::<K> {
        t: PhantomData::<K> {},
        next_mode: next_mode,
        count: count,
    })
}

pub fn recast_pending<K>(orig: &PendingMode<K>) -> Mode<K> {
    Mode::Pending(PendingMode::<K> {
        t: PhantomData::<K> {},
        next_mode: orig.next_mode,
        count: orig.count,
    })
}

pub fn insert<K>() -> Mode<K> {
    Mode::Insert(InsertMode::<K> { t: PhantomData::<K> {} })
}

impl<K> Transition<K> for Mode<K>
where
    K: Ord,
    K: Copy,
    K: Numeric,
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
