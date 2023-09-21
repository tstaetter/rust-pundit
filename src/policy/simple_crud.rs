//! Simple CRUD actions for policies
use crate::{Policy, PunditResult, Action};

/// All possible actions to allow or deny for simple CRUD feature
pub enum SimpleCrudAction {
    Create,
    Read,
    Update,
    Delete,
}

/// Simple CRUD actions
pub trait SimpleCrudPolicy<T> : Policy<T> {
    /// Actually create a new resource
    fn create(&self, object: T) -> PunditResult<T> {
        self.allow(Action::SimpleCrudAction(SimpleCrudAction::Create), object)
    }

    /// Return identified resource
    fn read(&self, object: T) -> PunditResult<T> {
        self.allow(Action::SimpleCrudAction(SimpleCrudAction::Read), object)
    }

    /// Update resource using provided object values
    fn update(&self, object: T) -> PunditResult<T> {
        self.allow(Action::SimpleCrudAction(SimpleCrudAction::Update), object)
    }

    /// Delete resource using provided identifier
    fn delete(&self, object: T) -> PunditResult<T> {
        self.allow(Action::SimpleCrudAction(SimpleCrudAction::Delete), object)
    }
}
