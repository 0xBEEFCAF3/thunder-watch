use bitcoin::blockdata::constants::genesis_block;
use bitcoin::secp256k1::{PublicKey, SecretKey};
use lightning::chain;
use lightning::routing::gossip::{NetworkGraph, P2PGossipSync};
use rand::{thread_rng, Rng, RngCore};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    convert::Infallible,
    io,
    ops::Deref,
    sync::{Arc, Mutex},
};

use lightning::{
    ln::{
        msgs::{self, LightningError, RoutingMessageHandler},
        peer_handler::{
            CustomMessageHandler, ErroringMessageHandler, IgnoringMessageHandler, MessageHandler,
            PeerManager, SocketDescriptor,
        },
        wire,
    },
    util::{
        events::{MessageSendEvent, MessageSendEventsProvider},
        logger::{Level, Logger, Record},
    },
};

use crate::disk::FilesystemLogger;

mod disk;

fn main() {
    let mut key = [0; 32];
    let mut random_data = [0; 32];
    thread_rng().fill_bytes(&mut key);
    thread_rng().fill_bytes(&mut random_data);
    let our_node_secret = SecretKey::from_slice(&key).unwrap();

    let ignorer = IgnoringMessageHandler {};
    let arc_ignorer = Arc::new(ignorer);

    let errorer = ErroringMessageHandler::new();
    let arc_errorer = Arc::new(errorer);

    // let mut logger = CasaLogger::new();
    let logger = Arc::new(FilesystemLogger::new("/data".to_string().clone()));
    // logger.enable(Level::Debug);
    // logger.enable(Level::Warn);

    let mut node_last_response_times: HashMap<PublicKey, SystemTime> = HashMap::new();

    struct CustomHandler {}

    impl CustomMessageHandler for CustomHandler {
        fn handle_custom_message(
            &self,
            _msg: Infallible,
            _sender_node_id: &PublicKey,
        ) -> Result<(), LightningError> {
            // node_last_response_times.insert();
            // Since we always return `None` in the read the handle method should never be called.
            unreachable!();
        }

        fn get_and_clear_pending_msg(&self) -> Vec<(PublicKey, Self::CustomMessage)> {
            Vec::new()
        }
    }

    impl wire::CustomMessageReader for CustomHandler {
        type CustomMessage = Infallible;
        fn read<R: io::Read>(
            &self,
            _message_type: u16,
            _buffer: &mut R,
        ) -> Result<Option<Self::CustomMessage>, msgs::DecodeError> {
            Ok(None)
        }
    }

    impl Deref for CustomHandler {
        type Target = CustomHandler;
        fn deref(&self) -> &Self {
            self
        }
    }

    // let genesis = genesis_block(bitcoin::Network::Bitcoin).header.block_hash();
    // let network_graph = Arc::new(NetworkGraph::new(genesis, logger.clone()));

    // let gossip_sync = Arc::new(P2PGossipSync::new(
    //     Arc::clone(&network_graph),
    //     None::<Arc<dyn chain::Access + Send + Sync>>,
    //     logger.clone(),
    // ));

    // Create a dummmy message handler. We don't care about channel updates or gossip
    let message_handler = MessageHandler {
        chan_handler: arc_errorer,
        route_handler: arc_ignorer,
    };
    let custom_handler = CustomHandler {};

    let peer_handler = Arc::new(PeerManager::new(
        message_handler,
        our_node_secret,
        &random_data,
        logger.clone(),
        custom_handler,
    ));

    let arc_peer_handler = Arc::new(peer_handler);

    println!("Hello, world!");
}
