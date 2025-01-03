use crossbeam_channel::Sender;
use leaf::LeafCommand;
use std::collections::HashSet;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NodeInfo {
    pub neighbours: HashSet<NodeId>,
    pub packet_in_channel: Sender<Packet>,
    pub type_info: TypeInfo,
}

pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

pub struct DroneInfo {
    pub pdr: f32,
    pub command_send_channel: Sender<DroneCommand>,
}

pub struct LeafInfo {
    pub command_send_channel: Sender<LeafCommand>,
}
