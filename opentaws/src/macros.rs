// A helper macro to implement the boring part of all AlertSystems
macro_rules! arm_inhibit {
    () => {
        fn is_armed(&self) -> bool {
            self.armed
        }

        fn arm(&mut self) {
            self.armed = true;
        }

        fn disarm(&mut self) {
            self.armed = false;
        }

        fn is_inhibited(&self) -> bool {
            self.inhibited
        }

        fn inhibit(&mut self) {
            self.inhibited = true;
        }

        fn uninhibit(&mut self) {
            self.inhibited = false;
        }
    };
}

macro_rules! armed_return {
    ($self:expr, $result:expr) => {
        if $self.is_armed() {
            $result
        } else {
            None
        }
    };
}
