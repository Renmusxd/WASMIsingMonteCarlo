extern crate console_error_panic_hook;
use ising_monte_carlo;
use ising_monte_carlo::sse::qmc_graph::DefaultQMCGraph;
use ising_monte_carlo::sse::qmc_traits::{LoopUpdater, OpContainer};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub struct Lattice {
    qmc_graph: DefaultQMCGraph<OsRng>,
    beta: f64,
}

/// A lattice for running monte carlo simulations. Takes a list of edges: ((a, b), j), ...
#[wasm_bindgen]
impl Lattice {
    /// Make a new lattice with `edges`, positive implies antiferromagnetic bonds, negative is
    /// ferromagnetic.
    pub fn new(
        edge_a: &[usize],
        edge_b: &[usize],
        edge_j: &[f64],
        transverse: f64,
        beta: f64,
    ) -> Result<Lattice, JsValue> {
        if transverse <= 0.0 {
            Err(JsValue::from_str(
                "Transverse field must be greater than 0.0",
            ))
        } else {
            let nvars = edge_a.iter().chain(edge_b.iter()).max().map(|n| *n+1);
            if let Some(nvars) = nvars {
                let edges = edge_a
                    .iter()
                    .cloned()
                    .zip(edge_b.iter().cloned())
                    .zip(edge_j.iter().cloned())
                    .collect();

                let mut rng = OsRng::default();
                rng.gen_bool(0.5);
                let qmc_graph = DefaultQMCGraph::<OsRng>::new_with_rng(
                    edges,
                    transverse,
                    nvars,
                    false,
                    false,
                    rng,
                    None,
                );
                Ok(Lattice { qmc_graph, beta })
            } else {
                Err(JsValue::from_str("Must supply some edges for graph"))
            }
        }
    }

    /// Run a quantum monte carlo simulation, return the average energy.
    ///
    /// # Arguments:
    /// * `timesteps`: number of timesteps to run.
    ///
    /// # Returns:
    ///  * average energy of system
    pub fn run_quantum_monte_carlo(&mut self, timesteps: usize) -> f64 {
        self.qmc_graph.timesteps(timesteps, self.beta)
    }

    pub fn get_nvars(&self) -> usize {
        self.qmc_graph.get_nvars()
    }

    pub fn get_state(&self, i: usize) -> bool {
        self.qmc_graph.state_ref()[i]
    }

    pub fn get_n(&self) -> usize {
        self.qmc_graph.get_manager_ref().get_n()
    }

    pub fn get_cutoff(&self) -> usize { self.qmc_graph.get_cutoff() }

    pub fn get_p_for_n(&self, n: usize) -> usize {
        self.qmc_graph.get_manager_ref().get_nth_p(n)
    }

    pub fn get_nvars_for_op(&self, p: usize) -> Option<usize> {
        let op = self.qmc_graph.get_manager_ref().get_pth(p)?;
        Some(op.vars.len())
    }

    pub fn get_nth_op_var_i(&self, p: usize, i: usize) -> Option<usize> {
        let op = self.qmc_graph.get_manager_ref().get_pth(p)?;
        if i < op.vars.len() {
            Some(op.vars[i])
        } else {
            None
        }
    }

    pub fn get_nth_op_var_i_input(&self, p: usize, i: usize) -> Option<bool> {
        let op = self.qmc_graph.get_manager_ref().get_pth(p)?;
        if i < op.vars.len() {
            Some(op.inputs[i])
        } else {
            None
        }
    }

    pub fn get_nth_op_var_i_output(&self, p: usize, i: usize) -> Option<bool> {
        let op = self.qmc_graph.get_manager_ref().get_pth(p).unwrap();
        if i < op.vars.len() {
            Some(op.outputs[i])
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}