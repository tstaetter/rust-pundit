//! Pundit library

mod policy;

pub use policy::*;

/// Error definitions
#[derive(thiserror::Error, Debug)]
pub enum PunditError {
    /// Occurs if given action is not authorized
    #[error("Not authorized: {0}")]
    NotAuthorized(anyhow::Error),
    /// Occurs if no authorization was performed
    #[error("No authorization performed: {0}")]
    AuthorizationNotPerformed(anyhow::Error),
    /// Occurs if no appropriate policy could be found
    #[error("No policy defined: {0}")]
    NotDefined(anyhow::Error),
    /// Occurs if registration failed
    #[error("Registration failed: {0}")]
    RegistrationFailed(anyhow::Error),
    #[error("Unknown error: {0}")]
    Unknown(anyhow::Error),
}

pub type PunditResult<E> = anyhow::Result<E, PunditError>;

/// Basic policy trait for creating real policies
pub trait Policy<T> {
    /// Try creating a new instance of policy instance
    fn try_new() -> PunditResult<Self> where Self: Sized;

    /// Allow given action
    fn allow(&self, object: T) -> PunditResult<T>;
}

/// You need to implement this trait in every type (e.g. some DAO) you want to apply a
/// policy on.
pub trait PunditAuthenticatable<T, P> {
    /// Apply provided policy
    fn apply_policy(&self, policy: impl Policy<T>) -> PunditResult<T> {
        policy.allow(self)
    }

    /// Return the value of the parameter used to apply the policy
    fn policy_parameter(&self) -> PunditResult<P>;
}


#[cfg(test)]
mod tests {
}
