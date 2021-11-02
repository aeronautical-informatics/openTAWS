// Allow us to iterate over the functionalities for a lack of inline comptime loop unrolling
macro_rules! functionalities {
    [$( $functionality_name:tt ),+] => {
        ///
        fn get_functionality(&self, alert_system: Alert) -> &dyn AlertSystem<'a> {
            match alert_system {
            $(
                $functionality_name => casey::lower!(&self.$functionality_name),
            )+
            }
        }

        fn get_mut_functionality(&mut self, alert_system: Alert) -> &mut dyn AlertSystem<'a> {
            match alert_system {
            $(
                $functionality_name => casey::lower!(&mut self.$functionality_name),
            )+
            }
        }

        fn functionality_mut_array(&mut self)->[(Alert, &mut dyn AlertSystem<'a>); count!($($functionality_name)+)]{
            [ $(
                (
                    $functionality_name,
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
