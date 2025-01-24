/**
 * Module for all player related plugins, types and systems
 * 
 * PlayerController as input interpreter and main entry point to the module, which 
 * then sends Trigger<_,_> that the other systems will add_observer for.
 */

pub mod player_controller;
pub mod graphical_component;