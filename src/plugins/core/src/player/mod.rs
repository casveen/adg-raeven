/**
 * Module for all player related plugins, types and systems
 *
 * PlayerController as input interpreter and main entry point to the module, which
 * then sends Trigger<_,_> that the other audio/visual systems will add_observer for.
 */
pub mod player_controller;
pub mod player_visuals;
pub mod states;