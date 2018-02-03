use na::{self, Real};

use joint::Joint;
use solver::IntegrationParameters;
use math::{Isometry, JacobianSliceMut, Translation, Vector, Velocity, DIM};

#[derive(Copy, Clone, Debug)]
pub struct CartesianJoint<N: Real> {
    position: Vector<N>,
}

impl<N: Real> CartesianJoint<N> {
    pub fn new(position: Vector<N>) -> Self {
        CartesianJoint { position }
    }
}

impl<N: Real> Joint<N> for CartesianJoint<N> {
    #[inline]
    fn ndofs(&self) -> usize {
        DIM
    }

    fn body_to_parent(&self, parent_shift: &Vector<N>, body_shift: &Vector<N>) -> Isometry<N> {
        let t = Translation::from_vector(parent_shift - body_shift + self.position);
        Isometry::from_parts(t, na::one())
    }

    fn update_jacobians(&mut self, _: &Vector<N>, _: &[N]) {}

    fn jacobian(&self, _: &Isometry<N>, out: &mut JacobianSliceMut<N>) {
        out.fill_diagonal(N::one())
    }

    fn jacobian_dot(&self, _: &Isometry<N>, _: &mut JacobianSliceMut<N>) {}

    fn jacobian_dot_veldiff_mul_coordinates(
        &self,
        _: &Isometry<N>,
        _: &[N],
        _: &mut JacobianSliceMut<N>,
    ) {

    }

    fn jacobian_mul_coordinates(&self, vels: &[N]) -> Velocity<N> {
        Velocity::new_with_vectors(Vector::from_row_slice(&vels[..DIM]), na::zero())
    }

    fn jacobian_dot_mul_coordinates(&self, _: &[N]) -> Velocity<N> {
        Velocity::zero()
    }

    fn apply_displacement(&mut self, params: &IntegrationParameters<N>, vels: &[N]) {
        self.position += Vector::from_row_slice(&vels[..DIM]) * params.dt;
    }
}