#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use arbitrary::{Arbitrary, Result, Unstructured};

use knyst::{
    audio_backend::{CpalBackend, CpalBackendOptions},
    graph::{Gen, Mult},
    prelude::*,
    wavetable::{Wavetable, WavetableOscillatorOwned},
};
use std::time::Duration;

struct ArbitraryGen {
    num_inputs: usize,
    num_outputs: usize,
    gen_state: GenState,
}

impl ArbitraryGen {
    pub fn new(u: &mut Unstructured) -> Self {
        let states = vec![
            GenState::Continue,
            GenState::FreeSelf,
            GenState::FreeGraph(u.arbitrary().unwrap()),
            GenState::FreeGraphMendConnections(u.arbitrary().unwrap()),
        ];
        let choosen_state = *u.choose(&states).unwrap();
        ArbitraryGen {
            num_inputs: u.arbitrary().unwrap(),
            num_outputs: u.arbitrary().unwrap(),
            gen_state: choosen_state,
        }
    }
}

impl Gen for ArbitraryGen {
    fn process(&mut self, ctx: GenContext, resources: &mut Resources) -> GenState {
        todo!()
    }

    fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    fn num_outputs(&self) -> usize {
        self.num_outputs
    }
}

fuzz_target!(|data: &[u8]| {
    // Original test
    /*
       let mut backend = CpalBackend::new(CpalBackendOptions::default())?;

    let sample_rate = backend.sample_rate() as f32;
    let block_size = backend.block_size().unwrap_or(64);
    let resources = Resources::new(ResourcesSettings {
        sample_rate,
        ..Default::default()
    });
    let mut graph: Graph = Graph::new(GraphSettings {
        block_size,
        sample_rate,
        latency: Duration::from_millis(100),
        num_outputs: backend.num_outputs(),
        ..Default::default()
    });
    backend.start_processing(&mut graph, resources)?;
    let node0 = graph.push(WavetableOscillatorOwned::new(Wavetable::sine()));
    graph.connect(constant(440.).to(node0).to_label("freq"))?;
    let modulator = graph.push(WavetableOscillatorOwned::new(Wavetable::sine()));
    graph.connect(constant(5.).to(modulator).to_label("freq"))?;
    let mod_amp = graph.push(Mult);
    graph.connect(modulator.to(mod_amp))?;
    graph.connect(constant(0.25).to(mod_amp).to_index(1))?;
    let amp = graph.push(Mult);
    graph.connect(node0.to(amp))?;
    graph.connect(constant(0.5).to(amp).to_index(1))?;
    graph.connect(mod_amp.to(amp).to_index(1))?;
    graph.connect(amp.to_graph_out())?;
    graph.connect(amp.to_graph_out().to_index(1))?;
    graph.commit_changes();
    graph.update(); // Required because constant connections get converted to
                    // scheduled changes when the graph is running.
    ...
    */

    // Lets create the unstructured data instance
    let mut u = Unstructured::new(data);
    let mut backend = CpalBackend::new(CpalBackendOptions::default()).unwrap();

    let block_size: usize = u.arbitrary().unwrap();
    let block_size = block_size % 1000; // no more than 1000 nodes

    let sample_rate: f32 = u.arbitrary().unwrap();

    let resources = Resources::new(ResourcesSettings {
        sample_rate,
        ..Default::default()
    });

    let mut graph = Graph::new(GraphSettings {
        block_size,
        sample_rate,
        latency: u.arbitrary().unwrap(),
        num_outputs: backend.num_outputs(),
        max_node_inputs: 10000000000, // Big enough
        ..Default::default()
    });

    // Lets store the nodes to be able of connecting several
    let mut nodes = vec![];
    for i in 0..block_size {
        // Generate random node types

        let node = ArbitraryGen::new(&mut u);
        let nodeid = graph.push(node);

        // Save the node id
        nodes.push(nodeid);

        let mut node1 = u.choose(&nodes).unwrap();
        let mut node2 = u.choose(&nodes).unwrap();

        // Same node cannot be connected
        if node1 != node2 {
            // Connect two random nodes
            graph.connect(node1.to(*node2)).unwrap();
        }
        graph.commit_changes();
        graph.update();
    }
});
