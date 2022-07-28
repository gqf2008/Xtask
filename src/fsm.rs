//! 通用状态机

use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalTransition,
}

pub struct Transition<S, E> {
    from: S,
    event: Option<E>,
    guard: Option<Box<dyn Fn() -> Result<()>>>,
    enter: Option<Box<dyn Fn() -> Result<()>>>,
    exit: Option<Box<dyn Fn() -> Result<()>>>,
    to: Option<S>,
}

impl<S: PartialEq, E: PartialEq> PartialEq for Transition<S, E> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.event == other.event && self.to == other.to
    }
}

impl<S, E> Transition<S, E>
where
    S: PartialEq + Eq + Copy,
    E: PartialEq + Eq + Copy,
{
    pub fn when(state: S) -> Self {
        Self {
            from: state,
            event: None,
            to: None,
            guard: None,
            enter: None,
            exit: None,
        }
    }
    pub fn on(mut self, event: E) -> Self {
        self.event = Some(event);
        self
    }
    pub fn to(mut self, state: S) -> Self {
        self.to = Some(state);
        self
    }

    pub fn guard<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Result<()> + Send + 'static,
    {
        self.guard = Some(Box::new(f));
        self
    }

    pub fn exit<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Result<()> + Send + 'static,
    {
        self.exit = Some(Box::new(f));
        self
    }

    pub fn enter<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Result<()> + Send + 'static,
    {
        self.enter = Some(Box::new(f));
        self
    }
}

pub struct Machine<S, E> {
    state: S,
    transitions: Vec<Transition<S, E>>,
}

impl<S, E> Machine<S, E>
where
    S: PartialEq + Eq + Copy,
    E: PartialEq + Eq + Copy,
{
    pub fn new(state: S) -> Self {
        Self {
            state,
            transitions: vec![],
        }
    }

    pub fn transit(mut self, trans: Transition<S, E>) -> Self {
        self.transitions.push(trans);
        self
    }
}

impl<S, E> Machine<S, E>
where
    S: PartialEq + Eq + Copy,
    E: PartialEq + Eq + Copy,
{
    pub fn state(&self) -> S {
        self.state
    }

    pub fn on(&mut self, ev: E) -> Result<Action<S>> {
        for trans in &self.transitions {
            if trans.from == self.state && trans.event.unwrap() == ev {
                if let Some(f) = trans.guard.as_ref() {
                    f()?;
                }
                if self.state != trans.to.unwrap() {
                    if let Some(f) = trans.exit.as_ref() {
                        f()?;
                    }
                    self.state = trans.to.unwrap();
                    if let Some(f) = trans.enter.as_ref() {
                        f()?;
                    }
                }
                return Ok(Action(self.state));
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
