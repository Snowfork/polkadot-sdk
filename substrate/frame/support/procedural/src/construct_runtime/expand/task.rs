// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License

use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Expands aggregate `RuntimeTask` enum.
pub fn expand_outer_task(pallet_decls: &[Pallet], scrate: &TokenStream2) -> TokenStream2 {
	let mut from_impls = Vec::new();
	let mut task_variants = Vec::new();
	for decl in pallet_decls {
		if let Some(_) = decl.find_part("Task") {
			let variant_name = &decl.name;
			let path = &decl.path;
			let index = decl.index;

			// Todo: Replace `Runtime` with the actual runtime ident
			// `pallet` will probably not be needed when `Task` is generated by macro

			from_impls.push(quote! {
				impl From<#path::pallet::Task<Runtime>> for RuntimeTask {
					fn from(hr: #path::pallet::Task<Runtime>) -> Self {
						RuntimeTask::#variant_name(hr)
					}
				}
			});

			task_variants.push(quote! {
				#[codec(index = #index)]
				#variant_name(#path::pallet::Task<Runtime>),
			});
		}
	}
	use quote::ToTokens;
	if !task_variants.is_empty() {
		println!(
			"{:#?}",
			task_variants
				.iter()
				.map(|item| item.to_token_stream().to_string())
				.collect::<Vec<_>>()
		);
	}

	let prelude = quote!(#scrate::traits::tasks::prelude);

	quote! {
		/// An aggregation of all `Task` enums across all pallets included in the current runtime.
		#[derive(
			Clone, Eq, PartialEq,
			#scrate::__private::codec::Encode, #scrate::__private::codec::Decode,
			// #scrate::__private::codec::MaxEncodedLen,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::RuntimeDebug,
		)]
		pub enum RuntimeTask {
			#( #task_variants )*
		}

		impl #scrate::traits::Task for RuntimeTask {
			type Enumeration = #prelude::IntoIter<#scrate::traits::Task<T>>;

			const TASK_INDEX: Option<u64> = None;

			fn is_valid(&self) -> bool {
				use #prelude::*;
				todo!();
			}

			fn run(&self) -> Result<(), #scrate::traits::tasks::prelude::DispatchError> {
				todo!();
			}

			fn weight(&self) -> #scrate::pallet_prelude::Weight {
				todo!();
			}
		}

		#( #from_impls )*
	}
}
