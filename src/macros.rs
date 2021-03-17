// Allow us to iterate over the functionalities for a lack of inline comptime loop unrolling
macro_rules! functionalities {
    [$( $functionality_name:tt ),+] => {
        ///
        fn get_functionality(&self, alert_system: Alert) -> &dyn AlertSystem {
            match alert_system {
            $(
                $crate::alerts::Alert::$functionality_name => casey::lower!(&self.$functionality_name),
            )+
            }
        }

        fn get_mut_functionality(&mut self, alert_system: Alert) -> &mut dyn AlertSystem {
            match alert_system {
            $(
                $crate::alerts::Alert::$functionality_name => casey::lower!(&mut self.$functionality_name),
            )+
            }
        }

        fn functionality_mut_array(&mut self)->[(Alert, &mut dyn AlertSystem); count!($($functionality_name)+)]{
            [ $(
                (
                    $crate::alerts::Alert::$functionality_name,
                    casey::lower!(&mut self.$functionality_name) as &mut dyn AlertSystem
                ),
            )+ ]
        }
    };
}

// Count stuff, for a lack of advanced const generics
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

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
