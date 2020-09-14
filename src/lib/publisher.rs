use crate::lib::core::Publisher;

pub fn connect() -> impl Publisher + Sync + Send + 'static {
	LocalPublisher {}
}

struct LocalPublisher {}

impl Publisher for LocalPublisher {}

