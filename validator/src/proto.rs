/*
 * Copyright 2018 Bitwise IO, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use std::ops::{Deref, DerefMut};

pub use sawtooth_protos::*;

pub struct StateChangeFfi(pub sawtooth_protos::transaction_receipt::StateChange);

pub struct EventFfi(pub sawtooth_protos::events::Event);

pub struct TransactionReceiptFfi(pub sawtooth_protos::transaction_receipt::TransactionReceipt);
