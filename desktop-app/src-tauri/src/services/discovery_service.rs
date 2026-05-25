use mdns_sd::{ServiceDaemon, ServiceInfo};

pub fn start_discovery_service() {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");

    let service_type = "_openbridge._tcp.local.";
    let instance_name = "OpenBridge Desktop";

    let host_name = "openbridge.local.";
    let port = 9001;

    let my_ip = "127.0.0.1";

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        my_ip,
        port,
        None,
    )
    .expect("Failed to create service info");

    mdns.register(service_info)
        .expect("Failed to register mDNS service");

    println!("mDNS discovery service started");
}