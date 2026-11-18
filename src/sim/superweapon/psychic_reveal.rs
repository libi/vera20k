//! The Psychic Reveal's launch: `SuperClass::Launch @ 0x006CC390` case 11
//! (`0x006CD70C..0x006CD7E7`).
//!
//! A charged Super (`+0x6F`) takes the target cell's GetCoords (vt+0x48,
//! `0x00486840`: its centre at its ground height, no bridge deck) and calls
//! `MapClass::RevealArea @ 0x005678E0` around it twice, with `[CombatDamage]
//! PsychicRevealRadius=` (`Rules+0xFEC`), the Super's house and no outline,
//! unreveal or line of sight: final 0 (`0x006CD773`), then final 1
//! (`0x006CD79C`) — [`Simulation::reveal_area`]. It then plays
//! `[AudioVisual] PsychicRevealActivateSound=` at the cell's coordinate
//! (`0x006CD7BF`). No EVA line, anim or radar event.
//!
//! RevealArea lifts the centre toward the map's north by the coordinate's
//! height, reveals nothing when the lifted centre is off the map `Size=`
//! diamond, skips cells off it, clamps the radius to 10, and writes the
//! Super's house's map and, under `[AudioVisual] AllyReveal=`, the map of
//! each house that house counts as allied.
//!
//! The local player's tail (`0x006CD7C4..0x006CD7E2`, then the shared
//! `0x006CD51E`) clears the selected Super and drops the queued
//! `EVA_PsychicRevealReady` line; the app does both from the launch event.
//!
//! Evidence: `tools/superweapon_oracle.py` section `psychic_launch` (Unicorn
//! on gamemd.exe: case 11, RevealArea a recorded stub), replayed in
//! `psychic_reveal_tests.rs`; RevealArea's walk and house gate against
//! `tools/spatial_oracle/reveal_area.py` in `vision/reveal_area_tests.rs`.
//!
//! RESIDUAL: a negative `PsychicRevealRadius=` indexes before RevealArea's
//! count table natively; VERA reveals nothing. No retail value is negative.
//!
//! ## With Ares
//!
//! Ares hands Type 11 to its PsychicReveal handler (HandlesType
//! `0x100752E0`), whose Activate (`0x10075150`) reveals instead of
//! RevealArea. Its MapRevealer (`0x100691C0`) takes the cell's GetCoords
//! raised by the bridge deck ([`super::deck_coords`]) and lifts it north by
//! its height ([`vision::conceal_area_centre`]); its house gate
//! (`0x10075182..0x100751C6`) is RevealArea's. Its lambda (`0x10074FF0`)
//! runs with final 0, then 1 (`0x100751DE`, `0x100751E9`), over GetRange
//! (`AresSuperWeaponLaunch::psychic_reveal_range`): a negative width maps
//! every allocated cell inside the `Size=` diamond; any other range maps the
//! cells of Helpers::Alex::for_each_in_rect_or_range (`0x10074E40`,
//! `0x10074C90`) inside the diamond, a rectangle whole and a radius strictly
//! within its width. Each cell goes to MapRevealer::Process1, RevealArea's
//! leaf ([`Simulation::reveal_map_revealer_cells`]): an `SW.Range` past 10
//! is not capped, and no line of sight applies.
//!
//! RESIDUAL: Ares then runs its common Launch tail (`0x10032ED0`, Flags 0:
//! nothing skipped) instead of the stock case's sound and player tail.
//! PsychicReveal is no `SuperWeaponKind::is_ares_redirect`, so VERA keeps
//! the stock ones: the launch count (`SW.Shots` never runs out), money,
//! `SW.Animation`, `SW.Sound` (`0x100752B0`), `EVA.Activated`,
//! `Message.Launch` and an explicit `SW.CreateRadarEvent=yes` (Initialize
//! `0x100752F0` defaults it off) play as stock.
//!
//! Ledger: no Scenario draws, timer writes or detach calls.
//!
//! ## Dependency rules
//! - Part of sim/ — depends on rules/, sim/vision, sim/world.
//! - sim/ NEVER depends on render/, ui/, audio/, net/.

#[cfg(all(test, tests_sim_superweapon))]
#[path = "psychic_reveal_tests.rs"]
mod tests;

use crate::rules::ruleset::RuleSet;
use crate::sim::intern::InternedId;
use crate::sim::vision;
use crate::sim::world::{SimSoundEvent, Simulation};

/// What [`launch`] called, in the oracle's terms (observation only).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Observed {
    /// RevealArea's pair of calls: the coordinate and the radius.
    RevealArea([i32; 3], u16),
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Observation only: what [`launch`] called on this thread while a test
    /// holds `Some`.
    static OBSERVED: std::cell::RefCell<Option<Vec<Observed>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, feature = "test-support"))]
fn observe(call: Observed) {
    OBSERVED.with(|log| {
        if let Some(log) = log.borrow_mut().as_mut() {
            log.push(call);
        }
    });
}

/// Launch case 11 for `owner`'s Super of type `sw_type` at (target_rx,
/// target_ry): see the module doc. Returns whether the Super was charged.
pub fn launch(
    sim: &mut Simulation,
    rules: &RuleSet,
    owner: InternedId,
    target_rx: u16,
    target_ry: u16,
    sw_type: InternedId,
) -> bool {
    if !super::is_charged(sim, owner, sw_type) {
        return false;
    }
    let ground = crate::sim::projectile::cell_ground_coord(
        sim.resolved_terrain.as_ref(),
        target_rx,
        target_ry,
    );
    let coord = [ground.x, ground.y, ground.z];
    let config = sim.sight_reveal_config(rules);
    let viewers = vision::direct_reveal_viewers(
        sim.house_alliances.as_map(),
        owner,
        &config,
        &sim.interner,
        &sim.fog_viewers(),
    );
    if rules.ares_form() {
        ares_reveal(sim, rules, &viewers, (target_rx, target_ry), sw_type);
    } else {
        let radius = rules
            .general
            .psychic_reveal_radius
            .clamp(0, i32::from(u16::MAX)) as u16;
        #[cfg(any(test, feature = "test-support"))]
        observe(Observed::RevealArea(coord, radius));
        for final_pass in [false, true] {
            let _ = sim.reveal_area(
                &viewers,
                (target_rx, target_ry),
                coord[2],
                radius,
                None,
                final_pass,
                &config,
                crate::sim::world::UninitContext::with_rules(rules),
                crate::sim::vision::MapRevealProjection::Transient,
                sim.session.current_house(),
            );
        }
    }
    // `0x006CD717..0x006CD737`, `0x006CD7A1..0x006CD7BF`: the cue plays at
    // the cell's GetCoords, the reveal's own centre.
    sim.sound_events.emit(SimSoundEvent::SuperWeaponLaunched {
        owner,
        sw_type,
        world: ground.into(),
    });
    true
}

/// Installed Ares: the reveal of `SW_PsychicReveal::Activate @ 0x10075150`
/// at `cell` (see the module doc).
fn ares_reveal(
    sim: &mut Simulation,
    rules: &RuleSet,
    viewers: &[InternedId],
    cell: (u16, u16),
    sw_type: InternedId,
) {
    let Some(metadata) = rules.super_weapon_metadata(sim.interner.resolve(sw_type)) else {
        return;
    };
    let (width, height) = metadata
        .launch()
        .psychic_reveal_range(rules.general.psychic_reveal_radius);
    // The lambda `0x10074FF0`: a negative width maps the whole map
    // (`0x1007501E`), anything else the area around the MapRevealer's base.
    let cells = if width < 0.0 {
        None
    } else {
        let base = vision::conceal_area_centre(super::deck_coords(sim, cell));
        let diamond = sim.map_size_diamond();
        let area =
            super::protect::area_cells(sim, (width, height), (base.0 as u16, base.1 as u16), false);
        Some(
            area.into_iter()
                .filter(|&(x, y)| {
                    // `0x10074D49..0x10074DE8`: inside the Size diamond, then
                    // a rectangle whole, else sqrt(dx*dx + dy*dy) < width,
                    // compared here squared (exact in f64).
                    let dx = f64::from(i32::from(x as i16) - i32::from(base.0));
                    let dy = f64::from(i32::from(y as i16) - i32::from(base.1));
                    diamond.is_none_or(|(w, h)| {
                        crate::map::playfield::size_diamond_contains(w, h, (x as i16, y as i16))
                    }) && (height > 0 || dx * dx + dy * dy < f64::from(width) * f64::from(width))
                })
                .collect::<Vec<_>>(),
        )
    };
    let player = sim.session.current_house();
    for final_pass in [false, true] {
        sim.reveal_map_revealer_cells(
            viewers,
            cells.as_deref(),
            final_pass,
            crate::sim::world::UninitContext::with_rules(rules),
            player,
        );
    }
}
