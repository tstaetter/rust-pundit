//! Pundit library

mod policy;

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
}

pub type PunditResult<E> = anyhow::Result<E, PunditError>;

pub trait PunditAuthenticatable: Sized {}

pub trait Policy {
    const NAME: &'static str = "Zebra";
    /// Try creating a new instance of policy instance
    fn try_new() -> PunditResult<Self> where Self: Sized;
}


#[cfg(test)]
mod tests {
    use crate::{Policy, PunditResult};

    #[test]
    fn can_create_policy() -> anyhow::Result<()> {
        struct TestStruct {}
        impl Policy for TestStruct {
            fn try_new() -> PunditResult<Self> where Self: Sized {
                Ok(Self {})
            }
        }

        Ok(())
    }
}
