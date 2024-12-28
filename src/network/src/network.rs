use std::collections::HashMap;
use crossbeam_channel::{Receiver, Sender};
use wg_2024::controller::DroneEvent;
use wg_2024::network::NodeId;
use leaf::LeafPacketSentEvent;
use crate::info::NodeInfo;

#[allow(dead_code)]
pub struct Network {
    pub topology: HashMap<NodeId, NodeInfo>,
    pub simulation_channels: SimulationChannels,
}

#[allow(dead_code)]
pub struct SimulationChannels {
    pub drone_event_listener: Receiver<DroneEvent>,
    pub drone_event_sender: Sender<DroneEvent>,
    pub leaf_event_listener: Receiver<LeafPacketSentEvent>,
    pub leaf_event_sender: Sender<LeafPacketSentEvent>,
}
