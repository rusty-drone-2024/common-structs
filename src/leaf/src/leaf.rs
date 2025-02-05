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
#[cfg_attr(feature = "debug", derive(PartialEq))]
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

#[cfg(feature = "debug")]
impl PartialEq for LeafCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LeafCommand::Kill, LeafCommand::Kill) => true,
            (LeafCommand::RemoveSender(id1), LeafCommand::RemoveSender(id2)) => id1 == id2,
            (LeafCommand::AddSender(id1, sen1), LeafCommand::AddSender(id2, sen2)) => {
                id1 == id2 && sen1.same_channel(sen2)
            }
            _ => false,
        }
    }
}
