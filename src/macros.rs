macro_rules! update_state {
    // macth like arm for macro
    ($state:ident.$field:ident = $value:expr) => {
        // macro expand to this code
        {
            $state.edit(|x| {
                x.$field = $value;
            });
        }
    };
}

macro_rules! use_mut_state {
    ($ui:ident.$fun:ident <= $state:ident.$field:ident) => {
        // macro expand to this code
        {
            $state.edit(|x| {
                $ui.$fun(&mut x.$field);
            });
        }
    };
}

macro_rules! get_state {

    ($self:ident) => {
        {
            let state = &*($self.state);
            let ro_state = state.read();
            (state, ro_state)
        }
    };
}

macro_rules! tokio_sleep {

    ($millis:tt) => {
        {
            tokio::time::sleep(tokio::time::Duration::from_millis($millis)).await;
        }
    };
}
// tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
pub(crate) use get_state;
pub(crate) use use_mut_state;
pub(crate) use update_state;
pub(crate) use tokio_sleep;