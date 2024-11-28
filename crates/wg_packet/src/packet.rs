use crate::{FloodRequest, FloodResponse};
use wg_network::{NodeId, SourceRoutingHeader};

// Is atomic unit to be sent
#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub pack_type: PacketType,
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketType {
    MsgFragment(Fragment),
    Nack(Nack),
    Ack(Ack),
    FloodRequest(FloodRequest),
    FloodResponse(FloodResponse),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nack {
    pub fragment_index: u64,
    pub nack_type: NackType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
    UnexpectedRecipient(NodeId),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Ack {
    pub fragment_index: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub length: u8,
    pub data: [u8; 80],
}
