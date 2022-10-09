//! 通用状态机
use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalTransition,
}

pub struct Transition<S, E, C> {
    from: S,
    event: Option<E>,
    to: Option<S>,
    guard: Option<Box<dyn Fn(S, E, &mut C) -> Result<()>>>,
    exit: Option<Box<dyn Fn(S, E, &mut C) -> Result<()>>>,
    enter: Option<Box<dyn Fn(S, E, S, &mut C) -> Result<()>>>,
    action: Option<Box<dyn Fn(E, S, &mut C) -> Result<()>>>,
}

impl<S: PartialEq, E: PartialEq, C> PartialEq for Transition<S, E, C> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.event == other.event && self.to == other.to
    }
}

pub struct Machine<S, E, C> {
    state: S,
    transitions: BTreeMap<S, BTreeMap<E, Transition<S, E, C>>>,
}

impl<S, E, C> Machine<S, E, C>
where
    S: PartialEq + Eq + Ord + Copy,
    E: PartialEq + Eq + Ord + Copy,
{
    pub fn state(&self) -> S {
        self.state
    }

    pub fn event(&mut self, ev: E, ctx: &mut C) -> Result<()> {
        if let Some(tran) = self.transitions.get_mut(&self.state) {
            if let Some(tran) = tran.get_mut(&ev) {
                if let Some(f) = tran.guard.as_ref() {
                    f(self.state, ev, ctx)?;
                }
                if self.state != tran.to.unwrap() {
                    if let Some(f) = tran.exit.as_ref() {
                        f(self.state, ev, ctx)?;
                    }
                    let from = self.state;
                    self.state = tran.to.unwrap();
                    if let Some(f) = tran.enter.as_ref() {
                        f(from, ev, self.state, ctx)?;
                    }
                }
                if let Some(f) = tran.action.as_ref() {
                    f(ev, self.state, ctx)?;
                }
                return Ok(());
            }
        }

        Err(Error::IllegalTransition)
    }
}

pub struct Action<S>(S);

impl<S> Action<S> {
    pub fn enforce<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&S) -> Result<T> + Send + 'static,
    {
        f(&self.0)
    }
}

pub struct Builder<S: PartialEq + Eq + Ord + Copy, E: PartialEq + Eq + Ord + Copy, C> {
    transitions: BTreeMap<S, BTreeMap<E, Transition<S, E, C>>>,
    transition: Option<Transition<S, E, C>>,
}

impl<S: PartialEq + Eq + Ord + Copy, E: PartialEq + Eq + Ord + Copy, C> Builder<S, E, C> {
    pub fn new() -> Self {
        Self {
            transitions: BTreeMap::new(),
            transition: None,
        }
    }

    pub fn build(self, state: S) -> Machine<S, E, C> {
        Machine {
            state,
            transitions: self.transitions,
        }
    }

    pub fn when(mut self, state: S) -> Self {
        self.transition = Some(Transition {
            from: state,
            event: None,
            to: None,
            guard: None,
            exit: None,
            enter: None,
            action: None,
        });
        self
    }
    pub fn on(mut self, event: E) -> Self {
        if let Some(trans) = self.transition.as_mut() {
            trans.event.replace(event);
        }
        self
    }

    pub fn to(mut self, state: S) -> Self {
        if let Some(transition) = self.transition.as_mut() {
            transition.to.replace(state);
            let from = transition.from;
            let event = transition.event;
            let tran = Transition {
                from,
                event,
                to: Some(state),
                guard: None,
                exit: None,
                enter: None,
                action: None,
            };
            if let Some(trans) = self.transitions.get_mut(&from) {
                trans.insert(event.unwrap(), tran);
            } else {
                let mut trans = BTreeMap::new();
                trans.insert(event.unwrap(), tran);
                self.transitions.insert(from, trans);
            }
        }

        self
    }

    pub fn transit(self, on: E, to: S) -> Self {
        self.on(on).to(to)
    }

    pub fn guard<F>(mut self, f: F) -> Self
    where
        F: Fn(S, E, &mut C) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.guard = Some(Box::new(f));
        }
        self
    }

    pub fn exit<F>(mut self, f: F) -> Self
    where
        F: Fn(S, E, &mut C) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.exit = Some(Box::new(f));
        }
        self
    }

    pub fn enter<F>(mut self, f: F) -> Self
    where
        F: Fn(S, E, S, &mut C) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.enter = Some(Box::new(f));
        }
        self
    }

    pub fn action<F>(mut self, f: F) -> Self
    where
        F: Fn(E, S, &mut C) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.action = Some(Box::new(f));
        }
        self
    }

    #[inline]
    fn find<'a>(&'a mut self) -> Option<&'a mut Transition<S, E, C>> {
        if let Some(transition) = self.transition.as_ref() {
            let from = &transition.from;
            if let Some(event) = transition.event.as_ref() {
                if let Some(trans) = self.transitions.get_mut(from) {
                    trans.get_mut(event)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
