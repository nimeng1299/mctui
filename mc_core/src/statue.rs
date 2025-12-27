pub enum Status<T,P,E> {
    Success(T),
    Progress(P),
    Failed(E),
}

impl<T,P,E> Status<T,P,E> {
    pub fn is_success(&self) -> bool {
        matches!(self, Status::Success(_))
    }
    pub fn is_progress(&self) -> bool {
        matches!(self, Status::Progress(_))
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Status::Failed(_))
    }
    pub fn to_success(self) -> Option<T> {
        match self {
            Status::Success(t) => Some(t),
            _ => None,
        }
    }
    pub fn to_progress(self) -> Option<P> {
        match self {
            Status::Progress(p) => Some(p),
            _ => None,
        }
    }
    pub fn to_error(self) -> Option<E> {
        match self {
            Status::Failed(e) => Some(e),
            _ => None,
        }
    }
}
