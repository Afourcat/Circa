// Circa, efficient logic simulator
// Copyright (C) 2019 Lorenzo Cecchini
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod tristate;
pub use tristate::Tristate;

mod component;
pub use component::Component;

mod net;
pub use net::Net;

mod error;
pub use error::{LogicResult, LogicError};
