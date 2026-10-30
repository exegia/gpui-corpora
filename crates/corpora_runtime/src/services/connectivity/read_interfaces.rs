#[allow(unused_imports)]
use super::*;

pub(crate) fn read_interfaces() -> Vec<String> {
    network_interface::NetworkInterface::show()
        .map(|ifaces| {
            ifaces
                .into_iter()
                .map(|iface| format!("{}:{}", iface.name, iface.addr.len()))
                .collect()
        })
        .unwrap_or_default()
}
