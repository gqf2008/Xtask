//! 通用状态机
use core::fmt::Debug;

use alloc::collections::BTreeMap;
use alloc::rc::Rc;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalTransition,
}

#[derive(Clone)]
pub struct Transition<S, E> {
    from: S,
    event: Option<E>,
    to: Option<S>,
    guard: Option<Rc<dyn Fn(&S, &E) -> Result<()>>>,
    exit: Option<Rc<dyn Fn(&S, &E) -> Result<()>>>,
    enter: Option<Rc<dyn Fn(&E, &S) -> Result<()>>>,
    action: Option<Rc<dyn Fn(&S, &E) -> Result<()>>>,
}

impl<S: PartialEq, E: PartialEq> PartialEq for Transition<S, E> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.event == other.event && self.to == other.to
    }
}

pub struct Machine<S, E> {
    state: S,
    transitions: BTreeMap<S, BTreeMap<E, Transition<S, E>>>,
}

impl<S, E> Machine<S, E>
where
    S: PartialEq + Eq + Ord + Copy,
    E: PartialEq + Eq + Ord + Copy,
{
    pub fn state(&self) -> S {
        self.state
    }

    pub fn event(&mut self, ev: E) -> Result<()> {
        if let Some(tran) = self.transitions.get_mut(&self.state) {
            if let Some(tran) = tran.get_mut(&ev) {
                if let Some(f) = tran.guard.as_ref() {
                    f(&self.state, &ev)?;
                }
                if self.state != tran.to.unwrap() {
                    if let Some(f) = tran.exit.as_ref() {
                        f(&self.state, &ev)?;
                    }
                    self.state = tran.to.unwrap();
                    if let Some(f) = tran.enter.as_ref() {
                        f(&ev, &self.state)?;
                    }
                }
                if let Some(f) = tran.action.as_ref() {
                    f(&self.state, &ev)?;
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

pub struct Builder<S: PartialEq + Eq + Ord + Copy, E: PartialEq + Eq + Ord + Copy> {
    transitions: BTreeMap<S, BTreeMap<E, Transition<S, E>>>,
    transition: Option<Transition<S, E>>,
}

impl<S: PartialEq + Eq + Ord + Copy, E: PartialEq + Eq + Ord + Copy> Builder<S, E> {
    pub fn new() -> Self {
        Self {
            transitions: BTreeMap::new(),
            transition: None,
        }
    }

    pub fn build(self, state: S) -> Machine<S, E> {
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
            let event = transition.event.unwrap();
            if let Some(trans) = self.transitions.get_mut(&from) {
                trans.insert(event, transition.clone());
            } else {
                let mut trans = BTreeMap::new();
                trans.insert(event, transition.clone());
                self.transitions.insert(from, trans);
            }
        }

        self
    }
    pub fn guard<F>(mut self, f: F) -> Self
    where
        F: Fn(&S, &E) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.guard = Some(Rc::new(f));
        }
        self
    }
    pub fn action<F>(mut self, f: F) -> Self
    where
        F: Fn(&S, &E) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.action = Some(Rc::new(f));
        }
        self
    }

    pub fn exit<F>(mut self, f: F) -> Self
    where
        F: Fn(&S, &E) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.exit = Some(Rc::new(f));
        }
        self
    }

    pub fn enter<F>(mut self, f: F) -> Self
    where
        F: Fn(&E, &S) -> Result<()> + Send + 'static,
    {
        if let Some(tran) = self.find() {
            tran.enter = Some(Rc::new(f));
        }
        self
    }

    #[inline]
    fn find<'a>(&'a mut self) -> Option<&'a mut Transition<S, E>> {
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
