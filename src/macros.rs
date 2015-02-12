#[macro_export]
macro_rules! behaviour_sequence(
    {$name:expr, [$($value:expr),*]} => {
        {
            Sequence::with_children($name, vec![
                $(
                    Box::new($value),
                )+
            ])
        }
    };
);

#[macro_export]
macro_rules! behaviour_selector(
    {$name:expr, [$($value:expr),*]} => {
        {
            Selector::with_children($name, vec![
                $(
                    Box::new($value),
                )+
            ])
        }
    };
);

#[macro_export]
macro_rules! condition_decorator(
    {$name:expr, $value:expr, $child:expr} => {
        {
            ConditionalDecorator::with_child($name, $value, Box::new($child))
        }
    };
);

#[macro_export]
macro_rules! condition(
    {$name:expr, $value:expr} => {
        {
            Conditional::new($name, $value)
        }
    };
);

#[macro_export]
macro_rules! action(
    {$name:expr, $value:expr} => {
        {
            Action::new($name, $value)
        }
    };
);

#[macro_export]
macro_rules! node(
    {$name:expr, $value:expr} => {
        {
            Node::new($name, $value)
        }
    };
);
