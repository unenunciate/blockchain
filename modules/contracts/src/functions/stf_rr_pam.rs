// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of the Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use super::*;

impl<T: Trait> Module<T> {
    pub fn stf_rr_pam(event: Event, t0: &Time, mut contract: Contract) -> Contract {
        contract.states.accrued_interest = contract.states.accrued_interest
            + utilities::year_fraction(
                contract.states.status_date,
                event.time,
                contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
            ) * contract.states.nominal_interest_rate
                * contract.states.notional_principal;
        if contract.terms.fee_basis == Some(FeeBasis::N) {
            contract.states.fee_accrued = contract.states.fee_accrued
                + utilities::year_fraction(
                    contract.states.status_date,
                    event.time,
                    contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                ) * contract.states.notional_principal
                    * contract.terms.fee_rate;
        } else {
            let mut t_minus = Time(None);
            let mut t_plus = Time(None);
            for e in contract.schedule.clone() {
                if e.event_type == EventType::FP {
                    if e.time >= *t0 {
                        t_plus = e.time;
                        break;
                    }
                    t_minus = e.time;
                }
            }
            contract.states.fee_accrued = utilities::year_fraction(
                t_minus,
                event.time,
                contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
            ) / year_fraction(
                t_minus,
                t_plus,
                contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
            ) * utilities::contract_role_sign(
                contract.terms.contract_role,
            ) * contract.terms.fee_rate;
        }
        let delta_r = Real::min(
            Real::max(
                <oracle::Module<T>>::oracles(
                    contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                )
                .value
                    * contract.terms.rate_multiplier
                    + contract.terms.rate_spread
                    - contract.states.nominal_interest_rate,
                contract.terms.period_floor,
            ),
            contract.terms.period_cap,
        );
        contract.states.nominal_interest_rate = Real::min(
            Real::max(
                contract.states.nominal_interest_rate + delta_r,
                contract.terms.life_floor,
            ),
            contract.terms.life_cap,
        );
        contract.states.status_date = event.time;
        // Return the progressed contract state
        contract
    }
}