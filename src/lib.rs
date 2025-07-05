//! Vizo (derived from 'Visualize') is a modular CLI tool designed to visualize structured data formats such as JSON, TOML, and YAML in a more pretty way.
//! Its architecture supports customization, enabling users to incorporate specific processors during compilation to tailor functionality to their needs.

/// A global values for Vizo.
pub mod values;

/// An arguments parser for Vizo app.
pub mod args;

/// A prints module for Vizo app to display formatted data.
pub mod prints;

/// A processors for Vizo to process data.
pub mod processors;

// An application module.
pub mod app;
