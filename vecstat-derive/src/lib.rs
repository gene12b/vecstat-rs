use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(StatisticsUnorderedF32)]
pub fn statistics_unordered_f32_export_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let tokens = quote! {
        impl StatisticsUnorderedF32<#struct_name>  for Vec<#struct_name> {

        }
    };
    tokens.into()
}
#[proc_macro_derive(StatisticsUnorderedF64)]
pub fn statistics_unordered_f64_export_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let tokens = quote! {
        impl StatisticsUnorderedF64<#struct_name>  for Vec<#struct_name> {}
    };
    tokens.into()
}

#[proc_macro_derive(StatisticsOrdered)]
pub fn statistics_ordered_export_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let tokens = quote! {
        //pub use vecstat::StatisticsOrdered;
        impl<#struct_name> vecstat::StatisticsOrdered<#struct_name> for Vec<#struct_name>
        where
            #struct_name: Ord
        {
            fn min_stat(&self) -> Option<&#struct_name> {
                self.iter().min()
            }
            fn max_stat(&self) -> Option<&#struct_name> {
                self.iter().max()
            }
        }

    };
    tokens.into()
}

//StatisticsOrdered
