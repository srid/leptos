use crate::{Runtime, RuntimeId};

/// A unique identifier for the reactive system connected to a
/// particular HTTP request to the server.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestScope(RuntimeId);

impl RequestScope {
    /// Gets the identifier for the current request.
    #[inline(always)]
    pub fn current() -> Self {
        Self(Runtime::current())
    }

    /// Sets this as the active request.
    #[inline(always)]
    pub fn enter(&self) {
        #[cfg(not(any(feature = "csr", feature = "hydrate")))]
        Runtime::set_runtime(Some(self.0));
    }
}

impl Default for RequestScope {
    fn default() -> RequestScope {
        Self::current()
    }
}
