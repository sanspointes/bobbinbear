use std::marker::PhantomData;
use std::ops::Deref;

use bevy::{
    ecs::{prelude::*, reflect::ReflectComponent},
    log::error,
    reflect::Reflect,
};
use bevy_spts_uid::{Uid, UidRegistry};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ProxiedComponent<T: Component, TState = ()> {
    target: Uid,
    state: TState,
    #[reflect(ignore)]
    pd: PhantomData<T>,
}

impl<T: Component, TState> ProxiedComponent<T, TState> {
    pub fn new(target: Uid, state: TState) -> Self {
        Self {
            target,
            state,
            pd: PhantomData,
        }
    }

    pub fn target(&self) -> &Uid {
        &self.target
    }

    pub fn state(&self) -> &TState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut TState {
        &mut self.state
    }
}

/// Simple generic system implementation that copies the value from the target to the proxy.
/// Ignores the ProxiedComponent state.  See [ProxiedPosition] for a proxy with special behaviour.
pub fn sys_update_proxied_component<T: Component + PartialEq + Copy>(
    mut q_proxied: Query<(&mut T, &ProxiedComponent<T>)>,
    q_proxy_source: Query<&T, Without<ProxiedComponent<T>>>,
    uid_registry: Res<UidRegistry>,
) {
    for (mut proxy_value, proxy) in q_proxied.iter_mut() {
        let Ok(target_entity) = uid_registry.get_entity(*proxy.target()) else {
            error!(
                "Couldn't update value on proxy from target ({}) because it references a uid that's not in the UidRegistry.",
                proxy.target(),
            );
            continue;
        };

        match q_proxy_source.get(target_entity) {
            Ok(target_value) => {
                if *target_value != *proxy_value.deref() {
                    *proxy_value = *target_value;
                }
            }
            Err(reason) => error!(
                "sys_update_proxied_component<T>() couldn't get target({}) of proxy because: {}",
                proxy.target(),
                reason
            ),
        }
    }
}
