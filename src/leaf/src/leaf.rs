use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub trait Leaf: Send {
    fn new(
        id: NodeId,
        controller_send: Sender<LeafEvent>,
        controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}

#[derive(Debug, Clone)]
pub enum LeafEvent {
    PacketSend(Packet),
    // Used especially for FloodResponse but also
    // if all other methods of sending ack/nack fail
    ControllerShortcut(Packet),
}

#[derive(Debug, Clone)]
pub enum LeafCommand {
    RemoveSender(NodeId),
    AddSender(NodeId, Sender<Packet>),
    Kill, // Stop blocking the thread on which this leaf is run, used for testing only
}
