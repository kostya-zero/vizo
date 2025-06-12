//! Viz (derived from 'Visualize') is a modular CLI tool designed to visualize structured data formats such as JSON, TOML, and YAML in a more pretty way.
//! Its architecture supports customization, enabling users to incorporate specific processors during compilation to tailor functionality to their needs.

/// A global values for Viz.
pub mod values;

/// An arguments parser for Viz app.
pub mod args;

/// A terminal for Viz app to display data.
pub mod terminal;

/// A prints module for Viz app to display formatted data.
pub mod prints;

/// A processors for Viz to process data.
pub mod processors;
