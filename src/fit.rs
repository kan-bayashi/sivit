// Copyright 2025 Tomoki Hayashi
// MIT License (https://opensource.org/licenses/MIT)

//! Fit mode selection for resizing.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FitMode {
    Normal,
    Fit,
}

impl FitMode {
    /// Toggle between `Normal` and `Fit`.
    pub fn next(self) -> Self {
        match self {
            FitMode::Normal => FitMode::Fit,
            FitMode::Fit => FitMode::Normal,
        }
    }
}
