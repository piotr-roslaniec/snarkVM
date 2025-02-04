// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<A: Aleo> ComputeKey<A> {
    /// Returns the account compute key for this account private key.
    pub fn from_private_key(private_key: &PrivateKey<A>) -> Self {
        // Extract (sk_sig, r_sig, sk_vrf).
        let (sk_sig, r_sig, sk_vrf) = (private_key.sk_sig(), private_key.r_sig(), private_key.sk_vrf());

        // Compute `pk_sig` := G^sk_sig.
        let pk_sig = A::g_scalar_multiply(sk_sig);
        // Compute `pr_sig` := G^r_sig.
        let pr_sig = A::g_scalar_multiply(r_sig);
        // Compute `pk_vrf` := G^sk_vrf.
        let pk_vrf = A::g_scalar_multiply(sk_vrf);
        // Compute `sk_prf` := RO(G^sk_sig || G^r_sig || G^sk_vrf).
        let sk_prf =
            A::hash_to_scalar_psd4(&[pk_sig.to_x_coordinate(), pr_sig.to_x_coordinate(), pk_vrf.to_x_coordinate()]);

        // Return the compute key.
        Self { pk_sig, pr_sig, pk_vrf, sk_prf }
    }
}

#[cfg(all(test, console))]
mod tests {
    use super::*;
    use crate::{helpers::generate_account, Circuit};

    use anyhow::Result;

    const ITERATIONS: u64 = 100;

    fn check_from_private_key(
        mode: Mode,
        num_constants: u64,
        num_public: u64,
        num_private: u64,
        num_constraints: u64,
    ) -> Result<()> {
        for i in 0..ITERATIONS {
            // Generate a private key, compute key, view key, and address.
            let (private_key, compute_key, _view_key, _address) = generate_account()?;

            // Retrieve the native private key components.
            let sk_sig = private_key.sk_sig();
            let r_sig = private_key.r_sig();
            let sk_vrf = private_key.sk_vrf();

            // Retrieve the native compute key components.
            let pk_sig = compute_key.pk_sig();
            let pr_sig = compute_key.pr_sig();
            let pk_vrf = compute_key.pk_vrf();
            let sk_prf = compute_key.sk_prf();

            // Initialize the private key.
            let private_key = PrivateKey::<Circuit>::new(mode, (sk_sig, r_sig, sk_vrf));

            Circuit::scope(&format!("{} {}", mode, i), || {
                let candidate = ComputeKey::from_private_key(&private_key);
                assert_eq!(pk_sig, candidate.pk_sig().eject_value());
                assert_eq!(pr_sig, candidate.pr_sig().eject_value());
                assert_eq!(pk_vrf, candidate.pk_vrf().eject_value());
                assert_eq!(sk_prf, candidate.sk_prf().eject_value());

                // TODO (howardwu): Resolve skipping the cost count checks for the burn-in round.
                if i > 0 {
                    assert_scope!(<=num_constants, num_public, num_private, num_constraints);
                }
            });
            Circuit::reset();
        }
        Ok(())
    }

    #[test]
    fn test_from_private_key_constant() -> Result<()> {
        check_from_private_key(Mode::Constant, 3254, 0, 0, 0)
    }

    #[test]
    fn test_from_private_key_public() -> Result<()> {
        check_from_private_key(Mode::Public, 1501, 0, 4348, 4349)
    }

    #[test]
    fn test_from_private_key_private() -> Result<()> {
        check_from_private_key(Mode::Private, 1501, 0, 4348, 4349)
    }
}
