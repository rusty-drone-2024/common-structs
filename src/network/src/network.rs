use crate::info::NodeInfo;
use crossbeam_channel::{Receiver, Sender};
use leaf::LeafEvent;
use std::collections::HashMap;
use wg_2024::controller::DroneEvent;
use wg_2024::network::NodeId;

pub struct Network {
    pub topology: HashMap<NodeId, NodeInfo>,
    pub simulation_channels: SimulationChannels,
}

pub struct SimulationChannels {
    // Used for comunication with SC
    pub drone_event_listener: Receiver<DroneEvent>,
    pub leaf_event_listener: Receiver<LeafEvent>,
    // Sent to the nodes of the network by the NI
    pub drone_event_sender: Sender<DroneEvent>,
    pub leaf_event_sender: Sender<LeafEvent>,
}
